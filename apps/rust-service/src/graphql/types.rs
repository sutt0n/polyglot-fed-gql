use async_graphql::SimpleObject;

#[derive(SimpleObject, Clone)]
pub(super) struct Message {
  pub id: String,
  pub text: String,
  pub to_player_id: String,
  pub from_player_id: String,
}
