# Project:   ci-test-manifests
# File:      Dockerfile
# Purpose:   Container image for the ci-test-manifests CLI (Path A hadolint fixture)
# License:   FSL-1.1-ALv2
# Copyright: (c) 2026 HYPERI PTY LIMITED
#
# Multi-stage, non-root, clean at hadolint ERROR severity. Warning-tier
# findings (DL3013 unpinned pip) are expected and are exactly the droast /
# advisory surface this fixture exercises - do not "fix" them.

# -- Build stage ---------------------------------------------------------------
FROM python:3.12-slim AS builder

ENV PIP_NO_CACHE_DIR=1 \
    PIP_DISABLE_PIP_VERSION_CHECK=1 \
    PYTHONDONTWRITEBYTECODE=1

WORKDIR /build

# Copy manifests first for layer-cache efficiency; source after, so editing
# the app code does not bust the (heavier) build-dependency layer.
COPY pyproject.toml VERSION ./
COPY src/ ./src/

# Build a wheel and install it into a venv we copy into the runtime stage.
RUN pip install --no-cache-dir build \
    && python -m build --wheel --outdir dist . \
    && python -m venv /opt/venv \
    && /opt/venv/bin/pip install --no-cache-dir dist/*.whl

# -- Runtime stage -------------------------------------------------------------
FROM python:3.12-slim

# Inherit the venv from the builder; the CLI lands at
# /opt/venv/bin/ci-test-manifests.
COPY --from=builder /opt/venv /opt/venv
ENV PATH="/opt/venv/bin:${PATH}"

# Run as a non-root user (uid 10001).
RUN useradd --create-home --uid 10001 --shell /bin/bash appuser
USER appuser
WORKDIR /home/appuser

ENTRYPOINT ["ci-test-manifests"]
