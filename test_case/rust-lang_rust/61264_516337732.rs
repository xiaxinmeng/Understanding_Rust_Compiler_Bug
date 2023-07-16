docker
FROM rust:1.35.0
#         ^-- 1.35.0 build breaks, 1.34.2 here and build is good

ENV TARGET=x86_64-unknown-linux-musl

ARG LIBRESSL=libressl-2.9.1
ARG MUSL=musl-1.1.22

ARG DEBIAN_FRONTEND=noninteractive
RUN apt -y update && \
    apt -y install clang && \
    rustup target install ${TARGET}

RUN printf '[target.x86_64-unknown-linux-musl]\nlinker = "musl-clang"\n' >/usr/local/cargo/config

WORKDIR /work

RUN curl -fO https://www.musl-libc.org/releases/${MUSL}.tar.gz && \
    tar zxf ${MUSL}.tar.gz && \
    cd ${MUSL} && \
    ./configure CC=clang CFLAGS="-fPIC" --prefix=/opt/musl --enable-shared=no && \
    make V=1 -j4 && \
    make V=1 install

ENV PATH=/opt/musl/bin:$PATH

# We do Musl build hence point to correct tool
ENV CC_x86_64_unknown_linux_musl=musl-clang

# Musl is missing some kernel specific header files. Do symbolic links to fix
# this.
RUN \
    ln -s /usr/include/linux /opt/musl/include/ && \
    ln -s /usr/include/x86_64-linux-gnu/asm /opt/musl/include/ && \
    ln -s /usr/include/asm-generic /opt/musl/include/

RUN curl -fO https://ftp.openbsd.org/pub/OpenBSD/LibreSSL/${LIBRESSL}.tar.gz && \
    tar zxf ${LIBRESSL}.tar.gz && \
    cd ${LIBRESSL} && \
    ./configure CC=musl-clang LD=musl-clang CFLAGS="-fPIC" --prefix=/opt/libressl --enable-shared=no --sysconfdir=/etc && \
    make V=1 -j4 && make -j4 check && \
    make install

ENV X86_64_UNKNOWN_LINUX_MUSL_OPENSSL_LIB_DIR=/opt/libressl/lib
ENV X86_64_UNKNOWN_LINUX_MUSL_OPENSSL_INCLUDE_DIR=/opt/libressl/include

# cargo-outdated is innocent crate, it's just to trigger the error in build
#RUN cargo install --target=$TARGET cargo-outdated

RUN rustup toolchain install nightly
RUN rustup target install --toolchain nightly ${TARGET}
WORKDIR /
RUN git clone https://github.com/kbknapp/cargo-outdated
WORKDIR /cargo-outdated
RUN cargo +nightly rustc --target ${TARGET} -- -Z print-link-args 
