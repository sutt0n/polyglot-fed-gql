
use async_graphql::{*, futures_util::Stream, futures_util::future};

use super::{types::*, broker::SimpleBroker};

pub struct Query;

#[Object]
impl Query {
  async fn message(&self, id: String) -> async_graphql::Result<Message> {
    Ok(Message {
      id,
      text: "Hello, world!".to_string(),
      to_player_id: "1".to_string(),
      from_player_id: "2".to_string(),
    })
  }
}

pub struct Mutation;

#[derive(SimpleObject)]
pub struct MessagePayload {
  message: Message,
}

#[derive(InputObject)]
struct SendMessageInput {
  text: String,
  to_player_id: String,
  from_player_id: String,
}

#[Object]
impl Mutation {
  async fn send_message(&self, input: SendMessageInput) -> async_graphql::Result<MessagePayload> {
    let message = Message {
      id: "1".to_string(),
      text: input.text,
      to_player_id: input.to_player_id,
      from_player_id: input.from_player_id,
    };

    SimpleBroker::<Message>::publish(message.clone());

    Ok(MessagePayload { message })
  }
}

pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn messages(&self, player_id: String) -> impl Stream<Item = Message> {
        SimpleBroker::<Message>::subscribe_filtered(move |message| future::ready(message.to_player_id == player_id))
    }
}
