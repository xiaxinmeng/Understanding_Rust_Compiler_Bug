\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-2149.rs","byte_start":311,"byte_end":315,"line_start":13,"line_end":13,"column_start":12,"column_end":16,"is_primary":true,"text":[{"text":"    [\"hi\"].bind(|x| [x] );","highlight_start":12,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"items from traits can only be used if the trait is implemented and in scope","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"the following trait defines an item `bind`, perhaps you need to implement it:\ncandidate #1: `VecMonad`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"did you mean","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/issues/issue-2149.rs","byte_start":311,"byte_end":315,"line_start":13,"line_end":13,"column_start":12,"column_end":16,"is_primary":true,"text":[{"text":"    [\"hi\"].bind(|x| [x] );","highlight_start":12,"highlight_end":16}],"label":null,"suggested_replacement":"find","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0599]: no method named `bind` found for type `[&str; 1]` in the current scope\n  --> /checkout/src/test/ui/issues/issue-2149.rs:13:12\n   |\nLL |     [\"hi\"].bind(|x| [x] );\n   |            ^^^^ help: did you mean: `find`\n   |\n   = help: items from traits can only be used if the trait is implemented and in scope\n   = note: the following trait defines an item `bind`, perhaps you need to implement it:\n           candidate #1: `VecMonad`\n\n"}
[01:14:19] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:14:19] {"message":"Some errors occurred: E0277, E0599.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0277, E0599.\n"}
[01:14:19] 
[01:14:19] ------------------------------------------
[01:14:19] 
[01:14:19] thread '[ui] ui/issues/issue-2149.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:14:19] thread '[ui] ui/issues/issue-2149.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
[01:14:19] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:14:19] 
[01:14:19] ---- [ui] ui/issues/issue-48364.rs stdout ----
[01:14:19] diff of stderr:
[01:14:19] 
[01:14:19] - error[E0308]: mismatched types
[01:14:19] -   --> $DIR/issue-48364.rs:2:21
[01:14:19] + error[E0277]: the trait bound `&str: std::needle::Needle<&[u8]>` is not satisfied
[01:14:19] 3    |
[01:14:19] 3    |
[01:14:19] 4 LL |     b"".starts_with(stringify!(foo))
[01:14:19] -    |                     ^^^^^^^^^^^^^^^ expected slice, found str
[01:14:19] +    |         ^^^^^^^^^^^ the trait `std::needle::Needle<&[u8]>` is not implemented for `&str`
[01:14:19] -    = note: expected type `&[u8]`
[01:14:19] -               found type `&'static str`
[01:14:19] +    = help: the following implementations were found:
[01:14:19] +    = help: the following implementations were found:
[01:14:19] +              <&'p str as std::needle::Needle<&'h std::ffi::OsStr>>
[01:14:19] +              <&'p str as std::needle::Needle<H>>
[01:14:19] +              <&'q &'p str as std::needle::Needle<H>>
[01:14:19] 10 error: aborting due to previous error
[01:14:19] 11 
[01:14:19] 
[01:14:19] - For more information about this error, try `rustc --explain E0308`.
[01:14:19] - For more information about this error, try `rustc --explain E0308`.
[01:14:19] + For more information about this error, try `rustc --explain E0277`.
[01:14:19] 13 
[01:14:19] 
[01:14:19] 
[01:14:19] The actual stderr differed from the expected stderr.
[01:14:19] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-48364/issue-48364.stderr
[01:14:19] To update references, rerun the tests and pass the `--bless` flag
[01:14:19] To only update this specific test, also pass `--test-args issues/issue-48364.rs`
[01:14:19] error: 1 errors occurred comparing output.
[01:14:19] status: exit code: 1
[01:14:19] status: exit code: 1
[01:14:19] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-48364.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-48364/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-48364/auxiliary" "-A" "unused"
[01:14:19] ------------------------------------------
[01:14:19] 
[01:14:19] ------------------------------------------
[01:14:19] stderr:
[01:14:19] stderr:
[01:14:19] ------------------------------------------
[01:14:19] {"message":"the trait bound `&str: std::needle::Needle<&[u8]>` is not satisfied","code":{"code":"E0277","explanation":"\nYou tried to use a type which doesn't implement some trait in a place which\nexpected that trait. Erroneous code example:\n\n