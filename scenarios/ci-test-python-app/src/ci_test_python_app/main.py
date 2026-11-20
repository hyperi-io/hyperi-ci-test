# Project:   ci-test-python-app
# License:   FSL-1.1-ALv2
# Copyright: (c) 2026 HYPERI PTY LIMITED
"""Entry point for the ci-test-python-app CLI.

Validates two parts of the hyperi-ci pipeline at once:

1. The scalo dependency resolves at install time — the app is built on
   scalo's ``ServiceApp`` framework, so a missing dep fails immediately.
2. The container build path picks up the Dockerfile (custom mode),
   pushes the resulting image to GHCR, and the image's ``ENTRYPOINT``
   actually runs (the HEALTHCHECK exercises the ``version`` subcommand).

Built on ``scalo.cli.ServiceApp`` (scalo 2.29+), which provides the
standard subcommands hyperi-ci depends on — in particular
``generate-artefacts``, invoked by the CI generate stage as
``<app> generate-artefacts --output-dir ci-tmp``.
"""

from __future__ import annotations

import sys
from typing import Any

from scalo.cli import ServiceApp, VersionInfo
from scalo.logger import logger

from ci_test_python_app import __version__


class CiTestPythonApp(ServiceApp):
    """Trivial service proving the scalo CLI lifecycle boots in CI."""

    name = "ci-test-python-app"
    env_prefix = "CI_TEST_PYTHON_APP"

    def version_info(self) -> VersionInfo:
        """Return version metadata for the ``version`` subcommand."""
        return VersionInfo(self.name, __version__)

    def run_service(self, config: Any) -> None:
        """Log a line and print the version — that's the whole service.

        The service is intentionally trivial: it only needs to prove
        the binary boots, can resolve scalo at runtime, and emits a
        log line. The full container build/push path is what's actually
        being validated by CI.

        Args:
            config: Dynaconf settings object (unused).
        """
        logger.info("ci-test-python-app booted", version=__version__)
        print(f"ci-test-python-app {__version__}")


def main(argv: list[str] | None = None) -> int:
    """Run the CLI and return an exit code.

    Args:
        argv: Optional argument list. Defaults to ``sys.argv[1:]`` when None.

    Returns:
        Exit code (0 on success). In practice Typer's standalone mode
        raises ``SystemExit`` itself for help/version/parse outcomes.
    """
    CiTestPythonApp().cli(argv)
    return 0


if __name__ == "__main__":
    sys.exit(main())
