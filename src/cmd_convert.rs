use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub fn cmd_convert(
    input: PathBuf,
    from: Option<String>,
    to: String,
    output: Option<PathBuf>,
    _type_hints: Option<PathBuf>,
) -> Result<()> {
    // Validate input file exists
    if !input.exists() {
        anyhow::bail!("File not found: {}", input.display());
    }

    // Auto-detect source format if not specified
    let source_format = from.unwrap_or_else(|| detect_format(&input));

    // Read and convert
    let content = fs::read_to_string(&input)
        .with_context(|| format!("Failed to read file: {}", input.display()))?;

    let result = match (source_format.as_str(), to.as_str()) {
        ("gbln", "json") => convert_gbln_to_json(&content)?,
        ("json", "gbln") => {
            anyhow::bail!("JSON → GBLN conversion requires --type-hints file (not yet implemented)")
        }
        ("gbln", "yaml") => {
            anyhow::bail!("GBLN → YAML conversion not yet implemented")
        }
        ("yaml", "gbln") => {
            anyhow::bail!("YAML → GBLN conversion not yet implemented")
        }
        _ => anyhow::bail!("Unsupported conversion: {} → {}", source_format, to),
    };

    // Output
    if let Some(output_path) = output {
        fs::write(&output_path, &result)
            .with_context(|| format!("Failed to write output: {}", output_path.display()))?;
        println!("{}", output_path.display());
    } else {
        println!("{}", result);
    }

    Ok(())
}

fn detect_format(path: &PathBuf) -> String {
    if let Some(ext) = path.extension() {
        match ext.to_string_lossy().as_ref() {
            "gbln" => "gbln".to_string(),
            "json" => "json".to_string(),
            "yaml" | "yml" => "yaml".to_string(),
            _ => "unknown".to_string(),
        }
    } else {
        "unknown".to_string()
    }
}

fn convert_gbln_to_json(content: &str) -> Result<String> {
    // Parse GBLN
    let value = gbln::parse(content)?;

    // Convert to JSON via serde_json
    let json_value = gbln_value_to_json(&value);
    let json = serde_json::to_string_pretty(&json_value)?;

    Ok(json)
}

fn gbln_value_to_json(value: &gbln::Value) -> serde_json::Value {
    use gbln::Value;
    use serde_json::Value as JsonValue;

    match value {
        Value::I8(n) => JsonValue::Number((*n).into()),
        Value::I16(n) => JsonValue::Number((*n).into()),
        Value::I32(n) => JsonValue::Number((*n).into()),
        Value::I64(n) => JsonValue::Number((*n).into()),
        Value::U8(n) => JsonValue::Number((*n).into()),
        Value::U16(n) => JsonValue::Number((*n).into()),
        Value::U32(n) => JsonValue::Number((*n).into()),
        Value::U64(n) => JsonValue::Number((*n).into()),
        Value::F32(f) => {
            if let Some(num) = serde_json::Number::from_f64(*f as f64) {
                JsonValue::Number(num)
            } else {
                JsonValue::Null
            }
        }
        Value::F64(f) => {
            if let Some(num) = serde_json::Number::from_f64(*f) {
                JsonValue::Number(num)
            } else {
                JsonValue::Null
            }
        }
        Value::Str(s) => JsonValue::String(s.clone()),
        Value::Bool(b) => JsonValue::Bool(*b),
        Value::Null => JsonValue::Null,
        Value::Object(map) => {
            let mut obj = serde_json::Map::new();
            for (k, v) in map {
                obj.insert(k.clone(), gbln_value_to_json(v));
            }
            JsonValue::Object(obj)
        }
        Value::Array(arr) => JsonValue::Array(arr.iter().map(gbln_value_to_json).collect()),
    }
}
