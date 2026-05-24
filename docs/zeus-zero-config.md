# Zero-config defaults

Zeus's first-run experience must be "open it, work" — Ghostty's bar. This document audits VS Code's defaults and proposes Zeus overrides where the inherited default is friction.

## Principle

A defensible default is one that 80% of users would pick. Anything below that bar needs an override.

For each setting we change, we say: **what** changes, **why** the inherited default is friction, and **who** the override pessimizes for (because every default trades off something).

## Welcome / walkthrough

- `workbench.startupEditor`: `welcomePage` → **`none`**
  - Why: the welcome page gates real work. Power users dismiss it; new users get inconsistent messaging.
  - Trade-off: new users miss the curated tour. Acceptable — we ship a 1-line "press Cmd+Shift+P for commands" tooltip on first launch instead.
- `workbench.welcomePage.walkthroughs.openOnInstall`: `true` → **`false`**

## Telemetry

- `telemetry.telemetryLevel`: `all` → **`off`**
  - Why: opt-in not opt-out is the only honest position for a 2026 OSS tool. We can pop a one-shot dialog asking after first 30 days of use, but ship `off`.

## Settings sync / accounts

- Sign-in prompts at startup: **disabled**
  - Built-in sync requires a signed-in account; we leave the feature available but never proactively prompt.

## Files

- `files.autoSave`: `off` → **`afterDelay`** (1s)
  - Why: every junior dev's first complaint is "I lost my work." Auto-save is standard in 2026.
  - Trade-off: users who script around `onDidSaveDocument` need to know files save themselves. Documented in release notes.
- `files.trimTrailingWhitespace`: `false` → **`true`**
- `files.insertFinalNewline`: `false` → **`true`**
- `files.eol`: `auto` → **`\n`**
  - Trade-off: Windows-native devs may want `\r\n`. Setting is preserved so they can override.

## Editor

- `editor.formatOnSave`: `false` → **`true`** (only if a formatter is configured for the language)
  - Implementation: a one-line activation hook checks for a formatter; if present, format-on-save is enabled.
- `editor.minimap.enabled`: `true` → **`false`**
  - Why: minimap is noise for most users. The 20% who love it can flip it back.
- `editor.linkedEditing`: `false` → **`true`**
- `editor.bracketPairColorization.enabled`: `true` → **`true`** (keep)
- `editor.guides.bracketPairs`: `false` → **`"active"`**
- `editor.fontFamily`: VS Code default → **OS-native monospace stack** (SF Mono on mac, Cascadia Code on Windows, system on Linux)
  - Why: VS Code defaults to Consolas / Menlo. Modern OS-native looks better in 2026.

## Terminal

- `terminal.integrated.copyOnSelection`: `false` → **`true`**
- `terminal.integrated.shellIntegration.enabled`: `true` → **`true`** (keep)

## Search

- `search.exclude`: include `**/.zeus/memory/**` so memory files don't pollute global search by default. Override is one click.

## Git

- `git.autofetch`: `false` → **`true`**
- `git.confirmSync`: `true` → **`false`** (just sync; if a user wants confirmation, they get a notification on first sync rather than every time)

## AI defaults (Zeus-specific)

- `zeus.ai.provider`: `null` → **`null`** (keep — we don't pick for the user)
- `zeus.ai.firstRunPrompt`: shown once on first launch — three-choice quick pick:
  - "Use my Anthropic API key" → opens key entry
  - "Use a local model (Ollama)" → opens Ollama install guide if Ollama isn't running
  - "Skip — I'll set this up later" → fully usable editor, AI features dimmed

## Implementation notes

- These defaults are applied via a profile-shaped JSON shipped at `resources/profiles/zeus-default.json` (planned location)
- The set of overrides is auditable: this doc is the source of truth, the JSON is generated from it
- Users who reset to defaults get the Zeus defaults, not VS Code's

## Acceptance criteria

- [ ] Profile JSON exists and is loaded at first start
- [ ] `workbench.startupEditor` etc are set as listed above
- [ ] First-run AI prompt shows once, never re-prompts
- [ ] Reset-to-defaults restores the Zeus profile, not VS Code's
- [ ] A `docs/zeus-zero-config.md` ↔ profile JSON parity check runs in CI (no setting in JSON missing from doc, and vice versa)

## Status

Audit only. Implementation lands in a follow-up PR.
