# UI design pass

A user who opens Zeus for the first time should notice the chrome, not just the textbuffer. VS Code's UI is functional but visually anonymous — Zed's Rust + GPU + 120fps rendering has reset what "modern editor design" feels like.

This is the design pass plan. Implementation is incremental; each section below becomes its own commit when work starts.

## Three things to fix immediately

1. **Title bar** — VS Code's default title bar is just a window chrome strip. Treat it as a real surface: project name on the left, current branch + sync state in the center, AI HUD on the right
2. **Activity bar** — VS Code stacks icons vertically with no hierarchy. Group by function (files / search / git / agents / debug), separate visual treatment for the "AI agents" group
3. **Command palette** — VS Code's palette is the most-used surface and the most boring. Larger, more typographic, with sectioned results and inline previews

## Typography

Monospace stack — OS-native first, JetBrains Mono (bundled with the app) as the cross-platform safety net:

| OS | Primary | Fallback |
|---|---|---|
| macOS | SF Mono | JetBrains Mono (bundled), Menlo |
| Windows | Cascadia Code | JetBrains Mono (bundled), Consolas |
| Linux | Ubuntu Mono | JetBrains Mono (bundled), Hack, DejaVu Sans Mono |

UI font (chrome, not the editor):

- macOS: SF Pro Text → SF Pro Display for large headings
- Windows: Segoe UI Variable
- Linux: Inter (bundled) → system UI font

JetBrains Mono (monospace) and Inter (UI) are bundled with the app so the experience stays consistent even when system fonts are unusual or non-Latin glyph coverage is missing. Bundled fonts sit one rung down from the OS-native primary so users who prefer their system font don't have it overridden silently — but on a fresh Linux desktop or a Windows machine without Cascadia, the bundled face still wins over a generic fallback.

## Motion

VS Code is mostly static. Zeus adds motion only where it earns its place:

- **Focus change**: 120ms ease-out on the cursor blob position (caret carry effect)
- **Command palette open**: 120ms fade-in with 4px translateY; never bounces (matches the focus-change duration above for visual consistency, and is long enough to read as a transition on 60Hz panels)
- **Save**: 200ms checkmark pulse on the status bar item, no modal
- **Agent completion**: brief glow on the parallel-agents status item, no sound
- **No transitions on tab switch** — too high frequency, motion gets in the way

All motion respects `prefers-reduced-motion`.

## Themes

Hand-tuned, shipped enabled:

- **Zeus Light** — warm light, OLED-friendly contrast on black text
- **Zeus Dark** — cool dark, accent gold (matches Z monogram in icon)
- **Zeus High Contrast** — AAA contrast, distinct from VS Code's existing HC themes
- **Zeus Dim** — between Dark and Light, for users who hate full dark

Existing VS Code themes still install fine — these are additions, not replacements.

## Iconography

The Z monogram (lightning) becomes part of the visual language:

- App icon (already present, `resources/zeus-icon.svg`)
- Status bar "AI ready" indicator uses the same lightning glyph
- Notification banner for agent completion has a small Z accent

Avoid: putting the Z everywhere. Each appearance should mean something.

## Status bar redesign

Today's VS Code status bar is a tag soup. Zeus groups it:

```text
[ 🔀 main ↑2  ] [ ✦ Zeus  92% cache  $0.41 ] [ ⚠ 2 ]    [ Ln 12 · UTF-8 · Spaces:2 ]
  ↑ left: git / project           ↑ middle: AI HUD     ↑ problems   ↑ right: editor state
```

Each cluster has its own visual treatment. Middle cluster is Zeus-specific. The `[ ⚠ N ]` chip is the existing vscode Problems indicator (errors + warnings count) pulled out of the right cluster so it sits next to the AI HUD — putting "the editor's diagnostics" and "the AI's diagnostics" side by side makes the relationship obvious.

The cost segment (`$0.41`) is configurable via the `zeus.ai.hud.enabled` setting — users who don't want a live "ticker" can hide it without losing the rest of the HUD. The detailed HUD design (cache window, pricing source, persisted state) lands with `feat/prompt-cache-hud` (#28); once both PRs are merged this paragraph will link to `docs/zeus-prompt-cache-hud.md`.

## Density modes

Three density presets via `zeus.ui.density`, in descending order from tallest to most compact:

- `comfortable` (default, tallest) — extra vertical breathing room vs VS Code
- `cozy` (intermediate) — between `comfortable` and `compact`
- `compact` — matches VS Code's default density

## Open questions

- Do we ship a custom monospaced font of our own (like Berkeley Mono vibe) or stick to per-OS native? Default: native.
- Do we add a "showcase" page at first launch demonstrating the design? Conflicts with `feat/zero-config-defaults` ("no welcome page gates real work"). Probably no.
- Animated app icon for "agents are running" — appealing but a fast way to look gimmicky. Hold.

## Acceptance criteria

- [ ] Title bar layout matches the spec in this doc
- [ ] Themes ship enabled, not as marketplace prompts
- [ ] All motion respects `prefers-reduced-motion`
- [ ] Density toggle works without restart
- [ ] Each section gets at least one before/after screenshot **when its implementation PR lands** (not in this design-only PR — nothing to screenshot yet)

## Status

Design only. Implementation is incremental; each section becomes its own commit + design review when work starts.
