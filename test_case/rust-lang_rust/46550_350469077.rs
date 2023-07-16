
...
[01:37:23] ---- [pretty] run-pass/uninit-empty-types.rs stdout ----
[01:37:23] 	
[01:37:23] error: pretty-printed source (expanded) does not typecheck
[01:37:23] status: exit code: 101
[01:37:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "-" "-Zno-trans" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/uninit-empty-types.pretty-out" "--target=x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/uninit-empty-types.stage2-x86_64-unknown-linux-gnu.pretty.aux" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers"
[01:37:23] stdout:
[01:37:23] ------------------------------------------
[01:37:23] 
[01:37:23] ------------------------------------------
[01:37:23] stderr:
[01:37:23] ------------------------------------------
[01:37:23] error[E0433]: failed to resolve. Did you mean `core::clone`?
[01:37:23]   --> <anon>:26:8
[01:37:23]    |
[01:37:23] 26 | impl ::clone::Clone for Foo {
[01:37:23]    |        ^^^^^ Did you mean `core::clone`?
[01:37:23] 
[01:37:23] warning: unused import: `std::prelude::v1::*`
[01:37:23]  --> <anon>:4:5
[01:37:23]   |
[01:37:23] 4 | use std::prelude::v1::*;
[01:37:23]   |     ^^^^^^^^^^^^^^^^^^^
[01:37:23]   |
[01:37:23]   = note: #[warn(unused_imports)] on by default
[01:37:23] 
[01:37:23] warning: unused `#[macro_use]` import
[01:37:23]  --> <anon>:5:1
[01:37:23]   |
[01:37:23] 5 | #[macro_use]
[01:37:23]   | ^^^^^^^^^^^^
[01:37:23] 
[01:37:23] error: cannot continue compilation due to previous error
[01:37:23] 
[01:37:23] 
[01:37:23] ------------------------------------------
[01:37:23] 
[01:37:23] thread '[pretty] run-pass/uninit-empty-types.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2770:8
[01:37:23] 
[01:37:23] 
[01:37:23] failures:
[01:37:23]     [pretty] run-pass/associated-types-binding-in-where-clause.rs
[01:37:23]     [pretty] run-pass/deriving-clone-enum.rs
[01:37:23]     [pretty] run-pass/deriving-clone-generic-enum.rs
[01:37:23]     [pretty] run-pass/deriving-clone-generic-struct.rs
[01:37:23]     [pretty] run-pass/deriving-clone-generic-tuple-struct.rs
[01:37:23]     [pretty] run-pass/deriving-clone-struct.rs
[01:37:23]     [pretty] run-pass/deriving-clone-tuple-struct.rs
[01:37:23]     [pretty] run-pass/deriving-enum-single-variant.rs
[01:37:23]     [pretty] run-pass/deriving-in-macro.rs
[01:37:23]     [pretty] run-pass/deriving-meta-multiple.rs
[01:37:23]     [pretty] run-pass/deriving-meta.rs
[01:37:23]     [pretty] run-pass/deriving-via-extension-hash-struct.rs
[01:37:23]     [pretty] run-pass/issue-14399.rs
[01:37:23]     [pretty] run-pass/issue-15689-2.rs
[01:37:23]     [pretty] run-pass/issue-19037.rs
[01:37:23]     [pretty] run-pass/issue-21402.rs
[01:37:23]     [pretty] run-pass/issue-6341.rs
[01:37:23]     [pretty] run-pass/uninit-empty-types.rs
[01:37:23] 
[01:37:23] test result: [31mFAILED(B[m. 2792 passed; 18 failed; 39 ignored; 0 measured; 0 filtered out
