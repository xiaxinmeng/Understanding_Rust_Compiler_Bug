
../../third_party/android_rust_toolchain/toolchain/bin/rustc \
  repro.rs  -o librepro.rlib \
   --edition=2018 -Copt-level=3 \
  --crate-type rlib

../../third_party/android_rust_toolchain/toolchain/bin/rustc \
  repro.rs -o librepro.a \
  --edition=2018 -Copt-level=3 \
  --crate-type staticlib

../../third_party/android_rust_toolchain/toolchain/bin/rustc \
  repro.rs -o librepro.o \
  --edition=2018 -Copt-level=3 \
  --crate-type rlib  --emit obj
