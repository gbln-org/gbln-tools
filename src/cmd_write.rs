use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub fn cmd_write(
    input: PathBuf,
    output: Option<PathBuf>,
    no_compress: bool,
    no_mini: bool,
    compression_level: u8,
    _config: Option<PathBuf>,
    verbose: bool,
) -> Result<()> {
    // Validate input file exists
    if !input.exists() {
        anyhow::bail!("File not found: {}", input.display());
    }

    // Read and parse input file
    if verbose {
        println!(
            "Reading: {} ({} bytes)",
            input.display(),
            fs::metadata(&input)?.len()
        );
    }

    let content = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read file: {}", input.display()))?;

    let value = gbln::parse(&content)
        .with_context(|| format!("Failed to parse GBLN file: {}", input.display()))?;

    if verbose {
        println!("Parsing: OK");
    }

    // Determine output path
    let output_path = output.unwrap_or_else(|| {
        let mut path = input.clone();
        if no_compress {
            path.set_extension("io.gbln");
        } else {
            path.set_file_name(format!(
                "{}.io.gbln.xz",
                path.file_stem().unwrap().to_string_lossy()
            ));
        }
        path
    });

    // Create configuration
    let config = gbln::GblnConfig {
        mini_mode: !no_mini,
        compress: !no_compress,
        compression_level,
        indent: 2,
        strip_comments: true,
    };

    // Write I/O file
    let original_size = content.len();
    gbln::write_io(&value, &output_path, &config)
        .with_context(|| format!("Failed to write output: {}", output_path.display()))?;

    if verbose {
        let output_size = fs::metadata(&output_path)?.len();
        let reduction =
            ((original_size as f64 - output_size as f64) / original_size as f64) * 100.0;

        if config.mini_mode {
            println!("MINI GBLN: enabled");
        }

        if config.compress {
            println!(
                "XZ compress (level {}): {} bytes → {} bytes ({:.1}% reduction)",
                compression_level, original_size, output_size, reduction
            );
        }

        println!("Written: {}", output_path.display());
    } else {
        println!("{}", output_path.display());
    }

    Ok(())
}
