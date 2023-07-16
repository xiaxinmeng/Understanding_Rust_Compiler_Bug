\n"},"level":"error","spans":[{"file_name":"<::alloc::macros::vec macros>","byte_start":171,"byte_end":178,"line_start":3,"line_end":3,"column_start":57,"column_end":64,"is_primary":true,"text":[{"text":"{ let tmp = [ $ ( $ x ) , * ] ; < [ _ ] > :: into_vec ( box tmp ) } ) ; (","highlight_start":57,"highlight_end":64}],"label":"expected slice, found array of 2 elements","suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/issues/issue-23589.rs","byte_start":35,"byte_end":49,"line_start":2,"line_end":2,"column_start":24,"column_end":38,"is_primary":false,"text":[{"text":"    let v: Vec(&str) = vec!['1', '2'];","highlight_start":24,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"vec!","def_site_span":{"file_name":"<::alloc::macros::vec macros>","byte_start":0,"byte_end":242,"line_start":1,"line_end":4,"column_start":1,"column_end":54,"is_primary":false,"text":[{"text":"( $ elem : expr ; $ n : expr ) => (","highlight_start":1,"highlight_end":36},{"text":"$ crate :: vec :: from_elem ( $ elem , $ n ) ) ; ( $ ( $ x : expr ) , * ) => (","highlight_start":1,"highlight_end":79},{"text":"{ let tmp = [ $ ( $ x ) , * ] ; < [ _ ] > :: into_vec ( box tmp ) } ) ; (","highlight_start":1,"highlight_end":74},{"text":"$ ( $ x : expr , ) * ) => ( vec ! [ $ ( $ x ) , * ] )","highlight_start":1,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"expected type `std::boxed::Box<[&str]>`\n   found type `std::boxed::Box<[char; 2]>`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/issues/issue-23589.rs:2:24\n   |\nLL |     let v: Vec(&str) = vec!['1', '2'];\n   |                        ^^^^^^^^^^^^^^ expected slice, found array of 2 elements\n   |\n   = note: expected type `std::boxed::Box<[&str]>`\n              found type `std::boxed::Box<[char; 2]>`\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:05:24] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:05:24] {"message":"Some errors occurred: E0214, E0308.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0214, E0308.\n"}
[01:05:24] 
[01:05:24] ------------------------------------------
[01:05:24] 
[01:05:24] thread '[ui] ui/issues/issue-23589.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:05:24] thread '[ui] ui/issues/issue-23589.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:05:24] 
[01:05:24] ---- [ui] ui/issues/issue-47184.rs stdout ----
[01:05:24] diff of stderr:
[01:05:24] 
[01:05:24] 2   --> $DIR/issue-47184.rs:4:44
[01:05:24] 3    |
[01:05:24] 4 LL |     let _vec: Vec<&'static String> = vec![&String::new()];
[01:05:24] -    |               --------------------         ^^^^^^^^^^^^^ - temporary value is freed at the end of this statement
[01:05:24] -    |               |                            |
[01:05:24] -    |               |                            creates a temporary which is freed while still in use
[01:05:24] +    |               |                      |     |
[01:05:24] +    |               |                      |     |
[01:05:24] +    |               |                      |     creates a temporary which is freed while still in use
[01:05:24] +    |               |                      temporary value is freed at the end of this statement
[01:05:24] 8    |               type annotation requires that borrow lasts for `'static`
[01:05:24] +    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:05:24] 9 
[01:05:24] 10 error: aborting due to previous error
[01:05:24] 11 
[01:05:24] 11 
[01:05:24] 
[01:05:24] 
[01:05:24] The actual stderr differed from the expected stderr.
[01:05:24] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47184/issue-47184.stderr
[01:05:24] To update references, rerun the tests and pass the `--bless` flag
[01:05:24] To only update this specific test, also pass `--test-args issues/issue-47184.rs`
[01:05:24] error: 1 errors occurred comparing output.
[01:05:24] status: exit code: 1
[01:05:24] status: exit code: 1
[01:05:24] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-47184.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47184/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47184/auxiliary" "-A" "unused"
[01:05:24] ------------------------------------------
[01:05:24] 
[01:05:24] ------------------------------------------
[01:05:24] stderr:
[01:05:24] stderr:
[01:05:24] ------------------------------------------
[01:05:24] {"message":"temporary value dropped while borrowed","code":{"code":"E0716","explanation":"\nThis error indicates that a temporary value is being dropped\nwhile a borrow is still in active use.\n\nErroneous code example:\n\n