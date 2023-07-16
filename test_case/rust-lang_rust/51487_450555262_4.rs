\n\nMake sure that you have defined the associated type in the trait body.\nAlso, verify that you used the right trait or you didn't misspell the\nas/issue-20772.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:01:44] ---- [ui] ui/issues/issue-20825.rs stdout ----
[01:01:44] diff of stderr:
[01:01:44] 
[01:01:44] 5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:01:44] 5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:01:44] 6    |
[01:01:44] 7    = note: ...which again requires computing the supertraits of `Processor`, completing the cycle
[01:01:44] + note: cycle used when processing ``
[01:01:44] +   --> $DIR/issue-20825.rs:5:1
[01:01:44] +    |
[01:01:44] + LL | pub trait Processor: Subscriber<Input = Self::Input> {
[01:01:44] 8 
[01:01:44] 9 error: aborting due to previous error
[01:01:44] 10 
[01:01:44] 
[01:01:44] 
[01:01:44] 
[01:01:44] The actual stderr differed from the expected stderr.
[01:01:44] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20825/issue-20825.stderr
[01:01:44] To update references, rerun the tests and pass the `--bless` flag
[01:01:44] To only update this specific test, also pass `--test-args issues/issue-20825.rs`
[01:01:44] error: 1 errors occurred comparing output.
[01:01:44] status: exit code: 1
[01:01:44] status: exit code: 1
[01:01:44] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-20825.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20825/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-te{","highlight_start":1,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0391]: cycle detected when computing the supertraits of `Processor`\n  --> /checkout/src/test/ui/issues/issue-20825.rs:5:1\n   |\nLL | pub trait Processor: Subscriber<Input = Self::Input> {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: ...which again requires computing the supertraits of `Processor`, completing the cycle\nnote: cycle used when processing ``\n  --> /checkout/src/test/ui/issues/issue-20825.rs:5:1\n   |\nLL | pub trait Processor: Subscriber<Input = Self::Input> {\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:01:44] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:01:44] {"message":"For more information about this error, try `rustc --explain E0391`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0391`.\n"}
[01:01:44] ------------------------------------------
[01:01:44] 
[01:01:44] thread '[ui] ui/issues/issue-20825.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:01:44] 
[01:01:44] 
[01:01:44] ---- [ui] ui/issues/issue-22673.rs stdout ----
[01:01:44] diff of stderr:
[01:01:44] 
[01:01:44] 5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:01:44] 6    |
[01:01:44] 7    = note: ...which again requires computing the supertraits of `Expr`, completing the cyclelp","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: use of unstable library feature 'unstable_test_feature'\n  --> /checkout/src/test/ui/lint/lint-output-format.rs:6:1\n   |\nLL | extern crate lint_output_format; //~ ERROR use of unstable library feature\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: add #![feature(unstable_test_feature)] to the crate attributes to enable\n\n"}
[01:01:44] {"message":"use of unstable library feature 'unstable_test_feature'","code":{"code":"E0658","explanation":"\nAn unstable feature was used.\n\nErroneous code example:\n\n