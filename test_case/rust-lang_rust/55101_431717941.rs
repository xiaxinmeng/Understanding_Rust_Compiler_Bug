plain
[00:48:51] ...................................i................................................................ 4200/4932
[00:48:56] .................................................................................................... 4300/4932
[00:48:59] .................................................................................................... 4400/4932
[00:49:03] .................................................................................................... 4500/4932
[00:49:06] .......i...................F..F..................................................................... 4600/4932
[00:49:13] .................................................................................................... 4800/4932
[00:49:15] .......................................................................i............................ 4900/4932
[00:49:16] ................................
[00:49:16] failures:
[00:49:16] failures:
[00:49:16] 
[00:49:16] ---- [ui] ui/trait-alias-fail.rs stdout ----
[00:49:16] diff of stderr:
[00:49:16] 
[00:49:16] 22 LL | impl Alias1 for () { //~ERROR expected trait, found trait alias
[00:49:16] 24 
[00:49:16] 24 
[00:49:16] - error[E0658]: trait aliases are not yet fully implemented (see issue #41517)
[00:49:16] + error[E0658]: trait aliases are experimental (see issue #41517)
[00:49:16] 27    |
[00:49:16] 27    |
[00:49:16] 28 LL | trait Alias1<T> = Default where T: Clone; // ok
[00:49:16] 30    |
[00:49:16] 31    = help: add #![feature(trait_alias)] to the crate attributes to enable
[00:49:16] 32 
[00:49:16] 32 
[00:49:16] - error[E0658]: trait aliases are not yet fully implemented (see issue #41517)
[00:49:16] + error[E0658]: trait aliases are experimental (see issue #41517)
[00:49:16] 35    |
[00:49:16] 35    |
[00:49:16] 36 LL | trait Alias2<T: Clone = ()> = Default;
[00:49:16] 
[00:49:16] The actual stderr differed from the expected stderr.
[00:49:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-alias-fail/trait-alias-fail.stderr
[00:49:16] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-alias-fail/trait-alias-fail.stderr
[00:49:16] To update references, rerun the tests and pass the `--bless` flag
[00:49:16] To only update this specific test, also pass `--test-args trait-alias-fail.rs`
[00:49:16] error: 1 errors occurred comparing output.
[00:49:16] status: exit code: 1
[00:49:16] status: exit code: 1
[00:49:16] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trait-alias-fail.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-alias-fail/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-alias-fail/auxiliary" "-A" "unused"
[00:49:16] ------------------------------------------
[00:49:16] 
[00:49:16] ------------------------------------------
[00:49:16] stderr:
[00:49:16] stderr:
[00:49:16] ------------------------------------------
[00:49:16] {"message":"type parameters on the left side of a trait alias cannot be bounded","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/trait-alias-fail.rs","byte_start":612,"byte_end":613,"line_start":15,"line_end":15,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"trait Alias2<T: Clone = ()> = Default;","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: type parameters on the left side of a trait alias cannot be bounded\n  --> /checkout/src/test/ui/trait-alias-fail.rs:15:14\n   |\nLL | trait Alias2<T: Clone = ()> = Default;\n   |              ^\n\n"}
[00:49:16] {"message":"type parameters on the left side of a trait alias cannot have defaults","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/trait-alias-fail.rs","byte_start":612,"byte_end":613,"line_start":15,"line_end":15,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"trait Alias2<T: Clone = ()> = Default;","highlight_start":14,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: type parameters on the left side of a trait alias cannot have defaults\n  --> /checkout/src/test/ui/trait-alias-fail.rs:15:14\n   |\nLL | trait Alias2<T: Clone = ()> = Default;\n   |              ^\n\n"}
[00:49:16] {"message":"expected type, found trait alias `Alias1`","code":{"code":"E0573","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/trait-alias-fail.rs","byte_start":872,"byte_end":878,"line_start":20,"line_end":20,"column_start":6,"column_end":12,"is_primary":true,"text":[{"text":"impl Alias1 { //~ERROR expected type, found trait alias","highlight_start":6,"highlight_end":12}],"label":"not a type","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0573]: expected type, found trait alias `Alias1`\n  --> /checkout/src/test/ui/trait-alias-fail.rs:20:6\n   |\nLL | impl Alias1 { //~ERROR expected type, found trait alias\n   |      ^^^^^^ not a type\n\n"}
[00:49:16] {"message":"expected trait, found trait alias `Alias1`","code":{"code":"E0404","explanation":"\nYou tried to use something which is not a trait in a trait position, such as\na bound or `impl`.\n\nErroneous code example:\n\n