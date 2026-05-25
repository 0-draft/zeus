# Parallel agents UI

Multiple Zeus subagents run concurrently. Each one needs a visible, cancellable, observable surface — but **never as the editor's primary view**.

This is the deliberate counter-positioning vs Cursor 3 / Antigravity 2.0 / Windsurf Cascade, all of which made the agent manager the home screen. The market signal (Pragmatic Engineer Feb 2026: Claude Code "most loved" 46%, more than 2× Cursor) says many senior developers prefer agents as a tool, not as their workflow's center.

## Where the UI lives

An **auxiliary bar view** (the right-side container in VS Code's workbench — formally "auxiliary bar"; the "side bar" is the left-side container by VS Code's own naming). Not the activity bar's first slot, not the editor center. Closeable. Reopen via command palette.

```text
┌──┬───────────────────────────┬─ Parallel agents view ──────────┐
│  │                           │  ▶ docs-writer    streaming…    │
│A │   Editor (primary)        │     tool: read_file             │
│c │   user code, just like    │  ▶ test-writer    waiting on    │
│t │   VS Code                 │     tool approval               │
│iv│                           │  ▼ security-reviewer  done ·    │
│it│                           │      3 findings                 │
│y │                           │      findings.md modified       │
│  │                           │                                 │
├──┴───────────────────────────┴─────────────────────────────────┤
│ Status bar  ⚡ 2 agents running · $0.41 today · cache: 92% hit │
└────────────────────────────────────────────────────────────────┘
   ↑ status item is always visible; clicking it focuses the
     auxiliary-bar view (right by default, draggable to left).
     The status bar sits at the bottom — below both the editor
     and the auxiliary bar — per VS Code's workbench layout.
```

## Behavior

- New agent: `Ctrl+K Ctrl+N` (chord) opens a quick-pick of skills from `.zeus/skills/`, runs the chosen one. The earlier draft used `Ctrl+Shift+A`, but that clashes with VS Code's `toggleSearchEditorContextLines` in the Search Editor; using a chord keeps the new shortcut out of every default scope
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
- [ ] Auxiliary bar shows live tool-call timeline per agent
- [ ] Cancel works mid-stream
- [ ] Done agents persist in the view for the session
- [ ] No auto-open behavior on startup
- [ ] File edits are shown as diff, never applied without user "Apply"
- [ ] Keyboard: `Ctrl+K Ctrl+N` new agent, `Ctrl+K Ctrl+A` focus list — both chords, to avoid VS Code defaults (`Ctrl+Shift+A` is `toggleSearchEditorContextLines`; `Ctrl+Shift+L` is `editor.action.selectHighlights`)

## Status

Slot reserved at `src/vs/workbench/contrib/parallelAgents/`.
