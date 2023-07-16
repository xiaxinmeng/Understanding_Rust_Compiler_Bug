
INFO:rustc::traits::fulfill: selecting trait `Binder(TraitPredicate(<u64 as convert::From<isize>>))` at depth 0 yielded Err
INFO:rustc::traits::trans: Cache miss: Binder(<T as num::dec2flt::rawfp::RawFloat>) => VtableParam([])
INFO:rustc::traits::trans: Cache miss: Binder(<f32 as num::dec2flt::rawfp::RawFloat>) => VtableImpl(impl_def_id=DefId { krate: CrateNum(0), index: DefIndex(0:388) => core[9097]::num[0]::dec2flt[0]::rawfp[0]::{{impl}}[1] }, substs=Slice([]), nested=[])
INFO:rustc::traits::trans: Cache miss: Binder(<f64 as num::dec2flt::rawfp::RawFloat>) => VtableImpl(impl_def_id=DefId { krate: CrateNum(0), index: DefIndex(0:401) => core[9097]::num[0]::dec2flt[0]::rawfp[0]::{{impl}}[2] }, substs=Slice([]), nested=[])
INFO:cargo::ops::cargo_rustc::job_queue: end: core v0.0.0 (file:///tmp/portage/dev-lang/rust-9999-r1/work/rust-git-src/src/libcore) => Target(lib)/Profile(build) => Target
error: Could not compile `core`.

Caused by:
  failed to parse process output: `/tmp/portage/dev-lang/rust-9999-r1/work/rust-git-src/build/bootstrap/debug/rustc --crate-name core src/libcore/lib.rs --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=63e74be82340645b -C extra-filename=-63e74be82340645b --out-dir /tmp/portage/dev-lang/rust-9999-r1/work/rust-git-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/tmp/portage/dev-lang/rust-9999-r1/work/rust-git-src/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/tmp/portage/dev-lang/rust-9999-r1/work/rust-git-src/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 0)

Caused by:
  compiler stdout is not empty: `Pre-trans`
thread 'main' panicked at 'command did not execute successfully: "/tmp/portage/dev-lang/rust-9999-r1/work/rust-git-src/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "7" "-v" "--release" "--locked" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/tmp/portage/dev-lang/rust-9999-r1/work/rust-git-src/src/libstd/Cargo.toml" "--message-format" "json"
expected success, got: exit code: 101', src/bootstrap/compile.rs:882:8
