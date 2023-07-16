\n\nIf the trait `Foo` was deriving from something like `Super<String>` or\n`Super<T>` (where `Foo` itself is `Foo<T>`), this is okay, because given a type\n`get_a()` will definitely return an object of that type.\n\nHowever, if it derives from `Super<Self>`, even thounothing!();
---
[00:40:38] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'feature-gate-macros_in_extern.rs'
[00:40:38]
[00:40:38] error: 1 errors occurred comparing output.
[00:40:38] status: exit code: 101
[00:40:38] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gate-macros_in_extern.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-macros_in_extern.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate-macros_in_extern.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:40:38] {"message":"Macro invocations in `extern {}` blocks are experimental. (see issue #49476)","code":{"code":"E0658","explanation":"\nAn unstable feale_fail,E658\n#[repr(u128)] // error: use of unstable library feature 'repr128'\nenum Foo {\n    Bar(u64),\n}\n