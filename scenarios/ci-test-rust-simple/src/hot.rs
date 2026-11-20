//! Project:      ci-test-rust-simple
//! File:         src/hot.rs
//! Purpose:      Parse / transform / serialise hot path the PGO workload drives
//! Language:     Rust
//!
//! License:      BUSL-1.1
//! Copyright:    (c) 2026 HYPERI PTY LIMITED
//!
//! A PGO profile is only as good as the code it records, and a CLI whose whole
//! body is `println!` records nothing but startup. This module gives the fixture
//! the shape every DFE Rust app has -- parse, validate, transform, serialise --
//! driven from synthetic input so the workload needs no broker, no database and
//! no network.

use std::fmt::Write as _;
use std::time::{Duration, Instant};

/// Severity carried by a parsed line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Level {
    fn parse(token: &str) -> Option<Self> {
        match token {
            "TRACE" | "trace" => Some(Self::Trace),
            "DEBUG" | "debug" => Some(Self::Debug),
            "INFO" | "info" => Some(Self::Info),
            "WARN" | "warn" => Some(Self::Warn),
            "ERROR" | "error" => Some(Self::Error),
            _ => None,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        }
    }
}

/// One parsed line, borrowing its text fields from the input buffer.
struct Record<'a> {
    timestamp: u64,
    level: Level,
    service: &'a str,
    latency_us: u32,
    message: &'a str,
}

/// Why a line did not parse. Production sees these rarely, so the workload does too.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParseError {
    TooShort,
    BadTimestamp,
    UnknownLevel,
    MissingField,
}

const SERVICES: [&str; 6] = [
    "receiver",
    "loader",
    "archiver",
    "fetcher",
    "engine",
    "transform",
];

const MESSAGES: [&str; 6] = [
    "batch accepted",
    "flush completed",
    "upstream retry scheduled",
    "partition rebalanced",
    "checkpoint written",
    "backpressure released",
];

const LEVEL_TOKENS: [&str; 5] = ["INFO", "INFO", "DEBUG", "WARN", "ERROR"];

/// Deterministic xorshift64*, so the same seed replays the same traffic mix.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed | 1)
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, bound: u32) -> u32 {
        ((self.next_u64() >> 32) as u32) % bound
    }
}

/// Line shapes in the proportions the profile should reflect: 60% syslog-style,
/// 25% key=value, 10% JSON, 5% malformed.
fn generate(rng: &mut Rng, seq: u64, out: &mut String) {
    out.clear();
    let service = SERVICES[rng.below(SERVICES.len() as u32) as usize];
    let message = MESSAGES[rng.below(MESSAGES.len() as u32) as usize];
    let level = LEVEL_TOKENS[rng.below(LEVEL_TOKENS.len() as u32) as usize];
    let latency = rng.below(25_000);
    let ts = 1_700_000_000_u64.wrapping_add(seq);

    match rng.below(100) {
        0..=59 => {
            let _ = write!(out, "{ts} {service} {level} {latency} {message}");
        }
        60..=84 => {
            let _ = write!(
                out,
                "ts={ts} svc={service} level={level} latency_us={latency} msg={message}"
            );
        }
        85..=94 => {
            let _ = write!(
                out,
                r#"{{"ts":{ts},"svc":"{service}","level":"{level}","latency_us":{latency},"msg":"{message}"}}"#
            );
        }
        _ => {
            let _ = write!(out, "{ts} {service}");
        }
    }
}

fn parse(line: &str) -> Result<Record<'_>, ParseError> {
    if line.len() < 24 {
        return Err(ParseError::TooShort);
    }
    if line.starts_with('{') {
        parse_json(line)
    } else if line.starts_with("ts=") {
        parse_kv(line)
    } else {
        parse_syslog(line)
    }
}

fn parse_syslog(line: &str) -> Result<Record<'_>, ParseError> {
    let mut fields = line.splitn(5, ' ');
    let timestamp = fields
        .next()
        .ok_or(ParseError::MissingField)?
        .parse::<u64>()
        .map_err(|_| ParseError::BadTimestamp)?;
    let service = fields.next().ok_or(ParseError::MissingField)?;
    let level = Level::parse(fields.next().ok_or(ParseError::MissingField)?)
        .ok_or(ParseError::UnknownLevel)?;
    let latency_us = fields
        .next()
        .ok_or(ParseError::MissingField)?
        .parse::<u32>()
        .map_err(|_| ParseError::MissingField)?;
    let message = fields.next().ok_or(ParseError::MissingField)?;

    Ok(Record {
        timestamp,
        level,
        service,
        latency_us,
        message,
    })
}

fn parse_kv(line: &str) -> Result<Record<'_>, ParseError> {
    let mut timestamp = None;
    let mut service = None;
    let mut level = None;
    let mut latency_us = None;
    let mut message = None;

    for pair in line.split(' ') {
        let Some((key, value)) = pair.split_once('=') else {
            continue;
        };
        match key {
            "ts" => timestamp = value.parse::<u64>().ok(),
            "svc" => service = Some(value),
            "level" => level = Level::parse(value),
            "latency_us" => latency_us = value.parse::<u32>().ok(),
            "msg" => message = Some(value),
            _ => {}
        }
    }

    Ok(Record {
        timestamp: timestamp.ok_or(ParseError::BadTimestamp)?,
        level: level.ok_or(ParseError::UnknownLevel)?,
        service: service.ok_or(ParseError::MissingField)?,
        latency_us: latency_us.ok_or(ParseError::MissingField)?,
        message: message.ok_or(ParseError::MissingField)?,
    })
}

/// Flat-object reader, not a general JSON parser -- the fixture generates the
/// shape it reads, and a dependency would change what this fixture tests.
fn parse_json(line: &str) -> Result<Record<'_>, ParseError> {
    let body = line
        .strip_prefix('{')
        .and_then(|rest| rest.strip_suffix('}'))
        .ok_or(ParseError::MissingField)?;

    let mut timestamp = None;
    let mut service = None;
    let mut level = None;
    let mut latency_us = None;
    let mut message = None;

    for member in body.split(',') {
        let Some((key, value)) = member.split_once(':') else {
            return Err(ParseError::MissingField);
        };
        let key = key.trim_matches('"');
        let value = value.trim_matches('"');
        match key {
            "ts" => timestamp = value.parse::<u64>().ok(),
            "svc" => service = Some(value),
            "level" => level = Level::parse(value),
            "latency_us" => latency_us = value.parse::<u32>().ok(),
            "msg" => message = Some(value),
            _ => {}
        }
    }

    Ok(Record {
        timestamp: timestamp.ok_or(ParseError::BadTimestamp)?,
        level: level.ok_or(ParseError::UnknownLevel)?,
        service: service.ok_or(ParseError::MissingField)?,
        latency_us: latency_us.ok_or(ParseError::MissingField)?,
        message: message.ok_or(ParseError::MissingField)?,
    })
}

/// A slow record is the one an operator wants flagged, so the transform is the
/// branch the profile should learn.
fn render(record: &Record<'_>, out: &mut String) {
    out.clear();
    let slow = record.latency_us > 10_000;
    let _ = write!(
        out,
        r#"{{"ts":{},"level":"{}","svc":"{}","latency_us":{},"slow":{},"msg":"{}"}}"#,
        record.timestamp,
        record.level.as_str(),
        record.service,
        record.latency_us,
        slow,
        record.message
    );
}

/// FNV-1a over the rendered output, so nothing above can be optimised away.
fn checksum(bytes: &[u8], mut acc: u64) -> u64 {
    for byte in bytes {
        acc ^= u64::from(*byte);
        acc = acc.wrapping_mul(0x0000_0100_0000_01b3);
    }
    acc
}

/// What one workload run processed.
pub struct Stats {
    pub elapsed: Duration,
    pub parsed: u64,
    pub rejected: u64,
    pub bytes_in: u64,
    pub bytes_out: u64,
    pub checksum: u64,
}

/// Clock reads are a syscall, so the deadline is checked once per batch.
const BATCH: u64 = 4_096;

/// Drive the pipeline until `duration` elapses.
#[must_use]
pub fn run(duration: Duration, seed: u64) -> Stats {
    let mut rng = Rng::new(seed);
    let mut input = String::with_capacity(256);
    let mut output = String::with_capacity(256);
    let mut stats = Stats {
        elapsed: Duration::ZERO,
        parsed: 0,
        rejected: 0,
        bytes_in: 0,
        bytes_out: 0,
        checksum: 0xcbf2_9ce4_8422_2325,
    };

    let started = Instant::now();
    let mut seq = 0_u64;
    loop {
        for _ in 0..BATCH {
            generate(&mut rng, seq, &mut input);
            seq += 1;
            stats.bytes_in += input.len() as u64;
            match parse(&input) {
                Ok(record) => {
                    render(&record, &mut output);
                    stats.bytes_out += output.len() as u64;
                    stats.checksum = checksum(output.as_bytes(), stats.checksum);
                    stats.parsed += 1;
                }
                Err(_) => stats.rejected += 1,
            }
        }
        stats.elapsed = started.elapsed();
        if stats.elapsed >= duration {
            break;
        }
    }

    stats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syslog_lines_parse() {
        let record = parse("1700000001 loader INFO 4200 batch accepted").expect("valid syslog");
        assert_eq!(record.timestamp, 1_700_000_001);
        assert_eq!(record.level, Level::Info);
        assert_eq!(record.service, "loader");
        assert_eq!(record.latency_us, 4_200);
        assert_eq!(record.message, "batch accepted");
    }

    #[test]
    fn kv_lines_parse() {
        let line = "ts=1700000002 svc=receiver level=WARN latency_us=19000 msg=flush completed";
        let record = parse(line).expect("valid kv");
        assert_eq!(record.level, Level::Warn);
        assert_eq!(record.latency_us, 19_000);
    }

    #[test]
    fn json_lines_parse() {
        let line = r#"{"ts":1700000003,"svc":"engine","level":"ERROR","latency_us":31,"msg":"x"}"#;
        let record = parse(line).expect("valid json");
        assert_eq!(record.level, Level::Error);
        assert_eq!(record.service, "engine");
    }

    #[test]
    fn short_lines_are_rejected() {
        assert_eq!(parse("1700000004 x").err(), Some(ParseError::TooShort));
    }

    #[test]
    fn the_slow_flag_follows_the_latency() {
        let record = parse("1700000005 fetcher INFO 20000 checkpoint written").expect("valid");
        let mut out = String::new();
        render(&record, &mut out);
        assert!(out.contains(r#""slow":true"#));
    }

    #[test]
    fn a_run_parses_most_of_what_it_generates() {
        let stats = run(Duration::from_millis(50), 7);
        assert!(stats.parsed > 0);
        assert!(stats.parsed > stats.rejected * 4);
    }
}
