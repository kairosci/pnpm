---
id: env
title: "pnpm env <cmd>"
---

`pnpm env` remains available for users accustomed to its name. It is an alias for [`pnpm runtime`](./runtime.md). `pnpm env use <version>` aliases `pnpm runtime set node <version>`.

`pnpm env list` and `pnpm env remove` remain available with their existing behavior.

```sh
pnpm env list [<version>]
pnpm env remove --global <version>
```
