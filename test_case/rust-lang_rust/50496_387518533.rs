dockerfile
FROM ubuntu:16.04

RUN apt-get update -y && apt-get install -y curl gcc libssl-dev file make pkg-config time

RUN curl https://sh.rustup.rs | sh -s -- -y

ENV PATH=$PATH:/root/.cargo/bin
RUN cargo install rustup-toolchain-install-master
RUN rustup-toolchain-install-master \
  c1168be5360f17516b233be85ebb193bb4e612bf  \
  a2726846f6d6280b752019472b6bd791c0752006

RUN curl -Lo main.rs \
  https://github.com/rust-lang-nursery/rustc-perf/raw/254d28f6181cd5d20cedcd3fa9ae36df847da958/collector/benchmarks/tuple-stress/src/main.rs

RUN time rustc +a2726846f6d6280b752019472b6bd791c0752006 main.rs
RUN time rustc +c1168be5360f17516b233be85ebb193bb4e612bf main.rs
