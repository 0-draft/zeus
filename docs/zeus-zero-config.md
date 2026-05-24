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
  - Why: same rationale as above — walkthrough overlays steal focus from the editor on first open.
  - Trade-off: users who like the walkthrough have to find it in the command palette (`Open Walkthrough`).

## Telemetry

- `telemetry.telemetryLevel`: `all` → **`off`**
  - Why: opt-in not opt-out is the only honest position for a 2026 OSS tool. We can pop a one-shot dialog asking after first 30 days of use, but ship `off`.

## Settings sync / accounts

- Sign-in prompts at startup: **disabled**
  - Built-in sync requires a signed-in account; we leave the feature available but never proactively prompt.

## Files

- `files.autoSave`: `off` → **`onFocusChange`**
  - Why: every junior dev's first complaint is "I lost my work." `onFocusChange` saves on context switch (tab away, terminal focus, palette open) — strictly safer than `off` without the rearrange-while-typing problem `afterDelay` has when combined with `formatOnSave`.
  - Trade-off: writes to disk happen at focus boundaries, slightly later than `afterDelay` but earlier than `onWindowChange`.
- `files.trimTrailingWhitespace`: `false` → **`true`** *with a per-language carve-out for Markdown*
  - Why: matches what almost every linter / formatter does anyway; the editor doing it preemptively means cleaner diffs.
  - Carve-out: the Zeus default profile also ships `"[markdown]": { "files.trimTrailingWhitespace": false }` so Markdown's "two trailing spaces = line break" convention isn't silently broken. Users get clean diffs everywhere else and standards-compliant Markdown without thinking about it.
- `files.insertFinalNewline`: `false` → **`true`**
  - Why: POSIX-friendly; many tools (`tail`, `cat`, `git diff`) handle file-with-trailing-newline better.
  - Trade-off: noisy first-diff if a file in an existing repo previously lacked one.
- `files.eol`: `auto` → **`auto`** (keep VS Code's default)
  - Why: cross-platform / Windows-native repos that mix CRLF need this. Pinning to `\n` here would silently rewrite `.bat`, PowerShell scripts, and legacy Windows-CRLF files when saved. Project-level `.editorconfig` is the right place to enforce `\n` per repo.

## Editor

- `editor.formatOnSave`: `false` → **`true`** (only if a formatter is configured for the language)
  - Why: catches forgotten formatting on every save instead of relying on pre-commit hooks.
  - Implementation: a one-line activation hook checks for a formatter; if present, format-on-save is enabled.
  - Also: `editor.formatOnSaveMode`: `file` → **`modifications`**. Only the lines the user touched are reformatted. Saves legacy / messy codebases from one-shot formatter churn the first time they're opened in Zeus.
  - Caveat: `modifications` requires the file to be tracked by Git (or another supported SCM) — VS Code uses the SCM diff to know which ranges are "yours". For standalone files or projects without `git init`, the mode silently falls back to formatting nothing. Zeus's first-run flow surfaces this as a hint when the user opens an untracked folder: "Run `git init` to enable smart format-on-save." Users who don't want that behaviour can set `"editor.formatOnSaveMode": "file"` in workspace settings.
  - Trade-off: users on green-field projects sometimes want full-file format; one-line override per workspace.
- `editor.minimap.enabled`: `true` → **`false`**
  - Why: minimap is noise for most users. The 20% who love it can flip it back.
  - Trade-off: keyboard scroll fans love the visual map; documented in the cheat sheet.
- `editor.linkedEditing`: `false` → **`true`**
  - Why: HTML/JSX tag renames stay in sync without a refactor command.
  - Trade-off: extra mutation events some extensions don't expect.
- `editor.bracketPairColorization.enabled`: `true` → **`true`** (keep)
- `editor.guides.bracketPairs`: `false` → **`"active"`**
  - Why: highlights the bracket nesting around the cursor without painting every line.
  - Trade-off: more visual noise for users who prefer a totally clean gutter.
- `editor.fontFamily`: VS Code default → **OS-native monospace stack**
  - Concrete string shipped in the profile JSON: `'SF Mono', 'Cascadia Code', 'JetBrains Mono', 'Ubuntu Mono', 'Liberation Mono', Menlo, Monaco, Consolas, monospace`
  - Why: VS Code defaults to platform-specific Consolas / Menlo only. The stack above prefers the OS-native fixed-width face when it exists (SF Mono on macOS, Cascadia on Windows, Ubuntu Mono on Ubuntu) and falls back to the generic `monospace` keyword on Linux — `monospace` is the standard CSS generic-family that fontconfig maps to the system's preferred fixed-width family, where `system` would be ambiguous and could resolve to a proportional face.

## Terminal

- `terminal.integrated.copyOnSelection`: `false` → **`false`** (keep VS Code's default)
  - Why we considered changing it: iTerm2 and X11 copy-on-select muscle memory is strong for terminal heavy users.
  - Why we kept the default: stock macOS Terminal does *not* copy on selection (Cmd+C is required), so "matches macOS Terminal" is wrong, and the change clobbers the system clipboard for the select-to-replace workflow that's common when editing in the integrated terminal. Users who want it can flip the one setting.
- `terminal.integrated.shellIntegration.enabled`: `true` → **`true`** (keep)

## Search

- `search.exclude`: include `**/.zeus/memory/**` so memory files don't pollute global search by default. Override is one click.

## Git

- `git.autofetch`: `false` → **`true`**
  - Why: keeps remote-aware UI (ahead/behind indicators, "Pull from main") current without manual fetch.
  - Trade-off: users with credential-manager-less SSH setups (passphrase-protected keys, no agent) get repeated prompts. Documented in install notes — the answer is "set up a credential manager", not "disable autofetch".
- `git.confirmSync`: `true` → **`true`** (keep)
  - Why: "Sync Changes" pushes and pulls in one shot, so a stray click can publish unfinished work. The confirmation is one extra return key but is what stops a category of accidents. We optimize for safety here, not for one fewer keystroke.

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
