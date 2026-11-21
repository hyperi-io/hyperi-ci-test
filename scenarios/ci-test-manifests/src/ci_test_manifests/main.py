# Project:   ci-test-manifests
# License:   FSL-1.1-ALv2
# Copyright: (c) 2026 HYPERI PTY LIMITED
"""Entry point for the ci-test-manifests CLI.

A deliberately trivial, stdlib-only program. The fixture's job is to give
hyperi-ci something to lint (a Dockerfile, a Helm chart, plain manifests
and a small .tf), not to demonstrate application behaviour, so the CLI
does nothing beyond printing its version and exiting cleanly.
"""

from __future__ import annotations

import sys

from ci_test_manifests import __version__


def main(argv: list[str] | None = None) -> int:
    """Print the version banner and return an exit code.

    Args:
        argv: Optional argument list, defaulting to ``sys.argv[1:]`` when
            None. Accepted for parity with argparse-style entry points;
            the trivial program ignores its arguments.

    Returns:
        Process exit code. Always 0 - the program cannot fail.
    """
    _ = sys.argv[1:] if argv is None else argv
    print(f"ci-test-manifests {__version__}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
