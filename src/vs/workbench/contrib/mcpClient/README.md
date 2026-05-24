# `mcpClient` contribution

Slot for the built-in MCP client. Design lives at [`docs/zeus-mcp-client.md`](../../../../../docs/zeus-mcp-client.md).

When the real implementation lands, this directory will contain:

- `browser/mcpClient.contribution.ts` — workbench registration
- `browser/mcpConfigLoader.ts` — `.zeus/mcp.json` watcher
- `browser/mcpClientRegistry.ts` — per-server clients
- `browser/mcpToolAggregator.ts` — unified tool registry
- `common/mcpTypes.ts` — shared types
- `test/browser/*.test.ts` — unit tests

Until then, this README is the placeholder so other PRs can reference the path without merge conflicts.
