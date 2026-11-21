// Project:    ci-test-go-app
// License:    FSL-1.1-ALv2
// Copyright:  (c) 2026 HYPERI PTY LIMITED

// Package utils provides simple utility functions for the ci-test-go-app CLI.
package utils

import "strings"

// Add returns the sum of two integers.
func Add(a, b int) int {
	return a + b
}

// Greet returns a greeting string for the given name.
func Greet(name string) string {
	return "Hello, " + name + "!"
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
