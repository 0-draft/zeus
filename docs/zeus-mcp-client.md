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

- **Trust prompt** — the first time a workspace is opened with a non-empty `mcp.json`, *or* whenever a server entry is added **or modified** (any change to `command`, `args`, `env`, `url`, or `transport`), Zeus blocks startup of those servers and shows a per-server confirmation pane (similar to vscode's "Restricted Mode" workspace trust). The fingerprint stored in per-user (not in-git) state is a hash of the entire normalised server-config object — any tweak invalidates the prior consent so a colleague editing in `args:` re-prompts the user.
- **Inherit Workspace Trust** — if the workspace is in Restricted Mode, refuse to spawn any server (stdio *and* SSE). A remote SSE endpoint never executes local code itself, but the tools it exposes can still cause file writes, shell calls, or prompt-injection via the agent, so the trust prompt covers it too.
- **Refuse shell wrappers, not just `bash -c`** — `command:` must resolve to an actual executable path; argument vectors must go through `args:`. Reject `command:` values whose basename matches any shell (`sh`, `bash`, `zsh`, `ksh`, `fish`, `pwsh`, `cmd`, `cmd.exe`, `powershell`, `powershell.exe`) when paired with a `-c` / `/c` / `-Command` flag in `args:`. The point is to make the executable + argv structurally visible, not to chase shell-specific bypasses.

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
- [ ] Zeus's own MCP **server** half publishes its tools under a `zeus_` prefix (e.g. `zeus_buffer_read`, `zeus_editor_open`). Third-party servers are free to use any name they like — including `buffer_` or `editor_` — because tool name conflicts across servers are resolved by the `<server-name>__<tool-name>` namespacing rule below, not by reserving a global prefix
- [ ] Collisions across servers are resolved by namespacing exposed tools as `<server-name>__<tool-name>` in the aggregated registry; the underlying call still goes to the originally-named tool on the right server. UI surfaces show the short tool name with the server name as secondary text.

## Status

Scaffold only. Slot reserved at `src/vs/workbench/contrib/mcpClient/`.
