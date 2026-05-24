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
- `files.trimTrailingWhitespace`: `false` → **`true`**
  - Why: matches what almost every linter / formatter does anyway; the editor doing it preemptively means cleaner diffs.
  - Trade-off: Markdown's "two trailing spaces = line break" convention loses on save. Users who care about that syntax can scope an override per-language.
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
- `editor.fontFamily`: VS Code default → **OS-native monospace stack** (SF Mono on macOS, Cascadia Code on Windows, `monospace` on Linux — fontconfig resolves it to the system's preferred fixed-width family)
  - Why: VS Code defaults to Consolas / Menlo. Modern OS-native looks better in 2026.

## Terminal

- `terminal.integrated.copyOnSelection`: `false` → **`true`**
  - Why: matches macOS Terminal / iTerm muscle memory and Linux X11 default.
  - Trade-off: clobbers the system clipboard mid-task for users who rely on long-lived clipboard contents.
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
