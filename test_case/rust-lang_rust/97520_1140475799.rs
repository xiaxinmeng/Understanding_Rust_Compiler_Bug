dockerfile
ARG ARCH
FROM ${ARCH}debian:stable-slim

ENV PATH=$PATH:/root/.cargo/bin
RUN apt-get update && apt-get -y install curl gcc
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN cargo install empty-library || true
RUN cargo install stylua
