# Project:   ci-test-python-app
# License:   FSL-1.1-ALv2
# Copyright: (c) 2026 HYPERI PTY LIMITED
"""Smoke tests for the ci-test-python-app entry point."""

from __future__ import annotations

from pathlib import Path

import pytest

from ci_test_python_app import __version__
from ci_test_python_app.main import main


def test_version_string_is_non_empty() -> None:
    """__version__ is a non-empty string."""
    assert isinstance(__version__, str)
    assert __version__


def test_help_output(capsys: pytest.CaptureFixture[str]) -> None:
    """--help prints usage information and exits cleanly."""
    with pytest.raises(SystemExit) as exc_info:
        main(["--help"])
    assert exc_info.value.code == 0
    captured = capsys.readouterr()
    assert "ci-test-python-app" in captured.out
    assert "usage" in captured.out.lower()


def test_no_args_shows_help(capsys: pytest.CaptureFixture[str]) -> None:
    """main() with no args prints help via no_args_is_help.

    click >= 8.2 exits 2 (usage error) for the no-args help path.
    """
    with pytest.raises(SystemExit) as exc_info:
        main([])
    assert exc_info.value.code == 2
    captured = capsys.readouterr()
    assert "usage" in captured.out.lower()


def test_version_subcommand(capsys: pytest.CaptureFixture[str]) -> None:
    """The version subcommand prints the app version."""
    with pytest.raises(SystemExit) as exc_info:
        main(["version"])
    assert exc_info.value.code == 0
    captured = capsys.readouterr()
    assert __version__ in captured.out


def test_generate_artefacts_parses(
    tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    """generate-artefacts is a recognised subcommand and exits 0.

    hyperi-ci's generate stage runs
    ``<app> generate-artefacts --output-dir ci-tmp`` — the subcommand
    comes from scalo's ServiceApp framework. With no deployment
    contract overridden it warns and exits 0.
    """
    with pytest.raises(SystemExit) as exc_info:
        main(["generate-artefacts", "--output-dir", str(tmp_path)])
    assert exc_info.value.code == 0


def test_scalo_import_resolves() -> None:
    """The scalo dep was actually installed (not just declared)."""
    from scalo import logger

    assert logger is not None
