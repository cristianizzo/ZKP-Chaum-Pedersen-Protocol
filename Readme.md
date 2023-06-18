# ZKP-Chaum-Pedersen-Protocol

## Introduction
This application provide a secure and privacy-preserving authentication method using Zero-Knowledge Proofs (ZKP).
It's built on the foundation of the Chaum–Pedersen Protocol as outlined in ["Cryptography: An Introduction (3rd Edition) Nigel Smart" on page 377, section "3. Sigma Protocols" subsection "3.2. Chaum–Pedersen Protocol"](https://www.cs.umd.edu/~waa/414-F11/IntroToCrypto.pdf). It is specifically adapted to support 1-factor authentication, which involves the exact matching of a registration password with a login password. The application is built using Rust and communicates using the gRPC protocol according to the provided interface described in the "protobuf" schema.

## The ZKP Protocol

Zero-Knowledge Proofs are cryptographic protocols that allow one party to prove to another that they know a specific piece of information without revealing the actual information. The protocol is called a Zero-Knowledge Proof because no knowledge is gained by the verifying party, other than the fact that the statement is true.

In our adaptation, we utilize the Chaum–Pedersen Protocol to allow users to authenticate themselves by proving that they know the registration password without revealing it. The protocol is described in detail in "Cryptography: An Introduction (3rd Edition) Nigel Smart" and is particularly suitable for authentication purposes due to its security properties.

### Registration Process

During registration, the user is asked to enter a numeric password. This password is used to generate a commitment and a corresponding opening. The commitment is stored by the application, while the opening is used by the user during the authentication process.

### Login Process

During the login process, the user is challenged to prove that they know the password they registered with, without revealing the password itself. They must construct a proof using their opening and send it to the application via gRPC. The application then uses the Zero-Knowledge Proof protocol to verify if the proof is valid or not. If the proof is valid, the user is authenticated.

## Dependencies

Rust: Make sure you have the Rust programming language installed.
gRPC: This application uses the gRPC protocol. Follow the gRPC installation guide for Rust.
Docker: The PostgreSQL database used by the app is containerized using Docker. Ensure Docker is installed on your system.

## Build the project:

```cargo build```

## Test

- copy the example env variables to .env.test

- Launch Postgres test database in docker:
``` docker run --name zkp-test-db -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=password -e POSTGRES_DB=zkp-test -p 5432:5432 -d postgres ```

- Run Test:
  ```cargo test```

## Run:

- copy the example env variables to .env

- Launch Postgres test database in docker:
  ``` docker run --name zkp-db -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=password -e POSTGRES_DB=zkp -p 5432:5432 -d postgres ```

- Run Server:
```cargo run```
