
 3:08.54    Compiling proc-macro2 v0.2.2
 3:08.54      Running `/usr/bin/rustc --crate-name proc_macro2 /builddir/build/BUILD/gecko-dev-6afe06056604b90382860b521273ea13677dc140/third_party/rust/proc-macro2/src/lib.rs --crate-type lib --emit=dep-info,link -C opt-level=2 -C codegen-units=1 -C metadata=9b6cee4c8396e152 -C extra-filename=-9b6cee4c8396e152 --out-dir /builddir/build/BUILD/gecko-dev-6afe06056604b90382860b521273ea13677dc140/objdir/toolkit/library/release/deps -C linker=/builddir/build/BUILD/gecko-dev-6afe06056604b90382860b521273ea13677dc140/build/cargo-linker -L dependency=/builddir/build/BUILD/gecko-dev-6afe06056604b90382860b521273ea13677dc140/objdir/toolkit/library/release/deps --extern unicode_xid=/builddir/build/BUILD/gecko-dev-6afe06056604b90382860b521273ea13677dc140/objdir/toolkit/library/release/deps/libunicode_xid-7cd3826576421223.rlib --cap-lints allow`
 3:08.72 error[E0523]: found two different crates with name `std` that are not distinguished by differing `-C metadata`. This will result in symbol conflicts between the two.
 3:08.72   --> /builddir/build/BUILD/gecko-dev-6afe06056604b90382860b521273ea13677dc140/third_party/rust/proc-macro2/src/lib.rs:27:1
 3:08.72    |
 3:08.72 27 | extern crate proc_macro;
 3:08.72    | ^^^^^^^^^^^^^^^^^^^^^^^^
 3:08.73 
 3:08.96 error: Could not compile `proc-macro2`.
 3:08.96 
 3:08.96 Caused by:
 3:08.97   process didn't exit successfully: `/usr/bin/rustc --crate-name proc_macro2 /builddir/build/BUILD/gecko-dev-6afe06056604b90382860b521273ea13677dc140/third_party/rust/proc-macro2/src/lib.rs --crate-type lib --emit=dep-info,link -C opt-level=2 -C codegen-units=1 -C metadata=9b6cee4c8396e152 -C extra-filename=-9b6cee4c8396e152 --out-dir /builddir/build/BUILD/gecko-dev-6afe06056604b90382860b521273ea13677dc140/objdir/toolkit/library/release/deps -C linker=/builddir/build/BUILD/gecko-dev-6afe06056604b90382860b521273ea13677dc140/build/cargo-linker -L dependency=/builddir/build/BUILD/gecko-dev-6afe06056604b90382860b521273ea13677dc140/objdir/toolkit/library/release/deps --extern unicode_xid=/builddir/build/BUILD/gecko-dev-6afe06056604b90382860b521273ea13677dc140/objdir/toolkit/library/release/deps/libunicode_xid-7cd3826576421223.rlib --cap-lints allow` (exit code: 101)