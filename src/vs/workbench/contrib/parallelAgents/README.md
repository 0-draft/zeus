# `parallelAgents` contribution

Slot for the side-bar view that surfaces running and completed subagents. Design: [`docs/zeus-parallel-agents-ui.md`](../../../../../docs/zeus-parallel-agents-ui.md).

The view is deliberately **not** the editor's home surface — Zeus's counter-positioning vs Cursor 3 / Antigravity 2.0 / Windsurf Cascade.

This README is the slot marker; the actual `*.contribution.ts` lands together with `IAgentRuntime` in `feat/agent-sdk`, because the view body is meaningless without the runtime event stream it subscribes to.
