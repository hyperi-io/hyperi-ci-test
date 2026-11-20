//! Project:      ci-test-rust-simple
//! File:         src/main.rs
//! Purpose:      Minimal CLI entry point with --version, --help and bench support
//! Language:     Rust
//!
//! License:      BUSL-1.1
//! Copyright:    (c) 2026 HYPERI PTY LIMITED

mod hot;

use std::time::Duration;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Matches the hyperi-ci workload floor in docs/runtime/pgo-bolt.md Rule 3.
const DEFAULT_BENCH_SECS: u64 = 60;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--version" | "-V" => {
                println!("{NAME} {VERSION}");
            }
            "--help" | "-h" => {
                println!("Usage: {NAME} [OPTIONS]");
                println!();
                println!("A minimal Rust CLI for testing HyperI CI pipelines.");
                println!();
                println!("Options:");
                println!("  -h, --help       Print help information");
                println!("  -V, --version    Print version information");
                println!();
                println!("Commands:");
                println!("  bench [SECS] [SEED]   Drive the parse/transform/serialise");
                println!("                        pipeline for SECS seconds. This is the");
                println!("                        hot path scripts/pgo-workload.sh profiles.");
            }
            "bench" => run_bench(&args),
            other => {
                eprintln!("error: unknown option '{other}'");
                eprintln!("Usage: {NAME} [OPTIONS]");
                std::process::exit(1);
            }
        }
    } else {
        println!("Hello from {NAME}!");
    }
}

/// A bad number falls back to the default rather than exiting: the PGO workload
/// aborts the release build on a non-zero exit, and an argument typo is not a
/// reason to lose the profile.
fn run_bench(args: &[String]) {
    let secs = args
        .get(2)
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(DEFAULT_BENCH_SECS);
    let seed = args
        .get(3)
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(0x5eed_1234);

    println!("bench: driving the record pipeline for {secs}s (seed {seed})");
    let stats = hot::run(Duration::from_secs(secs), seed);

    let elapsed = stats.elapsed.as_secs_f64();
    let rate = if elapsed > 0.0 {
        (stats.parsed + stats.rejected) as f64 / elapsed
    } else {
        0.0
    };
    println!(
        "bench: {:.1}s elapsed, {} parsed, {} rejected, {:.0} records/s",
        elapsed, stats.parsed, stats.rejected, rate
    );
    println!(
        "bench: {} bytes in, {} bytes out, checksum {:#018x}",
        stats.bytes_in, stats.bytes_out, stats.checksum
    );
}
