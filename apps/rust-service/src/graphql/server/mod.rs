
use async_graphql::{http::GraphiQLSource, Schema};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{routing::get, Router, response::{IntoResponse, self}};
use tokio::net::TcpListener;

use crate::graphql;

pub async fn run_server() -> anyhow::Result<()> {
  let schema = Schema::build(graphql::Query, graphql::Mutation, graphql::Subscription)
    .finish();

  let app = Router::new()
        .route(
            "/",
            get(graphiql).post_service(GraphQL::new(schema.clone())),
        )
        .route_service("/ws", GraphQLSubscription::new(schema));

    println!("Starting graphql server on port {}", 8081);
    axum::serve(TcpListener::bind("127.0.0.1:8081").await.unwrap(), app)
        .await
        .unwrap();
    Ok(())
}

async fn graphiql() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
            .finish(),
    )
}
