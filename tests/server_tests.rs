// use tokio::runtime::Runtime;
// use tonic::transport::Server;
// // use zkp_auth::auth_server::{Auth, AuthServer};
// // use zkp_auth::{RegisterRequest, RegisterResponse, AuthenticationChallengeRequest, AuthenticationChallengeResponse, AuthenticationAnswerRequest, AuthenticationAnswerResponse};
// use std::collections::HashMap;
// use std::sync::Mutex;
// use uuid::Uuid;
// use lazy_static::lazy_static;
// use curve25519_dalek::scalar::Scalar;
// use curve25519_dalek::constants;
// use curve25519_dalek::ristretto::RistrettoPoint;
// use curve25519_dalek::traits::VartimeMultiscalarMul;
// use curve25519_dalek::traits::IsIdentity;
//
// // Import the AuthService struct from the main module
// // use crate::AuthService;
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[tokio::test]
//     async fn test_register() {
//         let auth_service = AuthService::default();
//
//         // Create a RegisterRequest
//         let x1 = Scalar::from(1234567890u64); // User's secret value
//         let x2 = Scalar::from(1234567891u64); // User's secret value
//         let request = RegisterRequest{
//             user: "test_user".to_owned(),
//             y1: x1.to_bytes().to_vec(), // Public value y1 = g^x1
//             y2: x2.to_bytes().to_vec(), // Public value y2 = h^x2
//         };
//
//         // Call the register function
//         let response = auth_service.register(Request::new(request)).await;
//
//         // Check that the response is Ok
//         assert!(response.is_ok());
//
//         // Check that the user is now registered
//         let users = auth_service.users.lock().unwrap();
//         assert!(users.contains_key("test_user"));
//
//         // Try to register the same user again
//         let request = RegisterRequest{
//             user: "test_user".to_owned(),
//             y1: x1.to_bytes().to_vec(), // Public value y1 = g^x1
//             y2: x2.to_bytes().to_vec(), // Public value y2 = h^x2
//         };
//         let response = auth_service.register(Request::new(request)).await;
//
//         // Check that the response is an error
//         assert!(response.is_err());
//
//         // Check that the error is Status::already_exists
//         match response {
//             Err(Status::AlreadyExists(_)) => {}, // This is what we expect
//             _ => panic!("Unexpected result from register function"),
//         }
//     }
//
//     #[tokio::test]
//     async fn test_create_authentication_challenge() {
//         // Your test code here
//     }
//
//     #[tokio::test]
//     async fn test_verify_authentication() {
//         // Your test code here
//     }
// }
