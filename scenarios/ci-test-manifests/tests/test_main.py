# Project:   ci-test-manifests
# License:   FSL-1.1-ALv2
# Copyright: (c) 2026 HYPERI PTY LIMITED
"""Smoke test for the ci-test-manifests entry point."""

from __future__ import annotations

from ci_test_manifests.main import main


def test_main_returns_zero() -> None:
    assert main([]) == 0
