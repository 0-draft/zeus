# `parallelAgents` contribution

Slot for the side-bar view that surfaces running and completed subagents. Design: [`docs/zeus-parallel-agents-ui.md`](../../../../../docs/zeus-parallel-agents-ui.md).

The view is deliberately **not** the editor's home surface — Zeus's counter-positioning vs Cursor 3 / Antigravity 2.0 / Windsurf Cascade.

## Why no `*.contribution.ts` yet

Registering a view container with no view body, or with a view that only renders "loading…", ships a broken UI as soon as this PR lands. The real registration is paired with the runtime stream in `feat/agent-sdk` (#26) so the view becomes interactive on day one.

What this PR *does* commit to:

- `viewContainerId`: `zeus.parallelAgents`
- `viewId`: `zeus.parallelAgents.list`
- Default location: right-side bar, `hideIfEmpty: true`
- Activity-bar slot: **not** taken; opened via command palette or by clicking the agent count in the status bar

Other branches reference these IDs from `common/parallelAgents.contribution.ts` once `feat/agent-sdk` lands. Treat this README as the canonical pin until then.
