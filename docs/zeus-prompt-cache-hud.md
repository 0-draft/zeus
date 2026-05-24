# Prompt cache HUD

A status bar item that shows AI cost and prompt-cache state in real time. The point is transparency: developers should be able to feel each AI call's cost so they can change behavior, not be surprised at the end of the month.

This is positioned directly against Cursor's credit model, which most users describe as opaque. We show the raw numbers; users can hide it if they don't care.

## Status bar layout

```text
⚡ 2 agents · 92% cache · $0.003 / $0.41 today
```

- **`⚡ N agents`**: number of currently-running subagents. Clickable → opens parallel-agents view
- **`X% cache`**: rolling cache hit ratio over the last 100 requests
- **`$X.XXX`**: cost of the most recent AI call
- **`$X.XX today`**: cumulative cost for the local day (resets midnight)

Hovering over each segment shows a tooltip with the breakdown (input tokens / output tokens / cached tokens / cost per million).

## Source of data

- Live agent count: `IAgentRuntime` event stream (`feat/agent-sdk`)
- Per-call cost and cache state: Anthropic SDK `usage` field, mapped to current model pricing
- Today's cumulative: persisted to `IStorageService` **`APPLICATION`** (per-user, cross-workspace) scope under a single key `zeus.ai.cost` holding `{ date: "YYYY-MM-DD", total: number }`. Application scope (not workspace) because the user's per-day spend should not reset when switching between workspaces — the goal is to surface real cost, not per-project cost. When local midnight passes, the record is reset before the next write, so storage doesn't grow per day. A future `zeus.ai.hud.scope` setting can flip it to `WORKSPACE` if a user wants per-project tracking.

## Configuration

- `zeus.ai.hud.enabled` (default: `true`) — show the HUD at all
- `zeus.ai.hud.detail` (`"compact" | "verbose"`) — controls the format
- `zeus.ai.hud.todayLimit` (number | null) — soft cap; turns the cost segment red when exceeded, no enforcement

The HUD is implemented as **multiple adjacent `StatusBarItem`s** (agents, cache, cost, today). VS Code's `StatusBarItem` API does not support per-segment coloring inside a single item, so the colored "over limit" treatment lives on its own item.

## Why no enforcement

Hard credit caps are what makes Cursor frustrating. Zeus shows the number; the user decides whether to stop. If you want enforcement, the local LLM path or a custom MCP proxy can give you that.

## Acceptance criteria (real impl)

- [ ] Status bar item appears when an Anthropic AI feature is configured
- [ ] Real-time updates within ~200ms of each request completing
- [ ] Hover tooltip shows token / cost breakdown
- [ ] `today` value persists across editor restarts in the same local day
- [ ] Setting `zeus.ai.hud.enabled = false` hides the item entirely
- [ ] Pricing table lives in code, not over the network (no live pricing fetch)

## Status

Slot reserved at `src/vs/workbench/contrib/aiHud/`. Depends on `IAgentRuntime` (`feat/agent-sdk`).
