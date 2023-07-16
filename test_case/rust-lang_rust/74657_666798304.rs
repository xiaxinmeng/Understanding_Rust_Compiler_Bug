
FROM ubuntu:20.04
WORKDIR /root
RUN apt-get update && apt-get install -y curl git llvm-dev build-essential \
    && curl https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.45.0 \
    && git clone --depth=1 https://github.com/rusqlite/rusqlite
WORKDIR rusqlite
RUN ../.cargo/bin/cargo fetch
CMD . /root/.cargo/env; RUSTFLAGS="-Clinker-plugin-lto=LLVMgold.so" cargo build --release
