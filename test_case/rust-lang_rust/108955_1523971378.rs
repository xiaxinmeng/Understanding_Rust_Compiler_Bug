plain
tests/pass/concurrency/sync.rs (tree) ... ok
tests/pass/0weak_memory_consistency.rs ... ok
tests/pass/shims/time-with-isolation2.rs ... ok

tests/pass/dyn-arbitrary-self.rs (revision `stack`) FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "tests/pass/dyn-arbitrary-self.rs" "--cfg=stack"
actual output differed from expected
--- tests/pass/dyn-arbitrary-self.stack.stderr
+++ <stderr output>
+error: the feature `rustc_attrs` is internal to the compiler or standard library
+error: the feature `rustc_attrs` is internal to the compiler or standard library
+  --> $DIR/dyn-arbitrary-self.rs:LL:CC
+   |
+LL | #![feature(rustc_attrs)]
+   |            ^^^^^^^^^^^
+   |
+   = note: using it is strongly discouraged
+   = note: `#[deny(internal_features)]` on by default
+


There were 1 unmatched diagnostics at tests/pass/dyn-arbitrary-self.rs:4
---
   |
LL | #![feature(rustc_attrs)]
   |            ^^^^^^^^^^^
   |
   = note: using it is strongly discouraged
   = note: `#[deny(internal_features)]` on by default




tests/pass/dyn-arbitrary-self.rs (revision `tree`) FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "tests/pass/dyn-arbitrary-self.rs" "--cfg=tree" "-Zmiri-tree-borrows"
actual output differed from expected
--- tests/pass/dyn-arbitrary-self.tree.stderr
+++ <stderr output>
+error: the feature `rustc_attrs` is internal to the compiler or standard library
+error: the feature `rustc_attrs` is internal to the compiler or standard library
+  --> $DIR/dyn-arbitrary-self.rs:LL:CC
+   |
+LL | #![feature(rustc_attrs)]
+   |            ^^^^^^^^^^^
+   |
+   = note: using it is strongly discouraged
+   = note: `#[deny(internal_features)]` on by default
+


There were 1 unmatched diagnostics at tests/pass/dyn-arbitrary-self.rs:4
---
   |
LL | #![feature(rustc_attrs)]
   |            ^^^^^^^^^^^
   |
   = note: using it is strongly discouraged
   = note: `#[deny(internal_features)]` on by default



tests/pass/miri-alloc.rs FAILED:
tests/pass/miri-alloc.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "tests/pass/miri-alloc.rs"

actual output differed from expected
--- tests/pass/miri-alloc.stderr
+++ <stderr output>
+error: the feature `lang_items` is internal to the compiler or standard library
+   |
+LL | #![feature(lang_items, start)]
+   |            ^^^^^^^^^^
+   |
+   |
+   = note: using it is strongly discouraged
+   = note: `#[deny(internal_features)]` on by default
+


There were 1 unmatched diagnostics at tests/pass/miri-alloc.rs:1
There were 1 unmatched diagnostics at tests/pass/miri-alloc.rs:1
    Error: the feature `lang_items` is internal to the compiler or standard library
full stderr:
full stderr:
error: the feature `lang_items` is internal to the compiler or standard library
   |
LL | #![feature(lang_items, start)]
   |            ^^^^^^^^^^
   |
   |
   = note: using it is strongly discouraged
   = note: `#[deny(internal_features)]` on by default



tests/pass/no_std.rs FAILED:
tests/pass/no_std.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "tests/pass/no_std.rs"

actual output differed from expected
--- tests/pass/no_std.stderr
+++ <stderr output>
+error: the feature `lang_items` is internal to the compiler or standard library
+   |
+LL | #![feature(lang_items, start)]
+   |            ^^^^^^^^^^
+   |
+   |
+   = note: using it is strongly discouraged
+   = note: `#[deny(internal_features)]` on by default
+


There were 1 unmatched diagnostics at tests/pass/no_std.rs:1
There were 1 unmatched diagnostics at tests/pass/no_std.rs:1
    Error: the feature `lang_items` is internal to the compiler or standard library
full stderr:
full stderr:
error: the feature `lang_items` is internal to the compiler or standard library
   |
LL | #![feature(lang_items, start)]
   |            ^^^^^^^^^^
   |
   |
   = note: using it is strongly discouraged
   = note: `#[deny(internal_features)]` on by default



tests/pass/overloaded-calls-simple.rs FAILED:
tests/pass/overloaded-calls-simple.rs FAILED:
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/miri" "--error-format=json" "--edition" "2018" "-Astable-features" "-Aunused" "-Zui-testing" "--target" "x86_64-unknown-linux-gnu" "tests/pass/overloaded-calls-simple.rs"

actual output differed from expected
--- tests/pass/overloaded-calls-simple.stderr
+++ <stderr output>
+error: the feature `lang_items` is internal to the compiler or standard library
+   |
+   |
+LL | #![feature(lang_items, unboxed_closures, fn_traits)]
+   |
+   |
+   = note: using it is strongly discouraged
+   = note: `#[deny(internal_features)]` on by default
+


There were 1 unmatched diagnostics at tests/pass/overloaded-calls-simple.rs:1
There were 1 unmatched diagnostics at tests/pass/overloaded-calls-simple.rs:1
    Error: the feature `lang_items` is internal to the compiler or standard library
full stderr:
full stderr:
error: the feature `lang_items` is internal to the compiler or standard library
   |
   |
LL | #![feature(lang_items, unboxed_closures, fn_traits)]
   |
   |
   = note: using it is strongly discouraged
   = note: `#[deny(internal_features)]` on by default



tests/pass/function_calls/exported_symbol.rs FAILED:
---
+   |
+LL | #![feature(rustc_attrs)]
+   |            ^^^^^^^^^^^
+   |
+   = note: using it is strongly discouraged
+   = note: `#[deny(internal_features)]` on by default
+


There were 1 unmatched diagnostics at tests/pass/function_calls/exported_symbol.rs:1
---
   |
LL | #![feature(rustc_attrs)]
   |            ^^^^^^^^^^^
   |
   = note: using it is strongly discouraged
   = note: `#[deny(internal_features)]` on by default


FAILURES:
    tests/pass/dyn-arbitrary-self.rs
