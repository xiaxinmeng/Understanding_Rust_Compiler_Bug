
[0Ktravis_time:start:stage0-fabricate
[0KBuilding stage0 tool fabricate (x86_64-unknown-linux-gnu)
[01:15:43] [m[m[32m[1m   Compiling[m same-file v0.1.3
[01:15:43] [m[m[32m[1m   Compiling[m yaml-rust v0.3.5
[01:15:43] [m[m[32m[1m   Compiling[m pkg-config v0.3.9
[01:15:43] [m[m[32m[1m   Compiling[m miniz-sys v0.1.10
[01:15:44] [m[m[32m[1m   Compiling[m filetime v0.1.14
[01:15:44] [m[m[32m[1m   Compiling[m xattr v0.1.11
[01:15:45] [m[m[32m[1m   Compiling[m walkdir v1.0.7
[01:15:51] [m[m[32m[1m   Compiling[m tar v0.4.14
[01:15:51] [m[m[32m[1m   Compiling[m lzma-sys v0.1.9
[01:15:51] [m[m[32m[1m   Compiling[m clap v2.29.0
[01:15:51] [m[m[32m[1m   Compiling[m flate2 v1.0.1
[01:16:24] [m[m[32m[1m   Compiling[m xz2 v0.1.3
[01:17:06] [m[m[32m[1m   Compiling[m installer v0.0.0 (file:///checkout/src/tools/rust-installer)
[01:17:11] [m[m[32m[1m    Finished[m release [optimized] target(s) in 87.85 secs
[01:17:11] travis_fold:end:stage0-fabricate
[0K
[01:17:11] travis_time:end:stage0-fabricate:start=1518291862965457303,finish=1518291951467994290,duration=88502536987
[0K
[01:17:11] Error: failed to generate installer
[01:17:11] Caused by: failed to copy '/checkout/obj/build/tmp/dist/rust-docs-beta-x86_64-unknown-linux-musl-image/share/doc/rust/html/alloc/slice/struct.Chunks.html' to '/checkout/obj/build/tmp/dist/rust-docs-beta-x86_64-unknown-linux-musl/rust-docs/share/doc/rust/html/alloc/slice/struct.Chunks.html'
[01:17:11] Caused by: No space left on device (os error 28)
