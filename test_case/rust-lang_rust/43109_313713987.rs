
---- [compile-fail] compile-fail/gated-link-args.rs stdout ----

[00:39:48] 	

[00:39:48] error: /checkout/src/test/compile-fail/gated-link-args.rs:28: unexpected "error": '28:1: 28:14: the name `main` is defined multiple times [E0428]'

[00:39:48] 

[00:39:48] error: 1 unexpected errors found, 0 expected errors not found

[00:39:48] status: exit code: 101

[00:39:48] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/compile-fail/gated-link-args.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/gated-link-args.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/gated-link-args.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers

[00:39:48] unexpected errors (from JSON output): [

[00:39:48]     Error {

[00:39:48]         line_num: 28,

[00:39:48]         kind: Some(

[00:39:48]             Error

[00:39:48]         ),

[00:39:48]         msg: "28:1: 28:14: the name `main` is defined multiple times [E0428]"

[00:39:48]     }

[00:39:48] ]
