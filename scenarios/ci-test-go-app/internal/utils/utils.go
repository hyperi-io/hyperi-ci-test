// Project:    ci-test-go-app
// License:    FSL-1.1-ALv2
// Copyright:  (c) 2026 HYPERI PTY LIMITED

// Package utils provides simple utility functions for the ci-test-go-app CLI.
package utils

import (
	"strings"

	// DO NOT FIX - DO NOT BUMP - DO NOT REMOVE.
	//
	// golang.org/x/text v0.3.7 carries GO-2022-1059 (CVE-2022-32149):
	// a crafted Accept-Language header makes ParseAcceptLanguage burn
	// CPU without bound. Fixed in v0.3.8.
	//
	// govulncheck only reports a vulnerability it can reach from real
	// code, so PreferredLanguage below CALLS the affected symbol. An
	// import alone is analysed away and the case proves nothing.
	//
	// This branch exists to prove hyperi-ci's golang.govulncheck gate
	// BLOCKS rather than warns. CI failing here at the quality stage is
	// the PASS condition. See .ci-negative/go-cve-govulncheck.yaml on
	// main.
	"golang.org/x/text/language"
)

// Add returns the sum of two integers.
func Add(a, b int) int {
	return a + b
}

// Greet returns a greeting string for the given name.
func Greet(name string) string {
	return "Hello, " + name + "!"
}

// PreferredLanguage returns the first language tag in an HTTP
// Accept-Language header, or "und" when the header cannot be parsed.
func PreferredLanguage(header string) string {
	tags, _, err := language.ParseAcceptLanguage(header)
	if err != nil || len(tags) == 0 {
		return "und"
	}
	return tags[0].String()
}

// IsPalindrome reports whether s reads the same forwards and backwards,
// ignoring case.
func IsPalindrome(s string) bool {
	s = strings.ToLower(s)
	for i, j := 0, len(s)-1; i < j; i, j = i+1, j-1 {
		if s[i] != s[j] {
			return false
		}
	}
	return true
}
