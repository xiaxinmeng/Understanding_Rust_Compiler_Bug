
[01:24:43] Dist std stage2 (x86_64-unknown-linux-gnu -> x86_64-unknown-netbsd)
[01:26:25] Dist analysis
[01:26:25] image_src: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-netbsd/release/deps/save-analysis", dst: "/checkout/obj/build/tmp/dist/rust-analysis-nightly-x86_64-unknown-netbsd-image/lib/rustlib/x86_64-unknown-netbsd/analysis"
[01:26:26] Dist src
[01:26:35] Dist extended stage2 (x86_64-unknown-netbsd)
[01:26:35] Dist cargo stage2 (x86_64-unknown-netbsd)
[01:26:35]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[01:26:35]                                  Dload  Upload   Total   Spent    Left  Speed
[01:26:35] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   282    0   282    0     0    847      0 --:--:-- --:--:-- --:--:--   849
[01:26:35] thread 'main' panicked at 'downloaded openssl sha256 different
[01:26:35] expected: 6b3977c61f2aedf0f96367dcfb5c6e578cf37e7b8d913b4ecb6643c3cb88d8c0
[01:26:35] found:    8552e3169b5a6071edfab25bf7dacb3bbd40ae5325bd472690047b90c1cd0589
[01:26:35] ', /checkout/src/bootstrap/native.rs:380:16
[01:26:35] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:26:35] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-netbsd --target x86_64-unknown-netbsd
[01:26:35] Build completed unsuccessfully in 1:24:19
