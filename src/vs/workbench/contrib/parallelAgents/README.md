# `parallelAgents` contribution

Slot for the auxiliary-bar view that surfaces running and completed subagents. Design: [`docs/zeus-parallel-agents-ui.md`](../../../../../docs/zeus-parallel-agents-ui.md).

The view is deliberately **not** the editor's home surface — Zeus's counter-positioning vs Cursor 3 / Antigravity 2.0 / Windsurf Cascade.

## Canonical IDs

The view container, view, and command identifiers are exported from [`common/parallelAgents.ts`](./common/parallelAgents.ts):

- `PARALLEL_AGENTS_VIEW_CONTAINER_ID = 'zeus.parallelAgents'`
- `PARALLEL_AGENTS_VIEW_ID = 'zeus.parallelAgents.list'`
- `PARALLEL_AGENTS_COMMAND_NEW = 'zeus.parallelAgents.new'`
- `PARALLEL_AGENTS_COMMAND_FOCUS = 'zeus.parallelAgents.focus'`

Other branches (status-bar HUD, command palette, tests) should import these constants rather than retype the string IDs. Default location: right-hand **auxiliary bar**, `hideIfEmpty: true`. The activity bar's first slot is intentionally not taken; the view is opened via command palette or by clicking the agent count in the status bar.

## Why no `*.contribution.ts` yet

Registering a view container with no view body, or with a view that only renders "loading…", ships a broken UI as soon as this PR lands. The real registration is paired with the runtime stream in `feat/agent-sdk` (#26), where the view becomes interactive on day one and consumes the constants in `common/parallelAgents.ts`.
