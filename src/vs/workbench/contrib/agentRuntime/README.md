# `agentRuntime` contribution

Slot for the Claude Agent SDK-backed subagent runtime. Design at [`docs/zeus-agent-sdk.md`](../../../../../../docs/zeus-agent-sdk.md).

Planned layout:

- `common/agentRuntime.ts` — `IAgentRuntime` interface, types
- `common/anthropicAgentRuntime.ts` — Claude Agent SDK impl
- `browser/agentRuntime.contribution.ts` — workbench registration
- `browser/skillLoader.ts` — `.zeus/skills/` → system prompt
- `browser/memoryLoader.ts` — `.zeus/memory/` → context block
- `browser/policyLoader.ts` — `.zeus/policy.md` → hard rules
- `test/browser/*.test.ts` — unit tests

Anthropic-only on Day 1; abstraction allows OpenAI Assistants and Gemini Agent runtimes later without changing call sites.
