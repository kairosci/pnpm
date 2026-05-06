---
"@pnpm/pkg.commands": minor
"pnpm": minor
---

feat: add native `pnpm set-script` command with `ss` alias (#11287)

Added a new command to set scripts in package.json:

- `pnpm set-script <name> <command>` - sets a script in package.json
- `pnpm ss <name> <command>` - short alias
