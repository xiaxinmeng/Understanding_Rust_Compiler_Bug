dockerfile
> FROM rust:1.51-slim-buster
> 
> RUN apt-get update && \
>     apt-get install -y --no-install-recommends gcc-mingw-w64-i686
> 
> RUN rustup target add i686-pc-windows-gnu
> 
> RUN echo "fn main() {\nprintln!(\"Hello, Windows!\");\n}" > hello_world.rs && \
>     rustc --target=i686-pc-windows-gnu -C panic=abort -o hello_world.exe hello_world.rs
> 