---
name: secret-scan
description: Scan the buffer for secrets and propose .env.example replacements
allowed-tools: buffer_get, search_workspace
---

This is an example skill. Delete or replace as you set up `.zeus/` for your own project.

Scan recent edits for:

- AWS access keys (`\b(AKIA|ASIA)[A-Z0-9]{16}\b`)
- Private key headers (`-----BEGIN.*PRIVATE KEY-----`)
- GitHub tokens (prefixes `ghp_`, `gho_`, `ghu_`, `ghs_`, `ghr_`, `github_pat_`)
- Long alphanumeric values assigned to `*_KEY` / `*_SECRET` / `*_TOKEN` variables

When something matches, annotate the line and propose a `.env.example` placeholder.
