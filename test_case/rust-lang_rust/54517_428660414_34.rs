\n\nIf the item you are importing is not defined in some super-module of the\ncurrent module, then it must also be declared as public (e.g., `pub fn`).\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs.rs","byte_start":582,"byte_end":593,"line_start":30,"line_end":30,"column_start":5,"column_end":16,"is_primary":true,"text":[{"text":"    NonExistent; //~ ERROR cannot find value `NonExistent` in this scope","highlight_start":5,"highlight_end":16}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0425]: cannot find value `NonExistent` in this scope\n  --> /checkout/src/test/ui-fulldeps/proc-macro/ambiguous-builtin-attrs.rs:30:5\n   |\nLL |     NonExistent; //~ ERROR cannot find value `NonExistent` in this scope\n   |     ^^^^^^^^^^^ not found in this scope\n\n"}
[01:03:33] {"message":"aborting due to 8 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 8 previous errors\n\n"}
[01:03:33] {"message":"Some errors occurred: E0425, E0428, E0659.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0425, E0428, E0659.\n"}
[01:03:33] {"message":"For more information about an error, try `rustc --explain E0425`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0425`.\n"}
[01:03:33] ------------------------------------------
[01:03:33] 
[01:03:33] thread '[ui] ui-fulldeps/proc-macro/ambiguous-builtin-attrs.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3277:9
[01:03:33] 
[01:03:33] 
[01:03:33] ---- [ui] ui-fulldeps/proc-macro/generate-mod.rs stdout ----
[01:03:33] diff of stderr:
[01:03:33] 
[01:03:33] + error[E0428]: the name `Alias` is defined multiple times
[01:03:33] +   --> $DIR/generate-mod.rs:22:1
[01:03:33] +    |
[01:03:33] + LL | generate_mod::check!(); //~ ERROR cannot find type `FromOutside` in this scope
[01:03:33] +    | ----------------------- previous definition of the type `Alias` here
[01:03:33] + ...
[01:03:33] + LL | #[generate_mod::check_attr] //~ ERROR cannot find type `FromOutside` in this scope
[01:03:33] +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Alias` redefined here
[01:03:33] +    |
[01:03:33] +    = note: `Alias` must be defined only once in the type namespace of this module
[01:03:33] + 
[01:03:33] + error[E0428]: the name `Outer` is defined multiple times
[01:03:33] +   --> $DIR/generate-mod.rs:22:1
[01:03:33] +    |
[01:03:33] + LL | generate_mod::check!(); //~ ERROR cannot find type `FromOutside` in this scope
[01:03:33] +    | ----------------------- previous definition of the type `Outer` here
[01:03:33] + ...
[01:03:33] + LL | #[generate_mod::check_attr] //~ ERROR cannot find type `FromOutside` in this scope
[01:03:33] +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Outer` redefined here
[01:03:33] +    |
[01:03:33] +    = note: `Outer` must be defined only once in the type namespace of this module
[01:03:33] + 
[01:03:33] + error[E0428]: the name `inner` is defined multiple times
[01:03:33] +   --> $DIR/generate-mod.rs:22:1
[01:03:33] +    |
[01:03:33] + LL | generate_mod::check!(); //~ ERROR cannot find type `FromOutside` in this scope
[01:03:33] +    | ----------------------- previous definition of the module `inner` here
[01:03:33] + ...
[01:03:33] + LL | #[generate_mod::check_attr] //~ ERROR cannot find type `FromOutside` in this scope
[01:03:33] +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `inner` redefined here
[01:03:33] +    |
[01:03:33] +    = note: `inner` must be defined only once in the type namespace of this module
[01:03:33] + 
[01:03:33] + error[E0428]: the name `Alias` is defined multiple times
[01:03:33] +   --> $DIR/generate-mod.rs:26:10
[01:03:33] +    |
[01:03:33] + LL | generate_mod::check!(); //~ ERROR cannot find type `FromOutside` in this scope
[01:03:33] +    | ----------------------- previous definition of the type `Alias` here
[01:03:33] + ...
[01:03:33] + LL | #[derive(generate_mod::CheckDerive)] //~ WARN cannot find type `FromOutside` in this scope
[01:03:33] +    |          ^^^^^^^^^^^^^^^^^^^^^^^^^ `Alias` redefined here
[01:03:33] +    |
[01:03:33] +    = note: `Alias` must be defined only once in the type namespace of this module
[01:03:33] + 
[01:03:33] + error[E0428]: the name `Outer` is defined multiple times
[01:03:33] +   --> $DIR/generate-mod.rs:26:10
[01:03:33] +    |
[01:03:33] + LL | generate_mod::check!(); //~ ERROR cannot find type `FromOutside` in this scope
[01:03:33] +    | ----------------------- previous definition of the type `Outer` here
[01:03:33] + ...
[01:03:33] + LL | #[derive(generate_mod::CheckDerive)] //~ WARN cannot find type `FromOutside` in this scope
[01:03:33] +    |          ^^^^^^^^^^^^^^^^^^^^^^^^^ `Outer` redefined here
[01:03:33] +    |
[01:03:33] +    = note: `Outer` must be defined only once in the type namespace of this module
[01:03:33] + 
[01:03:33] + error[E0428]: the name `inner` is defined multiple times
[01:03:33] +   --> $DIR/generate-mod.rs:26:10
[01:03:33] +    |
[01:03:33] + LL | generate_mod::check!(); //~ ERROR cannot find type `FromOutside` in this scope
[01:03:33] +    | ----------------------- previous definition of the module `inner` here
[01:03:33] + ...
[01:03:33] + LL | #[derive(generate_mod::CheckDerive)] //~ WARN cannot find type `FromOutside` in this scope
[01:03:33] +    |          ^^^^^^^^^^^^^^^^^^^^^^^^^ `inner` redefined here
[01:03:33] +    |
[01:03:33] +    = note: `inner` must be defined only once in the type namespace of this module
[01:03:33] + 
[01:03:33] + error[E0428]: the name `Alias` is defined multiple times
[01:03:33] +   --> $DIR/generate-mod.rs:40:10
[01:03:33] +    |
[01:03:33] + LL | generate_mod::check!(); //~ ERROR cannot find type `FromOutside` in this scope
[01:03:33] +    | ----------------------- previous definition of the type `Alias` here
[01:03:33] + ...
[01:03:33] + LL | #[derive(generate_mod::CheckDeriveLint)] // OK, lint is suppressed
[01:03:33] +    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Alias` redefined here
[01:03:33] +    |
[01:03:33] +    = note: `Alias` must be defined only once in the type namespace of this module
[01:03:33] + 
[01:03:33] + error[E0428]: the name `Outer` is defined multiple times
[01:03:33] +   --> $DIR/generate-mod.rs:40:10
[01:03:33] +    |
[01:03:33] + LL | generate_mod::check!(); //~ ERROR cannot find type `FromOutside` in this scope
[01:03:33] +    | ----------------------- previous definition of the type `Outer` here
[01:03:33] + ...
[01:03:33] + LL | #[derive(generate_mod::CheckDeriveLint)] // OK, lint is suppressed
[01:03:33] +    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Outer` redefined here
[01:03:33] +    |
[01:03:33] +    = note: `Outer` must be defined only once in the type namespace of this module
[01:03:33] + 
[01:03:33] + error[E0428]: the name `inner` is defined multiple times
[01:03:33] +   --> $DIR/generate-mod.rs:40:10
[01:03:33] +    |
[01:03:33] + LL | generate_mod::check!(); //~ ERROR cannot find type `FromOutside` in this scope
[01:03:33] +    | ----------------------- previous definition of the module `inner` here
[01:03:33] + ...
[01:03:33] + LL | #[derive(generate_mod::CheckDeriveLint)] // OK, lint is suppressed
[01:03:33] +    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `inner` redefined here
[01:03:33] +    |
[01:03:33] +    = note: `inner` must be defined only once in the type namespace of this module
[01:03:33] + 
[01:03:33] 1 error[E0412]: cannot find type `FromOutside` in this scope
[01:03:33] 3    |
[01:03:33] 
[01:03:33] 
[01:03:33] 16 LL | #[generate_mod::check_attr] //~ ERROR cannot find type `FromOutside` in this scope
[01:03:33] 18 
[01:03:33] 18 
[01:03:33] - error[E0412]: cannot find type `OuterAttr` in this scope
[01:03:33] + error[E0412]: cannot find type `Outer` in this scope
[01:03:33] 21    |
[01:03:33] 21    |
[01:03:33] 22 LL | #[generate_mod::check_attr] //~ ERROR cannot find type `FromOutside` in this scope
[01:03:33] 32    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[01:03:33] 33    = note: for more information, see issue #50504 <https://github.com/rust-lang/rust/issues/50504>
[01:03:33] 34 
[01:03:33] 34 
[01:03:33] - warning: cannot find type `OuterDerive` in this scope
[01:03:33] + warning: cannot find type `Outer` in this scope
[01:03:33] 37    |
[01:03:33] 37    |
[01:03:33] 38 LL | #[derive(generate_mod::CheckDerive)] //~ WARN cannot find type `FromOutside` in this scope
[01:03:33] 50    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[01:03:33] 51    = note: for more information, see issue #50504 <https://github.com/rust-lang/rust/issues/50504>
[01:03:33] 52 
[01:03:33] 52 
[01:03:33] - warning: cannot find type `OuterDerive` in this scope
[01:03:33] + warning: cannot find type `Outer` in this scope
[01:03:33] 55    |
[01:03:33] 55    |
[01:03:33] 56 LL |     #[derive(generate_mod::CheckDerive)] //~ WARN cannot find type `FromOutside` in this scope
[01:03:33] 59    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[01:03:33] 60    = note: for more information, see issue #50504 <https://github.com/rust-lang/rust/issues/50504>
[01:03:33] 61 
[01:03:33] - error: aborting due to 4 previous errors
[01:03:33] - error: aborting due to 4 previous errors
[01:03:33] + warning: cannot find type `FromOutside` in this scope
[01:03:33] +   --> $DIR/generate-mod.rs:40:10
[01:03:33] +    |
[01:03:33] + LL | #[derive(generate_mod::CheckDeriveLint)] // OK, lint is suppressed
[01:03:33] +    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
[01:03:33] +    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[01:03:33] +    = note: for more information, see issue #50504 <https://github.com/rust-lang/rust/issues/50504>
[01:03:33] 63 
[01:03:33] - For more information about this error, try `rustc --explain E0412`.
[01:03:33] - For more information about this error, try `rustc --explain E0412`.
[01:03:33] + warning: cannot find type `Outer` in this scope
[01:03:33] +   --> $DIR/generate-mod.rs:40:10
[01:03:33] +    |
[01:03:33] + LL | #[derive(generate_mod::CheckDeriveLint)] // OK, lint is suppressed
[01:03:33] +    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
[01:03:33] +    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
[01:03:33] +    = note: for more information, see issue #50504 <https://github.com/rust-lang/rust/issues/50504>
[01:03:33] + 
[01:03:33] + error: aborting due to 13 previous errors
[01:03:33] + error: aborting due to 13 previous errors
[01:03:33] + 
[01:03:33] + Some errors occurred: E0412, E0428.
[01:03:33] + For more information about an error, try `rustc --explain E0412`.
[01:03:33] 65 
[01:03:33] 
[01:03:33] 
[01:03:33] The actual stderr differed from the expected stderr.
[01:03:33] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/generate-mod/generate-mod.stderr
[01:03:33] To update references, rerun the tests and pass the `--bless` flag
[01:03:33] To only update this specific test, also pass `--test-args proc-macro/generate-mod.rs`
[01:03:33] error: 1 errors occurred comparing output.
[01:03:33] status: exit code: 1
[01:03:33] status: exit code: 1
[01:03:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/proc-macro/generate-mod.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/generate-mod/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/proc-macro/generate-mod/auxiliary" "-A" "unused"
[01:03:33] ------------------------------------------
[01:03:33] 
[01:03:33] ------------------------------------------
[01:03:33] stderr:
[01:03:33] stderr:
[01:03:33] ------------------------------------------
[01:03:33] {"message":"the name `Alias` is defined multiple times","code":{"code":"E0428","explanation":"\nA type or module has been defined more than once.\n\nErroneous code example:\n\n