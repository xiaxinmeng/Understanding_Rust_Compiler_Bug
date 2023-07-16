plain
[00:02:41]  Downloading log_settings v0.1.1
[00:02:41]  Downloading num-integer v0.1.36
[00:02:42]  Downloading backtrace v0.3.6
[00:02:42]  Downloading tempdir v0.3.7
[00:02:42]  Downloading chalk-engine v0.6.0
[00:02:42]  Downloading flate2 v1.0.1
[00:02:42]  Downloading mdbook v0.1.7
[00:02:42]  Downloading clap v2.31.2
[00:02:42]  Downloading failure v0.1.1
---
[00:14:11] [RUSTC-TIMING] atty test:false 0.211
[00:14:11]    Compiling log_settings v0.1.1
[00:14:12] [RUSTC-TIMING] log_settings test:false 0.419
[00:14:12]    Compiling backtrace v0.3.6
[00:14:13] [RUSTC-TIMING] chalk_macros test:false 1.280
[00:14:14] [RUSTC-TIMING] backtrace test:false 2.217
[00:14:15]    Compiling humantime v1.1.1
[00:14:15]    Compiling ena v0.9.3
[00:14:16] [RUSTC-TIMING] ena test:false 0.912
---
[00:14:20]    Compiling crossbeam-epoch v0.3.1
[00:14:20]    Compiling rls-span v0.4.0
[00:14:21] [RUSTC-TIMING] rustc_platform_intrinsics test:false 15.619
[00:14:21] [RUSTC-TIMING] rls_span test:false 1.526
[00:14:22]    Compiling chalk-engine v0.6.0
[00:14:22]    Compiling parking_lot_core v0.2.14
[00:14:22] [RUSTC-TIMING] jobserver test:false 5.670
[00:14:22]    Compiling tempdir v0.3.7
[00:14:23] [RUSTC-TIMING] tempdir test:false 1.288
[00:14:23] [RUSTC-TIMING] tempdir test:false 1.288
[00:14:24]    Compiling rustc_apfloat v0.0.0 (file:///Users/travis/build/rust-lang/rust/src/librustc_apfloat)
[00:14:25] [RUSTC-TIMING] parking_lot_core test:false 2.954
[00:14:25]    Compiling env_logger v0.5.8
[00:14:28] [RUSTC-TIMING] rustc_apfloat test:false 4.224
[00:14:28]    Compiling rls-data v0.16.0
[00:14:29] [RUSTC-TIMING] chalk_engine test:false 7.068
[00:14:29] [RUSTC-TIMING] env_logger test:false 4.427
[00:14:30] [RUSTC-TIMING] miniz_sys test:false 0.174
[00:14:30] [RUSTC-TIMING] crossbeam_deque test:false 0.493
[00:14:30]    Compiling parking_lot v0.5.5
---
[00:34:17]    Compiling num_cpus v1.8.0
[00:34:17] [RUSTC-TIMING] num_cpus test:false 0.354
[00:34:17]    Compiling atty v0.2.8
[00:34:18] [RUSTC-TIMING] atty test:false 0.248
[00:34:18]    Compiling chalk-macros v0.1.0
[00:34:19] [RUSTC-TIMING] chalk_macros test:false 1.418
[00:34:20] [RUSTC-TIMING] log_settings test:false 0.490
[00:34:20]    Compiling backtrace v0.3.6
[00:34:22] [RUSTC-TIMING] rand test:false 5.394
[00:34:22] [RUSTC-TIMING] backtrace test:false 2.196
---
[00:34:25] [RUSTC-TIMING] ena test:false 0.744
[00:34:26]    Compiling jobserver v0.1.11
[00:34:26]    Compiling crossbeam-epoch v0.3.1
[00:34:27] [RUSTC-TIMING] rustc_serialize test:false 15.456
[00:34:28]    Compiling chalk-engine v0.6.0
[00:34:28]    Compiling parking_lot_core v0.2.14
[00:34:28] [RUSTC-TIMING] rustc_platform_intrinsics test:false 16.370
[00:34:28]    Compiling tempdir v0.3.7
[00:34:29] [RUSTC-TIMING] tempdir test:false 0.994
[00:34:29] [RUSTC-TIMING] tempdir test:false 0.994
[00:34:30] [RUSTC-TIMING] jobserver test:false 3.993
[00:34:30]    Compiling env_logger v0.5.8
[00:34:31] [RUSTC-TIMING] parking_lot_core test:false 2.323
[00:34:31]    Compiling rustc_apfloat v0.0.0 (file:///Users/travis/build/rust-lang/rust/src/librustc_apfloat)
[00:34:34] [RUSTC-TIMING] chalk_engine test:false 5.778
[00:34:34]    Compiling rls-span v0.4.0
[00:34:34]    Compiling crossbeam-deque v0.2.0
[00:34:34] [RUSTC-TIMING] rustc_apfloat test:false 3.465
[00:34:34] [RUSTC-TIMING] miniz_sys test:false 0.162
---
$ travis_retry gem update --system
ERROR:  While executing gem ... (Gem::RemoteFetcher::UnknownHostError)
    no such name (https://api.rubygems.org/specs.4.8.gz)
The command "gem update --system" failed. Retrying, 2 of 3.
ERROR:  While executing gem ... (Gem::RemoteFetcher::FetchError)
    Errno::EHOSTUNREACH: Failed to open TCP connection to rubygems.org:443 (No route to host - connect(2) for "rubygems.org" port 443) (https://rubygems.org/specs.4.8.gz)
ERROR:  While executing gem ... (Gem::RemoteFetcher::UnknownHostError)
ERROR:  While executing gem ... (Gem::RemoteFetcher::UnknownHostError)
    no such name (https://api.rubygems.org/quick/Marshal.4.8/rubygems-update-2.7.7.gemspec.rz)
The command "gem update --system" failed 3 times.
travis_time:end:1a33c62c:start=1527194927078275000,finish=1527195208260579000,duration=281182304000


The command "travis_retry gem update --system" failed and exited with 1 during .
Your build has been stopped.
