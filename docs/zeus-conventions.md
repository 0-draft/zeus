# `.zeus/` conventions

Zeus reads project-local AI configuration from `.zeus/` at the workspace root. Everything in this directory is meant to be checked into git so the team shares one set of AI behaviours, not per-developer settings.

This document is the format spec. Implementation lands in [`feat/mcp-client`](https://github.com/0-draft/zeus/pulls?q=is%3Apr+head%3Afeat/mcp-client) and the agent SDK PR.

## Directory layout

```text
.zeus/
├── skills/                # team-shared AI behaviours
│   ├── secret-scan.md
│   └── commit-msg.md
├── memory/                # persistent project context
│   ├── README.md
│   ├── architecture.md
│   └── decisions/
│       └── 0001-mcp-first.md
├── policy.md              # constitutional rules (hard / soft)
└── mcp.json               # MCP servers to connect to
```

## `.zeus/skills/*.md`

Skill files share the same shape as Claude Code skills: a frontmatter block, then prose that becomes the skill's system prompt.

```markdown
---
name: secret-scan
description: Scan the buffer for secrets and propose .env.example replacements
allowed-tools: buffer_get, search_workspace
---

Scan recent edits for:

- AWS access keys (`\b(AKIA|ASIA)[A-Z0-9]{16}\b`)
- Private key headers (`-----BEGIN.*PRIVATE KEY-----`)
- GitHub tokens (prefixes `ghp_`, `gho_`, `ghu_`, `ghs_`, `ghr_`, `github_pat_`)
- Long alphanumeric values assigned to `*_KEY` / `*_SECRET` / `*_TOKEN` variables

When something matches, annotate the line and propose a `.env.example` placeholder.
```

Frontmatter fields:

- `name` (required): unique identifier within `.zeus/skills/`. Duplicates fail loading.
- `description` (required): one-line summary used by the model to decide when to invoke the skill.
- `allowed-tools` (optional): comma-separated list of MCP tool names this skill may call. **Omitted** = no tools allowed (least-privilege default — the skill is read-only unless it explicitly opts in); **present but empty** = also no tools allowed; **present with names** = exactly those tools. Use `'*'` to mean "all tools" if you really want that.

## `.zeus/memory/`

Persistent context that survives across sessions and developers.

- `README.md`: the entry point. Always injected at the start of an AI session.
- Free-form markdown files alongside: architecture notes, conventions, ADRs.

The AI may propose writes to memory via a diff that the user reviews. Manual edits are encouraged.

## `.zeus/policy.md`

Constitutional rules. Two sections:

```markdown
# Zeus policy for this project

## Hard rules

- Never write secrets, tokens, or passwords back into a buffer.
- Confirm before `rm -rf`, `DROP TABLE`, `git push --force`, `git push --force-with-lease`, or any destructive irreversible action.
- Do not auto-run database migrations. Generating them is fine; applying them requires explicit user approval.
- Do not call write APIs against production URLs (e.g., `*.prod.*`, `app.example.com`) unless the user explicitly opts in for the session.

## Soft preferences

- Imports are alphabetical.
- TypeScript function return types are explicit.
- Prefer `Result<T, E>` over throwing.
```

Hard rules become refusal-grade constraints. Soft preferences become style guidance.

## `.zeus/mcp.json`

MCP servers to connect to. Same shape as Claude Code's `mcp.json` for compatibility:

```json
{
  "$schema": "https://zeus.dev/schemas/mcp.json",
  "servers": {
    "github": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-github"],
      "env": { "GITHUB_TOKEN": "${env:GITHUB_TOKEN}" }
    },
    "postgres-staging": {
      "command": "uvx",
      "args": ["mcp-server-postgres"],
      "env": { "POSTGRES_URL": "${env:POSTGRES_STAGING_URL}" }
    },
    "linear": {
      "transport": "sse",
      "url": "https://mcp.linear.app/sse"
    }
  }
}
```

Secrets are referenced via `${env:NAME}` only. Plain values in `env` blocks are still allowed for non-secret config, but `mcp.json` is meant to live in git so don't commit secrets there.

## Reload behaviour (planned)

This is the target behaviour for the loaders in [`feat/mcp-client`](https://github.com/0-draft/zeus/pulls?q=is%3Apr+head%3Afeat/mcp-client) and [`feat/agent-sdk`](https://github.com/0-draft/zeus/pulls?q=is%3Apr+head%3Afeat/agent-sdk). **None of it is implemented in this PR**; this PR ships the on-disk format only.

What we plan to ship:

- On Zeus start, the entire `.zeus/` tree is read.
- Editing any file under `.zeus/` triggers an incremental reload of the affected piece (skill / memory / policy / mcp.json).
- `mcp.json` reload restarts only the affected MCP server connections, not all of them.

The closest existing piece in the inherited vscode codebase is the MCP `dev.watch` restart logic in `src/vs/workbench/contrib/mcp/common/mcpConfiguration.ts` and `mcpDevMode.ts`. That watches arbitrary file globs declared per-server, not `.zeus/mcp.json` itself. Zeus's reload spec sits one layer above it.

## Open questions

- Should `.zeus/skills/` allow nested directories (e.g., `skills/team-a/`)? Default: no, keep flat for simplicity.
- Symlinks: support or refuse? Default: refuse, to keep behaviour predictable across OSes.
- Per-user override layer (`~/.config/zeus/`) — separate spec, not part of this PR.
