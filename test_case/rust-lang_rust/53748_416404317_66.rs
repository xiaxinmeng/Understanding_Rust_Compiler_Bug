\n\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/resolve/issue-3907.rs","byte_start":589,"byte_end":592,"line_start":20,"line_end":20,"column_start":6,"column_end":9,"is_primary":true,"text":[{"text":"impl Foo for S { //~ ERROR expected trait, found type alias `Foo`","highlight_start":6,"highlight_end":9}],"label":"type aliases cannot be used for traits","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"possible better candidate is found in another module, you can import it into scope","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/resolve/issue-3907.rs","byte_start":525,"byte_end":525,"line_start":14,"line_end":14,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"type Foo = issue_3907::Foo;","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use issue_3907::Foo;\n\n","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0404]: expected trait, found type alias `Foo`\n  --> /checkout/src/test/ui/resolve/issue-3907.rs:20:6\n   |\nLL | impl Foo for S { //~ ERROR expected trait, found type alias `Foo`\n   |      ^^^ type aliases cannot be used for traits\nhelp: possible better candidate is found in another module, you can import it into scope\n   |\nLL | use issue_3907::Foo;\n   |\n\n"}
[00:47:57] thread 'main' panicked at 'internal error: entered unreachable code', librustc_typeck/astconv.rs:696:18
[00:47:57] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:57] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:57] {"message":"For more information about this error, try `rustc --explain E0404`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0404`.\n"}
[00:47:57] error: internal compiler error: unexpected panic
[00:47:57] 
[00:47:57] note: the compiler unexpectedly panicked. this is a bug.
[00:47:57] 
[00:47:57] 
[00:47:57] note: we would appreclias `K`\n   |      ^\n   |      |\n   |      did you mean `I`?\n   |      type aliases cannot be used for traits\n\n"}
[00:47:57] thread 'main' panicked at 'internal error: entered unreachable code', librustc_typeck/astconv.rs:696:18
[00:47:57] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:57] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:47:57] {"message":"Some errors occurred: E0404, E0432.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0404, E0432.\n"}
[00:47:57] {"message":"For more information about an error, try `rustc --explain E0404`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0404`.\n"}
[00:47:57] error: internal compiler error: unexpected panic
[00:47:57] 
[00:47:57] note: the compiler unexpectedly panicked. this is a bug.
[00:47:57] 
[00:47:57] 
[00:47:57] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:57] 
[00:47:57] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:47:57] 
[00:47:57] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:47:57] 
[00:47:57] ------------------------------------------
[00:47:57] 
[00:47:57] thread '[ui] ui/resolve/issue-5035.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:47:57] thread '[ui] ui/resolve/issue-5035.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:47:57] 
[00:47:57] ---- [ui] ui/resolve/resolve-self-in-impl-2.rs stdout ----
[00:47:57] 
[00:47:57] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:57] status: exit code: 101
[00:47:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/resolve-self-in-impl-2.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-self-in-impl-2/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/resolve-self-in-impl-2/auxiliary" "-A" "unused"
[00:47:57] ------------------------------------------
[00:47:57] 
[00:47:57] ------------------------------------------
[00:47:57] stderr:
[00:47:57] stderr:
[00:47:57] ------------------------------------------
[00:47:57] {"message":"expected trait, found self type `Self`","code":{"code":"E0411","explanation":"\nThe `Self` keyword was used outside an impl, trait, or type definition.\n\nErroneous code example:\n\n