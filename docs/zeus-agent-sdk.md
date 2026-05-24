# Claude Agent SDK integration

Zeus uses [`@anthropic-ai/claude-agent-sdk`](https://www.npmjs.com/package/@anthropic-ai/claude-agent-sdk) as the runtime for subagents. Each subagent (typescript-expert, test-writer, security-reviewer, …) is a managed session that owns a tool budget, a system prompt sourced from `.zeus/skills/*.md`, and a UI slot in the parallel-agents view.

This document is the design. Implementation lives in `src/vs/workbench/contrib/agentRuntime/` (slot reserved by this PR).

## Why Claude Agent SDK as the primitive

- Already a tested production runtime (powers Claude Code, hundreds of thousands of dev sessions/day)
- First-class support for the things Zeus wants: streaming, tool use, sub-agents, prompt caching, hooks, memory, MCP tool integration
- Same `mcp.json` shape as Claude Code → drop-in compatibility with what users have already configured

## Abstraction (provider neutrality)

Day 1 ships only Anthropic. To avoid betting the editor on a single vendor, the runtime sits behind an `IAgentRuntime` interface:

```ts
interface IAgentRuntime {
  start(opts: AgentStartOptions): Promise<AgentHandle>;
  status(id: AgentId): Promise<AgentStatus>;
  cancel(id: AgentId): Promise<void>;
  onEvent: Event<AgentEvent>;  // streaming tool-use / text events
}
```

`AnthropicAgentRuntime` (Claude Agent SDK) is the first implementation. `OpenAIAssistantsRuntime` and `GeminiAgentRuntime` are placeholders for later — same interface, different SDK behind it.

## Where settings come from

- System prompt: `.zeus/skills/<skill>.md` body (frontmatter trimmed)
- Tools: union of (a) MCP tools registered via the MCP client (`feat/mcp-client`) and (b) skill-declared `allowed-tools` (whitelist)
- Memory: `.zeus/memory/**` injected at session start
- Constitutional: `.zeus/policy.md` hard rules as system prompt append
- Model: `IConfigurationService` setting `zeus.ai.model` (default `claude-sonnet-4-5`)
- Credentials: secret storage (`vscode.SecretStorage`), keyed per provider

Everything except credentials is in git.

## Streaming UI

Agent events are forwarded to `feat/parallel-agents-ui`. Each agent's tab shows:

- Current tool call (with arguments)
- Streamed text deltas
- A timeline of tool calls so far
- A cancel button

## Prompt caching

The runtime opts into Anthropic's [prompt caching](https://docs.anthropic.com/en/docs/build-with-claude/prompt-caching) for:

- The system prompt (skill + memory + policy)
- The codebase context block

`feat/prompt-cache-hud` surfaces cache hit/miss state in the status bar.

## Acceptance criteria (real impl)

- [ ] `agentRuntime.start({ skill: 'docs-writer', prompt: 'document this function' })` returns a handle
- [ ] Tool-use events stream to the parallel-agents view
- [ ] Cancel works mid-tool-call
- [ ] Cost is reported correctly to the HUD
- [ ] Same skill file produces a reproducible system prompt across providers (Anthropic, future OpenAI / Gemini)

## Status

`@anthropic-ai/claude-agent-sdk` is already a top-level dependency (currently `0.2.128`, latest `0.3.150`). This PR only reserves the slot and documents the design. Implementation and a likely SDK version bump land in a follow-up.

## Notes on bumping

When we bump `@anthropic-ai/claude-agent-sdk` to `0.3.x`, follow the upgrader checklist in `extensions/copilot/.claude/CLAUDE.md` (testing core functionality / tools / hooks / slash commands). The copilot extension also pins this SDK so they move together.
