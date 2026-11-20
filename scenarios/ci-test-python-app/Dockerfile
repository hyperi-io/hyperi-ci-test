# Project:   ci-test-python-app
# File:      Dockerfile
# Purpose:   Container image for the ci-test-python-app CLI
# License:   FSL-1.1-ALv2
# Copyright: (c) 2026 HYPERI PTY LIMITED

# -- Build stage ---------------------------------------------------------------
FROM python:3.12-slim AS builder

ENV PIP_NO_CACHE_DIR=1 \
    PIP_DISABLE_PIP_VERSION_CHECK=1 \
    PYTHONDONTWRITEBYTECODE=1

WORKDIR /build

# Copy manifests first for layer-cache efficiency
COPY pyproject.toml VERSION ./
COPY src/ ./src/

# Build a wheel; install into a venv we'll copy into the runtime stage
RUN pip install --no-cache-dir build \
    && python -m build --wheel --outdir dist . \
    && python -m venv /opt/venv \
    && /opt/venv/bin/pip install --no-cache-dir dist/*.whl

# -- Runtime stage -------------------------------------------------------------
FROM python:3.12-slim

# Inherit virtualenv from builder; CLI binary is at /opt/venv/bin/ci-test-python-app
COPY --from=builder /opt/venv /opt/venv
ENV PATH="/opt/venv/bin:${PATH}"

# Non-root by default
RUN useradd --create-home --uid 10001 --shell /bin/bash appuser
USER appuser
WORKDIR /home/appuser

# Smoke check: the scalo ServiceApp `version` subcommand must exit 0
HEALTHCHECK --interval=30s --timeout=3s --start-period=2s --retries=2 \
    CMD ci-test-python-app version || exit 1

ENTRYPOINT ["ci-test-python-app"]
CMD ["version"]
