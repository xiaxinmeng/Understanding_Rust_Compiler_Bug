
[00:50:03] ---- [run-pass] run-pass/rfc-2126-extern-absolute-paths/test.rs stdout ----
[00:50:03] 	
[00:50:03] error: compilation failed!
[00:50:03] status: exit code: 101
[00:50:03] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/rfc-2126-extern-absolute-paths/test.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rfc-2126-extern-absolute-paths/test.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/rfc-2126-extern-absolute-paths/test.stage2-x86_64-unknown-linux-gnu.aux"
[00:50:03] stdout:
[00:50:03] ------------------------------------------
[00:50:03] 
[00:50:03] ------------------------------------------
[00:50:03] stderr:
[00:50:03] ------------------------------------------
[00:50:03] error[E0658]: `crate` in paths is experimental (see issue #45477)
[00:50:03]   --> /checkout/src/test/run-pass/rfc-2126-extern-absolute-paths/test.rs:20:1
[00:50:03]    |
[00:50:03] 20 | / fn test() {
[00:50:03] 21 | | }
[00:50:03]    | |_^
[00:50:03]    |
[00:50:03]    = help: add #![feature(crate_in_paths)] to the crate attributes to enable
