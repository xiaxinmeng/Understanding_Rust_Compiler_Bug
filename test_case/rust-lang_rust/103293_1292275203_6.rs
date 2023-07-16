
[INFO] [stderr] error: failed to run custom build command for `protobuf v2.17.0`
[INFO] [stderr] 
[INFO] [stderr] Caused by:
[INFO] [stderr]   process didn't exit successfully: `/opt/rustwide/target/debug/build/protobuf-95e2dcd7a29b0510/build-script-build` (exit status: 101)
[INFO] [stderr]   --- stderr
[INFO] [stderr]   /opt/rustwide/rustup-home/toolchains/51ad93974f526eaf4ede64932870490e6c96e4fd/bin/rustc: symbol lookup error: /opt/rustwide/rustup-home/toolchains/51ad93974f526eaf4ede64932870490e6c96e4fd/lib/librustc_driver-9b1cd69a927aa865.so: undefined symbol: _ZN9hashbrown3raw11Fallibility17capacity_overflow17hf80e9b1ba4f59154E
[INFO] [stderr]   thread 'main' panicked at 'assertion failed: child.wait().expect(\"wait\").success()', /opt/rustwide/cargo-home/registry/src/github.com-1ecc6299db9ec823/protobuf-2.17.0/build.rs:35:5
