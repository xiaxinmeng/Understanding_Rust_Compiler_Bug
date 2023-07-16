plain
[00:54:47] 
[00:54:47] ---- [ui] ui/extern/extern-const.rs stdout ----
[00:54:47] diff of stderr:
[00:54:47] 
[00:54:47] 4 LL |     const rust_dbg_static_mut: libc::c_int; //~ ERROR extern items cannot be `const`
[00:54:47] 5    |     ^^^^^ help: try using a static value: `static`
[00:54:47] - error: aborting due to previous error
[00:54:47] - error: aborting due to previous error
[00:54:47] + error[E0412]: cannot find type `c_int` in module `libc`
[00:54:47] +   --> $DIR/extern-const.rs:15:38
[00:54:47] +    |
[00:54:47] + LL |     const rust_dbg_static_mut: libc::c_int; //~ ERROR extern items cannot be `const`
[00:54:47] +    |                                      ^^^^^ not found in `libc`
[00:54:47] + help: possible candidate is found in another module, you can import it into scope
[00:54:47] +    |
[00:54:47] + LL | use std::os::raw::c_int;
[00:54:47] 8 
[00:54:47] + error: aborting due to 2 previous errors
[00:54:47] + 
[00:54:47] + For more information about this error, try `rustc --explain E0412`.
[00:54:47] + For more information about this error, try `rustc --explain E0412`.
[00:54:47] 9 
[00:54:47] 
[00:54:47] 
[00:54:47] The actual stderr differed from the expected stderr.
[00:54:47] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/extern-const.stderr
[00:54:47] diff of fixed:
[00:54:47] 
[00:54:47] 10 #![feature(libc)]
[00:54:47] 11 extern crate libc;
[00:54:47] + use std::os::raw::c_int;
[00:54:47] + 
[00:54:47] + 
[00:54:47] 13 #[link(name = "rust_test_helpers", kind = "static")]
[00:54:47] 14 extern "C" {
[00:54:47] 15     static rust_dbg_static_mut: libc::c_int; //~ ERROR extern items cannot be `const`
[00:54:47] 
[00:54:47] The actual fixed differed from the expected fixed.
[00:54:47] The actual fixed differed from the expected fixed.
[00:54:47] Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/extern-const.fixed
[00:54:47] To update references, rerun the tests and pass the `--bless` flag
[00:54:47] To only update this specific test, also pass `--test-args extern/extern-const.rs`
[00:54:47] error: 2 errors occurred comparing output.
[00:54:47] status: exit code: 1
[00:54:47] status: exit code: 1
[00:54:47] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/extern-const.rs" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/a.wasm" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-g" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-const/auxiliary" "-A" "unused"
[00:54:47] ------------------------------------------
[00:54:47] 
[00:54:47] ------------------------------------------
[00:54:47] stderr:
[00:54:47] stderr:
[00:54:47] ------------------------------------------
[00:54:47] {"message":"extern items cannot be `const`","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/extern/extern-const.rs","byte_start":457,"byte_end":462,"line_start":15,"line_end":15,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    const rust_dbg_static_mut: libc::c_int; //~ ERROR extern items cannot be `const`","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try using a static value","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/extern/extern-const.rs","byte_start":457,"byte_end":462,"line_start":15,"line_end":15,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    const rust_dbg_static_mut: libc::c_int; //~ ERROR extern items cannot be `const`","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":"static","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: extern items cannot be `const`\n  --> /checkout/src/test/ui/extern/extern-const.rs:15:5\n   |\nLL |     const rust_dbg_static_mut: libc::c_int; //~ ERROR extern items cannot be `const`\n   |     ^^^^^ help: try using a static value: `static`\n\n"}
[00:54:47] {"message":"cannot find type `c_int` in module `libc`","code":{"code":"E0412","explanation":"\nThe type name used is not in scope.\n\nErroneous code examples:\n\n