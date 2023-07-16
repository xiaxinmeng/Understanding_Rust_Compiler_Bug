dockerfile
FROM rust:1.64

WORKDIR /root/
RUN git clone --depth 1 https://github.com/rust-lang/rust

RUN echo "changelog-seen = 2"       > /root/rust/config.toml
RUN echo "[llvm]"                  >> /root/rust/config.toml
RUN echo "download-ci-llvm = true" >> /root/rust/config.toml

WORKDIR /root/rust

RUN /root/rust/x doc core --json --stage 0

RUN find /root/rust/build -name "core.json" | grep . || echo "not found"

