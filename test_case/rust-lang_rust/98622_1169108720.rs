plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs  foo.rs --target=my-awesome-platform.json --crate-type=lib --emit=asm
"/checkout/src/etc/cat-and-grep.sh" -v morestack < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs/foo.s
[[[ begin stdout ]]]
 .text
 .file "foo.91a024fd-cgu.0"
 .section ".note.GNU-stack","",@progbits

[[[ end stdout ]]]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs  foo.rs --target=my-invalid-platform.json 2>&1 | "/checkout/src/etc/cat-and-grep.sh" "Error loading target specification"
[[[ begin stdout ]]]
error: Error loading target specification: expected value at line 1 column 1. Run `rustc --print target-list` for a list of built-in targets


[[[ end stdout ]]]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs  foo.rs --target=my-incomplete-platform.json 2>&1 | "/checkout/src/etc/cat-and-grep.sh" 'Field llvm-target'
[[[ begin stdout ]]]
error: Error loading target specification: Field llvm-target in target specification is required. Run `rustc --print target-list` for a list of built-in targets


[[[ end stdout ]]]
RUST_TARGET_PATH=. LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs  foo.rs --target=my-awesome-platform --crate-type=lib --emit=asm
RUST_TARGET_PATH=. LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs  foo.rs --target=my-x86_64-unknown-linux-gnu-platform --crate-type=lib --emit=asm
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs  -Z unstable-options --target=my-awesome-platform.json --print target-spec-json > /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs/test-platform.json && LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs  -Z unstable-options --target=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs/test-platform.json --print target-spec-json | diff -q /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs/test-platform.json -
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs  foo.rs --target=definitely-not-builtin-target 2>&1 | "/checkout/src/etc/cat-and-grep.sh" 'may not set is_builtin'
[[[ begin stdout ]]]
error: Error loading target specification: may not set is_builtin for targets not built-in. Run `rustc --print target-list` for a list of built-in targets


[[[ end stdout ]]]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/target-specs/target-specs  foo.rs --target=mismatching-data-layout
--- stderr -------------------------------
--- stderr -------------------------------
warning: target json file contains unused fields: morestack

warning: `extern` block uses type `[u8; 16]`, which is not FFI-safe
   |
   |
19 |     fn _foo() -> [u8; 16];
   |                  ^^^^^^^^ not FFI-safe
   = note: `#[warn(improper_ctypes)]` on by default
   = help: consider passing a pointer to the array
   = help: consider passing a pointer to the array
   = note: passing raw arrays by value is not FFI-safe
warning: 1 warning emitted


warning: target json file contains unused fields: morestack

warning: `extern` block uses type `[u8; 16]`, which is not FFI-safe
   |
   |
19 |     fn _foo() -> [u8; 16];
   |                  ^^^^^^^^ not FFI-safe
   = note: `#[warn(improper_ctypes)]` on by default
   = help: consider passing a pointer to the array
   = help: consider passing a pointer to the array
   = note: passing raw arrays by value is not FFI-safe
warning: 1 warning emitted


warning: target json file contains unused fields: morestack

warning: `extern` block uses type `[u8; 16]`, which is not FFI-safe
   |
   |
19 |     fn _foo() -> [u8; 16];
   |                  ^^^^^^^^ not FFI-safe
   = note: `#[warn(improper_ctypes)]` on by default
   = help: consider passing a pointer to the array
   = help: consider passing a pointer to the array
   = note: passing raw arrays by value is not FFI-safe
warning: 1 warning emitted


warning: target json file contains unused fields: morestack
error[E0601]: `main` function not found in crate `foo`
  --> foo.rs:24:2
   |
24 | }
24 | }
   |  ^ consider adding a `main` function to `foo.rs`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0601`.
For more information about this error, try `rustc --explain E0601`.
make: *** [Makefile:11: all] Error 1



failures:
