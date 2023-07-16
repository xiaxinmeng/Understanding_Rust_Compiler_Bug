
FROM ghcr.io/cross-rs/x86_64-unknown-linux-musl:latest

ENV ZLIB_VERSION=1.2.13
ENV POSTGRESQL_VERSION=11.20
ENV OPENSSL_VERSION=1.1.1t
ENV MUSL_PREFIX=x86_64-linux-musl

RUN apt-get update && \
    apt-get --assume-yes install \
    protobuf-compiler

RUN echo "Building OpenSSL" && \
    cd /tmp && \
    curl -fLO "https://www.openssl.org/source/openssl-$OPENSSL_VERSION.tar.gz" && \
    tar xvzf "openssl-$OPENSSL_VERSION.tar.gz" && cd "openssl-$OPENSSL_VERSION" && \
    env CC=$MUSL_PREFIX-gcc ./Configure no-shared no-zlib -fPIC --prefix=/usr/local/$MUSL_PREFIX-target -DOPENSSL_NO_SECURE_MEMORY linux-x86_64 && \
    env C_INCLUDE_PATH=/usr/local/$MUSL_PREFIX/include/ make depend && \
    env C_INCLUDE_PATH=/usr/local/$MUSL_PREFIX/include/ make && \
    make install_sw && \
    rm -r /tmp/*

RUN echo "Building zlib" && \
    cd /tmp && \
    curl -fLO "https://zlib.net/zlib-$ZLIB_VERSION.tar.gz" && \
    tar xzf "zlib-$ZLIB_VERSION.tar.gz" && cd "zlib-$ZLIB_VERSION" && \
    CC=$MUSL_PREFIX-gcc ./configure --static --prefix=/usr/local/$MUSL_PREFIX-target && \
    make && make install && \
    rm -r /tmp/*


RUN echo "Building libpq" && \
    cd /tmp && \
    curl -fLO "https://ftp.postgresql.org/pub/source/v$POSTGRESQL_VERSION/postgresql-$POSTGRESQL_VERSION.tar.gz" && \
    tar xzf "postgresql-$POSTGRESQL_VERSION.tar.gz" && cd "postgresql-$POSTGRESQL_VERSION" && \
    CC=$MUSL_PREFIX-gcc CPPFLAGS="-I/usr/local/$MUSL_PREFIX/include -I/usr/local/$MUSL_PREFIX-target/include" LDFLAGS="-L/usr/local/$MUSL_PREFIX/lib -L/usr/local/$MUSL_PREFIX-target/lib" ./configure --with-openssl --without-readline --prefix=/usr/local/$MUSL_PREFIX-target && \
    cd src/interfaces/libpq && make all-static-lib && make install-lib-static && \
    rm -r /tmp/*

env OPENSSL_STATIC=yes
env PKG_CONFIG_PATH=/usr/local/$MUSL_PREFIX-target/lib/pkgconfig
