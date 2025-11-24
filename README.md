# GBLN Tools

CLI tools for GBLN (Goblin Bounded Lean Notation) - the first type-safe LLM-native serialisation format.

## Installation

```bash
cargo install gbln-tools
```

## Commands

### `gbln write` - Generate I/O Files

Generate optimised I/O files (`.io.gbln.xz`) from human-editable source (`.gbln`):

```bash
# Standard: Generate compressed I/O file
gbln write config.gbln
# → config.io.gbln.xz (MINI GBLN + XZ compressed)

# Without compression
gbln write config.gbln --no-compress
# → config.io.gbln (MINI GBLN only)

# Custom compression level (0-9)
gbln write config.gbln --compression-level 9

# Verbose output
gbln write config.gbln -v
```

**Options:**
- `-o, --output <file>` - Output file (default: `<input>.io.gbln.xz`)
- `--no-compress` - Disable XZ compression
- `--no-mini` - Keep pretty format (disable MINI mode)
- `--compression-level <0-9>` - XZ compression level (default: 6)
- `-v, --verbose` - Show processing details

### `gbln read` - Update Source Files

Read I/O files and update/create human-editable source:

```bash
# Smart lookup: Finds config.io.gbln.xz or config.io.gbln
gbln read config.gbln

# Auto-overwrite (no prompt)
gbln read config.gbln --overwrite
```

**Options:**
- `-o, --output <file>` - Output file (default: strip .io/.xz from input)
- `--overwrite` - Overwrite without asking
- `-v, --verbose` - Show processing details

**Lookup cascade:**
1. `config.io.gbln.xz` (compressed)
2. `config.io.gbln` (MINI)
3. `config.gbln` (source)

### `gbln validate` - Validate Files

Validate GBLN files and check formatting:

```bash
# Validate file
gbln validate config.gbln

# Auto-fix formatting issues
gbln validate --fix config.gbln

# JSON output (for CI/tooling)
gbln validate --json config.gbln
```

**Options:**
- `-v, --verbose` - Show all validation details
- `--json` - Output errors in JSON format
- `--fix` - Auto-fix formatting issues (in-place)

### `gbln convert` - Format Conversion

Convert between GBLN and other formats:

```bash
# GBLN → JSON
gbln convert config.gbln -t json -o config.json

# JSON → GBLN (requires type hints)
gbln convert config.json -t gbln --type-hints types.json
```

**Options:**
- `-f, --from <format>` - Source format (auto-detect if omitted)
- `-t, --to <format>` - Target format (required: gbln, json)
- `-o, --output <file>` - Output file (default: stdout)
- `--type-hints <file>` - JSON file with type hints (JSON → GBLN)

## File Types

### `.gbln` - Human-Editable Source
- **Purpose:** Edited by developers, committed to Git
- **Format:** Pretty-printed with 2-space indentation
- **Comments:** Preserved
- **Version Control:** Yes

```gbln
:| Application Configuration
server{
  host<s64>(api.example.com)
  port<u16>(8080)
  workers<u8>(4)
}
```

### `.io.gbln` - MINI GBLN (Intermediate)
- **Purpose:** Optimised for processing (no compression)
- **Format:** MINI GBLN (minimal whitespace)
- **Comments:** Stripped
- **Version Control:** No (add to .gitignore)

```gbln
server{host<s64>(api.example.com)port<u16>(8080)workers<u8>(4)}
```

### `.io.gbln.xz` - Compressed I/O (Default)
- **Purpose:** Optimised for storage/transmission
- **Format:** MINI GBLN + XZ compression
- **Size:** ~65-75% smaller than `.gbln`
- **Version Control:** No (add to .gitignore)

## Recommended .gitignore

```gitignore
# GBLN I/O files (generated, not source)
*.io.gbln
*.io.gbln.xz
```

## Exit Codes

- `0` - Success
- `1` - Validation error
- `2` - File error

## License

MIT License - see LICENSE file for details

## Links

- **Specification:** https://github.com/gbln-org/gbln
- **Rust Library:** https://github.com/gbln-org/gbln-rust
- **Website:** https://gbln.dev
