---
"@pnpm/resolve-dependencies": patch
---

Fix `catalogMode: strict` writing `catalog:` to pnpm-workspace.yaml instead of version specifier when re-adding dependencies.
