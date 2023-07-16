plain
[00:49:27] ....................................................................................................
[00:49:30] ....................................................................................................
[00:49:35] ....................................................................................................
[00:49:40] ....................................................................................................
[00:49:45] ........F...........................................................................................
[00:49:55] ...........................i........................................................................
[00:50:00] ...............................................ii...................................................
[00:50:05] ....................................................................................................
[00:50:12] ................................................i...................................................
[00:50:12] ................................................i...................................................
[00:50:13] ..........................
[00:50:13] failures:
[00:50:13] 
[00:50:13] ---- [ui] ui/issue-49851/compiler-builtins-error.rs stdout ----
[00:50:13]  diff of stderr:
[00:50:13] 
[00:50:13] - error: Could not create LLVM TargetMachine for triple: thumbv7em-none-eabihf: No available targets are compatible with this triple.
[00:50:13] + error[E0463]: can't find crate for `core`
[00:50:13] +    |
[00:50:13] +    = note: the `thumbv7em-none-eabihf` target may not be installed
[00:50:13] + error: aborting due to previous error
[00:50:13] + 
[00:50:13] + For more information about this error, try `rustc --explain E0463`.
[00:50:13] 3 
---
[00:50:13] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'issue-49851/compiler-builtins-error.rs'
[00:50:13] 
[00:50:13] error: 1 errors occurred comparing output.
[00:50:13] status: exit code: 101
[00:50:13] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-49851/compiler-builtins-error.rs" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-49851/compiler-builtins-error/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "thumbv7em-none-eabihf" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-49851/compiler-builtins-error/auxiliary" "-A" "unused"
[00:50:13] ------------------------------------------
[00:50:13] 
[00:50:13] ------------------------------------------
[00:50:13] stderr:
[00:50:13] stderr:
[00:50:13] ------------------------------------------
[00:50:13] {"message":"can't find crate for `core`","code":{"code":"E0463","explanation":"\nA plugin/crate was declared but cannot be found. Erroneous code example:\n\n