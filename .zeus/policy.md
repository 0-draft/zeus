# Zeus policy for this project

Constitutional rules Zeus injects into every AI call as system context. Hard rules are refusal-grade; soft preferences are style guidance.

## Hard rules

- Never write secrets, tokens, or passwords back into a buffer.
- Confirm before `rm -rf`, `DROP TABLE`, `git reset --hard`, `git push --force`, `git push --force-with-lease`, or any destructive irreversible action.
- Do not auto-run database migrations. Generating them is fine; applying them requires explicit user approval.
- Do not call write APIs (`POST`, `PUT`, `PATCH`, `DELETE`) against production URLs (e.g., `*.prod.*`, `app.example.com`) unless the user explicitly opts in for the session.

## Soft preferences

- (replace with your project's style preferences)
