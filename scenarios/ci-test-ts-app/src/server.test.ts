/*
 * Project:    ci-test-ts-app
 * File:       src/server.test.ts
 * License:    FSL-1.1-ALv2
 * Copyright:  (c) 2026 HYPERI PTY LIMITED
 */

import { describe, it, expect, afterEach } from 'vitest';
import { startServer, VERSION } from './server.js';

let server: ReturnType<typeof startServer> | null = null;

afterEach(() => {
  if (server) {
    server.close();
    server = null;
  }
});

describe('startServer', () => {
  it('listens on the given port and serves /healthz', async () => {
    server = startServer(0);
    const port = (server.address() as { port: number }).port;
    const res = await fetch(`http://127.0.0.1:${port}/healthz`);
    expect(res.status).toBe(200);
    const body = await res.text();
    expect(body.trim()).toBe('ok');
  });

  it('returns version metadata on root', async () => {
    server = startServer(0);
    const port = (server.address() as { port: number }).port;
    const res = await fetch(`http://127.0.0.1:${port}/`);
    expect(res.status).toBe(200);
    const body = (await res.json()) as { name: string; version: string };
    expect(body.name).toBe('ci-test-ts-app');
    expect(body.version).toBe(VERSION);
  });
});
