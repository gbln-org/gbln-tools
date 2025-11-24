use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "gbln")]
#[command(version, about = "GBLN I/O and formatting tools", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate I/O file from source (.gbln → .io.gbln.xz)
    Write {
        /// Input file (.gbln)
        input: PathBuf,

        /// Output file (default: <input>.io.gbln.xz)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Disable XZ compression
        #[arg(long)]
        no_compress: bool,

        /// Disable MINI mode (keep pretty format)
        #[arg(long)]
        no_mini: bool,

        /// XZ compression level (0-9)
        #[arg(long, default_value = "6", value_parser = clap::value_parser!(u8).range(0..=9))]
        compression_level: u8,

        /// Config file
        #[arg(short, long)]
        config: Option<PathBuf>,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Read I/O file and update source (.io.gbln.xz → .gbln)
    Read {
        /// Input file (.gbln, looks for .io.gbln.xz)
        input: PathBuf,

        /// Output file (default: strip .io/.xz)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Overwrite without asking
        #[arg(long)]
        overwrite: bool,

        /// Config file
        #[arg(short, long)]
        config: Option<PathBuf>,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Validate GBLN file
    Validate {
        /// Input file
        input: PathBuf,

        /// Verbose output
        #[arg(short, long)]
        verbose: bool,

        /// JSON output
        #[arg(long)]
        json: bool,

        /// Auto-fix formatting issues
        #[arg(long)]
        fix: bool,
    },

    /// Convert between formats
    Convert {
        /// Input file
        input: PathBuf,

        /// Source format (auto-detect if omitted)
        #[arg(short, long)]
        from: Option<String>,

        /// Target format (required: gbln, json, yaml)
        #[arg(short, long)]
        to: String,

        /// Output file (default: stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Type hints file (JSON → GBLN)
        #[arg(long)]
        type_hints: Option<PathBuf>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Write {
            input,
            output,
            no_compress,
            no_mini,
            compression_level,
            config,
            verbose,
        } => cmd_write(
            input,
            output,
            no_compress,
            no_mini,
            compression_level,
            config,
            verbose,
        ),
        Commands::Read {
            input,
            output,
            overwrite,
            config,
            verbose,
        } => cmd_read(input, output, overwrite, config, verbose),
        Commands::Validate {
            input,
            verbose,
            json,
            fix,
        } => cmd_validate(input, verbose, json, fix),
        Commands::Convert {
            input,
            from,
            to,
            output,
            type_hints,
        } => cmd_convert(input, from, to, output, type_hints),
    }
}

// Command implementations will be added in separate modules
mod cmd_convert;
mod cmd_read;
mod cmd_validate;
mod cmd_write;

use cmd_convert::cmd_convert;
use cmd_read::cmd_read;
use cmd_validate::cmd_validate;
use cmd_write::cmd_write;
