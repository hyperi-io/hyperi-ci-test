# Project:   ci-test-go-app
# File:      Dockerfile
# Purpose:   Container image for ci-test-go-app (multi-arch)
# License:   FSL-1.1-ALv2
# Copyright: (c) 2026 HYPERI PTY LIMITED

# -- Build stage ---------------------------------------------------------------
FROM golang:1.26-alpine AS builder

WORKDIR /build

# Copy module manifests first for layer caching
COPY go.mod go.sum* ./
RUN go mod download 2>/dev/null || true

# Copy source
COPY cmd/ ./cmd/
COPY internal/ ./internal/

ARG TARGETARCH=amd64
ARG VERSION=dev

# Static, stripped binary
RUN CGO_ENABLED=0 GOOS=linux GOARCH=${TARGETARCH} \
    go build \
        -trimpath \
        -ldflags "-s -w -X main.version=${VERSION}" \
        -o /build/ci-test-go-app \
        ./cmd/ci-test-go-app

# -- Runtime stage -------------------------------------------------------------
FROM gcr.io/distroless/static-debian12:nonroot

COPY --from=builder /build/ci-test-go-app /usr/local/bin/ci-test-go-app

USER nonroot:nonroot

ENTRYPOINT ["/usr/local/bin/ci-test-go-app"]
CMD ["-version"]
