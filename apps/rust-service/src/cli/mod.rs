use anyhow::Context;


pub async fn run() -> anyhow::Result<()> {
  let (send, mut recv) = tokio::sync::mpsc::channel(1);
  let mut handlers = vec![];

  println!("Starting messages graphql subgraph");
  let subgraph_send = send.clone();
  handlers.push(tokio::spawn(async move {
    let _ = subgraph_send.try_send(
      crate::graphql::server::run_server().await.context("Failed to start messages graphql server"),
    );
  }));

  let reason = recv.recv().await.expect("Didn't receive msg");
    for handle in handlers {
        handle.abort();
    }

    reason
}
