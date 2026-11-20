// Project:    ci-test-go-app
// License:    FSL-1.1-ALv2
// Copyright:  (c) 2026 HYPERI PTY LIMITED

package utils

import "testing"

func TestAdd(t *testing.T) {
	tests := []struct {
		a, b int
		want int
	}{
		{1, 2, 3},
		{0, 0, 0},
		{-1, 1, 0},
		{100, -50, 50},
	}

	for _, tt := range tests {
		got := Add(tt.a, tt.b)
		if got != tt.want {
			t.Errorf("Add(%d, %d) = %d, want %d", tt.a, tt.b, got, tt.want)
		}
	}
}

func TestGreet(t *testing.T) {
	tests := []struct {
		name string
		want string
	}{
		{"world", "Hello, world!"},
		{"Go", "Hello, Go!"},
		{"", "Hello, !"},
	}

	for _, tt := range tests {
		got := Greet(tt.name)
		if got != tt.want {
			t.Errorf("Greet(%q) = %q, want %q", tt.name, got, tt.want)
		}
	}
}

func TestIsPalindrome(t *testing.T) {
	tests := []struct {
		s    string
		want bool
	}{
		{"racecar", true},
		{"Racecar", true},
		{"hello", false},
		{"a", true},
		{"", true},
		{"AbBa", true},
		{"noon", true},
		{"go", false},
	}

	for _, tt := range tests {
		got := IsPalindrome(tt.s)
		if got != tt.want {
			t.Errorf("IsPalindrome(%q) = %t, want %t", tt.s, got, tt.want)
		}
	}
}
