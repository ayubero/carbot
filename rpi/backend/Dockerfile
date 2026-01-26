FROM --platform=$BUILDPLATFORM rust:bookworm AS builder

ARG TARGETPLATFORM

# 1. Enable arm64 architecture in dpkg and install cross-toolchains
RUN dpkg --add-architecture arm64 && \
    apt-get update && apt-get install -y \
    binutils-aarch64-linux-gnu \
    gcc-aarch64-linux-gnu \
    # The actual ARM64 library headers
    libudev-dev:arm64 \
    pkg-config

WORKDIR /app
COPY . .

RUN if [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
    rustup target add aarch64-unknown-linux-gnu; \
    # 2. Tell pkg-config where to find the ARM64 libraries
    export PKG_CONFIG_PATH="/usr/lib/aarch64-linux-gnu/pkgconfig"; \
    export PKG_CONFIG_ALLOW_CROSS=1; \
    # 3. Set the linker
    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc; \
    \
    cargo build --release --target aarch64-unknown-linux-gnu; \
    cp target/aarch64-unknown-linux-gnu/release/backend /app/backend; \
    else \
    cargo build --release; \
    cp target/release/backend /app/backend; \
    fi

# --- Runtime Stage ---
FROM debian:bookworm-slim
WORKDIR /app

# Install the ARM64 runtime version of libudev
RUN apt-get update && apt-get install -y libudev1 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/backend .

EXPOSE 5000
CMD ["./backend"]