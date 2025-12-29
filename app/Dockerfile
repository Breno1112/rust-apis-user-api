# --- Stage 1: Build ---
FROM rust:1.92-bookworm AS builder

# 1. Create a workspace and set the working directory
WORKDIR /usr/src/app

# 2. Pre-build dependencies only (to leverage Docker layer caching)
# This prevents re-downloading crates unless Cargo.toml/lock change.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src

# 3. Copy the actual source code and build the final binary
COPY src ./src
RUN echo "LS before build"
RUN find . -maxdepth 10 -not -path '*/.pwd*'
RUN cargo build --release
RUN echo "LS after build"
RUN find . -maxdepth 10 -not -path '*/.*'

# CMD ["tail", "-f", "/dev/null"]


# --- Stage 2: Run ---
FROM debian:bookworm-slim

# Install minimal runtime dependencies (SSL is often needed for web APIs)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Set up a non-root user for security
RUN useradd -ms /bin/bash appuser
USER appuser
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/hello-world /app/app_binary

# Set environment variables and expose port
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8080

CMD ["./app_binary"]
