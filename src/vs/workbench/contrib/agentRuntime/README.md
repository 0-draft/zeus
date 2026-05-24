# `agentRuntime` contribution

Slot for the Claude Agent SDK-backed subagent runtime. Design at [`docs/zeus-agent-sdk.md`](../../../../../docs/zeus-agent-sdk.md).

Planned layout (vscode layering: `common/` is platform-agnostic, `node/` is Node-only, `browser/` is renderer):

- `common/agentRuntime.ts` — `IAgentRuntime` interface, event/handle types
- `common/skillSchema.ts` — skill frontmatter parsing, prompt assembly (pure, no IO)
- `common/memorySchema.ts` — memory file shape, context-block builder (pure, no IO)
- `common/policySchema.ts` — policy parsing (pure, no IO)
- `node/skillLoader.ts` — `.zeus/skills/*.md` file IO + watcher, feeds `common/skillSchema`
- `node/memoryLoader.ts` — `.zeus/memory/**` file IO, feeds `common/memorySchema`
- `node/policyLoader.ts` — `.zeus/policy.md` file IO, feeds `common/policySchema`
- `node/anthropicAgentRuntime.ts` — Claude Agent SDK impl, consumes the loaders above
- `browser/agentRuntime.contribution.ts` — workbench registration that talks to `node/` via IPC
- `test/node/*.test.ts` — unit tests against the Node impl

Loaders live in `node/` because reading from `.zeus/` is real file IO. The schema / prompt-assembly logic stays in `common/` so the same code can run in tests, and so `node/anthropicAgentRuntime` and any future runtime can call it without crossing layers.

Anthropic-only on Day 1; abstraction allows OpenAI Assistants and Gemini Agent runtimes later without changing call sites.
