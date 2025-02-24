import { Args, Mutation, Resolver, Subscription } from '@nestjs/graphql';
import { Player } from './player.model';
import { createPubSub, Repeater } from 'graphql-yoga';

const pubSub = createPubSub();

@Resolver(() => Player)
export class PlayerResolver {
  @Mutation(() => Player)
  enterArea(
    @Args('playerId') playerId: string,
    @Args('area') area: string
  ): Player {
    pubSub.publish(`${area}`, { id: playerId });
    return { id: playerId };
  }

  @Subscription(() => Player, {
    nullable: true,
    resolve: (value: Player) => value,
  })
  nearbyPlayers(@Args('area') area: string): Repeater<Player> {
    return pubSub.subscribe(`${area}`);
  }
}
