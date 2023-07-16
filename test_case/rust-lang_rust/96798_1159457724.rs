plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 0182fd99afaf4a3d602de6b88506edaf6043c125 and add69f2576b823894cc1829cfaccbd1d8d306554
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building stage2 tool clippy-driver (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.24s
thread 'main' panicked at 'fs::read(stamp) failed with No such file or directory (os error 2) ("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-rustc/x86_64-unknown-linux-gnu/release/.librustc.stamp")', src/bootstrap/lib.rs:1398:24
Assembling stage3 compiler (x86_64-unknown-linux-gnu)
Build completed unsuccessfully in 0:00:02
