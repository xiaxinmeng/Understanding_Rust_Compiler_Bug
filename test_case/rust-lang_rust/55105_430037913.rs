
$ git clone https://github.com/hyperium/hyper
$ cd hyper
$ git checkout v0.12.10
$ cargo +nightly rustc --release -v --target "armv7-linux-androideabi" -- -C target-feature=+neon
[...]
LLVM ERROR: ran out of registers during register allocation                                                      
error: Could not compile `hyper`
