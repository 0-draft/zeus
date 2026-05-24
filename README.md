# Zeus

MCP-first code editor, built on a VS Code fork.

- Fork of [`microsoft/vscode`](https://github.com/microsoft/vscode) (`Code - OSS`), MIT
- Default extension gallery: [Open VSX](https://open-vsx.org)
- Release command name: `z`
- VS Code extensions install unmodified

Active work toward MCP-first: Zeus exposes itself as an MCP server, consumes external MCP servers, and ships `.zeus/` conventions (skills / memory / policy / `mcp.json`) so AI behavior is git-shared. The integration is in flight across the [open PRs](https://github.com/0-draft/zeus/pulls); design lives under [`docs/`](docs/).

## Building

```bash
npm install
npm run watch
./scripts/z.sh
```

Node is pinned to `.nvmrc` (currently `22.22.1`).

## License

MIT. See [`LICENSE.txt`](LICENSE.txt). Zeus is a derivative work of `Code - OSS` © Microsoft Corporation, also under MIT.
