
RUN CC=arm-linux-gnueabihf-gcc \
    CFLAGS="-march=armv6 -marm -mfpu=vfp" \
    ./configure --prefix=/musl-arm --enable-wrapper=yes
