import type { PackageJSON } from '@npm/types'
import type { ProjectManifest } from '@pnpm/types'
import { pipe } from 'ramda'

import { transformBin } from './bin.js'
import { transformPeerDependenciesMeta } from './peerDependenciesMeta.js'
import { transformRequiredFields } from './requiredFields.js'

export type ExportedManifest = PackageJSON & { registry?: string }

export type { PackageJSON }

export type Transform = (manifest: ProjectManifest) => ExportedManifest
export const transform: Transform = pipe(
  transformRequiredFields,
  transformBin,
  transformPeerDependenciesMeta
) as Transform
