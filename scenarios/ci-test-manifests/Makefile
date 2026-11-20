# Project:   ci-test-manifests
# File:      Makefile
# Purpose:   Thin CI targets wrapping hyperi-ci
#
# License:   FSL-1.1-ALv2
# Copyright: (c) 2026 HYPERI PTY LIMITED

.PHONY: quality test build check lint-manifests

quality:
	hyperi-ci run quality

test:
	hyperi-ci run test

build:
	hyperi-ci run build

check:
	hyperi-ci check

lint-manifests:
	hyperi-ci lint-manifests .
