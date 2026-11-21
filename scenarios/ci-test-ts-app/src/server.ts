/*
 * Project:    ci-test-ts-app
 * File:       src/server.ts
 * License:    FSL-1.1-ALv2
 * Copyright:  (c) 2026 HYPERI PTY LIMITED
 *
 * TypeScript app variant for the hyperi-ci test matrix. Trivial HTTP
 * server using only the Node.js standard library — no Express, no
 * Fastify, no extra deps. Container build proves the buildx + GHCR
 * push path; the server itself just needs to boot.
 */

import { createServer, type Server } from 'node:http';
import { env } from 'node:process';

export const VERSION = '1.0.0';

export function startServer(port = 8080): Server {
  const server = createServer((req, res) => {
    if (req.url === '/healthz') {
      res.writeHead(200, { 'Content-Type': 'text/plain' });
      res.end('ok\n');
      return;
    }
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end(
      JSON.stringify({ name: 'ci-test-ts-app', version: VERSION }) + '\n',
    );
  });

  server.listen(port, () => {
    process.stdout.write(`ci-test-ts-app v${VERSION} listening on :${port}\n`);
  });
  return server;
}

if (import.meta.url === `file://${process.argv[1]}`) {
  const port = Number.parseInt(env.PORT ?? '8080', 10);
  startServer(port);
}
