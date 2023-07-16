rust
$ time RUST_BACKTRACE=0 rustc -vv
!! LD_LIBRARY_PATH=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:
!! Executing '/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build//x86_64-unknown-linux-gnu/stage2/bin//rustc' with args: '-vv'
error: Option 'verbose' given more than once.


real	0m0.045s
user	0m0.033s
sys	0m0.012s
$ time RUST_BACKTRACE=0 rustc -vv
!! LD_LIBRARY_PATH=/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib:/home/xftroxgpx/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib:
!! Executing '/home/xftroxgpx/build/2nonpkgs/rust.stuff/rust/rust/build//x86_64-unknown-linux-gnu/stage2/bin//rustc' with args: '-vv'
error: Option 'verbose' given more than once.


real	0m0.046s
user	0m0.035s
sys	0m0.011s

$ time RUST_BACKTRACE=0 ~/.cargo/bin/rustc -vv
error: Option 'verbose' given more than once.


real	0m0.146s
user	0m0.131s
sys	0m0.017s
$ time RUST_BACKTRACE=0 ~/.cargo/bin/rustc -vv
error: Option 'verbose' given more than once.


real	0m0.138s
user	0m0.120s
sys	0m0.020s

