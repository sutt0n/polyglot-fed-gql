import { Field, ID, ObjectType } from '@nestjs/graphql';

@ObjectType()
export class Player {
  @Field(() => ID)
  id: string;
}
