# `zeus mcp` — Zeus as an MCP server

Zeus exposes its editor surface as an MCP server. Any MCP-aware client (Claude Code CLI, ChatGPT desktop, another editor, a script) can read buffers, apply edits, run commands, and start subagents through one protocol.

Implementation lives in two places:

- `cli/src/commands/mcp.rs` (this PR): the entry point — `code mcp` / `zeus mcp` subcommand
- `src/vs/workbench/contrib/mcpServer/` (later PR): the actual tool implementations, which talk to the workbench

## Subcommand surface

```text
zeus mcp [--transport stdio|sse] [--port N] [--workspace PATH]
```

- `--transport`: defaults to `stdio` (Anthropic / Claude Code CLI standard). `sse` for HTTP clients.
- `--port`: only meaningful with `--transport sse`. Defaults to a random ephemeral port; the chosen port is printed on stderr (stdout is reserved for protocol traffic on the stdio transport, and we keep stderr consistent across transports).
- `--workspace`: workspace root path. Defaults to `$PWD`.

The Rust CLI launches a headless workbench process (or attaches to a running one if available) and proxies MCP traffic.

## Initial tool surface

```text
buffer_get(path)               -> { content, language }
buffer_set(path, content)      -> { ok }
edit_apply(path, range, text)  -> { ok }
diagnostics_get(path?)         -> { diagnostics[] }
selection_get()                -> { path, range, text } | null
command_run(name, args?)       -> { result }
visible_files()                -> { paths[] }
search_workspace(query, opts?) -> { matches[] }
agent_start(skill, prompt)     -> { agent_id }
agent_status(agent_id)         -> { state, progress? }
agent_cancel(agent_id)         -> { ok }
git_diff(staged?)              -> { diff }
lsp_definitions(path, pos)     -> { locations[] }
lsp_references(path, pos)      -> { locations[] }
```

Detailed schemas land alongside the implementation PR.

## Authentication

MCP over stdio inherits the calling process's permissions; no extra auth.

MCP over SSE binds to `127.0.0.1` only by default and requires a bearer token printed on **stderr** on start (`Token: ...`). Stdout is reserved for protocol traffic on the stdio transport, and we keep stderr consistent across transports. Binding to non-loopback (`--bind 0.0.0.0`, a LAN IP, etc.) additionally requires `--allow-non-loopback` as a confirmation flag.

## Workspace isolation

A single Zeus install can host multiple `zeus mcp` instances, one per workspace. Each instance is scoped to its workspace root and refuses operations on paths outside it.

## Why headless and not just an extension?

A VS Code extension only runs inside the editor's UI process. We want this to work when no editor is open — for example, a CI job that wants to call `buffer_get` after applying a refactor PR. The CLI gives us that decoupling.

## Acceptance criteria for the implementation PR (not this scaffold PR)

- `code mcp` boots, prints transport info, and accepts an MCP `initialize` request
- `buffer_get` and `buffer_set` round-trip a small file in tests
- Refuses paths outside the workspace
- Plays nicely as a subprocess of Claude Code CLI

## Status

Scaffold only. The Rust command is registered and prints a stub message; real implementation is a follow-up.
