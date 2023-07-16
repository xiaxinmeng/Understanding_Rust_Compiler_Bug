plain

---- [ui] src/test/ui/panic-runtime/need-unwind-got-abort.rs stdout ----
diff of stderr:

- error: the linked panic runtime `panic_unwind` is not compiled with this crate's panic strategy `abort`
- 
3 error: the crate `needs_unwind` requires panic strategy `unwind` which is incompatible with this crate's strategy of `abort`
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
6 
7 
---
To only update this specific test, also pass `--test-args panic-runtime/need-unwind-got-abort.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panic-runtime/need-unwind-got-abort.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/need-unwind-got-abort" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panic-runtime/need-unwind-got-abort/auxiliary"
stdout: none
--- stderr -------------------------------
error: the crate `needs_unwind` requires panic strategy `unwind` which is incompatible with this crate's strategy of `abort`
error: aborting due to previous error
------------------------------------------


