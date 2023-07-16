plain
[00:52:35] 
[00:52:35] ---- [ui] ui-fulldeps/proc-macro/generate-mod.rs stdout ----
[00:52:35] diff of stderr:
[00:52:35] 
[00:52:35] - error[E0412]: cannot find type `FromOutside` in this scope
[00:52:35] -   --> $DIR/generate-mod.rs:21:1
[00:52:35] + warning: proc macro crates and `#[no_link]` crates have no effect without `#[macro_use]`
[00:52:35] +   --> $DIR/generate-mod.rs:17:1
[00:52:35] 3    |
[00:52:35] - LL | generate_mod::check!(); //~ ERROR cannot find type `FromOutside` in this scope
[00:52:35] -    | ^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:52:35] + LL | extern crate generate_mod;
[00:52:35] 6 
[00:52:35] 6 
[00:52:35] - error[E0412]: cannot find type `Outer` in this scope
[00:52:35] + error[E0658]: non-ident macro paths are experimental (see issue #35896)
[00:52:35] 9    |
[00:52:35] 9    |
[00:52:35] 10 LL | generate_mod::check!(); //~ ERROR cannot find type `FromOutside` in this scope
[00:52:35] -    | ^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:52:35] - 
[00:52:35] - 
[00:52:35] - error[E0412]: cannot find type `FromOutside` in this scope
[00:52:35] +    | ^^^^^^^^^^^^^^^^^^^
[00:52:35] 15    |
[00:52:35] 15    |
[00:52:35] - LL | #[generate_mod::check_attr] //~ ERROR cannot find type `FromOutside` in this scope
[00:52:35] +    = help: add #![feature(use_extern_macros)] to the crate attributes to enable
[00:52:35] 18 
[00:52:35] 18 
[00:52:35] - error[E0412]: cannot find type `OuterAttr` in this scope
[00:52:35] -    |
[00:52:35] -    |
[00:52:35] - LL | #[generate_mod::check_attr] //~ ERROR cannot find type `FromOutside` in this scope
[00:52:35] - 
[00:52:35] - 
[00:52:35] - warning: cannot find type `FromOutside` in this scope
[00:52:35] + error[E0658]: non-ident macro paths are experimental (see issue #35896)
[00:52:35] 27    |
[00:52:35] 27    |
[00:52:35] 28 LL | #[derive(generate_mod::CheckDerive)] //~ WARN cannot find type `FromOutside` in this scope
[00:52:35] 
[00:52:35] -    |          ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
[00:52:35] 30    |
[00:52:35] 30    |
[00:52:35] -    = note: #[warn(proc_macro_derive_resolution_fallback)] on by default
[00:52:35] -    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:52:35] +    = help: add #![feature(use_extern_macros)] to the crate attributes to enable
[00:52:35] 34 
[00:52:35] 34 
[00:52:35] - warning: cannot find type `OuterDerive` in this scope
[00:52:35] -    |
[00:52:35] -    |
[00:52:35] - LL | #[derive(generate_mod::CheckDerive)] //~ WARN cannot find type `FromOutside` in this scope
[00:52:35] -    |          ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
[00:52:35] -    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[00:52:35] -    = note: for more information, see issue #50504 <https://github.com/rust-lang/rust/issues/50504>
[00:52:35] + error: aborting due to 2 previous errors
[00:52:35] 43 
---
[00:52:35] 47 
[00:52:35] 
[00:52:35] 
[00:52:35] The actual stderr differed from the expected stderr.
[00:52:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/generate-mod/generate-mod.stderr
[00:52:35] To update references, rerun the tests and pass the `--bless` flag
[00:52:35] To only update this specific test, also pass `--test-args proc-macro/generate-mod.rs`
[00:52:35] error: 1 errors occurred comparing output.
[00:52:35] status: exit code: 101
[00:52:35] status: exit code: 101
[00:52:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/proc-macro/generate-mod.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/generate-mod/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/generate-mod/auxiliary" "-A" "unused"
[00:52:35] ------------------------------------------
[00:52:35] 
[00:52:35] ------------------------------------------
[00:52:35] stderr:
[00:52:35] stderr:
[00:52:35] ------------------------------------------
[00:52:35] {"message":"proc macro crates and `#[no_link]` crates have no effect without `#[macro_use]`","code":null,"level":"warning","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/generate-mod.rs","byte_start":659,"byte_end":685,"line_start":17,"line_end":17,"column_start":1,"column_end":27,"is_primary":true,"text":[{"text":"extern crate generate_mod;","highlight_start":1,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: proc macro crates and `#[no_link]` crates have no effect without `#[macro_use]`\n  --> /checkout/src/test/ui-fulldeps/proc-macro/generate-mod.rs:17:1\n   |\nLL | extern crate generate_mod;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:52:35] {"message":"non-ident macro paths are experimental (see issue #35896)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n