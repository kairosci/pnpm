import { prepareEmpty } from '@pnpm/prepare'
import { jest } from '@jest/globals'
import { DEFAULT_OPTS } from './utils/index.js'

jest.unstable_mockModule('@pnpm/run-npm', () => ({
  runNpm: jest.fn(() => ({ status: 0 })),
}))

const { runNpm } = await import('@pnpm/run-npm')
const { version } = await import('@pnpm/plugin-commands-script-runners')

beforeEach(() => {
  jest.clearAllMocks()
})

test('version should invoke runNpm with version params and dir', async () => {
  prepareEmpty()

  await version.handler({
    ...DEFAULT_OPTS,
    dir: process.cwd(),
    extraEnv: { FOO: 'bar' },
    selectedProjectsGraph: {
      [process.cwd()]: {
        dependencies: [],
        package: {
          dir: process.cwd(),
          manifest: {
            name: 'foo',
            version: '1.0.0',
          },
        },
      },
    },
  }, ['minor'])

  expect(runNpm).toHaveBeenCalledWith(undefined, ['version', 'minor'], expect.objectContaining({
    cwd: process.cwd(),
    env: { FOO: 'bar' },
  }))
})
