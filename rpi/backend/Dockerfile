FROM --platform=$BUILDPLATFORM rust:bookworm AS builder

ARG TARGETPLATFORM

# Install cross-toolchains and build dependencies
RUN dpkg --add-architecture arm64 && \
    apt-get update && apt-get install -y \
    binutils-aarch64-linux-gnu \
    gcc-aarch64-linux-gnu \
    g++-aarch64-linux-gnu \
    pkg-config git cmake build-essential \
    libudev-dev:arm64 libusb-1.0-0-dev:arm64 libssl-dev:arm64 \
    python3

WORKDIR /app

# Build librealsense from source for ARM64
RUN if [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
    git clone https://github.com/IntelRealSense/librealsense.git && \
    cd librealsense && git checkout v2.56.5 && \
    mkdir build && cd build && \
    # Cross-compilation CMake flags
    cmake .. \
    -DCMAKE_C_COMPILER=aarch64-linux-gnu-gcc \
    -DCMAKE_CXX_COMPILER=aarch64-linux-gnu-g++ \
    -DCMAKE_SYSTEM_NAME=Linux \
    -DCMAKE_SYSTEM_PROCESSOR=aarch64 \
    -DBUILD_EXAMPLES=false \
    -DBUILD_GRAPHICAL_EXAMPLES=false \
    -DCMAKE_BUILD_TYPE=Release && \
    make -j$(nproc) && make install; \
    fi

COPY . .

# Set environment variables and build Rust backend
RUN if [ "$TARGETPLATFORM" = "linux/arm64" ]; then \
    rustup target add aarch64-unknown-linux-gnu; \
    export PKG_CONFIG_PATH="/usr/local/lib/pkgconfig:/usr/lib/aarch64-linux-gnu/pkgconfig"; \
    export PKG_CONFIG_ALLOW_CROSS=1; \
    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc; \
    cargo build --release --target aarch64-unknown-linux-gnu --no-default-features; \
    cp target/aarch64-unknown-linux-gnu/release/backend /app/backend; \
    else \
    cargo build --release; \
    cp target/release/backend /app/backend; \
    fi

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libudev1 libusb-1.0-0 libssl3 ffmpeg && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled library from the builder
COPY --from=builder /usr/local/lib/librealsense2.so* /usr/local/lib/
RUN ldconfig

# Create a directory for recordings
RUN mkdir -p /recordings
VOLUME /recordings

COPY --from=builder /app/backend .

EXPOSE 5000
CMD ["./backend"]