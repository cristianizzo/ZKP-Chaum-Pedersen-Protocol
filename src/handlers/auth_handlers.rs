use tonic::{Request, Response, Status};

use crate::zkp_auth::auth_server::Auth;
use crate::zkp_auth::{
    AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest,
    AuthenticationChallengeResponse, RegisterRequest, RegisterResponse,
};

use crate::db::DbPool;
use std::sync::Arc;

pub struct AuthHandler {
    db: Arc<DbPool>,
}

impl AuthHandler {
    pub fn new(db: Arc<DbPool>) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl Auth for AuthHandler {
    async fn register(
        &self,
        _request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        // Implement the logic for the "register" method
        // Extract the data from the request using `request.into_inner()`
        // Perform the necessary operations
        // Create and return a response using `Response::new()`

        unimplemented!()
    }

    async fn create_authentication_challenge(
        &self,
        _request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        // Implement the logic for the "create_authentication_challenge" method
        // Extract the data from the request using `request.into_inner()`
        // Perform the necessary operations
        // Create and return a response using `Response::new()`

        unimplemented!()
    }

    async fn verify_authentication(
        &self,
        _request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        // Implement the logic for the "verify_authentication" method
        // Extract the data from the request using `request.into_inner()`
        // Perform the necessary operations
        // Create and return a response using `Response::new()`

        unimplemented!()
    }
}
