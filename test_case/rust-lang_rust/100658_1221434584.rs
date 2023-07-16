plain
To only update this specific test, also pass `--test-args issues/issue-100631.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-100631.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-100631" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-100631/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0084]: unsupported representation for zero-variant enum
   |
   |
LL | #[repr(C)] //~ ERROR: unsupported representation for zero-variant enum [E0084]
LL | #[repr(C)]
LL | enum Foo {}
LL | enum Foo {}
   | -------- zero-variant enum
error: aborting due to previous error

For more information about this error, try `rustc --explain E0084`.
------------------------------------------
