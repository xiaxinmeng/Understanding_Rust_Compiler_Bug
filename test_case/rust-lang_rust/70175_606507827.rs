
--- [mir-opt] mir-opt/generator-tiny.rs stdout ----

error: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/generator-tiny.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zdump-mir=all" "-Zmir-opt-level=3" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-tiny" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-tiny/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/generator-tiny/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: the linked panic runtime `panic_unwind` is not compiled with this crate's panic strategy `abort`

error: aborting due to previous error
