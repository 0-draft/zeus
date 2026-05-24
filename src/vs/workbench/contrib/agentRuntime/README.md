# `agentRuntime` contribution

Slot for the Claude Agent SDK-backed subagent runtime. Design at [`docs/zeus-agent-sdk.md`](../../../../../docs/zeus-agent-sdk.md).

Planned layout (following vscode's layering convention — `common/` is platform-agnostic, `node/` is Node-only, `browser/` is renderer):

- `common/agentRuntime.ts` — `IAgentRuntime` interface, types
- `node/anthropicAgentRuntime.ts` — Claude Agent SDK impl (Node-only)
- `browser/agentRuntime.contribution.ts` — workbench registration
- `browser/skillLoader.ts` — `.zeus/skills/` → system prompt
- `browser/memoryLoader.ts` — `.zeus/memory/` → context block
- `browser/policyLoader.ts` — `.zeus/policy.md` → hard rules
- `test/node/*.test.ts` — unit tests against the Node impl

Anthropic-only on Day 1; abstraction allows OpenAI Assistants and Gemini Agent runtimes later without changing call sites.
