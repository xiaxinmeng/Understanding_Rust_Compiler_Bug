
RUSTC="/home/rprichard/work/rust-android-local/bin/rustc \
    --target=arm-linux-androideabi \
    -C linker=/home/rprichard/android-ndk-standalone-19/bin/arm-linux-androideabi-gcc"

$RUSTC ~/work/rust/src/test/auxiliary/sepcomp_lib.rs \
    -C codegen-units=3 --crate-type=rlib,dylib

$RUSTC ~/work/rust/src/test/run-pass/sepcomp-lib-lto.rs -C opt-level=2 -C lto -L.
