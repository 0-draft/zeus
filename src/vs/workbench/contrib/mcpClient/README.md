# `mcpClient` contribution

Slot for the built-in MCP client. Design lives at [`docs/zeus-mcp-client.md`](../../../../../docs/zeus-mcp-client.md).

When the real implementation lands, this directory will contain (VS Code layering: `common/` is platform-agnostic, `browser/` is renderer, `node/` is the Node-only half that can spawn subprocesses):

- `common/mcpTypes.ts` — shared types
- `common/mcpToolAggregator.ts` — unified, namespaced tool registry
- `browser/mcpClient.contribution.ts` — workbench registration + status bar
- `browser/mcpConfigLoader.ts` — `.zeus/mcp.json` watcher + schema validation
- `browser/mcpTrustPrompt.ts` — user confirmation before spawning new stdio servers
- `node/mcpStdioRegistry.ts` — per-server stdio child processes (cannot live in `browser/`)
- `node/mcpSseRegistry.ts` — SSE connections (can also be browser; placed here for symmetry)
- `test/node/*.test.ts` — unit tests for the spawn paths

Until then, this README is the placeholder so other PRs can reference the path without merge conflicts.
