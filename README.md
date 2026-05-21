# Zeus

> A VS Code fork that aims to be **zero-config**, **delightful**, and **AI-native** — while staying fully compatible with the VS Code extension ecosystem.

Zeus is a downstream fork of [`microsoft/vscode`](https://github.com/microsoft/vscode) (`Code - OSS`), MIT licensed.

## Why another VS Code fork

The VS Code fork space (Cursor, Windsurf, Void, VSCodium, ...) is crowded. Zeus exists because each of them leaves something on the table:

- **Zero-configuration, Ghostty-style.** Install it, open it, work. No "first you need to set up these 14 settings" tutorial. Sensible defaults, fast cold start, no setup pages.
- **Fix the long-tail pain points of existing forks.** Things like opaque telemetry, AI features bolted on instead of integrated, mandatory accounts, extension marketplace lock-in, sluggish startup, fragile sync — these become non-goals for Zeus.
- **A UI that's actually exciting to look at.** Not "VS Code with a new accent color". Treat the editor chrome as something worth designing, not just a frame around a textbuffer.
- **AI-native, not AI-bolted-on.** Built assuming an LLM is in the loop from day one — completion, chat, edits, agents — integrated at the editor primitive level, not as a side panel.
- **Full VS Code extension compatibility.** Inherited from the fork. Everything you already use from the Open VSX / VS Code marketplace keeps working.

## Status

Day 1. This repo is a fresh fork of `microsoft/vscode` with a rename pass on `package.json` / `product.json`. Build pipeline, rebrand assets, and the differentiating features above are not yet implemented.

## Building from source

Zeus inherits VS Code's build system. See [`microsoft/vscode` — How to Contribute](https://github.com/microsoft/vscode/wiki/How-to-Contribute) until Zeus-specific build docs exist.

Quick start:

```bash
npm install
npm run watch
./scripts/code.sh   # macOS / Linux
```

Node version is pinned in `.nvmrc` (currently `22.22.1`).

## License

MIT. See [`LICENSE.txt`](LICENSE.txt).

Zeus is a derivative work of `Code - OSS` © Microsoft Corporation, also under MIT.
