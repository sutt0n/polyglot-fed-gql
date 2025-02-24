import { Module } from '@nestjs/common';
import { GraphQLModule } from '@nestjs/graphql';
import {
  YogaFederationDriver,
  YogaFederationDriverConfig,
} from '@graphql-yoga/nestjs-federation';
import { PlayerModule } from './player/player.module';

@Module({
  imports: [
    GraphQLModule.forRoot<YogaFederationDriverConfig>({
      driver: YogaFederationDriver,
      autoSchemaFile: {
        federation: {
          version: 2,
          importUrl: 'https://specs.apollo.dev/federation/v2.4',
        },
      },
      subscriptions: true,
    }),
    PlayerModule,
  ],
})
export class AppModule {}
