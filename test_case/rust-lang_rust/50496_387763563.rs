dockerfile
FROM ubuntu:16.04

RUN apt-get update -y && apt-get install -y curl gcc libssl-dev file make pkg-config time git

RUN curl https://sh.rustup.rs | sh -s -- -y

ENV PATH=$PATH:/root/.cargo/bin
RUN cargo install rustup-toolchain-install-master
RUN rustup-toolchain-install-master \
  c1168be5360f17516b233be85ebb193bb4e612bf  \
  a2726846f6d6280b752019472b6bd791c0752006

RUN git clone https://github.com/rust-lang-nursery/rustc-perf

RUN cargo +a2726846f6d6280b752019472b6bd791c0752006 \
  fetch --manifest-path rustc-perf/collector/benchmarks/html5ever/Cargo.toml
RUN cargo +a2726846f6d6280b752019472b6bd791c0752006 \
  build --manifest-path rustc-perf/collector/benchmarks/html5ever/Cargo.toml
RUN cargo +c1168be5360f17516b233be85ebb193bb4e612bf \
build --manifest-path rustc-perf/collector/benchmarks/html5ever/Cargo.toml
