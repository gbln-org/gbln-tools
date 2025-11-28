// Copyright (c) 2025 Vivian Burkhard Voss
// SPDX-License-Identifier: Apache-2.0

use anyhow::{Context, Result};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub fn cmd_read(
    input: PathBuf,
    output: Option<PathBuf>,
    overwrite: bool,
    _config: Option<PathBuf>,
    verbose: bool,
) -> Result<()> {
    // Smart lookup: Try .io.gbln.xz, then .io.gbln, then the file itself
    let io_path = find_io_file(&input)?;

    if verbose {
        println!("Found: {}", io_path.display());
    }

    // Read and parse I/O file
    let value = gbln::read_io(&io_path)
        .with_context(|| format!("Failed to read I/O file: {}", io_path.display()))?;

    if verbose {
        println!("Parsing: OK");
    }

    // Serialise to pretty format
    let pretty_content = gbln::to_string_pretty(&value);

    // Determine output path
    let output_path = output.unwrap_or_else(|| strip_io_extensions(&input));

    // Check if output exists and prompt if needed
    if output_path.exists() && !overwrite {
        let existing = fs::read_to_string(&output_path)?;
        if existing == pretty_content {
            if verbose {
                println!("No changes needed: {}", output_path.display());
            }
            return Ok(());
        }

        print!(
            "File {} exists and differs. Overwrite? [y/N] ",
            output_path.display()
        );
        io::stdout().flush()?;

        let mut response = String::new();
        io::stdin().read_line(&mut response)?;

        if !response.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    // Write pretty format
    fs::write(&output_path, pretty_content)
        .with_context(|| format!("Failed to write output: {}", output_path.display()))?;

    if verbose {
        println!("Updated: {}", output_path.display());
    } else {
        println!("{}", output_path.display());
    }

    Ok(())
}

/// Find the I/O file: try .io.gbln.xz, then .io.gbln, then the file itself
fn find_io_file(input: &Path) -> Result<PathBuf> {
    // If input already has .io extension, use it
    if let Some(name) = input.file_name() {
        let name_str = name.to_string_lossy();
        if name_str.contains(".io.gbln") {
            if input.exists() {
                return Ok(input.to_path_buf());
            } else {
                anyhow::bail!("File not found: {}", input.display());
            }
        }
    }

    // Try .io.gbln.xz
    let mut compressed_path = input.to_path_buf();
    compressed_path.set_file_name(format!(
        "{}.io.gbln.xz",
        input.file_stem().unwrap().to_string_lossy()
    ));
    if compressed_path.exists() {
        return Ok(compressed_path);
    }

    // Try .io.gbln
    let mut io_path = input.to_path_buf();
    io_path.set_extension("io.gbln");
    if io_path.exists() {
        return Ok(io_path);
    }

    // Try the file itself
    if input.exists() {
        return Ok(input.to_path_buf());
    }

    anyhow::bail!(
        "No I/O file found for {}. Tried:\n  - {}\n  - {}\n  - {}",
        input.display(),
        compressed_path.display(),
        io_path.display(),
        input.display()
    );
}

/// Strip .io.gbln.xz or .io.gbln extensions, leaving just .gbln
fn strip_io_extensions(path: &Path) -> PathBuf {
    let name = path.file_name().unwrap().to_string_lossy();

    if let Some(base) = name.strip_suffix(".io.gbln.xz") {
        let mut result = path.parent().unwrap().to_path_buf();
        result.push(format!("{}.gbln", base));
        return result;
    }

    if let Some(base) = name.strip_suffix(".io.gbln") {
        let mut result = path.parent().unwrap().to_path_buf();
        result.push(format!("{}.gbln", base));
        return result;
    }

    path.to_path_buf()
}
