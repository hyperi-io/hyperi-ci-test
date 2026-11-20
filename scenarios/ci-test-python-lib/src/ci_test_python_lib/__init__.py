# Project:   ci-test-python-lib
# License:   FSL-1.1-ALv2
# Copyright: (c) 2026 HYPERI PTY LIMITED
"""Python library variant for the hyperi-ci test matrix.

This package has no ``[project.scripts]`` entry and no Dockerfile, so
hyperi-ci's container detector classifies it as library-only and skips
the container build stage entirely. The publish path goes to PyPI only.

Being the fastest fixture to run quality on, it doubles as the fleet's
quality-stage canary: a PR here exercises the cross-language quality
tools (gitleaks, semgrep, the alint advisory, ...) end to end without
publishing anything.
"""

from __future__ import annotations

from scalo.logger import logger

__version__ = "1.0.2"

# Single bound logger so callers don't need to import scalo directly.
# This re-export is the only public surface of the lib — enough to
# validate that the scalo install resolves cleanly, imports without
# bloat, and is reachable at wheel-install time.
log = logger


def banner() -> str:
    """Return a single-line banner identifying the lib + its version.

    Used by tests to confirm the package is importable and that the
    re-exported ``logger`` symbol is actually the scalo logger.
    """
    return f"ci-test-python-lib v{__version__} (logger={log!r})"
