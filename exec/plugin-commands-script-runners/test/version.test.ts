import { jest } from '@jest/globals'
import { prepareEmpty } from '@pnpm/prepare'
import { version } from '@pnpm/plugin-commands-script-runners'
import { runNpm } from '@pnpm/run-npm'
import type { ProjectRootDir, ProjectsGraph } from '@pnpm/types'
import { DEFAULT_OPTS } from './utils/index.js'

jest.unstable_mockModule('@pnpm/run-npm', () => ({
  runNpm: jest.fn(() => ({ status: 0 })),
}))

beforeEach(() => {
  jest.clearAllMocks()
})

test('version should invoke runNpm with version params and dir', async () => {
  prepareEmpty()

  const cwd = process.cwd() as ProjectRootDir

  const result = await version.handler({
    ...DEFAULT_OPTS,
    dir: cwd,
    configDir: cwd,
    extraEnv: { FOO: 'bar' },
    selectedProjectsGraph: {
      [cwd]: {
        dependencies: [],
        package: {
          manifest: {
            name: 'foo',
            version: '1.0.0',
          },
          rootDir: cwd,
          rootDirRealPath: cwd as any, // eslint-disable-line @typescript-eslint/no-explicit-any
        },
      },
    } as unknown as ProjectsGraph,
  }, ['minor'])

  expect(result.exitCode).toBe(0)

  expect(runNpm).toHaveBeenCalledWith(undefined, ['version', 'minor'], expect.objectContaining({
    cwd,
    env: { FOO: 'bar' },
  }))
})
