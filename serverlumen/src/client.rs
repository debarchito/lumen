use crate::protolumen::v1::client::auth::{
  AuthenticateSessionRequest, AuthenticateSessionResponse, RegisterRequest, RegisterResponse,
  RevokeSessionRequest, RevokeSessionResponse, SessionChallengeRequest, SessionChallengeResponse,
};
use crate::protolumen::v1::client::client_server::Client;
use anyhow::Result;
use tonic::{Request, Response, Status};

#[allow(dead_code)]
pub(crate) struct ClientService;

#[tonic::async_trait]
impl Client for ClientService {
  async fn register(
    &self,
    _req: Request<RegisterRequest>,
  ) -> Result<Response<RegisterResponse>, Status> {
    Err(Status::aborted("TODO"))
  }

  async fn session_challenge(
    &self,
    _req: Request<SessionChallengeRequest>,
  ) -> Result<Response<SessionChallengeResponse>, Status> {
    Err(Status::aborted("TODO"))
  }

  async fn authenticate_session(
    &self,
    _req: Request<AuthenticateSessionRequest>,
  ) -> Result<Response<AuthenticateSessionResponse>, Status> {
    Err(Status::aborted("TODO"))
  }

  async fn revoke_session(
    &self,
    _req: Request<RevokeSessionRequest>,
  ) -> Result<Response<RevokeSessionResponse>, Status> {
    Err(Status::aborted("TODO"))
  }
}
