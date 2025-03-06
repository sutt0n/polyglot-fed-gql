import { defineConfig, LogLevel } from '@graphql-hive/gateway';

export const gatewayConfig = defineConfig({
  port: 8000,
  supergraph: 'apps/gateway/supergraph.graphql',
  logging: LogLevel.debug,
  transportEntries: {
    '*.http': {
      options: {
        subscriptions: {
          kind: 'ws',
        },
      },
    },
  },
});
