import { Module } from '@nestjs/common';
import { PlayerResolver } from './player.resolver';

@Module({
  providers: [PlayerResolver],
})
export class PlayerModule {}
