# Project:   ci-test-python-lib
# License:   FSL-1.1-ALv2
# Copyright: (c) 2026 HYPERI PTY LIMITED
"""Smoke tests — package imports and exposes its scalo logger bridge."""

from __future__ import annotations

import ci_test_python_lib


def test_package_version_is_set() -> None:
    """__version__ is a non-empty string."""
    assert isinstance(ci_test_python_lib.__version__, str)
    assert ci_test_python_lib.__version__


def test_logger_is_re_exported() -> None:
    """``log`` is the scalo logger object, reachable post-install."""
    from scalo.logger import logger as scalo_logger

    assert ci_test_python_lib.log is scalo_logger


def test_banner_includes_version_and_logger_repr() -> None:
    """``banner()`` smokes both the version constant and the logger import."""
    msg = ci_test_python_lib.banner()
    assert ci_test_python_lib.__version__ in msg
    assert "logger" in msg
