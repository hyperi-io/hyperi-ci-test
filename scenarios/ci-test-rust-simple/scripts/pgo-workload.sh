#!/usr/bin/env bash
# Project:      ci-test-rust-simple
# File:         scripts/pgo-workload.sh
# Purpose:      PGO/BOLT profiling workload for the hyperi-ci Tier 2 pipeline
# License:      BUSL-1.1
# Copyright:    (c) 2026 HYPERI PTY LIMITED
#
# hyperi-ci runs this twice per target -- once against the PGO-instrumented
# binary, once against the BOLT-instrumented one -- passing the binary as $1.
# Contract and the four rules: hyperi-ci docs/runtime/pgo-bolt.md.
#
# This fixture has no broker and no database on purpose: a CI job has 4 GB and
# a workload that needs a container cannot prove the Tier 2 path inside it. The
# binary generates its own traffic mix, so the profile records parsing,
# validation, the slow-record branch and serialisation, never startup.
#
# Env overrides:
#   PGO_WORKLOAD_DURATION_SECS  seconds to drive the pipeline (hyperi-ci sets it)
#   PGO_WORKLOAD_SEED           traffic-mix seed, fixed so runs replay

set -euo pipefail

if [[ $# -lt 1 ]]; then
    echo "usage: $0 <path-to-instrumented-binary>" >&2
    exit 1
fi
BINARY="$1"
[[ -x "$BINARY" ]] || { echo "error: $BINARY is not executable" >&2; exit 1; }

DURATION="${PGO_WORKLOAD_DURATION_SECS:-300}"
SEED="${PGO_WORKLOAD_SEED:-1700000000}"

# 60s is the floor in Rule 3, and nothing in hyperi-ci enforces it for us.
if (( DURATION < 60 )); then
    echo "pgo-workload: raising duration ${DURATION}s to the 60s floor" >&2
    DURATION=60
fi

echo "pgo-workload: binary=$BINARY duration=${DURATION}s seed=${SEED}"

# One process for the whole run, and it self-terminates: the instrumented
# binary writes its .profraw at exit, so a killed process profiles nothing.
# The traffic mix (60% syslog, 25% key=value, 10% JSON, 5% malformed) lives
# inside the binary, which keeps one profile per run and one run per script.
"$BINARY" bench "$DURATION" "$SEED"

echo "pgo-workload: done"
