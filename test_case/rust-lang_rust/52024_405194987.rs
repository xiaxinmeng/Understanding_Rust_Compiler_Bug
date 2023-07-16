plain
[00:45:32] ....................................................................................................
[00:45:42] ....................................................................................................
[00:45:51] ....................................................................................................
[00:46:05] ....................................................................................................
[00:46:13] ........F...........................................................................................
[00:46:36] ....................................................................................................
[00:46:45] ....................................................................................................
[00:46:55] ....................................................................................................
[00:47:03] ....................................................................................................
---
[00:50:37] ---- [run-pass] run-pass/existential_type.rs stdout ----
[00:50:37] 
[00:50:37] error: compilation failed!
[00:50:37] status: exit code: 101
[00:50:37] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass/existential_type.rs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/existential_type/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/existential_type/auxiliary"
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] ------------------------------------------
[00:50:37] stderr:
[00:50:37] stderr:
[00:50:37] ------------------------------------------
[00:50:37] error[E0091]: type parameter `T` is unused
[00:50:37]   --> /checkout/src/test/run-pass/existential_type.rs:78:31
[00:50:37]    |
[00:50:37] 78 | existential type GenericBound<T: Trait>: 'static;
[00:50:37]    |                               ^ unused type parameter
[00:50:37] 
[00:50:37] error[E0091]: type parameter `T` is unused
[00:50:37]   --> /checkout/src/test/run-pass/existential_type.rs:96:35
[00:50:37]    |
[00:50:37] 96 | existential type PartiallyDefined<T>: 'static;
[00:50:37]    |                                   ^ unused type parameter
[00:50:37] 
[00:50:37] error[E0091]: type parameter `T` is unused
[00:50:37]    --> /checkout/src/test/run-pass/existential_type.rs:104:36
[00:50:37]     |
[00:50:37] 104 | existential type PartiallyDefined2<T>: 'static;
[00:50:37]     |                                    ^ unused type parameter
[00:50:37] error: aborting due to 3 previous errors
[00:50:37] 
[00:50:37] For more information about this error, try `rustc --explain E0091`.
[00:50:37] 
[00:50:37] 
[00:50:37] ------------------------------------------
[00:50:37] 
[00:50:37] thread '[run-pass] run-pass/existential_type.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3162:9
[00:50:37] 
[00:50:37] 
[00:50:37] failures:
[00:50:37]     [run-pass] run-pass/existential_type.rs
