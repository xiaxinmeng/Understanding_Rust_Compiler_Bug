\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/trait-alias-fail.rs","byte_start":599,"byte_end":637,"line_start":15,"line_end":15,"column_start":1,"column_end":39,"is_primary":true,"text":[{"text":"trait Alias2<T: Clone = ()> = Default;","highlight_start":1,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add #![feature(trait_alias)] to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: trait aliases are experimental (see issue #41517)\n  --> /checkout/src/test/ui/trait-alias-fail.rs:15:1\n   |\nLL | trait Alias2<T: Clone = ()> = Default;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: add #![feature(trait_alias)] to the crate attributes to enable\n\n"}
[00:49:16] {"message":"aborting due to 6 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 6 previous errors\n\n"}
[00:49:16] {"message":"Some errors occurred: E0404, E0573, E0658.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0404, E0573, E0658.\n"}
[00:49:16] {"message":"For more information about an error, try `rustc --explain E0404`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0404`.\n"}
[00:49:16] ------------------------------------------
[00:49:16] 
[00:49:16] thread '[ui] ui/trait-alias-fail.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3284:9
[00:49:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:49:16] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:49:16] 
[00:49:16] ---- [ui] ui/traits/trait-alias.rs stdout ----
[00:49:16] diff of stderr:
[00:49:16] 
[00:49:16] - error[E0645]: trait aliases are not yet implemented (see issue #41517)
[00:49:16] -   --> $DIR/trait-alias.rs:13:1
[00:49:16] + error[E0283]: type annotations required: cannot resolve `_: CD`
[00:49:16] +   --> $DIR/trait-alias.rs:36:16
[00:49:16] 3    |
[00:49:16] - LL | trait SimpleAlias = Default; //~ERROR E0645
[00:49:16] - 
[00:49:16] - 
[00:49:16] - error[E0645]: trait aliases are not yet implemented (see issue #41517)
[00:49:16] -   --> $DIR/trait-alias.rs:14:1
[00:49:16] + LL |     let both = foo();
[00:49:16] 9    |
[00:49:16] 9    |
[00:49:16] - LL | trait GenericAlias<T> = Iterator<Item=T>; //~ERROR E0645
[00:49:16] - 
[00:49:16] - 
[00:49:16] - error[E0645]: trait aliases are not yet implemented (see issue #41517)
[00:49:16] -   --> $DIR/trait-alias.rs:15:1
[00:49:16] + note: required by `foo`
[00:49:16] +   --> $DIR/trait-alias.rs:29:1
[00:49:16] 15    |
[00:49:16] - LL | trait Partial<T> = IntoIterator<Item=T>; //~ERROR E0645
[00:49:16] -    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:49:16] + LL | fn foo<T: CD>() -> (T, T) {
[00:49:16] 18 
[00:49:16] 18 
[00:49:16] - error[E0645]: trait aliases are not yet implemented (see issue #41517)
[00:49:16] -    |
[00:49:16] -    |
[00:49:16] - LL | trait WithWhere<Art, Thou> = Romeo + Romeo where Fore<(Art, Thou)>: Romeo; //~ERROR E0645
[00:49:16] + error: aborting due to previous error
[00:49:16] 24 
[00:49:16] 24 
[00:49:16] - error[E0645]: trait aliases are not yet implemented (see issue #41517)
[00:49:16] -    |
[00:49:16] -    |
[00:49:16] - LL | trait BareWhere<Wild, Are> = where The<Wild>: Things<Are>; //~ERROR E0645
[00:49:16] - 
[00:49:16] - 
[00:49:16] - error[E0645]: trait aliases are not yet implemented (see issue #41517)
[00:49:16] -    |
[00:49:16] -    |
[00:49:16] - LL | trait CD = Clone + Default; //~ERROR E0645
[00:49:16] - 
[00:49:16] - error: aborting due to 6 previous errors
[00:49:16] - 
[00:49:16] - For more information about this error, try `rustc --explain E0645`.
[00:49:16] - For more information about this error, try `rustc --explain E0645`.
[00:49:16] + For more information about this error, try `rustc --explain E0283`.
[00:49:16] 40 
[00:49:16] 
[00:49:16] 
[00:49:16] The actual stderr differed from the expected stderr.
[00:49:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias/trait-alias.stderr
[00:49:16] To update references, rerun the tests and pass the `--bless` flag
[00:49:16] To only update this specific test, also pass `--test-args traits/trait-alias.rs`
[00:49:16] error: 1 errors occurred comparing output.
[00:49:16] status: exit code: 1
[00:49:16] status: exit code: 1
[00:49:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-alias.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-alias/auxiliary" "-A" "unused"
[00:49:16] ------------------------------------------
[00:49:16] 
[00:49:16] ------------------------------------------
[00:49:16] stderr:
[00:49:16] stderr:
[00:49:16] ------------------------------------------
[00:49:16] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:49:16] {"message":"type annotations required: cannot resolve `_: CD`","code":{"code":"E0283","explanation":"\nThis error occurs when the compiler doesn't have enough information\nto unambiguously choose an implementation.\n\nFor example:\n\n