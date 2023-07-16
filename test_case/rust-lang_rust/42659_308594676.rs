
[00:41:12] error: /checkout/src/test/compile-fail/coherence-impl-trait-for-trait-object-safe.rs:17: unexpected "error": '17:1: 17:37: the trait `NotObjectSafe` cannot be made into an object [E0038]'
[00:41:12] 
[00:41:12] error: 1 unexpected errors found, 0 expected errors not found
[00:41:12] status: exit code: 101
[00:41:12] command: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc /checkout/src/test/compile-fail/coherence-impl-trait-for-trait-object-safe.rs -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu --error-format json -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/coherence-impl-trait-for-trait-object-safe.stage2-x86_64-unknown-linux-gnu.compile-fail.libaux -C prefer-dynamic -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/coherence-impl-trait-for-trait-object-safe.stage2-x86_64-unknown-linux-gnu -Crpath -O -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers
[00:41:12] unexpected errors (from JSON output): [
[00:41:12]     Error {
[00:41:12]         line_num: 17,
[00:41:12]         kind: Some(
[00:41:12]             Error
[00:41:12]         ),
[00:41:12]         msg: "17:1: 17:37: the trait `NotObjectSafe` cannot be made into an object [E0038]"
[00:41:12]     }
[00:41:12] ]
