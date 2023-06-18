use tonic::Request;
use zkp_app::{
    config::CONFIG,
    models::user_exists,
    services::zkp_auth::{
        auth_server::Auth, AuthenticationAnswerRequest, AuthenticationChallengeRequest,
        RegisterRequest,
    },
    utils::zkp_utils::{
        calculate_default_hash, convert_string_to_bigint, perform_modular_exponentiation,
    },
};

use zkp_app::utils::{convert_hex_to_bigint, generate_random_bigint};

mod utils;

#[tokio::test]
async fn test_register_and_login_new_user() -> Result<(), Box<dyn std::error::Error>> {
    let (auth_handler, db_pool) = utils::setup().await?;

    // Load cryptographic parameters from configuration.
    let p = convert_string_to_bigint(&CONFIG.p);
    let q = convert_string_to_bigint(&CONFIG.q);
    let g = convert_string_to_bigint(&CONFIG.g);
    let h = convert_string_to_bigint(&CONFIG.h);

    // ----------------------------------------------------------------
    // Create a RegisterRequest with random user credentials.
    // ----------------------------------------------------------------
    let (username, _password, x) = utils::generate_user_credentials();

    let y1 = perform_modular_exponentiation(&g, &x, &p).to_str_radix(16);
    let y2 = perform_modular_exponentiation(&h, &x, &p).to_str_radix(16);

    println!("y2 {}", y1);
    println!("y2 {}", y2);

    let register_request = Request::new(RegisterRequest {
        user: username.clone(),
        y1,
        y2,
    });

    // Call the register method of AuthHandler to register the new user.
    let response = auth_handler.register(register_request).await;
    println!("{:?}", response);

    // Assert that the registration response is successful.
    assert!(response.is_ok());

    // ----------------------------------------------------------------
    // Verify that the user is registered
    // ----------------------------------------------------------------
    let mut pool = db_pool.acquire().await?;
    let user_hash = &calculate_default_hash(username.clone()).to_str_radix(16);
    let user_is_registered = user_exists(&mut pool, &user_hash).await?;
    assert!(user_is_registered);

    // ----------------------------------------------------------------
    // Log in as the registered user by creating an authentication challenge.
    // ----------------------------------------------------------------

    // Generate random k for the commitment.
    let k = generate_random_bigint(g.clone(), &q - &g);

    // Compute commitment r1 and r2.
    let r1 = perform_modular_exponentiation(&g, &k, &p);
    let r2 = perform_modular_exponentiation(&h, &k, &p);

    // Create a challenge request with the computed commitments.
    let challenge_request = Request::new(AuthenticationChallengeRequest {
        user: username.clone(),
        r1: r1.to_str_radix(16),
        r2: r2.to_str_radix(16),
    });

    // Call create_authentication_challenge to initiate the authentication process.
    let response = auth_handler
        .create_authentication_challenge(challenge_request)
        .await?;
    println!("Challenge {:?}", response);

    // ----------------------------------------------------------------
    // Authentication Challenge
    // ----------------------------------------------------------------

    // Extract challenge response and compute the answer.
    let challenge_response = response.into_inner();
    let auth_id = challenge_response.auth_id.clone();
    let c = convert_hex_to_bigint(&challenge_response.c);

    // Compute s for the authentication response. // s = k - c * x (mod q)
    let s = (((&k - &c * &x) % (&q)) + (&q)) % (&q);

    // Create an authentication answer request with the computed 's'.
    let auth_answer_request = Request::new(AuthenticationAnswerRequest {
        auth_id: String::from(auth_id),
        s: s.to_str_radix(16),
    });

    // Call verify_authentication to finalize the authentication process.
    let response = auth_handler
        .verify_authentication(auth_answer_request)
        .await?;
    println!("Verify Authentication {:?}", response);

    // Extract session ID from the response and output it.
    let session_id = response.into_inner().session_id.clone();
    println!("Login Succeeded - session id {:?}", session_id);

    Ok(())
}

#[tokio::test]
async fn test_fail_to_verify_authentication() -> Result<(), Box<dyn std::error::Error>> {
    let (auth_handler, _db_pool) = utils::setup().await?;

    // Load cryptographic parameters from configuration.
    let p = convert_string_to_bigint(&CONFIG.p);
    let q = convert_string_to_bigint(&CONFIG.q);
    let g = convert_string_to_bigint(&CONFIG.g);
    let h = convert_string_to_bigint(&CONFIG.h);

    // Register user first.
    let (username, _password, x) = utils::generate_user_credentials();
    let y1 = perform_modular_exponentiation(&g, &x, &p).to_str_radix(16);
    let y2 = perform_modular_exponentiation(&h, &x, &p).to_str_radix(16);
    let register_request = Request::new(RegisterRequest {
        user: username.clone(),
        y1,
        y2,
    });

    // Call the register method of AuthHandler to register the new user.
    let _ = auth_handler.register(register_request).await?;

    // Generate random k for the commitment.
    let k = generate_random_bigint(g.clone(), &q - &g);

    // Compute commitment r1 and r2.
    let r1 = perform_modular_exponentiation(&g, &k, &p);
    let r2 = perform_modular_exponentiation(&h, &k, &p);

    // Create a challenge request with the computed commitments.
    let challenge_request = Request::new(AuthenticationChallengeRequest {
        user: username.clone(),
        r1: r1.to_str_radix(16),
        r2: r2.to_str_radix(16),
    });

    // Call create_authentication_challenge to initiate the authentication process.
    let response = auth_handler
        .create_authentication_challenge(challenge_request)
        .await?;

    // Extract challenge response and compute the answer.
    let challenge_response = response.into_inner();
    let auth_id = challenge_response.auth_id.clone();

    // Compute incorrect s for the authentication response.
    let s = generate_random_bigint(g.clone(), &q - &g); // Random incorrect 's'

    // Create an authentication answer request with the incorrect 's'.
    let auth_answer_request = Request::new(AuthenticationAnswerRequest {
        auth_id: String::from(auth_id),
        s: s.to_str_radix(16),
    });

    // Call verify_authentication to finalize the authentication process.
    let response = auth_handler
        .verify_authentication(auth_answer_request)
        .await;

    // Assert that the verification is NOT successful.
    assert!(response.is_err());

    println!("As expected, failed to verify authentication.");

    Ok(())
}
