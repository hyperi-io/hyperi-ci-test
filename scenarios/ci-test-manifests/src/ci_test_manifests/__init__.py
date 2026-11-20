# Project:   ci-test-manifests
# License:   FSL-1.1-ALv2
# Copyright: (c) 2026 HYPERI PTY LIMITED
"""Generic container + k8s + IaC linting fixture for the hyperi-ci fleet.

Stdlib-only and unlinked from scalo / any HyperI library. The package
exists to exercise hyperi-ci's linting surfaces - Path A (hadolint gate +
droast advisory over the Dockerfile) and Path B (kubeconform gate +
kube-linter / checkov advisories over the Helm chart, manifests and .tf).
"""

from __future__ import annotations

__version__ = "0.1.0"
