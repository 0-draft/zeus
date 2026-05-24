# MCP client (built-in)

Zeus consumes MCP servers listed in `.zeus/mcp.json` and exposes their tools to the editor's AI features. This is the **client** half of the MCP-first design; the **server** half lives in `feat/mcp-server`.

## Where this lives

`src/vs/workbench/contrib/mcpClient/` — a workbench contribution that:

- Reads `.zeus/mcp.json` from the workspace root
- Spawns stdio MCP servers as subprocesses, or opens SSE connections
- Aggregates the tool list and exposes it to the agent runtime
- Reloads on `.zeus/mcp.json` change

This is intentionally a built-in contribution rather than a VS Code extension. MCP server lifecycles are too important to let users disable accidentally; we want them tied to the workspace lifecycle.

## Why not [VS Code's `vscode.lm` tool API](https://code.visualstudio.com/api/extension-guides/tools)?

We want the MCP-first stance to be honest. VS Code's `vscode.lm.registerTool` is a fine API but it's vscode-specific. By going through `@modelcontextprotocol/sdk` directly, the same `.zeus/mcp.json` works in:

- Claude Code CLI (already MCP-native)
- ChatGPT desktop / Codex (MCP support shipping)
- Future agents (MCP is an open spec)

VS Code extensions can still register `lm` tools — those continue to work — but Zeus's first-class story is MCP.

## Loader

```text
.zeus/mcp.json
  ↓
McpConfigLoader (watches file, validates schema)
  ↓
McpClientRegistry (one McpClient per server entry)
  ↓
McpToolAggregator (combined tool list, dispatches calls)
  ↓
IAgentRuntime (Agent SDK PR consumes this)
```

## Sub-PRs needed before this can land

1. `feat/zeus-conventions` (`.zeus/mcp.json` schema) — PR #23
2. This PR — scaffold + design
3. Follow-up — `@modelcontextprotocol/sdk` dep + real implementation

## Acceptance criteria (real impl)

- [ ] Loads `.zeus/mcp.json` at workbench startup
- [ ] Spawns each `stdio` server as a child process
- [ ] Connects to each `sse` server with bearer auth
- [ ] Aggregates all tool definitions into a single registry
- [ ] Reloads on file change without restarting unaffected servers
- [ ] Surfaces server connection errors in the status bar
- [ ] Refuses servers that try to register tools with reserved name prefixes (`buffer_`, `agent_`, `editor_` — those belong to the MCP **server** half)

## Status

Scaffold only. Slot reserved at `src/vs/workbench/contrib/mcpClient/`.
