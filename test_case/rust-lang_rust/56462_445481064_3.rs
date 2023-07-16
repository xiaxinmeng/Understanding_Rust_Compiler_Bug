\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-34373.rs","byte_start":557,"byte_end":567,"line_start":17,"line_end":17,"column_start":30,"column_end":40,"is_primary":true,"text":[{"text":"pub struct Foo<T = Box<Trait<DefaultFoo>>>;  //~ ERROR cycle detected","highlight_start":30,"highlight_end":40}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"...which requires finding type of DefaultFoo...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-34373.rs","byte_start":616,"byte_end":619,"line_start":18,"line_end":18,"column_start":19,"column_end":22,"is_primary":true,"text":[{"text":"type DefaultFoo = Foo;","highlight_start":19,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...which again requires finding type of Foo::T, completing the cycle       ^^^^
[00:47:57] 6    |
[00:47:57] -    = note: ...which again requires processing `<impl at $DIR/issue-23305.rs:15:1: 15:20>`, completing the cycle
[00:47:57] +    = note: ...which again requires finding type of <impl at $DIR/issue-23305.rs:15:1: 15:20>, completing the cycle
[00:47:57] 9 error: aborting due to previous error
[00:47:57] 10 
[00:47:57] 
[00:47:57] 
[00:47:57] 
[00:47:57] The actual stderr differed from the expected stderr.
[00:47:57] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-23305/issue-23305.stderr
[00:47:57] To update references, rerun the tests and pass the `--bless` flag
[00:47:57] To only update this specific test, also pass `--test-args resolve/issue-23305.rs`
[00:47:57] error: 1 errors occurred comparing output.
[00:47:57] status: exit code: 1
[00:47:57] status: exit code: 1
[00:47:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/issue-23305.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-23305/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-23305/auxiliary" "-A" "unused"
[00:47:57] ------------------------------------------
[00:47:57] 
[00:47:57] ------------------------------------------
[00:47:57] stderr:
[00:47:57] stderr:
[00:47:57] ------------------------------------------
[00:47:57] {"message":"cycle detected when finding type of <impl at /checkout/src/test/ui/resolve/issue-23305.rs:15:1: 15:20>","code":{"code":"E0391","explanation":"\nThis error indicates that some types or traits depend on each other\nand therefore cannot be constructed.\n\nThe following example contains a circular dependency between two traits:\n\n