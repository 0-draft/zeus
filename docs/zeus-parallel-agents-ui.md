# Parallel agents UI

Multiple Zeus subagents run concurrently. Each one needs a visible, cancellable, observable surface — but **never as the editor's primary view**.

This is the deliberate counter-positioning vs Cursor 3 / Antigravity 2.0 / Windsurf Cascade, all of which made the agent manager the home screen. The market signal (Pragmatic Engineer Feb 2026: Claude Code "most loved" 46%, more than 2× Cursor) says many senior developers prefer agents as a tool, not as their workflow's center.

## Where the UI lives

A **side bar view** (not the activity bar's first slot, not the editor center). Closeable. Reopen via command palette.

```text
┌─ Editor (primary surface) ──────────────────────────────────┐
│                                                              │
│   user code, just like vscode                                │
│                                                              │
├─ Status bar (always visible) ────────────────────────────────┤
│  ⚡ 2 agents running · $0.41 today · cache: 92% hit          │
└──────────────────────────────────────────────────────────────┘
        ↑ click status item → opens side bar view ↓
┌─ Parallel agents view (collapsible side bar) ───────────────┐
│  ▶ docs-writer       streaming…   tool: read_file           │
│  ▶ test-writer       waiting on tool approval               │
│  ▼ security-reviewer  done · 3 findings                      │
│       findings.md modified                                   │
└──────────────────────────────────────────────────────────────┘
```

## Behavior

- New agent: `Ctrl+Shift+A` opens a quick-pick of skills from `.zeus/skills/`, runs the chosen one
- Each agent: own subtree in the view with timeline (tool calls, text deltas), final summary, cancel button, "open in chat" button
- Done agents move to "Recent" collapsed group; never auto-dismissed
- A finished agent's file edits are surfaced as a diff PR-style review, not auto-applied
- Notifications: agent completion → optional status bar pulse, no modal

## What this UI does **not** do

- It is **not** the editor home page
- It does not auto-open on startup unless the user pinned it
- It does not show a "create a project plan" or "decompose this PRD" prompt
- It does not encourage running 10 agents at once just because you can

## File contracts

- Source of truth for runnable skills: `.zeus/skills/*.md` (loaded by `feat/agent-sdk`)
- Source of truth for running agents: `IAgentRuntime` events (from `feat/agent-sdk`)
- Cost / cache state: subscribed to from `feat/prompt-cache-hud` shared store

## Acceptance criteria (real impl)

- [ ] Quick-pick of skills picks one and starts it
- [ ] Side bar shows live tool-call timeline per agent
- [ ] Cancel works mid-stream
- [ ] Done agents persist in the view for the session
- [ ] No auto-open behavior on startup
- [ ] File edits are shown as diff, never applied without user "Apply"
- [ ] Keyboard: `Ctrl+Shift+A` new agent, `Ctrl+K Ctrl+A` focus list (chord, to avoid VS Code's `Ctrl+Shift+L` = `editor.action.selectHighlights` needed for multi-cursor)

## Status

Slot reserved at `src/vs/workbench/contrib/parallelAgents/`.
