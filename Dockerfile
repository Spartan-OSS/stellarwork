# Use Rust 1.99.0 as the base image (matches rust-toolchain.toml)
FROM rust:1.99.0-bookworm

# Install Python 3.12 and other utilities required for the model and CI
RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    curl \
    git \
    make \
    && rm -rf /var/lib/apt/lists/*

# Add WebAssembly target for Soroban contract compilation
RUN rustup target add wasm32v1-none

# Install stellar-cli 
# (this will take a few minutes on the first build, but will be cached for all future runs)
RUN cargo install --locked stellar-cli --features opt

WORKDIR /workspace

# Keep container running in the background for exec sessions
CMD ["tail", "-f", "/dev/null"]
