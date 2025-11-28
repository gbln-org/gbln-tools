// Copyright (c) 2025 Vivian Burkhard Voss
// SPDX-License-Identifier: Apache-2.0

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub fn cmd_validate(input: PathBuf, verbose: bool, json: bool, fix: bool) -> Result<()> {
    // Validate input file exists
    if !input.exists() {
        anyhow::bail!("File not found: {}", input.display());
    }

    // Read file
    let content = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read file: {}", input.display()))?;

    // Parse and validate
    match gbln::parse(&content) {
        Ok(value) => {
            if json {
                println!(r#"{{"valid": true, "errors": []}}"#);
            } else {
                if verbose {
                    println!("✓ Valid syntax");
                }

                // Check formatting
                let pretty = gbln::to_string_pretty(&value);
                if content.trim() != pretty.trim() {
                    if fix {
                        // Auto-fix: rewrite file with proper formatting
                        fs::write(&input, &pretty).with_context(|| {
                            format!("Failed to write fixed file: {}", input.display())
                        })?;
                        println!("✓ Fixed formatting issues");
                        println!("✓ Written to {}", input.display());
                    } else {
                        println!("⚠ Formatting issues detected");
                        println!(
                            "Hint: Run 'gbln validate --fix {}' to auto-fix",
                            input.display()
                        );
                    }
                } else if verbose {
                    println!("✓ Formatting correct");
                }
            }
            Ok(())
        }
        Err(e) => {
            if json {
                // Output JSON format for CI/tooling
                let suggestion = e.suggestion.as_deref().unwrap_or("");
                println!(
                    r#"{{"valid": false, "errors": [{{"kind": "{:?}", "line": {}, "column": {}, "message": "{}", "suggestion": "{}"}}]}}"#,
                    e.kind,
                    e.line,
                    e.column,
                    e.message.replace('"', "\\\""),
                    suggestion.replace('"', "\\\"")
                );
            } else {
                // Human-readable error
                eprintln!("✗ Error: {}", e.message);
                eprintln!("  at line {}, column {}", e.line, e.column);
                if let Some(ref suggestion) = e.suggestion {
                    eprintln!("  suggestion: {}", suggestion);
                }
            }
            std::process::exit(1);
        }
    }
}
