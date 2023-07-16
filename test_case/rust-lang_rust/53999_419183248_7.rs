\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/crate-in-paths.rs","byte_start":533,"byte_end":538,"line_start":16,"line_end":16,"column_start":5,"column_end":10,"is_primary":true,"text":[{"text":"    crate struct Foo;","highlight_start":5,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(crate_visibility_modifier)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: `crate` visibility modifier is experimental (see issue #45388)\n  --> /checkout/src/test/ui/crate-in-paths.rs:16:5\n   |\nLL |     crate struct Foo;\n   |     ^^^^^\n   |\n   = help: add #![feature(crate_visibility_modifier)] to the crate attributes to enable\n\n"}
[00:46:26] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:46:26] {"message":"Some errors occurred: E0425, E0658.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0425, E0658.\n"}
[00:46:26] {"message":"For more information about an error, try `rustc --explain E0425`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0425`.\n"}
[00:46:26] ------------------------------------------
[00:46:26] 
[00:46:26] thread '[ui] ui/crate-in-paths.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:46:26] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:26] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:46:26] 
[00:46:26] ---- [ui] ui/rust-2018/edition-lint-fully-qualified-paths.rs stdout ----
[00:46:26] diff of stderr:
[00:46:26] 
[00:46:26] - error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
[00:46:26] -   --> $DIR/edition-lint-fully-qualified-paths.rs:30:25
[00:46:26] + error[E0658]: `crate` visibility modifier is experimental (see issue #45388)
[00:46:26] +   --> $DIR/edition-lint-fully-qualified-paths.rs:17:5
[00:46:26] 3    |
[00:46:26] - LL |     let _: <foo::Baz as ::foo::Foo>::Bar = ();
[00:46:26] -    |                         ^^^^^^^^^^ help: use `crate`: `crate::foo::Foo`
[00:46:26] + LL |     crate trait Foo {
[00:46:26] 6    |
[00:46:26] - note: lint level defined here
[00:46:26] -   --> $DIR/edition-lint-fully-qualified-paths.rs:14:9
[00:46:26] -    |
[00:46:26] -    |
[00:46:26] - LL | #![deny(absolute_paths_not_starting_with_crate)]
[00:46:26] -    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
[00:46:26] -    = note: for more information, see issue TBD
[00:46:26] -    = note: for more information, see issue TBD
[00:46:26] +    = help: add #![feature(crate_visibility_modifier)] to the crate attributes to enable
[00:46:26] 14 
[00:46:26] - error: absolute paths must start with `self`, `super`, `crate`, or an external crate name in the 2018 edition
[00:46:26] -   --> $DIR/edition-lint-fully-qualified-paths.rs:34:13
[00:46:26] + error[E0658]: `crate` visibility modifier is experimental (see issue #45388)
[00:46:26] +   --> $DIR/edition-lint-fully-qualified-paths.rs:21:5
[00:46:26] 17    |
[00:46:26] - LL |     let _: <::foo::Baz as foo::Foo>::Bar = ();
[00:46:26] -    |             ^^^^^^^^^^ help: use `crate`: `crate::foo::Baz`
[00:46:26] + LL |     crate struct Baz { }
[00:46:26] 20    |
[00:46:26] -    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
[00:46:26] -    = note: for more information, see issue TBD
[00:46:26] -    = note: for more information, see issue TBD
[00:46:26] +    = help: add #![feature(crate_visibility_modifier)] to the crate attributes to enable
[00:46:26] 24 error: aborting due to 2 previous errors
[00:46:26] 25 
[00:46:26] 
[00:46:26] + For more information about this error, try `rustc --explain E0658`.
---
[00:46:26] diff of fixed:
[00:46:26] 
[00:46:26] 27 
[00:46:26] 28 
[00:46:26] 29 fn main() {
[00:46:26] -     let _: <foo::Baz as crate::foo::Foo>::Bar = ();
[00:46:26] +     let _: <foo::Baz as ::foo::Foo>::Bar = ();
[00:46:26] 31     //~^ ERROR absolute paths must start with
[00:46:26] 32     //~| this was previously accepted
[00:46:26] 
[00:46:26] 
[00:46:26] -     let _: <crate::foo::Baz as foo::Foo>::Bar = ();
[00:46:26] +     let _: <::foo::Baz as foo::Foo>::Bar = ();
[00:46:26] 35     //~^ ERROR absolute paths must start with
[00:46:26] 36     //~| this was previously accepted
[00:46:26] 
[00:46:26] 
[00:46:26] The actual fixed differed from the expected fixed.
[00:46:26] The actual fixed differed from the expected fixed.
[00:46:26] Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-fully-qualified-paths/edition-lint-fully-qualified-paths.fixed
[00:46:26] To update references, rerun the tests and pass the `--bless` flag
[00:46:26] To only update this specific test, also pass `--test-args rust-2018/edition-lint-fully-qualified-paths.rs`
[00:46:26] error: 2 errors occurred comparing output.
[00:46:26] status: exit code: 1
[00:46:26] status: exit code: 1
[00:46:26] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/edition-lint-fully-qualified-paths.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-fully-qualified-paths/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/edition-lint-fully-qualified-paths/auxiliary" "-A" "unused"
[00:46:26] ------------------------------------------
[00:46:26] 
[00:46:26] ------------------------------------------
[00:46:26] stderr:
[00:46:26] stderr:
[00:46:26] ------------------------------------------
[00:46:26] {"message":"`crate` visibility modifier is experimental (see issue #45388)","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n