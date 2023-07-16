\n\nLifetime elision in implementation headers was part of the lifetime elision\nRFC. It is, however, [currently unimplemented][iss15872].\n\n[book-le]: https://doc.rust-lang.org/nightly/book/first-edition/lifetimes.html#lifetime-elision\n[iss15872]: https://github.com/rust-lang/rust/issues/15872\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/async-fn-multiple-lifetimes.rs","byte_start":905,"byte_end":905,"line_start":26,"line_end":26,"column_start":39,"column_end":39,"is_primary":true,"text":[{"text":"async fn multiple_elided_lifetimes(_: &u8, _: &u8) {}","highlight_start":39,"highlight_end":39}],"label":"expected lifetime parameter","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `_` or `_`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0106]: missing lifetime specifier\n  --> /checkout/src/test/ui/async-fn-multiple-lifetimes.rs:26:39\n   |\nLL | async fn multiple_elided_lifetimes(_: &u8, _: &u8) {}\n   |                                       ^ expected lifetime parameter\n   |\n   = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `_` or `_`\n\n"}
[00:50:15] {"message":"aborting due to 3 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 3 previous errors\n\n"}
[00:50:15] {"message":"Some errors occurred: E0106, E0707, E0709.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0106, E0707, E0709.\n"}
[00:50:15] {"message":"For more information about an error, try `rustc --explain E0106`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0106`.\n"}
[00:50:15] fatal runtime error: failed to initiate panic, error 5
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] thread '[ui] ui/async-fn-multiple-lifetimes.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:50:15] 
[00:50:15] 
[00:50:15] ---- [ui] ui/attr-usage-repr.rs stdout ----
[00:50:15] 
[00:50:15] error: Error: expected failure status (Some(101)) but received status None.
[00:50:15] status: signal: 6
[00:50:15] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/attr-usage-repr.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attr-usage-repr/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attr-usage-repr/auxiliary" "-A" "unused"
[00:50:15] ------------------------------------------
[00:50:15] 
[00:50:15] ------------------------------------------
[00:50:15] stderr:
[00:50:15] stderr:
[00:50:15] ------------------------------------------
[00:50:15] {"message":"attribute should be applied to struct, enum or union","code":{"code":"E0517","explanation":"\nThis error indicates that a `#[repr(..)]` attribute was placed on an\nunsupported item.\n\nExamples of erroneous code:\n\n