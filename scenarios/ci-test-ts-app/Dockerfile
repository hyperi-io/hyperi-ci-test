# Project:   ci-test-ts-app
# File:      Dockerfile
# Purpose:   Container image for ci-test-ts-app (multi-arch, Node 22)
# License:   FSL-1.1-ALv2
# Copyright: (c) 2026 HYPERI PTY LIMITED

# -- Build stage ---------------------------------------------------------------
FROM node:24-alpine AS builder

WORKDIR /build

# Copy manifests first for layer-cache efficiency
COPY package.json package-lock.json* tsconfig.json ./
RUN npm ci --no-audit --no-fund 2>/dev/null || npm install --no-audit --no-fund

# Copy source + build
COPY src/ ./src/
RUN npm run build

# -- Runtime stage -------------------------------------------------------------
FROM node:24-alpine

# Reproducible non-root user
RUN addgroup -g 10001 -S app && adduser -u 10001 -S app -G app

WORKDIR /app

# Copy production-only deps + compiled JS
COPY --from=builder --chown=app:app /build/package.json ./
COPY --from=builder --chown=app:app /build/package-lock.json* ./
COPY --from=builder --chown=app:app /build/dist/ ./dist/

# Install only prod deps in the runtime stage. Currently the app has no
# runtime dependencies (only Node stdlib), but this stays here so future
# deps land cleanly without re-templating.
RUN npm ci --omit=dev --no-audit --no-fund 2>/dev/null \
    || npm install --omit=dev --no-audit --no-fund

USER app:app

ENV PORT=8080
EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD node -e "fetch('http://127.0.0.1:'+process.env.PORT+'/healthz').then(r=>r.ok?process.exit(0):process.exit(1)).catch(()=>process.exit(1))"

ENTRYPOINT ["node", "dist/server.js"]
