pub(crate) mod protolumen {
  pub(crate) mod v1 {
    pub(crate) mod client {
      pub(crate) mod conn {
        tonic::include_proto!("protolumen.v1.client.conn");
      }
      tonic::include_proto!("protolumen.v1.client");
    }
  }
}
use crate::protolumen::v1::client::client_client::ClientClient;
use crate::protolumen::v1::client::conn::SendRequest;
use anyhow::{Context, Result};
use std::env;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();
  let url = format!("http://{}", args.get(1).unwrap_or(&"[::1]:6543".into()));
  let mut client = ClientClient::connect(url)
    .await
    .context("error connecting to client")?;
  let msg = SendRequest {
    version: "1".into(),
    origin_id: "client".into(),
    destination_id: "destination".into(),
    payload_id: Uuid::new_v8(rand::random()).to_string(),
    payload: args[2].clone().into(),
    timestamp: Some(prost_types::Timestamp {
      seconds: 0,
      nanos: 0,
    }),
  };
  let request = tonic::Request::new(msg);
  _ = client
    .send(request)
    .await
    .context("failed to send message")?;
  Ok(())
}
