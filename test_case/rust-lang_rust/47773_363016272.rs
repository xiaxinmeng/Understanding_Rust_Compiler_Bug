
[01:54:19] ---- [pretty] run-pass-fulldeps/proc-macro/derive-b.rs stdout ----
[01:54:19] 	
[01:54:19] error: pretty-printed source does not typecheck
[01:54:19] status: exit code: 101
[01:54:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "-" "-Zno-trans" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/derive-b.pretty-out" "--target=x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/proc-macro/derive-b.stage2-x86_64-unknown-linux-gnu.pretty.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers"
[01:54:19] stdout:
[01:54:19] ------------------------------------------
[01:54:19] 
[01:54:19] ------------------------------------------
[01:54:19] stderr:
[01:54:19] ------------------------------------------
[01:54:19] error: cannot find attribute macro `B` in this scope
[01:54:19]   --> <anon>:19:25
[01:54:19]    |
[01:54:19] 19 | #[cfg_attr ( all (  ) , B arbitrary tokens )]
[01:54:19]    |                         ^
[01:54:19] 
[01:54:19] error[E0422]: cannot find struct, variant or union type `B` in this scope
[01:54:19]   --> <anon>:26:5
[01:54:19]    |
[01:54:19] 26 |     B{a: 3,};
[01:54:19]    |     ^ not found in this scope
[01:54:19] 
[01:54:19] error[E0422]: cannot find struct, variant or union type `B` in this scope
[01:54:19]   --> <anon>:27:16
[01:54:19]    |
[01:54:19] 27 |     assert_eq!(B { a : 3 } , B { a : 3 });
[01:54:19]    |                ^ not found in this scope
[01:54:19] 
[01:54:19] error[E0422]: cannot find struct, variant or union type `B` in this scope
[01:54:19]   --> <anon>:27:30
[01:54:19]    |
[01:54:19] 27 |     assert_eq!(B { a : 3 } , B { a : 3 });
[01:54:19]    |                              ^ not found in this scope
[01:54:19] 
[01:54:19] error[E0422]: cannot find struct, variant or union type `B` in this scope
[01:54:19]   --> <anon>:28:13
[01:54:19]    |
[01:54:19] 28 |     let b = B{a: 3,};
[01:54:19]    |             ^ not found in this scope
[01:54:19] 
[01:54:19] error: aborting due to 5 previous errors
[01:54:19] 
[01:54:19] 
[01:54:19] ------------------------------------------
[01:54:19] 
[01:54:19] thread '[pretty] run-pass-fulldeps/proc-macro/derive-b.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2883:9
[01:54:19] note: Run with `RUST_BACKTRACE=1` for a backtrace.
