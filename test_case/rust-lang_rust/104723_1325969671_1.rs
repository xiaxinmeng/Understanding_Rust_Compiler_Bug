console
$ ./x.py build -i library/std
[...]
    Finished release [optimized] target(s) in 0.13s
Copying stage1 std from stage1 (aarch64-apple-darwin -> aarch64-apple-darwin / aarch64-apple-darwin)
Build completed successfully in ...
$ rustup toolchain link stage1 build/aarch64-apple-darwin/stage1
$ rustc +stage1 -vV
rustc 1.67.0-dev
binary: rustc
commit-hash: unknown
commit-date: unknown
host: aarch64-apple-darwin
release: 1.67.0-dev
LLVM version: 15.0.4
