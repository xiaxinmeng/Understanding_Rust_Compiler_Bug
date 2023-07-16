
[INFO] [stderr] error: failed to run custom build command for `catboost2-sys v0.1.1+catboost.1.0.5`
[INFO] [stderr] 
[INFO] [stderr] Caused by:
[INFO] [stderr]   process didn't exit successfully: `/opt/rustwide/target/debug/build/catboost2-sys-58e9375bdc591b38/build-script-build` (exit status: 101)
[INFO] [stderr]   --- stderr
[INFO] [stderr]   error: 'rustfmt' is not installed for the toolchain '51ad93974f526eaf4ede64932870490e6c96e4fd'
[INFO] [stderr]   note: this is a custom toolchain, which cannot use `rustup component add`
[INFO] [stderr]   help: if you built this toolchain from source, and used `rustup toolchain link`, then you may be able to build the component with `x.py`
[INFO] [stderr]   Failed to run rustfmt: Internal rustfmt error (non-fatal, continuing)
[INFO] [stderr]   thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 17, kind: AlreadyExists, message: "File exists" }', /opt/rustwide/cargo-home/registry/src/github.com-1ecc6299db9ec823/catboost2-sys-0.1.1+catboost.1.0.5/build.rs:21:33
