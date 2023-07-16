
$ curl -I https://ci-artifacts.rust-lang.org/rustc-builds/bef2b7cd1c7bcb3393f10d5752fcf9ee3026bce8/rustc-nightly-x86_64-unknown-linux-gnu.tar.xz
HTTP/2 200 
$ curl -I https://ci-artifacts.rust-lang.org/rustc-builds/bef2b7cd1c7bcb3393f10d5752fcf9ee3026bce8/rustc-nightly-src.tar.xz
HTTP/2 200 
$ rustup component add rustc-src --target ''
error: toolchain 'nightly-x86_64-unknown-linux-gnu' does not contain component 'rustc-src' for target ''; did you mean 'rust-src'?
