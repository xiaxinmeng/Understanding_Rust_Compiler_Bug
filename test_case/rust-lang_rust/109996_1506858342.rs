console
$ curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain nightly
$ source "$HOME/.cargo/env"
$ git clone https://github.com/rust-lang/rust.git
$ cd rust
$ ./configure --set build.rustc="$(which rustc)" --set build.cargo="$(which cargo)"
$ ./x.py build --target i686-pc-windows-gnu
