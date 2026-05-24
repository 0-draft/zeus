# `agentRuntime` contribution

Slot for the Claude Agent SDK-backed subagent runtime. Design at [`docs/zeus-agent-sdk.md`](../../../../../docs/zeus-agent-sdk.md).

Planned layout (vscode layering: `common/` is platform-agnostic, `node/` is Node-only, `browser/` is renderer):

- `common/agentRuntime.ts` — `IAgentRuntime` interface, event/handle types
- `common/skillSchema.ts` — skill frontmatter parsing, prompt assembly (pure, no IO)
- `common/memorySchema.ts` — memory file shape, context-block builder (pure, no IO)
- `common/policySchema.ts` — policy parsing (pure, no IO)
- `common/skillLoader.ts` — `.zeus/skills/*.md` reader, built on `IFileService` so it works in desktop / remote / web
- `common/memoryLoader.ts` — `.zeus/memory/**` reader on `IFileService`
- `common/policyLoader.ts` — `.zeus/policy.md` reader on `IFileService`
- `node/anthropicAgentRuntime.ts` — Claude Agent SDK impl (Node-only because the SDK spawns child processes for MCP servers and uses Node streams). Consumes the `common/` loaders.
- `browser/agentRuntime.contribution.ts` — workbench registration that talks to `node/` via IPC
- `test/common/*.test.ts` — schema / loader tests
- `test/node/*.test.ts` — Anthropic runtime tests

Loaders sit in `common/` and go through `IFileService` rather than `fs.promises`, so the same code runs in desktop, remote-SSH and web (where there is no Node FS). The `node/` layer is reserved for things that genuinely need Node — the Claude Agent SDK itself spawns child processes for MCP stdio servers and uses Node streams, so the Anthropic runtime stays there.

Anthropic-only on Day 1; abstraction allows OpenAI Assistants and Gemini Agent runtimes later without changing call sites.
