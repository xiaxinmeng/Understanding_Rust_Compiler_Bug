
Copying stage1 std from stage1 (i586-alpine-linux-musl -> i586-alpine-linux-musl / i586-alpine-linux-musl)
Building stage1 compiler artifacts (i586-alpine-linux-musl -> powerpc-alpine-linux-musl)
   Compiling proc-macro2 v1.0.30
   Compiling libc v0.2.116
   ...
   Compiling rustc_macros v0.1.0 (/home/tux/aports/community/rust/src/rustc-1.61.0-src/compiler/rustc_macros)
   Compiling chalk-derive v0.80.0
   Compiling tracing v0.1.29
error: Error relocating /home/tux/aports/community/rust/src/rustc-1.61.0-src/build/i586-alpine-linux-musl/stage1-rustc/release/deps/libtracing_attributes-77686163bd4c6e7a.so: __mulodi4: symbol not found
   --> /home/tux/aports/community/rust/src/rustc-1.61.0-src/vendor/tracing-0.1.29/src/lib.rs:913:9
    |
913 | pub use tracing_attributes::instrument;
    |         ^^^^^^^^^^^^^^^^^^

   Compiling chalk-ir v0.80.0
error: could not compile `tracing` due to previous error
warning: build failed, waiting for other jobs to finish...
error: Error relocating /home/tux/aports/community/rust/src/rustc-1.61.0-src/build/i586-alpine-linux-musl/stage1-rustc/release/deps/libchalk_derive-1dc68cf7ceee1d83.so: __mulodi4: symbol not found
  --> /home/tux/aports/community/rust/src/rustc-1.61.0-src/vendor/chalk-ir-0.80.0/src/lib.rs:13:5
   |
13 | use chalk_derive::{Fold, HasInterner, SuperVisit, Visit, Zip};
   |     ^^^^^^^^^^^^
