# Use the official Rust image as a base
FROM rust:1.54 as builder

# Create a new directory for the application
WORKDIR /usr/src/zkp-rust-app

# Copy over all the files and directories
COPY . .

# Install the necessary build tools and compile the application in release mode
RUN cargo build --release

# Use a smaller image for the final build
FROM debian:buster-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/zkp-rust-app/target/release/zkp-rust-app /usr/local/bin

# Copy all other files
COPY --from=builder /usr/src/zkp-rust-app .

# Set the environment variable for gRPC
ENV GRPC_VERBOSITY=INFO

# Expose the port used by the gRPC server
EXPOSE 50051

# Set the startup command to run the binary
CMD ["zkp-rust-app"]
