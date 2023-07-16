dockerfile
FROM rust:1.51-slim-buster

RUN apt-get update && \
    apt-get install -y --no-install-recommends gcc-mingw-w64-i686 binutils-mingw-w64-i686 pkg-config libssl-dev 

RUN echo "fn main() {\nprintln!(\"Hello, Windows!\");\n}" > hello_world.rs
RUN echo "rustup target add i686-pc-windows-gnu\nrustc --target=i686-pc-windows-gnu -C panic=abort -C linker=/usr/bin/i686-w64-mingw32-gcc -o hello_world.exe hello_world.rs" > hello_world.sh
RUN chmod +x hello_world.sh
RUN cat hello_world.rs
RUN cat hello_world.sh
RUN rustup default nightly-2020-03-14 # Change this to 15 to fail
RUN ./hello_world.sh
