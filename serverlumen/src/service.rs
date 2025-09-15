use crate::protolumen::v1::server::conn::{ReceiveRequest, ReceiveResponse};
use crate::protolumen::v1::server::federation_server::Federation;
use anyhow::Result;
use tonic::{Request, Response, Status};
use tracing::info;

#[allow(dead_code)]
pub(crate) struct ServerFederationService;

#[tonic::async_trait]
impl Federation for ServerFederationService {
  async fn receive(
    &self,
    req: Request<ReceiveRequest>,
  ) -> Result<Response<ReceiveResponse>, Status> {
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
    Err(Status::ok("Received"))
  }
}
