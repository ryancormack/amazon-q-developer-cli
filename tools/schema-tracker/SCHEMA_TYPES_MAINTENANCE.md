# Schema Types Maintenance Guide

## Overview

The `schema_types.rs` file contains **copied and modified versions** of types from the main Amazon Q CLI codebase. These copies exist solely to enable complete JSON Schema generation via `schemars` without modifying the upstream codebase.

## Why This Approach?

1. **Complete Type Information**: Generates schemas with full nested type definitions instead of "null" or "unknown" types
2. **Zero Upstream Maintenance**: No merge conflicts or maintenance burden on the main codebase
3. **Isolated Changes**: Schema generation requirements don't affect production code
4. **Best of Both Worlds**: Complete schemas without complexity

## Source Mapping

### Primary Types Copied

| Schema Type | Source Location | Purpose |
|-------------|----------------|---------|
| `ConversationState` | `crates/chat-cli/src/cli/chat/conversation.rs` | Main schema root type |
| `HistoryEntry` | `crates/chat-cli/src/cli/chat/conversation.rs` | Conversation history structure |
| `UserMessage` | `crates/chat-cli/src/cli/chat/message.rs` | User input message structure |
| `AssistantMessage` | `crates/chat-cli/src/cli/chat/message.rs` | Assistant response structure |
| `RequestMetadata` | `crates/chat-cli/src/cli/chat/parser.rs` | Request tracking metadata |
| `ModelInfo` | `crates/chat-cli/src/cli/chat/cli/model.rs` | Model configuration info |
| `FileLineTracker` | `crates/chat-cli/src/cli/chat/line_tracker.rs` | File modification tracking |
| `ToolOrigin` | `crates/chat-cli/src/cli/chat/tools/mod.rs` | Tool source identification |
| `Tool` | `crates/chat-cli/src/api_client/model.rs` | Tool definition structure |
| `ChatConversationType` | `crates/chat-cli/src/telemetry/core.rs` | Conversation classification |
| `MessageMetaTag` | `crates/chat-cli/src/telemetry/core.rs` | Message metadata tags |

### Supporting Types Copied

| Schema Type | Source Location | Purpose |
|-------------|----------------|---------|
| `UserEnvContext` | `crates/chat-cli/src/cli/chat/message.rs` | User environment context |
| `ContentBlock` | `crates/chat-cli/src/api_client/model.rs` | Message content structure |
| `ToolUse` | `crates/chat-cli/src/api_client/model.rs` | Tool invocation data |
| `ToolResult` | `crates/chat-cli/src/api_client/model.rs` | Tool execution result |
| `ToolResultStatus` | `crates/chat-cli/src/api_client/model.rs` | Tool result status enum |
| `TranscriptEntry` | Custom | Simplified transcript structure |
| `ContextManager` | `crates/chat-cli/src/cli/chat/context.rs` | Context management (simplified) |
| `ImageBlock` | Custom | Image data structure |
| `EnvState` | Custom | Environment state structure |

## Key Modifications Made

### 1. Added JsonSchema Derives
**Original:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationState {
```

**Schema Version:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConversationState {
```

### 2. Added Required Traits for HashMap Keys
**Original:**
```rust
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ToolOrigin {
```

**Schema Version:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash)]
pub enum ToolOrigin {
```

### 3. Simplified Complex Types
Some types were simplified to avoid deep dependency chains:

**ContextManager**: Simplified `paths` field from `Vec<ContextFilePath>` to `Vec<String>`
**ContentBlock**: Used `serde_json::Value` for flexible content
**TranscriptEntry**: Created simplified version instead of copying complex original

### 4. Added Documentation Comments
Added descriptive comments for better schema generation:
```rust
/// Unique identifier for this conversation
pub conversation_id: String,
/// Next message to be processed (if any)  
pub next_message: Option<UserMessage>,
```

## Maintenance Rules for Coding Agents

### Rule 1: Detect Schema Drift
**When to Act:** Schema hashes change between captures without code changes
**Action:** Compare current upstream types with schema_types.rs copies
**Command:** `./schema-tracker capture --schemars` and compare with previous hash

### Rule 2: Field Addition Detection
**When to Act:** New fields appear in ConversationState or related types
**How to Detect:** 
1. Compilation errors in schema-tracker
2. Missing fields in generated schema
3. Serde deserialization errors

**Action Steps:**
1. Identify the new field in the source file
2. Add the field to the corresponding schema type
3. Ensure the field type has JsonSchema derive or is supported
4. Test with `cargo build` in schema-tracker directory

### Rule 3: Field Removal Detection  
**When to Act:** Fields are removed from upstream types
**How to Detect:**
1. Schema-tracker builds but generates different hash
2. Unused field warnings in schema_types.rs

**Action Steps:**
1. Remove the field from schema_types.rs
2. Update any dependent types
3. Test schema generation

### Rule 4: Type Changes Detection
**When to Act:** Field types change in upstream code
**How to Detect:**
1. Type mismatch compilation errors
2. Schema generation produces unexpected types
3. Serde serialization/deserialization errors

**Action Steps:**
1. Update the field type in schema_types.rs
2. Ensure new type supports JsonSchema
3. Add JsonSchema derive if it's a custom type
4. Test schema generation

### Rule 5: New Type Dependencies
**When to Act:** New types are introduced that ConversationState depends on
**How to Detect:**
1. Compilation errors about missing types
2. `$ref` references to undefined types in schema

**Action Steps:**
1. Copy the new type definition to schema_types.rs
2. Add JsonSchema derive
3. Add any required traits (Eq, Hash for HashMap keys)
4. Update imports if needed

### Rule 6: Enum Variant Changes
**When to Act:** Enum variants are added/removed/renamed
**How to Detect:**
1. Compilation errors
2. Serde deserialization errors
3. Schema shows unexpected enum values

**Action Steps:**
1. Update enum definition in schema_types.rs
2. Maintain same derive attributes
3. Test serialization/deserialization

## Maintenance Workflow

### Step 1: Regular Health Checks
```bash
# Run both approaches and compare
./schema-tracker capture -n "Health check hybrid"
./schema-tracker capture -n "Health check schemars" --schemars

# Compare hashes - they should be stable for same codebase
```

### Step 2: When Upstream Changes
```bash
# After pulling upstream changes
cd tools/schema-tracker
cargo build --release

# If build fails, follow the error messages to identify missing/changed types
# Update schema_types.rs accordingly
```

### Step 3: Validation
```bash
# Test both approaches work
./schema-tracker capture -n "Post-update test" --schemars
./schema-tracker capture -n "Post-update test hybrid"

# Verify schema quality by checking for:
# - No "unknown" types in schemars output
# - All expected $defs present  
# - Proper type references ($ref)
# - Schemas stored in tools/schema-tracker/schemas/

# Check generated files
ls -la tools/schema-tracker/schemas/
```

## Common Issues and Solutions

### Issue: "trait bound not satisfied" errors
**Cause:** Missing derives on copied types
**Solution:** Add required derives (usually Eq, Hash, PartialEq for HashMap keys)

### Issue: "JsonSchema not implemented" errors  
**Cause:** New type dependency without JsonSchema derive
**Solution:** Add JsonSchema derive to the type, or use serde_json::Value for flexibility

### Issue: Compilation errors about missing types
**Cause:** New type dependencies not copied
**Solution:** Copy the missing type definition and add JsonSchema derive

### Issue: Schema contains unexpected "unknown" types
**Cause:** Type not properly copied or missing JsonSchema derive
**Solution:** Verify all referenced types are in schema_types.rs with JsonSchema

## File Structure

```
tools/schema-tracker/
├── src/
│   ├── main.rs              # Main application logic
│   └── schema_types.rs      # COPIED TYPES - maintain this file
├── schemas/                 # Generated schema files stored here
│   ├── conversation_schema_YYYYMMDD_HHMMSS.json         # Hybrid approach
│   └── conversation_schema_YYYYMMDD_HHMMSS_schemars.json # Schemars approach
├── Cargo.toml               # Dependencies (includes schemars with chrono04)
├── validate_schemas.sh      # Automated validation script
├── SCHEMA_TYPES_MAINTENANCE.md     # This documentation
└── QUICK_MAINTENANCE_REFERENCE.md  # Quick reference guide
```

## Success Metrics

A successful maintenance update should result in:
1. ✅ `cargo build --release` succeeds without warnings
2. ✅ `./schema-tracker capture --schemars` generates complete schema
3. ✅ Generated schema contains no "unknown" or "null" types for known fields
4. ✅ All expected types appear in `$defs` section
5. ✅ Schema hash changes only when actual structure changes

## Emergency Rollback

If updates break schema generation:
1. Revert schema_types.rs to last working version
2. Use hybrid approach: `./schema-tracker capture` (without --schemars)
3. Fix issues incrementally
4. Test each change with `cargo build && ./schema-tracker capture --schemars`
