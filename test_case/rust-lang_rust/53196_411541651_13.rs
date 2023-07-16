\n\nYou need to link your code to the relevant crate in order to be able to use it\n(through Cargo or the `-L` option of rustc example). Plugins are crates as\nwell, and you link to them the same way.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/crateresolve1.rs","byte_start":626,"byte_end":653,"line_start":16,"line_endj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/invalid-module-declaration/invalid-module-declaration.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid-module-declaration/invalid-module-declaration/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid-module-declaration/invalid-module-declaration/auxiliary" "-A" "unused"
[00:49:36] ------------------------------------------
[00:49:36] 
[00:49:36] ------------------------------------------
[00:49:36] stderr:
[00:49:36] stderr:
[00:49:36] ------------------------------------------
[00:49:36] {"message":"file not found for module `baz`","code":{"code":"E0583","explanation":"\nA file wasn't found for an out-of-line module.\n\nErroneous code example:\n\n