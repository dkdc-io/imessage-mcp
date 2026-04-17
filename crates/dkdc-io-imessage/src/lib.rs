//! iMessage MCP plugin.
//!
//! A standalone MCP server that exposes three tools to an LLM CLI (Codex CLI,
//! Claude Code, or any JSON-RPC stdio MCP client):
//!
//! - `reply(chat_id, text)` — send an iMessage via `osascript` → Messages.app.
//! - `list_messages(query, limit)` — search `~/Library/Messages/chat.db`.
//! - `read_message(id)` — fetch the full body of one message by GUID.
//!
//! All three are gated by an allowlist at `~/.config/dkdc-io/imessage/access.toml`.
//! Empty allowlist = fail closed. See README for setup.

pub mod access;
pub mod attributed;
pub mod cli;
pub mod db;
pub mod mcp;
pub mod send;
pub mod tools;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
