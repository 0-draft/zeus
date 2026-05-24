# MCP client (built-in)

Zeus consumes MCP servers listed in `.zeus/mcp.json` and exposes their tools to the editor's AI features. This is the **client** half of the MCP-first design; the **server** half lives in `feat/mcp-server`.

## Where this lives

`src/vs/workbench/contrib/mcpClient/` — a workbench contribution split across the `common/`, `browser/`, and `node/` layers per vscode's architecture:

- `common/` — config schema, types, the workspace-side `McpToolAggregator` that hands a unified registry to the agent runtime
- `browser/` — UI surfaces (status bar entries, error notifications, the trust-prompt for newly-added stdio entries)
- `node/` — process spawning. `stdio` MCP servers must be launched from the main / node side; `browser/` cannot spawn child processes in vscode's sandbox. SSE connections can live in either layer

Behaviour:

- Reads `.zeus/mcp.json` from the workspace root
- Spawns stdio MCP servers (`node/`) as subprocesses, or opens SSE connections
- Aggregates the tool list and exposes it to the agent runtime
- Reloads on `.zeus/mcp.json` change

This is intentionally a built-in contribution rather than a VS Code extension. MCP server lifecycles are too important to let users disable accidentally; we want them tied to the workspace lifecycle.

## Trust model — RCE risk

`.zeus/mcp.json` lists *commands to execute*. Anyone with commit rights can add an arbitrary command, and a colleague who pulls and opens the workspace would silently spawn it. That's a real RCE vector.

Mitigations:

- **Trust prompt** — the first time a workspace is opened with a non-empty `mcp.json`, or whenever a new server entry is added in a subsequent pull, Zeus blocks startup of those servers and shows a per-server confirmation pane (similar to vscode's "Restricted Mode" workspace trust). Accepting writes a fingerprint into per-user (not in-git) state so the prompt doesn't re-fire on every edit.
- **Inherit Workspace Trust** — if the workspace is in Restricted Mode, refuse to spawn any stdio server. SSE-only entries can be allowed because they don't execute local code.
- **Hard refuse on `command: bash -c "<arbitrary>"` patterns** — block shell-form commands in `command:`; require `args:` arrays for argument vectors.

## Secret storage

`mcp.json` lives in git. We only allow `${env:NAME}` and `${secret:<store>:NAME}` references in `env` blocks for credentials:

- `${env:NAME}` resolves at spawn time from the user's environment
- `${secret:keychain:NAME}` reads from `vscode.SecretStorage` (per-user, OS-keychain backed)
- Plain string values are accepted only for non-secret config; the loader warns when a field name matches a heuristic list (`*_TOKEN`, `*_KEY`, `*_SECRET`, `PASSWORD`) and the value isn't a reference

Never write secret values back into the file. The loader is read-only against `mcp.json`.

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
- [ ] Collisions across servers are resolved by namespacing exposed tools as `<server-name>__<tool-name>` in the aggregated registry; the underlying call still goes to the originally-named tool on the right server. UI surfaces show the short tool name with the server name as secondary text.

## Status

Scaffold only. Slot reserved at `src/vs/workbench/contrib/mcpClient/`.
