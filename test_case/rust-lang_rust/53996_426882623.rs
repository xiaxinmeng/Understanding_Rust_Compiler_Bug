 console
$ grep '^target' config.toml
target = ["thumbv7m-none-eabi"]

$ ./x.py build --stage 1 src/libstd

$ rustup toolchain link stage1 build/x86_64-unknown-linux-gnu/stage1

$ cd /path/to/some/project

$ # this fails if proc_macro appears somewhere in the dependency graph
$ cargo +stage1 build --target thumbv7m-none-eabi
