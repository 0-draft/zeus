# Zeus

A fork of [`microsoft/vscode`](https://github.com/microsoft/vscode) (`Code - OSS`), MIT licensed.

## Status

Day 1. `package.json` / `product.json` / `cli/Cargo.toml` are renamed to Zeus; placeholder icon and Open VSX gallery wired up. Most differentiating work is not yet done.

## Building from source

Zeus inherits VS Code's build system. Until Zeus-specific build docs exist, the upstream [How to Contribute](https://github.com/microsoft/vscode/wiki/How-to-Contribute) wiki is the reference.

```bash
npm install
npm run watch
./scripts/z.sh      # macOS / Linux (alias of code.sh)
```

Release builds expose the editor on `PATH` as `z`.

Node version is pinned in `.nvmrc` (currently `22.22.1`).

## License

MIT. See [`LICENSE.txt`](LICENSE.txt).

Zeus is a derivative work of `Code - OSS` © Microsoft Corporation, also under MIT.
