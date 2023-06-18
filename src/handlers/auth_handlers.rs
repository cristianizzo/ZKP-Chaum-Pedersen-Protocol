use crate::config::CONFIG;
use crate::db::{acquire_db_connection, DbPool};
use crate::errors::AppError;
use crate::models::auth::{delete_auth_by_id, find_auth_by_id, insert_or_update_auth};
use crate::models::user::{create_user, find_user_by_id, user_exists};
use crate::utils::zkp_utils::{
    calculate_default_hash, convert_hex_to_bigint, convert_string_to_bigint,
    generate_random_bigint, generate_uuid, perform_modular_exponentiation,
};
use crate::zkp_auth::auth_server::Auth;
use crate::zkp_auth::{
    AuthenticationAnswerRequest, AuthenticationAnswerResponse, AuthenticationChallengeRequest,
    AuthenticationChallengeResponse, RegisterRequest, RegisterResponse,
};
use log::info;
use num_bigint::BigInt;
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct AuthHandler {
    db_pool: Arc<DbPool>,
}

impl AuthHandler {
    pub fn new(db_pool: Arc<DbPool>) -> Self {
        Self { db_pool }
    }
}

#[tonic::async_trait]
impl Auth for AuthHandler {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        info!("Registration initiated");
        let request = request.into_inner();
        let user = &request.user;
        let y1 = &request.y1;
        let y2 = &request.y2;

        let mut pool = acquire_db_connection(&self.db_pool)
            .await
            .map_err(|err| AppError::DatabaseConnectionError(err.to_string()))?;

        let user_hash = &calculate_default_hash(user).to_str_radix(16);

        let user_is_registered = user_exists(&mut pool, &user_hash)
            .await
            .map_err(AppError::DatabaseError)?;

        if !user_is_registered {
            create_user(&mut pool, &user_hash, y1, y2)
                .await
                .map_err(AppError::DatabaseError)?;

            info!("Registration successful for user: {}", user);
        } else {
            info!(
                "User: {} is already registered. Please log in instead.",
                user
            );
            return Err(AppError::UserAlreadyExists.into());
        }

        Ok(Response::new(RegisterResponse {}))
    }

    async fn create_authentication_challenge(
        &self,
        request: Request<AuthenticationChallengeRequest>,
    ) -> Result<Response<AuthenticationChallengeResponse>, Status> {
        info!("Authentication Challenge initiated");

        let request = request.into_inner();

        let q = convert_string_to_bigint(&CONFIG.q);
        let g = convert_string_to_bigint(&CONFIG.g);
        let user = &request.user;
        let r1 = &request.r1;
        let r2 = &request.r2;
        let c_hex = generate_random_bigint(g, &q - 1).to_str_radix(16);

        let mut pool = acquire_db_connection(&self.db_pool)
            .await
            .map_err(|err| AppError::DatabaseConnectionError(err.to_string()))?;

        let user_hash = calculate_default_hash(user).to_str_radix(16);
        let user_is_registered = user_exists(&mut pool, &user_hash)
            .await
            .map_err(AppError::DatabaseError)?;

        if !user_is_registered {
            return Err(AppError::UserDoesNotExist.into());
        }

        insert_or_update_auth(&mut pool, &user_hash, r1, r2, &c_hex)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(Response::new(AuthenticationChallengeResponse {
            auth_id: user_hash,
            c: c_hex,
        }))
    }

    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> Result<Response<AuthenticationAnswerResponse>, Status> {
        info!("Verify Authentication initiated");

        let request = request.into_inner();
        let auth_id = &request.auth_id;
        let s = &request.s;

        let mut pool = acquire_db_connection(&self.db_pool)
            .await
            .map_err(|err| AppError::DatabaseConnectionError(err.to_string()))?;

        // Retrieving parameters for verification
        let auth = find_auth_by_id(&mut pool, auth_id)
            .await
            .map_err(AppError::DatabaseError)?
            .ok_or_else(|| AppError::UserDoesNotExist)?;

        let user = find_user_by_id(&mut pool, auth_id)
            .await
            .map_err(AppError::DatabaseError)?
            .ok_or_else(|| AppError::UserDoesNotExist)?;

        // Convert to BigInt
        let y1: BigInt = convert_hex_to_bigint(&user.y1);
        let y2: BigInt = convert_hex_to_bigint(&user.y2);
        let r1: BigInt = convert_hex_to_bigint(&auth.r1);
        let r2: BigInt = convert_hex_to_bigint(&auth.r2);
        let c: BigInt = convert_hex_to_bigint(&auth.c);
        let s: BigInt = convert_hex_to_bigint(s);

        let p = convert_string_to_bigint(&CONFIG.p);
        let g = convert_string_to_bigint(&CONFIG.g);
        let h = convert_string_to_bigint(&CONFIG.h);

        // g^s * y1^c
        let result1 = ((perform_modular_exponentiation(&g, &s, &p)
            * perform_modular_exponentiation(&y1, &c, &p)
            % &p)
            + &p)
            % &p;

        // h^s * y2^c
        let result2 = ((perform_modular_exponentiation(&h, &s, &p)
            * perform_modular_exponentiation(&y2, &c, &p)
            % &p)
            + &p)
            % &p;

        // Valid challenge
        if &r1 == &result1 && &r2 == &result2 {
            info!("Authentication Success!");

            // Delete the auth document
            delete_auth_by_id(&mut pool, auth_id)
                .await
                .map_err(AppError::DatabaseError)?;

            let session_id = generate_uuid();
            return Ok(Response::new(AuthenticationAnswerResponse { session_id }));
        }

        info!("Authentication Failed!");
        return Err(AppError::BadCredentials.into());
    }
}
