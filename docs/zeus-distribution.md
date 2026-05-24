# Distribution

How users install Zeus once we are ready to ship binaries. Tracks the full path: from `.app` produced by the gulp build to Homebrew Cask.

## macOS

Two channels, parity in version:

1. **Direct download (`.dmg`)**: GitHub Releases, `Zeus-<version>-<arch>.dmg`
2. **Homebrew Cask**: `brew install --cask 0-draft/zeus/zeus`

### Homebrew Cask

The Cask lives in a separate tap repo named `0-draft/homebrew-zeus` (Homebrew convention requires the `homebrew-` prefix).

```ruby
# Casks/zeus.rb in 0-draft/homebrew-zeus
cask "zeus" do
  arch arm: "arm64", intel: "x86_64"

  version "0.0.1"
  sha256 arm:   "PLACEHOLDER_SHA256_ARM",
         intel: "PLACEHOLDER_SHA256_INTEL"

  url "https://github.com/0-draft/zeus/releases/download/v#{version}/Zeus-#{version}-#{arch}.dmg"
  name "Zeus"
  desc "MCP-first code editor built on VS Code"
  homepage "https://github.com/0-draft/zeus"

  app "Zeus.app"
  binary "#{appdir}/Zeus.app/Contents/Resources/app/bin/z"

  zap trash: [
    "~/.zeus",
    "~/.zeus-shared",
    "~/Library/Application Support/Zeus",
    "~/Library/Caches/com.0draft.zeus",
    "~/Library/Caches/com.0draft.zeus.ShipIt",
    "~/Library/HTTPStorages/com.0draft.zeus",
    "~/Library/Logs/Zeus",
    "~/Library/Preferences/com.0draft.zeus.plist",
    "~/Library/Saved Application State/com.0draft.zeus.savedState",
    "~/Library/WebKit/com.0draft.zeus",
    "~/Library/Application Support/com.apple.sharedfilelist/com.apple.LSSharedFileList.ApplicationRecentDocuments/com.0draft.zeus.sfl3"
  ]
end
```

A template lives at [`build/distribution/homebrew-cask.rb.template`](../build/distribution/homebrew-cask.rb.template). Releasing the first version means:

1. Run `gulp vscode-darwin-arm64` and `gulp vscode-darwin-x64` to produce both `.app`s
2. Notarize via `xcrun notarytool` (requires Apple Developer cert — open question)
3. Pack into `.dmg` (use [`@vscode/dmg`](https://www.npmjs.com/package/@vscode/dmg) or `create-dmg`)
4. Compute SHA256 of each `.dmg`
5. Upload `.dmg`s to GitHub Releases
6. Update `0-draft/homebrew-zeus/Casks/zeus.rb` with version + SHAs
7. Push to the tap repo

### `zoxide` collision

`zoxide` uses `z` as its primary command. Release-install notes will need to call this out:

- If you actively use `zoxide`, alias Zeus's `z` to something else: `alias zc=z`
- Or rename Zeus's exposed binary at install time (configurable via `applicationName` override)

This is documented but not auto-resolved on install.

## Linux

- `.deb` for Debian/Ubuntu via apt repo
- `.rpm` for Fedora/RHEL
- `.AppImage` for general use
- `snap` (later)

## Windows

- `.exe` MSI installer
- `winget` (later)
- Microsoft Store (not planned)

## Code signing

- **macOS**: Apple Developer ID Application certificate ($99/year individual). Notarization required for Gatekeeper-friendly install
- **Windows**: EV code signing cert ($300-600/year) or sigstore-based later
- **Linux**: GPG-signed releases

Open question: do we get certs in the project owner's name or under an org (LLC / nonprofit)? Tied to the sustainability model decision in `MARKET_RESEARCH.md`.

## Auto-update

Zeus inherits VS Code's auto-update infrastructure (`product.json` `updateUrl`). Until we run our own update server, auto-update is disabled. Users update via Homebrew or by downloading a new `.dmg`.

When the update server lands, Squirrel.Mac (already in vscode) handles macOS, Linux updates are channel-based (apt/rpm), Windows is Squirrel.Windows.

## Status

This PR ships:

- The design at `docs/zeus-distribution.md`
- A Cask template at `build/distribution/homebrew-cask.rb.template`

Real release pipeline lands separately — needs Apple Developer cert + the tap repo.
