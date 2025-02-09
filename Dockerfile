FROM rust:1.75-slim as builder

WORKDIR /usr/src/app

# Install protobuf compiler and build dependencies
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy source files to cache dependencies
RUN mkdir src proto && \
    echo "fn main() {}" > src/main.rs && \
    echo "fn main() {}" > src/lib.rs && \
    touch proto/service.proto

# Build dependencies
RUN cargo build --release

# Remove the dummy files
RUN rm -rf src proto

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

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create data directory with proper permissions
RUN mkdir -p /usr/local/bin/data && \
    groupadd -r app && useradd -r -g app app && \
    chown -R app:app /usr/local/bin/data

# Run as non-root user
USER app

# Expose ports
EXPOSE 3000 50051

# Set environment variables
ENV RUST_LOG=info

# Run the binary
CMD ["./server"] 