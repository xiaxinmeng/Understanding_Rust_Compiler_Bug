plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between f9b2e3c87b76c03cc398be8b8f65d096c0583eb2 and b4f7c33245b650dd4dbf7a78c74ebffce6ead398
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage2 tool clippy-driver (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.18s
thread 'main' panicked at 'fs::read(stamp) failed with No such file or directory (os error 2) ("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/.librustc.stamp")', src/bootstrap/lib.rs:1375:24
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Build completed unsuccessfully in 0:00:01
