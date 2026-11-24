// Project:    ci-test-go-app
// License:    FSL-1.1-ALv2
// Copyright:  (c) 2026 HYPERI PTY LIMITED

package main

import (
	"flag"
	"fmt"
	"os"

	"github.com/hyperi-io/ci-test-go-app/internal/utils"
)

// version is set at build time via -ldflags.
var version = "dev"

func main() {
	showVersion := flag.Bool("version", false, "Print the version and exit")
	flag.Parse()

	if *showVersion {
		fmt.Printf("ci-test-go-app %s\n", version)
		os.Exit(0)
	}

	fmt.Println(utils.Greet("world"))
	fmt.Printf("2 + 3 = %d\n", utils.Add(2, 3))
	fmt.Printf("\"racecar\" is a palindrome: %t\n", utils.IsPalindrome("racecar"))
	// DO NOT FIX - see the DO NOT FIX block in internal/utils/utils.go.
	fmt.Printf("preferred language: %s\n", utils.PreferredLanguage("en-AU,en;q=0.9"))
}
