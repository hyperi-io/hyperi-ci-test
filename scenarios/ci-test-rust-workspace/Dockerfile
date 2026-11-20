# Project:   ci-test-rust-workspace
# File:      Dockerfile
# Purpose:   Runtime container image — consumes pre-built binary from
#            hyperi-ci's Build stage (dist/) instead of compiling
#            from source. Mirrors ci-test-rust-app + dfe-loader pattern.
# License:   FSL-1.1-ALv2
# Copyright: (c) 2026 HYPERI PTY LIMITED
#
# Build context: hyperi-ci's Container job runs after Build and uses
# actions/download-artifact to populate dist/ with cross-compiled
# binaries named:
#
#   dist/ci-test-rust-workspace-linux-amd64
#   dist/ci-test-rust-workspace-linux-arm64
#
# buildx multi-arch sets TARGETARCH per platform run; we COPY the
# matching binary in. No Rust toolchain needed in the image.
#
# Workspace member that produces the binary is crates/cli with bin
# name `ci-test-rust-workspace`. No rdkafka in this workspace so the
# runtime base only needs ca-certificates + libssl3.

FROM ubuntu:24.04

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
       ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

# buildx exposes TARGETARCH (amd64 | arm64) per platform run. Pick the
# pre-built cross-compiled binary that matches.
ARG TARGETARCH
COPY dist/ci-test-rust-workspace-linux-${TARGETARCH} /usr/local/bin/ci-test-rust-workspace
RUN chmod +x /usr/local/bin/ci-test-rust-workspace

# Ubuntu 24.04 ships with a default `ubuntu` user at uid 1000 — remove
# before adding our nonroot appuser.
RUN userdel -r ubuntu 2>/dev/null || true \
    && useradd --create-home --uid 10001 appuser

USER appuser

HEALTHCHECK --interval=30s --timeout=3s --start-period=2s --retries=2 \
    CMD ci-test-rust-workspace || exit 1

ENTRYPOINT ["ci-test-rust-workspace"]
