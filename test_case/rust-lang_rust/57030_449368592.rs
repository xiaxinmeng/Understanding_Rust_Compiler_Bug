
ENV CC_armv7_unknown_linux_gnueabihf=armv7-unknown-linux-gnueabihf-gcc \
    AR_armv7_unknown_linux_gnueabihf=armv7-unknown-linux-gnueabihf-ar \
    CXX_armv7_unknown_linux_gnueabihf=armv7-unknown-linux-gnueabihf-g++

ENV CC_thumbv7neon-unknown-linux-gnueabihf=armv7-unknown-linux-gnueabihf-gcc \
    AR_thumbv7neon-unknown-linux-gnueabihf=armv7-unknown-linux-gnueabihf-ar \
    CXX_thumbv7neon-unknown-linux-gnueabihf=armv7-unknown-linux-gnueabihf-g++

ENV HOSTS=armv7-unknown-linux-gnueabihf

ENV TARGETS=armv7-unknown-linux-gnueabihf,thumbv7neon-unknown-linux-gnueabihf
