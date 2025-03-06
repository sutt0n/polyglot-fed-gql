import { Directive, Field, ID, ObjectType } from '@nestjs/graphql';

@ObjectType()
export class Player {
  @Directive('@shareable')
  @Field(() => ID)
  id: string;
}
