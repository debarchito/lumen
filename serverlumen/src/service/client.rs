use crate::protolumen::v1::client::client_server::Client;
use crate::protolumen::v1::client::conn::{SendRequest, SendResponse};
use crate::protolumen::v1::server::federation_client::FederationClient;
use tonic::{Request, Response, Status};
use tracing::info;
use uuid::Uuid;

#[allow(dead_code)]
pub(crate) struct ClientService {
  pub(crate) state: crate::SharedState,
}

#[tonic::async_trait]
impl Client for ClientService {
  async fn send(&self, req: Request<SendRequest>) -> Result<Response<SendResponse>, Status> {
    let input = req.get_ref();
    let timestamp = input.timestamp.as_ref().map_or_else(
      || "N/A".to_string(),
      |ts| format!("{}.{:09}s", ts.seconds, ts.nanos),
    );
    let payload = String::from_utf8_lossy(&input.payload);
    info!(
      "[from {} to {} at {}] {}",
      input.origin_id, input.destination_id, timestamp, payload
    );

    for peer in &self.state.config.trust_list {
      let url = format!("http://{}", &peer.address);
      let mut client = FederationClient::connect(url).await.map_err(|err| {
        tracing::error!("failed to connect to peer {}: {}", peer.id, err);
        Status::internal("failed to connect to peer")
      })?;
      let msg = crate::protolumen::v1::server::conn::ReceiveRequest {
        version: "1".to_string(),
        origin_id: self.state.config.node.id.clone(),
        destination_id: input.destination_id.clone(),
        payload_id: Uuid::new_v8(rand::random()).to_string(),
        payload: input.payload.clone(),
        timestamp: input.timestamp,
      };
      let request = tonic::Request::new(msg);
      _ = client.receive(request).await?
    }

    Ok(tonic::Response::new(SendResponse {
      version: "1".to_string(),
      payload_id: input.payload_id.clone(),
      status: 1,
      details: "Message sent successfully".to_string(),
      timestamp: input.timestamp,
    }))
  }
}
