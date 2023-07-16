Dockerfile
FROM rust:1.51-slim-buster

RUN apt-get update && \
    apt-get install -y --no-install-recommends gcc-mingw-w64-i686 openssl libssl-dev pkg-config

RUN rustup target add i686-pc-windows-gnu

RUN cargo install cargo-bisect-rustc

RUN mkdir /home/issue-79609
WORKDIR /home/issue-79609
RUN echo "fn main() {\nprintln!(\"Hello, Windows!\");\n}" > hello_world.rs
RUN echo '#!/bin/bash\nrustc --target=i686-pc-windows-gnu -C panic=abort hello_world.rs |& grep "undefined reference to \`_Unwind_Resume'\''"\n[ $? -eq 1 ]' > build.sh && chmod u+x build.sh

ENTRYPOINT ["cargo", "bisect-rustc", "--target", "i686-pc-windows-gnu", "--script", "./build.sh"]
