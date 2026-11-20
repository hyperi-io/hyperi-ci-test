# Project:   ci-test-manifests
# File:      terraform/main.tf
# Purpose:   Minimal IaC input for the checkov advisory (Path B)
# License:   FSL-1.1-ALv2
# Copyright: (c) 2026 HYPERI PTY LIMITED
#
# HyperI uses OpenTofu (`tofu`), not Terraform - the `.tf` format is shared.
# Just enough for checkov's terraform framework to have something to scan;
# findings here are advisory only.

terraform {
  required_version = ">= 1.6.0"
}

resource "null_resource" "example" {}
