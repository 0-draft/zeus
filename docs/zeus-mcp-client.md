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

- **Trust prompt** — the first time a workspace is opened with a non-empty `mcp.json`, *or* whenever a server entry is added **or modified** (any change to `command`, `args`, `env`, `url`, or `transport`), Zeus blocks startup of those servers and shows a per-server confirmation pane (similar to VS Code's "Restricted Mode" workspace trust). The fingerprint stored in per-user (not in-git) state is `sha256(canonical_json(serverConfig))`, where `canonical_json` (a) sorts object keys lexicographically at every depth, (b) resolves relative `command:` paths against the workspace root to absolute, (c) drops trailing whitespace in env values, and (d) uses LF line endings. Equal-meaning configurations therefore hash identically across platforms and JSON serializers, and any tweak invalidates the prior consent.
- **Inherit Workspace Trust** — if the workspace is in Restricted Mode, refuse to spawn any server (stdio *and* SSE). A remote SSE endpoint never executes local code itself, but the tools it exposes can still cause file writes, shell calls, or prompt-injection via the agent, so the trust prompt covers it too.
- **Refuse shell wrappers, not just `bash -c`** — `command:` must resolve to an actual executable path; argument vectors must go through `args:`. Reject `command:` values whose basename matches any shell (`sh`, `dash`, `bash`, `zsh`, `ksh`, `fish`, `pwsh`, `cmd`, `cmd.exe`, `powershell`, `powershell.exe`) when paired with an execution flag in `args:`: `-c` / `/c` / `-Command` / `-EncodedCommand`. (PowerShell's `-EncodedCommand` is a common obfuscation vector and gets the same treatment as `-Command`.) The point is to make the executable + argv structurally visible, not to chase shell-specific bypasses.

## Secret storage

`mcp.json` lives in git. We only allow `${env:NAME}` and `${secret:<store>:NAME}` references in `env` blocks for credentials:

- `${env:NAME}` resolves at spawn time from the user's environment
- `${secret:keychain:NAME}` reads from `vscode.SecretStorage` (per-user, OS-keychain backed)
- Plain string values are accepted only for non-secret config. The loader **refuses to start** a server (rather than just warning) when a field name matches the heuristic list (`*_TOKEN`, `*_KEY`, `*_SECRET`, `PASSWORD`) and the value is not a `${env:...}` / `${secret:...}` reference. The user sees a per-server error in the status bar with a "Move to keychain" quick-fix that creates the secret and rewrites the reference for them.
- A separate high-entropy heuristic (≥ 32 chars, base64 / hex-ish) on **any** plain `env` value triggers the same refuse-and-prompt path, so secrets that don't match the field-name list still get caught.

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
- [ ] Connects to each `sse` server with bearer auth, where the token **must** come from a `${secret:keychain:...}` (or `${env:...}`) reference in the server entry — hard-coded bearer tokens in `mcp.json` are refused by the same secret-storage rule above
- [ ] Aggregates all tool definitions into a single registry
- [ ] Reloads on file change without restarting unaffected servers
- [ ] Surfaces server connection errors in the status bar
- [ ] Zeus's own MCP **server** half publishes its tools under the `zeus__` prefix (double underscore, matching the third-party namespacing rule below — e.g. `zeus__buffer_read`, `zeus__editor_open`). The double-underscore separator means a third-party server *literally* named `zeus` does not collide with our internal surface
- [ ] **All** third-party tools are always exposed as `<server-name>__<tool-name>` in the aggregated registry, regardless of whether another server has registered the same short name. Always-namespacing (rather than namespacing-on-collision) means the tool name the agent sees is stable: it does not change when a new server is added later. UI surfaces still show the short tool name with the server name as secondary text. The underlying call dispatches to the originally-named tool on the right server.

## Status

Scaffold only. Slot reserved at `src/vs/workbench/contrib/mcpClient/`.
