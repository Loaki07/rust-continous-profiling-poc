FROM rust:1.75-slim as builder

WORKDIR /usr/src/app

# Install protobuf compiler and build dependencies
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY proto ./proto

# Build the application
RUN cargo build --bin server --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /usr/local/bin

# Copy the built binary
COPY --from=builder /usr/src/app/target/release/server .

# Install runtime dependencies if any
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Run as non-root user
RUN groupadd -r app && useradd -r -g app app
USER app

# Expose the port the server listens on
EXPOSE 3000

# Run the binary
CMD ["./server"] 