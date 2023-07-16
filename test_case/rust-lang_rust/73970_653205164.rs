
FROM ubuntu:20.04
RUN apt-get update && apt-get install -qyy curl binutils build-essential git
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN ln -s /root/.cargo/bin/* /usr/bin/
RUN git clone https://github.com/cristicbz/rust-regression-441
WORKDIR /rust-regression-441
RUN cargo build
