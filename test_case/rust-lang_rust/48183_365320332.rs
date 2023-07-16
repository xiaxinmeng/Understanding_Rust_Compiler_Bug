
[01:13:21] ---- [run-pass] run-pass-fulldeps/env.rs stdout ----
[01:13:21] 	
[01:13:21] error: compilation failed!
[01:13:21] status: exit code: 101
[01:13:21] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/env.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/env.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/env.stage2-x86_64-unknown-linux-gnu.aux"
[01:13:21] stdout:
[01:13:21] ------------------------------------------
[01:13:21] 
[01:13:21] ------------------------------------------
[01:13:21] stderr:
[01:13:21] ------------------------------------------
[01:13:21] error[E0464]: multiple matching crates for `rand`
[01:13:21]   --> /checkout/src/test/run-pass-fulldeps/env.rs:15:1
[01:13:21]    |
[01:13:21] 15 | extern crate rand;
[01:13:21]    | ^^^^^^^^^^^^^^^^^^
[01:13:21]    |
[01:13:21]    = note: candidates:
[01:13:21]            crate `rand`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-f666896b2039530f.rlib
[01:13:21]            crate `rand`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/librand-0df21e9bf78b9472.rlib
[01:13:21] 
[01:13:21] error[E0463]: can't find crate for `rand`
[01:13:21]   --> /checkout/src/test/run-pass-fulldeps/env.rs:15:1
[01:13:21]    |
[01:13:21] 15 | extern crate rand;
[01:13:21]    | ^^^^^^^^^^^^^^^^^^ can't find crate
[01:13:21] 
[01:13:21] error: aborting due to 2 previous errors

...

[01:13:21] failures:
[01:13:21]     [run-pass] run-pass-fulldeps/binary-heap-panic-safe.rs
[01:13:21]     [run-pass] run-pass-fulldeps/env.rs
[01:13:21]     [run-pass] run-pass-fulldeps/flt2dec.rs
[01:13:21]     [run-pass] run-pass-fulldeps/sort-unstable.rs
[01:13:21]     [run-pass] run-pass-fulldeps/vector-sort-panic-safe.rs
