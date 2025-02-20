mod schema;
mod types;
mod broker;

pub mod server;

use async_graphql::*;

pub use schema::*;

pub fn schema() -> Schema<Query, Mutation, EmptySubscription> {
    let schema = Schema::build(Query, Mutation, EmptySubscription);
    schema.finish()
}
