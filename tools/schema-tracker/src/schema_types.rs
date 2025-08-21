// ============================================================================
// SCHEMA TYPES - COPIED FROM UPSTREAM FOR JSON SCHEMA GENERATION
// ============================================================================
//
// ⚠️  MAINTENANCE REQUIRED: These are COPIES of types from the main codebase
//
// PURPOSE: Enable complete JSON Schema generation via schemars without 
//          modifying upstream code or dealing with merge conflicts
//
// MAINTENANCE: When upstream types change, update these copies accordingly
//              See SCHEMA_TYPES_MAINTENANCE.md for detailed instructions
//
// LAST UPDATED: 2025-08-20 (Initial creation)
// UPSTREAM COMMIT: 71c00814247c3c2d6e134c3cbd0f23f6745b1466
//
// KEY CHANGES FROM UPSTREAM:
// 1. Added JsonSchema derives to all types
// 2. Added PartialEq, Eq, Hash to HashMap key types (ToolOrigin)
// 3. Simplified some complex types to avoid deep dependency chains
// 4. Added documentation comments for better schema generation
//
// ============================================================================

// Schema-specific copies of ConversationState and related types
// These are copies from the main codebase with JsonSchema derives added
// This allows us to generate complete schemas without modifying upstream code

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::collections::{HashMap, VecDeque};
use chrono::{DateTime, Utc};

/// Schema-aware copy of ConversationState
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConversationState {
    /// Unique identifier for this conversation
    pub conversation_id: String,
    /// Next message to be processed (if any)
    pub next_message: Option<UserMessage>,
    /// History of conversation turns
    pub history: VecDeque<HistoryEntry>,
    /// Valid range for history indexing
    pub valid_history_range: [usize; 2],
    /// Full conversation transcript
    pub transcript: Vec<TranscriptEntry>,
    /// Available tools organized by origin
    pub tools: HashMap<ToolOrigin, Vec<Tool>>,
    /// Context manager for file/resource context
    pub context_manager: Option<ContextManager>,
    /// Length of context messages
    pub context_message_length: Option<usize>,
    /// Latest conversation summary with metadata
    pub latest_summary: Option<(String, RequestMetadata)>,
    /// Model configuration
    pub model: Option<String>,
    /// Detailed model information
    pub model_info: Option<ModelInfo>,
    /// File line tracking for modifications
    pub file_line_tracker: HashMap<String, FileLineTracker>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HistoryEntry {
    pub user: UserMessage,
    pub assistant: AssistantMessage,
    pub request_metadata: Option<RequestMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UserMessage {
    pub additional_context: String,
    pub env_context: UserEnvContext,
    pub timestamp: Option<DateTime<Utc>>,
    pub images: Option<Vec<ImageBlock>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AssistantMessage {
    pub message_id: String,
    pub content: Vec<ContentBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UserEnvContext {
    pub operating_system: String,
    pub architecture: String,
    pub current_directory: String,
    pub env_state: Option<EnvState>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ContentBlock {
    pub content_type: String,
    pub text: Option<String>,
    pub tool_use: Option<ToolUse>,
    pub tool_result: Option<ToolResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ToolUse {
    pub tool_use_id: String,
    pub name: String,
    pub input: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ToolResult {
    pub tool_use_id: String,
    pub content: String,
    pub status: ToolResultStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum ToolResultStatus {
    Success,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TranscriptEntry {
    pub role: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq, Hash)]
pub enum ToolOrigin {
    Native,
    McpServer(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
    pub tool_origin: ToolOrigin,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ContextManager {
    pub current_profile: String,
    pub paths: Vec<String>, // Simplified for schema purposes
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RequestMetadata {
    pub request_id: Option<String>,
    pub message_id: String,
    pub conversation_id: String,
    pub response_size: usize,
    pub chat_conversation_type: Option<ChatConversationType>,
    pub tool_use_ids_and_names: Vec<(String, String)>,
    pub model_id: Option<String>,
    pub message_meta_tags: Vec<MessageMetaTag>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ModelInfo {
    pub model_id: String,
    pub model_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FileLineTracker {
    pub last_line_count: usize,
    pub user_lines_added: usize,
    pub agent_lines_added: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ImageBlock {
    pub image_type: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EnvState {
    pub variables: HashMap<String, String>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, JsonSchema)]
pub enum ChatConversationType {
    NotToolUse,
    ToolUse,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, JsonSchema)]
pub enum MessageMetaTag {
    Compact,
}
