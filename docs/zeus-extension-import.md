# VS Code extension import compatibility

Zeus inherits VS Code's extension host, so almost everything from the VS Code marketplace and Open VSX should install unmodified. "Almost" is the catch — Microsoft-published extensions enforce a runtime check that rejects non-Microsoft hosts. We want CI that proves the **inherited extension surface is intact**, so regressions show up early.

## What we test

A curated set of popular extensions, ranked by Open VSX install count:

| Extension | Source | Reason |
|---|---|---|
| `dbaeumer.vscode-eslint` | Open VSX | Top-3 install count, exercises language server |
| `esbenp.prettier-vscode` | Open VSX | Top-5, exercises formatter API |
| `rust-lang.rust-analyzer` | Open VSX | Heavy LSP, custom views |
| `golang.go` | Open VSX | LSP + debugger |
| `redhat.vscode-yaml` | Open VSX | LSP, schema integration |
| `eamodio.gitlens` | Open VSX | Heavy UI integration |
| `bradlc.vscode-tailwindcss` | Open VSX | Custom language server |
| `vue.volar` | Open VSX | Replaces TS server for Vue |

We deliberately **do not** test Microsoft-published extensions that enforce runtime checks (C/C++, Pylance) — those are known not to work in any non-Microsoft host. Documented, not tested.

## Test harness

`test/extension-import/run.sh` does:

1. Spins up a clean user data dir
2. Installs all extensions from `manifest.json` in a single `--install-extension ... --install-extension ...` call (binary startup is expensive, batching is faster)
3. Calls `code --list-extensions --show-versions` and asserts each entry from the manifest is present

Activation-failure log scanning and `code --status` boot checks are listed as follow-up work in the acceptance criteria below — they are useful but not part of the v1 harness.

Reproduces a real install pattern. Run locally:

```bash
./test/extension-import/run.sh
```

## CI

A new workflow `.github/workflows/ext-import.yml` runs this on every push to main and weekly on cron. Failures block merge.

It runs on top of the build job from `ci.yml` — same artifact, no double build.

## What "compatible" means

- Extension installs without error
- Extension activates without crashing the workbench
- Basic commands the extension registers are listed in `code --list-commands` (sanity check)

We do not assert UI behavior (would need a real Playwright run, which is on a separate workflow and currently has 8 known regressions).

## Status

This PR ships:

- `test/extension-import/manifest.json` — initial extension list
- `test/extension-import/run.sh` — harness (bash -n passes)
- `.github/workflows/ext-import.yml` — CI workflow

Real assertions land when the build artifact pipeline (currently only `compile`, not `compile-extensions-build`) is in better shape — `compile-extensions-build` has the known gulp-vinyl-zip + Open VSX hang issue.

## Acceptance criteria

- [ ] `run.sh` installs all extensions in `manifest.json` against a built Zeus
- [ ] Asserts no activation failures in the log
- [ ] CI workflow blocks merge on failure
- [ ] Weekly cron catches upstream regressions in tracked extensions
