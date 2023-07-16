plain
failures:

---- [incremental] src/test/incremental/hashes/extern_mods.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/extern_mods.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods/extern_mods.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: functions with the "rust-call" ABI must take a single non-self tuple argument
   |
   |
LL |     pub fn change_calling_convention(c: i32);
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Tuple` is not implemented for `i32`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
