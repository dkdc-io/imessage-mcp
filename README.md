# imessage-plugin

An iMessage MCP plugin for any LLM CLI that speaks MCP over stdio —
[Codex CLI](https://github.com/openai/codex),
[Claude Code](https://docs.claude.com/en/docs/claude-code), or your own harness.

Install one binary, point your CLI at it, and the LLM can:

- send iMessage via `reply(chat_id, text)`,
- search your messages via `list_messages(query, limit)`,
- fetch one message by GUID via `read_message(id)`.

Gated by a local allowlist. Fail-closed by default.

## Install

```sh
cargo install dkdc-io-imessage
```

Then grant the terminal (or the CLI's host process) Full Disk Access on macOS
so it can read `~/Library/Messages/chat.db`, and edit
`~/.config/dkdc-io/imessage/access.toml` to add at least one handle or a
`self.chat_id`.

Details + Codex and Claude Code config snippets live in the
[crate README](crates/dkdc-io-imessage/README.md).

## Repo layout

```
imessage-plugin/
  Cargo.toml                              # workspace
  LICENSE-MIT / LICENSE-APACHE
  crates/
    dkdc-io-imessage/                     # the MCP server crate
      Cargo.toml
      README.md
      src/
      tests/injection.rs                  # allowlist + osascript argv tests
      tests/stdio_smoke.rs                # end-to-end JSON-RPC smoke
```

One crate, one binary. No netsky coupling.

## Develop

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

All three run clean on macOS.

## License

Dual MIT OR Apache-2.0.
