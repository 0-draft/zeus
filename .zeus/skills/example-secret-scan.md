---
name: secret-scan
description: Scan the buffer for secrets and propose .env.example replacements
allowed-tools: buffer_get, search_workspace
---

This is an example skill. Delete or replace as you set up `.zeus/` for your own project.

Scan recent edits for:

- AWS access keys (`AKIA[0-9A-Z]{16}`)
- Private key headers (`-----BEGIN.*PRIVATE KEY-----`)
- GitHub tokens (`ghp_[0-9a-zA-Z]{36}`)
- Long alphanumeric values assigned to `*_KEY` / `*_SECRET` / `*_TOKEN` variables

When something matches, annotate the line and propose a `.env.example` placeholder.
