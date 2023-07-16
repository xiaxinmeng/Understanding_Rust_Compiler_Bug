
$ RUSTBUILD_NATIVE_DIR=/scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/native \
RUSTC=/scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/bootstrap/debug/rustc \
RUSTC_REAL=/scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/stage2/bin/rustc \
RUSTC_STAGE=2 RUSTC_DEBUG_ASSERTIONS=false \
RUSTC_SYSROOT=/scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/stage2 \
RUSTC_LIBDIR=/scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/stage2/lib \
RUSTC_RPATH=true \
RUSTDOC=/scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/bootstrap/debug/rustdoc \
RUSTDOC_REAL=/path/to/nowhere/rustdoc/not/required \
RUSTC_BOOTSTRAP=1 RUST_TEST_THREADS=4 \
RUSTC_SNAPSHOT=/scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/stage2/bin/rustc \
RUSTC_SNAPSHOT_LIBDIR=/scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/stage2/lib \
RUSTC_VERBOSE=0 \
RUSTDOC_CRATE_VERSION=1.23.0-dev \
CFG_COMPILER_HOST_TRIPLE=sparcv9-sun-solaris \
CFG_RELEASE_CHANNEL=dev RUSTC_THINLTO=1 \
RUSTC_NO_PREFER_DYNAMIC=1 \
CFG_VERSION=1.23.0-dev RUSTC_DEBUGINFO=false \
RUSTC_DEBUGINFO_LINES=false CARGO_PKG_VERSION_PRE= \
/scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/bootstrap/debug/rustc --crate-name bitflags /scratch/userland-rust-s11.3/components/rust/rustc/rustc-1.23.0-src-vendored-sources/bitflags-0.9.1/src/lib.rs --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=e6ef588569977e18 -C extra-filename=-e6ef588569977e18 --out-dir /scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/stage2-tools/sparcv9-sun-solaris/release/deps --target sparcv9-sun-solaris -L  dependency=/scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/stage2-tools/sparcv9-sun-solaris/release/deps  -L  dependency=/scratch/userland-rust-s11.3/components/rust/rustc/build/sparcv9/build/sparcv9-sun-solaris/stage2-tools/release/deps --cap-lints allow
Segmentation Fault (core dumped)
