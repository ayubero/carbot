# Use Rust image
FROM rust:latest AS builder

# Install system dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libudev-dev \
    pkg-config && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

# Build the release version
RUN cargo build --release

# Use a minimal runtime image
FROM rust:latest
WORKDIR /app

# Copy the built binary
COPY --from=builder /app/target/release/backend .

# Expose the port your app runs on
EXPOSE 5000

CMD ["./backend"]
