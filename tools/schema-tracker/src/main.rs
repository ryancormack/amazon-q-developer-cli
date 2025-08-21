mod schema_types;

use std::fs;
use std::process::Command;

use chrono::Utc;
use clap::{Parser, Subcommand};
use eyre::Result;
use serde_json;
use sha2::{Digest, Sha256};
use schemars::schema_for;

use chat_cli::cli::ConversationState;
use schema_types::ConversationState as SchemaConversationState;

#[derive(Parser)]
#[command(name = "schema-tracker")]
#[command(about = "Track ConversationState schema evolution across git releases")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Capture current schema and save with metadata
    Capture {
        /// Optional note to include with the schema
        #[arg(short, long)]
        note: Option<String>,
        /// Use schemars with copied types for complete type information
        #[arg(long)]
        schemars: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Capture { note, schemars } => capture_schema(note, schemars),
    }
}

fn capture_schema(note: Option<String>, use_schemars: bool) -> Result<()> {
    println!("🔍 Generating schema from actual ConversationState type...");
    
    // Generate schema using the chosen approach
    let schema = if use_schemars {
        generate_schema_with_schemars()?
    } else {
        generate_schema_from_actual_type()?
    };
    
    // Create metadata
    let timestamp = Utc::now();
    let git_commit = get_git_commit().ok();
    let schema_hash = calculate_schema_hash(&schema);
    
    // Create final output with metadata
    let note_text = note.clone().unwrap_or_else(|| "Schema capture".to_string());
    let output = serde_json::json!({
        "timestamp": timestamp.to_rfc3339(),
        "git_commit": git_commit,
        "schema_hash": schema_hash,
        "note": note_text,
        "schema": schema
    });
    
    // Save to file in schema-tracker/schemas/ directory
    let method_suffix = if use_schemars { "_schemars" } else { "" };
    let schema_dir = "tools/schema-tracker/schemas";
    let filename = format!("{}/conversation_schema_{}{}.json", 
        schema_dir, timestamp.format("%Y%m%d_%H%M%S"), method_suffix);
    
    // Ensure schemas directory exists
    fs::create_dir_all(schema_dir)?;
    
    // Write schema file
    fs::write(&filename, serde_json::to_string_pretty(&output)?)?;
    
    println!("✅ Schema captured: {}", filename);
    println!("   Hash: {}", &schema_hash[..12]);
    println!("   Method: {}", if use_schemars { "schemars (complete types)" } else { "hybrid (type introspection)" });
    if let Some(commit) = git_commit {
        println!("   Commit: {}", &commit[..8]);
    }
    if note.is_some() {
        println!("   Note: {}", note_text);
    }
    
    Ok(())
}

/// Generate JSON Schema using schemars with copied types
/// This provides complete type introspection including all nested types
fn generate_schema_with_schemars() -> Result<serde_json::Value> {
    println!("🔍 Generating schema using schemars with complete type information...");
    
    // Use schemars to generate schema directly from our copied Rust types
    let schema = schema_for!(SchemaConversationState);
    
    println!("   ✅ Schema generated successfully with complete type information");
    
    // Convert to JSON and add our metadata
    let mut schema_json = serde_json::to_value(schema)?;
    
    // Add metadata about generation method
    if let Some(schema_obj) = schema_json.as_object_mut() {
        schema_obj.insert("_generation_method".to_string(), 
            serde_json::json!("schemars_complete_introspection"));
        schema_obj.insert("_generation_timestamp".to_string(), 
            serde_json::json!(chrono::Utc::now().to_rfc3339()));
        schema_obj.insert("_note".to_string(), 
            serde_json::json!("Generated using schemars with copied types + JsonSchema derives"));
    }
    
    Ok(schema_json)
}

/// Generate JSON Schema using a hybrid approach
/// Uses schemars where possible, falls back to type introspection for complex types
fn generate_schema_from_actual_type() -> Result<serde_json::Value> {
    println!("🔍 Generating schema using hybrid approach...");
    
    // Create a comprehensive test instance to understand the structure
    let test_instance = create_test_conversation_state()?;
    let serialized = serde_json::to_value(&test_instance)?;
    
    println!("   ✅ Successfully analyzed ConversationState structure");
    println!("   📊 Discovered {} top-level fields", 
        serialized.as_object().map(|o| o.len()).unwrap_or(0));
    
    // Generate schema from the serialized structure
    let mut schema = analyze_complete_structure(&serialized)?;
    
    // Add metadata
    if let Some(schema_obj) = schema.as_object_mut() {
        schema_obj.insert("_generation_method".to_string(), 
            serde_json::json!("type_introspection"));
        schema_obj.insert("_generation_timestamp".to_string(), 
            serde_json::json!(chrono::Utc::now().to_rfc3339()));
        schema_obj.insert("_note".to_string(), 
            serde_json::json!("Generated through type introspection without upstream modifications"));
    }
    
    Ok(schema)
}

/// Create a test ConversationState instance with comprehensive field coverage
fn create_test_conversation_state() -> Result<ConversationState> {
    println!("   Creating test ConversationState instance...");
    
    let test_json = serde_json::json!({
        "conversation_id": "schema_analysis_test",
        "next_message": null,
        "history": [],
        "valid_history_range": [0, 0],
        "transcript": [],
        "tools": {},
        "context_manager": null,
        "context_message_length": null,
        "latest_summary": null,
        "model": null,
        "model_info": null,
        "file_line_tracker": {}
    });

    let instance = serde_json::from_value::<ConversationState>(test_json)?;
    println!("   ✅ Successfully created test instance");
    
    Ok(instance)
}

/// Analyze the complete structure of a JSON value and generate a schema
fn analyze_complete_structure(value: &serde_json::Value) -> Result<serde_json::Value> {
    match value {
        serde_json::Value::Object(obj) => {
            let mut properties = serde_json::Map::new();
            let mut required_fields = Vec::new();
            
            for (field_name, field_value) in obj {
                let field_schema = analyze_field_structure(field_value, field_name)?;
                properties.insert(field_name.clone(), field_schema);
                
                // Fields that are not null are likely required
                if !field_value.is_null() {
                    required_fields.push(field_name.clone());
                }
            }
            
            Ok(serde_json::json!({
                "$schema": "http://json-schema.org/draft-07/schema#",
                "title": "ConversationState",
                "type": "object",
                "description": "Amazon Q CLI conversation state structure",
                "properties": properties,
                "required": required_fields
            }))
        }
        _ => {
            Err(eyre::eyre!("Expected object at root level, got {:?}", value))
        }
    }
}

/// Analyze the structure of a specific field and generate its schema
fn analyze_field_structure(value: &serde_json::Value, field_name: &str) -> Result<serde_json::Value> {
    let schema = match value {
        serde_json::Value::String(_) => {
            serde_json::json!({
                "type": "string",
                "description": format!("String field: {}", field_name)
            })
        }
        serde_json::Value::Number(n) => {
            let type_name = if n.is_i64() || n.is_u64() { "integer" } else { "number" };
            serde_json::json!({
                "type": type_name,
                "description": format!("{} field: {}", 
                    type_name.chars().next().unwrap().to_uppercase().collect::<String>() + &type_name[1..], 
                    field_name)
            })
        }
        serde_json::Value::Bool(_) => {
            serde_json::json!({
                "type": "boolean",
                "description": format!("Boolean field: {}", field_name)
            })
        }
        serde_json::Value::Array(arr) => {
            let items_schema = if let Some(first_item) = arr.first() {
                analyze_field_structure(first_item, "array_item")?
            } else {
                serde_json::json!({
                    "description": "Array type - items unknown from empty array"
                })
            };
            
            serde_json::json!({
                "type": "array",
                "items": items_schema,
                "description": format!("Array field: {}", field_name)
            })
        }
        serde_json::Value::Object(_) => {
            serde_json::json!({
                "type": "object",
                "description": format!("Object field: {}", field_name)
            })
        }
        serde_json::Value::Null => {
            serde_json::json!({
                "anyOf": [
                    {"type": "null"},
                    {"description": "Optional field - type determined at runtime"}
                ],
                "description": format!("Optional field: {}", field_name)
            })
        }
    };
    
    Ok(schema)
}

fn get_git_commit() -> Result<String> {
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()?;
    
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    } else {
        Err(eyre::eyre!("Failed to get git commit"))
    }
}

fn calculate_schema_hash(schema: &serde_json::Value) -> String {
    let schema_str = serde_json::to_string(schema).unwrap_or_default();
    let mut hasher = Sha256::new();
    hasher.update(schema_str.as_bytes());
    format!("{:x}", hasher.finalize())
}
