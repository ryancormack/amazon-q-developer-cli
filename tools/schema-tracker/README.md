# Amazon Q CLI Schema Tracker

A standalone tool for tracking changes to the Amazon Q CLI conversation schema over time. This tool helps monitor how the `ConversationState` structure evolves as you merge upstream changes.

## Features

- **Schema Capture**: Generate and save timestamped schema snapshots
- **Diff Analysis**: Compare schemas between versions with detailed diffs
- **Field Usage Analysis**: Analyze real conversation files to see which fields are actually used
- **Validation**: Validate conversation files against the current schema
- **Git Integration**: Track schema changes alongside git commits

## Installation

From the Amazon Q CLI repository root:

```bash
cd tools/schema-tracker
cargo build --release
```

The binary will be available at `target/release/schema-tracker`.

## Usage

### Capture Current Schema

Generate a schema snapshot with timestamp:

```bash
# Basic capture
./schema-tracker capture

# With custom output directory and note
./schema-tracker capture -o ./my-schemas -n "Added new field for context tracking"
```

This creates files like `schemas/conversation_schema_20250819_220000.json` containing:
- Timestamp
- Git commit hash (if available)
- Schema hash for quick comparison
- Optional note
- Full JSON schema

### Compare Schemas

Compare two schema versions:

```bash
# Compare two specific files
./schema-tracker diff schema1.json schema2.json

# Compare with latest (finds most recent in same directory)
./schema-tracker diff schema1.json

# Show full diff instead of summary
./schema-tracker diff schema1.json schema2.json --full
```

### List All Captured Schemas

```bash
./schema-tracker list -s ./schemas
```

Output:
```
📋 Captured schemas:
  2025-08-19 22:00:00 | a1b2c3d4e5f6 | conversation_schema_20250819_220000.json
    📝 Added new field for context tracking
    🔗 abc12345
  2025-08-19 23:15:30 | f6e5d4c3b2a1 | conversation_schema_20250819_231530.json
    📝 Merged upstream changes
    🔗 def67890
```

### Analyze Real Conversation Files

Analyze actual saved conversations to understand field usage:

```bash
# Analyze all JSON files in current directory
./schema-tracker analyze

# Use specific pattern
./schema-tracker analyze -p "conversations/**/*.json"

# Save analysis to file
./schema-tracker analyze -o field_usage_report.json
```

This shows:
- Which fields are actually used in real conversations
- Usage percentages
- Sample values for each field
- Schema versions found in files

### Validate Conversations

Check if existing conversation files are compatible with current schema:

```bash
# Validate all JSON files
./schema-tracker validate

# Use specific pattern
./schema-tracker validate -p "saved_conversations/*.json"
```

## Workflow for Tracking Changes

### 1. Initial Setup

```bash
# Capture baseline schema
./schema-tracker capture -n "Initial schema before upstream merge"
```

### 2. Before Merging Upstream

```bash
# Capture current state
./schema-tracker capture -n "Before merging upstream $(date)"

# Analyze existing conversations
./schema-tracker analyze -p "my_conversations/*.json" -o pre_merge_analysis.json
```

### 3. After Merging Upstream

```bash
# Capture new schema
./schema-tracker capture -n "After merging upstream $(git log -1 --oneline)"

# Compare with previous
./schema-tracker diff schemas/conversation_schema_BEFORE.json

# Validate existing conversations still work
./schema-tracker validate -p "my_conversations/*.json"
```

### 4. Regular Monitoring

Set up a git hook or CI job to automatically capture schemas:

```bash
#!/bin/bash
# .git/hooks/post-merge
cd tools/schema-tracker
./schema-tracker capture -n "Auto-capture after merge $(git log -1 --oneline)"
```

## Output Examples

### Schema Diff Summary
```
Comparing schemas:
  From: 2025-08-19 22:00:00 (a1b2c3d4e5f6)
  To:   2025-08-19 23:15:30 (f6e5d4c3b2a1)

📊 Summary:
  Additions: 15
  Deletions: 3

🔍 Key changes:
  + "file_line_tracker": {
  +   "type": "object",
  +   "additionalProperties": {
  - "model": {
  -   "type": ["string", "null"]
  + "model_info": {
  +   "type": ["object", "null"]
```

### Field Usage Analysis
```json
{
  "total_files": 42,
  "field_usage": {
    "conversation_id": {
      "count": 42,
      "percentage": 100.0,
      "sample_values": ["uuid-1", "uuid-2"]
    },
    "model": {
      "count": 15,
      "percentage": 35.7,
      "sample_values": ["claude-3-sonnet", null]
    },
    "file_line_tracker": {
      "count": 8,
      "percentage": 19.0,
      "sample_values": [{}]
    }
  },
  "schema_versions": {
    "https://raw.githubusercontent.com/aws/amazon-q-developer-cli/main/schemas/conversation-v1.json": 25,
    "": 17
  }
}
```

## Integration with Your Fork

This tool is designed to work with your fork workflow:

1. **Keep it in your fork**: The tool lives in `tools/schema-tracker/` and won't conflict with upstream
2. **Track upstream merges**: Capture schemas before/after merging upstream changes
3. **Monitor compatibility**: Validate your saved conversations still work after updates
4. **Document changes**: Use notes to track what changed and why

The tool uses the actual `ConversationState` struct from your codebase, so it always reflects the current state of your fork.
