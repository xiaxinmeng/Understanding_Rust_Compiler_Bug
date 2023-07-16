plain
[01:29:39] normalized stderr:
[01:29:39] error: use of `expect` followed by a function call
[01:29:39]   --> $DIR/expect_fun_call.rs:36:26
[01:29:39]    |
[01:29:39] LL |     with_none_and_format.expect(&format!("Error {}: fake error", error_code));
[01:29:39]    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("Error {}: fake error"))`
[01:29:39]    = note: `-D clippy::expect-fun-call` implied by `-D warnings`
[01:29:39] 
[01:29:39] error: use of `expect` followed by a function call
[01:29:39]   --> $DIR/expect_fun_call.rs:39:26
[01:29:39]   --> $DIR/expect_fun_call.rs:39:26
[01:29:39]    |
[01:29:39] LL |     with_none_and_as_str.expect(format!("Error {}: fake error", error_code).as_str());
[01:29:39]    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("Error {}: fake error"))`
[01:29:39] error: use of `expect` followed by a function call
[01:29:39]   --> $DIR/expect_fun_call.rs:49:25
[01:29:39]    |
[01:29:39]    |
[01:29:39] LL |     with_err_and_format.expect(&format!("Error {}: fake error", error_code));
[01:29:39]    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|_| panic!("Error {}: fake error"))`
[01:29:39] error: use of `expect` followed by a function call
[01:29:39]   --> $DIR/expect_fun_call.rs:52:25
[01:29:39]    |
[01:29:39]    |
[01:29:39] LL |     with_err_and_as_str.expect(format!("Error {}: fake error", error_code).as_str());
[01:29:39]    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|_| panic!("Error {}: fake error"))`
[01:29:39] error: use of `expect` followed by a function call
[01:29:39]   --> $DIR/expect_fun_call.rs:67:17
[01:29:39]    |
[01:29:39]    |
[01:29:39] LL |     Some("foo").expect({ &format!("error") });
[01:29:39]    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { let msg = { &format!("error") }; panic!(msg) }))`
[01:29:39] error: use of `expect` followed by a function call
[01:29:39]   --> $DIR/expect_fun_call.rs:68:17
[01:29:39]    |
[01:29:39]    |
[01:29:39] LL |     Some("foo").expect(format!("error").as_ref());
[01:29:39]    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("error"))`
[01:29:39] error: aborting due to 6 previous errors
[01:29:39] 
[01:29:39] 
[01:29:39] 
[01:29:39] 
[01:29:39] expected stderr:
[01:29:39] error: use of `expect` followed by a function call
[01:29:39]   --> $DIR/expect_fun_call.rs:36:26
[01:29:39]    |
[01:29:39] LL |     with_none_and_format.expect(&format!("Error {}: fake error", error_code));
[01:29:39]    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("Error {}: fake error", error_code))`
[01:29:39]    = note: `-D clippy::expect-fun-call` implied by `-D warnings`
[01:29:39] 
[01:29:39] error: use of `expect` followed by a function call
[01:29:39]   --> $DIR/expect_fun_call.rs:39:26
[01:29:39]   --> $DIR/expect_fun_call.rs:39:26
[01:29:39]    |
[01:29:39] LL |     with_none_and_as_str.expect(format!("Error {}: fake error", error_code).as_str());
[01:29:39]    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("Error {}: fake error", error_code))`
[01:29:39] error: use of `expect` followed by a function call
[01:29:39]   --> $DIR/expect_fun_call.rs:49:25
[01:29:39]    |
[01:29:39]    |
[01:29:39] LL |     with_err_and_format.expect(&format!("Error {}: fake error", error_code));
[01:29:39]    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|_| panic!("Error {}: fake error", error_code))`
[01:29:39] error: use of `expect` followed by a function call
[01:29:39]   --> $DIR/expect_fun_call.rs:52:25
[01:29:39]    |
[01:29:39]    |
[01:29:39] LL |     with_err_and_as_str.expect(format!("Error {}: fake error", error_code).as_str());
[01:29:39]    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|_| panic!("Error {}: fake error", error_code))`
[01:29:39] error: use of `expect` followed by a function call
[01:29:39]   --> $DIR/expect_fun_call.rs:67:17
[01:29:39]    |
[01:29:39]    |
[01:29:39] LL |     Some("foo").expect({ &format!("error") });
[01:29:39]    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { let msg = { &format!("error") }; panic!(msg) }))`
[01:29:39] error: use of `expect` followed by a function call
[01:29:39]   --> $DIR/expect_fun_call.rs:68:17
[01:29:39]    |
[01:29:39]    |
[01:29:39] LL |     Some("foo").expect(format!("error").as_ref());
[01:29:39]    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("error"))`
[01:29:39] error: aborting due to 6 previous errors
[01:29:39] 
[01:29:39] 
[01:29:39] 
[01:29:39] 
[01:29:39] diff of stderr:
[01:29:39] 
[01:29:39]  error: use of `expect` followed by a function call
[01:29:39]    --> $DIR/expect_fun_call.rs:36:26
[01:29:39]     |
[01:29:39]  LL |     with_none_and_format.expect(&format!("Error {}: fake error", error_code));
[01:29:39] -   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("Error {}: fake error", error_code))`
[01:29:39] +   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("Error {}: fake error"))`
[01:29:39]     = note: `-D clippy::expect-fun-call` implied by `-D warnings`
[01:29:39]  
[01:29:39]  error: use of `expect` followed by a function call
[01:29:39]    --> $DIR/expect_fun_call.rs:39:26
[01:29:39]    --> $DIR/expect_fun_call.rs:39:26
[01:29:39]     |
[01:29:39]  LL |     with_none_and_as_str.expect(format!("Error {}: fake error", error_code).as_str());
[01:29:39] -   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("Error {}: fake error", error_code))`
[01:29:39] +   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("Error {}: fake error"))`
[01:29:39]  error: use of `expect` followed by a function call
[01:29:39]    --> $DIR/expect_fun_call.rs:49:25
[01:29:39]     |
[01:29:39]     |
[01:29:39]  LL |     with_err_and_format.expect(&format!("Error {}: fake error", error_code));
[01:29:39] -   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|_| panic!("Error {}: fake error", error_code))`
[01:29:39] +   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|_| panic!("Error {}: fake error"))`
[01:29:39]  error: use of `expect` followed by a function call
[01:29:39]    --> $DIR/expect_fun_call.rs:52:25
[01:29:39]     |
[01:29:39]     |
[01:29:39]  LL |     with_err_and_as_str.expect(format!("Error {}: fake error", error_code).as_str());
[01:29:39] -   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|_| panic!("Error {}: fake error", error_code))`
[01:29:39] +   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|_| panic!("Error {}: fake error"))`
[01:29:39]  error: use of `expect` followed by a function call
[01:29:39]    --> $DIR/expect_fun_call.rs:67:17
[01:29:39]     |
[01:29:39]     |
[01:29:39]  LL |     Some("foo").expect({ &format!("error") });
[01:29:39]     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { let msg = { &format!("error") }; panic!(msg) }))`
[01:29:39]  error: use of `expect` followed by a function call
[01:29:39]    --> $DIR/expect_fun_call.rs:68:17
[01:29:39]     |
[01:29:39]     |
[01:29:39]  LL |     Some("foo").expect(format!("error").as_ref());
[01:29:39]     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!("error"))`
[01:29:39]  error: aborting due to 6 previous errors
[01:29:39]  
[01:29:39]  
[01:29:39] 
[01:29:39] 
[01:29:39] The actual stderr differed from the expected stderr.
[01:29:39] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-f6a900eb04107d45/out/test_build_base/expect_fun_call.stderr
[01:29:39] To update references, run this command from build directory:
[01:29:39] tests/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-f6a900eb04107d45/out/test_build_base' 'expect_fun_call.rs'
[01:29:39] 
[01:29:39] error: 1 errors occurred comparing output.
[01:29:39] status: exit code: 1
[01:29:39] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/expect_fun_call.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-f6a900eb04107d45/out/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-f6a900eb04107d45/out/test_build_base/expect_fun_call.stage-id" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-Dwarnings" "-Zui-testing" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/build/clippy-f6a900eb04107d45/out/test_build_base/expect_fun_call.stage-id.aux" "-A" "unused"
[01:29:39] ------------------------------------------
[01:29:39] 
[01:29:39] ------------------------------------------
[01:29:39] stderr:
[01:29:39] stderr:
[01:29:39] ------------------------------------------
[01:29:39] {"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":994,"byte_end":1046,"line_start":36,"line_end":36,"column_start":26,"column_end":78,"is_primary":true,"text":[{"text":"    with_none_and_format.expect(&format!(\"Error {}: fake error\", error_code));","highlight_start":26,"highlight_end":78}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::expect-fun-call` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":994,"byte_end":1046,"line_start":36,"line_end":36,"column_start":26,"column_end":78,"is_primary":true,"text":[{"text":"    with_none_and_format.expect(&format!(\"Error {}: fake error\", error_code));","highlight_start":26,"highlight_end":78}],"label":null,"suggested_replacement":"unwrap_or_else(|| panic!(\"Error {}: fake error\"))","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:36:26\n   |\nLL |     with_none_and_format.expect(&format!(\"Error {}: fake error\", error_code));\n   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!(\"Error {}: fake error\"))`\n   |\n   = note: `-D clippy::expect-fun-call` implied by `-D warnings`\n\n"}
[01:29:39] {"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":1124,"byte_end":1184,"line_start":39,"line_end":39,"column_start":26,"column_end":86,"is_primary":true,"text":[{"text":"    with_none_and_as_str.expect(format!(\"Error {}: fake error\", error_code).as_str());","highlight_start":26,"highlight_end":86}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":1124,"byte_end":1184,"line_start":39,"line_end":39,"column_start":26,"column_end":86,"is_primary":true,"text":[{"text":"    with_none_and_as_str.expect(format!(\"Error {}: fake error\", error_code).as_str());","highlight_start":26,"highlight_end":86}],"label":null,"suggested_replacement":"unwrap_or_else(|| panic!(\"Error {}: fake error\"))","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:39:26\n   |\nLL |     with_none_and_as_str.expect(format!(\"Error {}: fake error\", error_code).as_str());\n   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!(\"Error {}: fake error\"))`\n\n"}
[01:29:39] {"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":1443,"byte_end":1495,"line_start":49,"line_end":49,"column_start":25,"column_end":77,"is_primary":true,"text":[{"text":"    with_err_and_format.expect(&format!(\"Error {}: fake error\", error_code));","highlight_start":25,"highlight_end":77}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":1443,"byte_end":1495,"line_start":49,"line_end":49,"column_start":25,"column_end":77,"is_primary":true,"text":[{"text":"    with_err_and_format.expect(&format!(\"Error {}: fake error\", error_code));","highlight_start":25,"highlight_end":77}],"label":null,"suggested_replacement":"unwrap_or_else(|_| panic!(\"Error {}: fake error\"))","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:49:25\n   |\nLL |     with_err_and_format.expect(&format!(\"Error {}: fake error\", error_code));\n   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|_| panic!(\"Error {}: fake error\"))`\n\n"}
[01:29:39] {"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":1577,"byte_end":1637,"line_start":52,"line_end":52,"column_start":25,"column_end":85,"is_primary":true,"text":[{"text":"    with_err_and_as_str.expect(format!(\"Error {}: fake error\", error_code).as_str());","highlight_start":25,"highlight_end":85}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":1577,"byte_end":1637,"line_start":52,"line_end":52,"column_start":25,"column_end":85,"is_primary":true,"text":[{"text":"    with_err_and_as_str.expect(format!(\"Error {}: fake error\", error_code).as_str());","highlight_start":25,"highlight_end":85}],"label":null,"suggested_replacement":"unwrap_or_else(|_| panic!(\"Error {}: fake error\"))","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:52:25\n   |\nLL |     with_err_and_as_str.expect(format!(\"Error {}: fake error\", error_code).as_str());\n   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|_| panic!(\"Error {}: fake error\"))`\n\n"}
[01:29:39] {"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2116,"byte_end":2145,"line_start":67,"line_end":67,"column_start":17,"column_end":46,"is_primary":true,"text":[{"text":"    Some(\"foo\").expect({ &format!(\"error\") });","highlight_start":17,"highlight_end":46}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2116,"byte_end":2145,"line_start":67,"line_end":67,"column_start":17,"column_end":46,"is_primary":true,"text":[{"text":"    Some(\"foo\").expect({ &format!(\"error\") });","highlight_start":17,"highlight_end":46}],"label":null,"suggested_replacement":"unwrap_or_else(|| { let msg = { &format!(\"error\") }; panic!(msg) }))","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:67:17\n   |\nLL |     Some(\"foo\").expect({ &format!(\"error\") });\n   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| { let msg = { &format!(\"error\") }; panic!(msg) }))`\n\n"}
[01:29:39] {"message":"use of `expect` followed by a function call","code":{"code":"clippy::expect_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2163,"byte_end":2196,"line_start":68,"line_end":68,"column_start":17,"column_end":50,"is_primary":true,"text":[{"text":"    Some(\"foo\").expect(format!(\"error\").as_ref());","highlight_start":17,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/expect_fun_call.rs","byte_start":2163,"byte_end":2196,"line_start":68,"line_end":68,"column_start":17,"column_end":50,"is_primary":true,"text":[{"text":"    Some(\"foo\").expect(format!(\"error\").as_ref());","highlight_start":17,"highlight_end":50}],"label":null,"suggested_replacement":"unwrap_or_else(|| panic!(\"error\"))","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `expect` followed by a function call\n  --> tests/ui/expect_fun_call.rs:68:17\n   |\nLL |     Some(\"foo\").expect(format!(\"error\").as_ref());\n   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| panic!(\"error\"))`\n\n"}
[01:29:39] {"message":"aborting due to 6 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 6 previous errors\n\n"}
[01:29:39] ------------------------------------------
[01:29:39] 
[01:29:39] thread '[ui] ui/expect_fun_call.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.18/src/runtest.rs:2632:9
[01:29:39] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:29:39] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:29:39] 
[01:29:39] ---- [ui] ui/format.rs stdout ----
[01:29:39] normalized stderr:
[01:29:39] error: useless use of `format!`
[01:29:39]    |
[01:29:39] LL |     format!("foo");
[01:29:39] LL |     format!("foo");
[01:29:39]    |     ^^^^^^^^^^^^^^^ help: consider using .to_string(): `"foo".to_string()`
[01:29:39]    = note: `-D clippy::useless-format` implied by `-D warnings`
[01:29:39] 
[01:29:39] 
[01:29:39] error: useless use of `format!`
[01:29:39]    |
[01:29:39]    |
[01:29:39] LL |     format!("{:+}", "foo"); // warn when the format makes no difference
[01:29:39]    |     ^^^^^^^^^^^^^^^^^^^^^^^ help: consider using .to_string(): `"foo".to_string()`
[01:29:39]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:29:39] 
[01:29:39] 
[01:29:39] error: useless use of `format!`
[01:29:39]    |
[01:29:39]    |
[01:29:39] LL |     format!("{:<}", "foo"); // warn when the format makes no difference
[01:29:39]    |     ^^^^^^^^^^^^^^^^^^^^^^^ help: consider using .to_string(): `"foo".to_string()`
[01:29:39]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:29:39] 
[01:29:39] 
[01:29:39] error: useless use of `format!`
[01:29:39]    |
[01:29:39]    |
[01:29:39] LL |     format!("{:+}", arg); // warn when the format makes no difference
[01:29:39]    |     ^^^^^^^^^^^^^^^^^^^^^ help: consider using .to_string(): `arg.to_string()`
[01:29:39]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:29:39] 
[01:29:39] 
[01:29:39] error: useless use of `format!`
[01:29:39]    |
[01:29:39]    |
[01:29:39] LL |     format!("{:<}", arg); // warn when the format makes no difference
[01:29:39]    |     ^^^^^^^^^^^^^^^^^^^^^ help: consider using .to_string(): `arg.to_string()`
[01:29:39]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:29:39] 
[01:29:39] error: aborting due to 5 previous errors
[01:29:39] 
[01:29:39] 
[01:29:39] 
[01:29:39] 
[01:29:39] expected stderr:
[01:29:39] error: useless use of `format!`
[01:29:39]    |
[01:29:39] LL |     format!("foo");
[01:29:39] LL |     format!("foo");
[01:29:39]    |     ^^^^^^^^^^^^^^^ help: consider using .to_string(): `"foo".to_string()`
[01:29:39]    = note: `-D clippy::useless-format` implied by `-D warnings`
[01:29:39] 
[01:29:39] 
[01:29:39] error: useless use of `format!`
[01:29:39]    |
[01:29:39] LL |     format!("{}", "foo");
[01:29:39] LL |     format!("{}", "foo");
[01:29:39]    |     ^^^^^^^^^^^^^^^^^^^^^ help: consider using .to_string(): `"foo".to_string()`
[01:29:39]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:29:39] 
[01:29:39] 
[01:29:39] error: useless use of `format!`
[01:29:39]    |
[01:29:39]    |
[01:29:39] LL |     format!("{:+}", "foo"); // warn when the format makes no difference
[01:29:39]    |     ^^^^^^^^^^^^^^^^^^^^^^^ help: consider using .to_string(): `"foo".to_string()`
[01:29:39]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:29:39] 
[01:29:39] 
[01:29:39] error: useless use of `format!`
[01:29:39]    |
[01:29:39]    |
[01:29:39] LL |     format!("{:<}", "foo"); // warn when the format makes no difference
[01:29:39]    |     ^^^^^^^^^^^^^^^^^^^^^^^ help: consider using .to_string(): `"foo".to_string()`
[01:29:39]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:29:39] 
[01:29:39] 
[01:29:39] error: useless use of `format!`
[01:29:39]    |
[01:29:39] LL |     format!("{}", arg);
[01:29:39] LL |     format!("{}", arg);
[01:29:39]    |     ^^^^^^^^^^^^^^^^^^^ help: consider using .to_string(): `arg.to_string()`
[01:29:39]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:29:39] 
[01:29:39] 
[01:29:39] error: useless use of `format!`
[01:29:39]    |
[01:29:39]    |
[01:29:39] LL |     format!("{:+}", arg); // warn when the format makes no difference
[01:29:39]    |     ^^^^^^^^^^^^^^^^^^^^^ help: consider using .to_string(): `arg.to_string()`
[01:29:39]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:29:39] 
[01:29:39] 
[01:29:39] error: useless use of `format!`
[01:29:39]    |
[01:29:39]    |
[01:29:39] LL |     format!("{:<}", arg); // warn when the format makes no difference
[01:29:39]    |     ^^^^^^^^^^^^^^^^^^^^^ help: consider using .to_string(): `arg.to_string()`
[01:29:39]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:29:39] 
[01:29:39] 
[01:29:39] error: useless use of `format!`
[01:29:39]    |
[01:29:39]    |
[01:29:39] LL |     format!("{}", 42.to_string());
[01:29:39]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: `to_string()` is enough: `42.to_string()`
[01:29:39]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:29:39] 
[01:29:39] 
[01:29:39] error: useless use of `format!`
[01:29:39]    |
[01:29:39]    |
[01:29:39] LL |     format!("{}", x.display().to_string());
[01:29:39]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: `to_string()` is enough: `x.display().to_string()`
[01:29:39]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:29:39] 
[01:29:39] error: aborting due to 9 previous errors
[01:29:39] 
[01:29:39] 
[01:29:39] 
[01:29:39] 
[01:29:39] diff of stderr:
[01:29:39] 
[01:29:39]  error: useless use of `format!`
[01:29:39]     |
[01:29:39]  LL |     format!("foo");
[01:29:39]  LL |     format!("foo");
[01:29:39]     |     ^^^^^^^^^^^^^^^ help: consider using .to_string(): `"foo".to_string()`
[01:29:39]     = note: `-D clippy::useless-format` implied by `-D warnings`
[01:29:39]  
[01:29:39]  
[01:29:39]  error: useless use of `format!`
[01:29:39] -   |
[01:29:39] -   |
[01:29:39] -LL |     format!("{}", "foo");
[01:29:39] -   |     ^^^^^^^^^^^^^^^^^^^^^ help: consider using .to_string(): `"foo".to_string()`
[01:29:39] -   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:29:39] -
[01:29:39] -
[01:29:39] -error: useless use of `format!`
[01:29:39]     |
[01:29:39]     |
[01:29:39]  LL |     format!("{:+}", "foo"); // warn when the format makes no difference
[01:29:39]     |     ^^^^^^^^^^^^^^^^^^^^^^^ help: consider using .to_string(): `"foo".to_string()`
[01:29:39]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:29:39]  
[01:29:39]  
[01:29:39]  error: useless use of `format!`
[01:29:39]     |
[01:29:39]     |
[01:29:39]  LL |     format!("{:<}", "foo"); // warn when the format makes no difference
[01:29:39]     |     ^^^^^^^^^^^^^^^^^^^^^^^ help: consider using .to_string(): `"foo".to_string()`
[01:29:39]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:29:39]  
[01:29:39]  
[01:29:39]  error: useless use of `format!`
[01:29:39] -   |
[01:29:39] -   |
[01:29:39] -LL |     format!("{}", arg);
[01:29:39] -   |     ^^^^^^^^^^^^^^^^^^^ help: consider using .to_string(): `arg.to_string()`
[01:29:39] -   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:29:39] -
[01:29:39] -
[01:29:39] -error: useless use of `format!`
[01:29:39]     |
[01:29:39]     |
[01:29:39]  LL |     format!("{:+}", arg); // warn when the format makes no difference
[01:29:39]     |     ^^^^^^^^^^^^^^^^^^^^^ help: consider using .to_string(): `arg.to_string()`
[01:29:39]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:29:39]  
[01:29:39]  
[01:29:39]  error: useless use of `format!`
[01:29:39]     |
[01:29:39]     |
[01:29:39]  LL |     format!("{:<}", arg); // warn when the format makes no difference
[01:29:39]     |     ^^^^^^^^^^^^^^^^^^^^^ help: consider using .to_string(): `arg.to_string()`
[01:29:39]     = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:29:39]  
[01:29:39]  
[01:29:39] -error: useless use of `format!`
[01:29:39] -   |
[01:29:39] -   |
[01:29:39] -LL |     format!("{}", 42.to_string());
[01:29:39] -   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: `to_string()` is enough: `42.to_string()`
[01:29:39] -   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:29:39] -
[01:29:39] -
[01:29:39] -error: useless use of `format!`
[01:29:39] -   |
[01:29:39] -   |
[01:29:39] -LL |     format!("{}", x.display().to_string());
[01:29:39] -   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: `to_string()` is enough: `x.display().to_string()`
[01:29:39] -   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:29:39] -
[01:29:39] -error: aborting due to 9 previous errors
[01:29:39] +error: aborting due to 5 previous errors
---
[01:29:39] 
[01:29:39] ------------------------------------------
[01:29:39] stderr:
[01:29:39] ------------------------------------------
[01:29:39] {"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":602,"byte_end":617,"line_start":21,"line_end":21,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    format!(\"foo\");","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::useless-format` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"consider using .to_string()","code":null,"level":"help","spans":[{"file_name":"tests/ui/format.rs","byte_start":602,"byte_end":617,"line_start":21,"line_end":21,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    format!(\"foo\");","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":"\"foo\".to_string()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:21:5\n   |\nLL |     format!(\"foo\");\n   |     ^^^^^^^^^^^^^^^ help: consider using .to_string(): `\"foo\".to_string()`\n   |\n   = note: `-D clippy::useless-format` implied by `-D warnings`\n\n"}
[01:29:39] {"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":775,"byte_end":798,"line_start":27,"line_end":27,"column_start":5,"column_end":28,"is_primary":true,"text":[{"text":"    format!(\"{:+}\", \"foo\"); // warn when the format makes no difference","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using .to_string()","code":null,"level":"help","spans":[{"file_name":"<::alloc::macros::format macros>","byte_start":55,"byte_end":86,"line_start":2,"line_end":2,"column_start":28,"column_end":59,"is_primary":true,"text":[{"text":"$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":28,"highlight_end":59}],"label":null,"suggested_replacement":"\"foo\".to_string()","suggestion_applicability":"MachineApplicable","expansion":{"span":{"file_name":"<::alloc::macros::format macros>","byte_start":55,"byte_end":86,"line_start":2,"line_end":2,"column_start":28,"column_end":59,"is_primary":false,"text":[{"text":"$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":28,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/format.rs","byte_start":775,"byte_end":798,"line_start":27,"line_end":27,"column_start":5,"column_end":28,"is_primary":false,"text":[{"text":"    format!(\"{:+}\", \"foo\"); // warn when the format makes no difference","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"format!","def_site_span":{"file_name":"<::alloc::macros::format macros>","byte_start":0,"byte_end":90,"line_start":1,"line_end":2,"column_start":1,"column_end":63,"is_primary":false,"text":[{"text":"( $ ( $ arg : tt ) * ) => (","highlight_start":1,"highlight_end":28},{"text":"$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":1,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"format_args!","def_site_span":null}}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:27:5\n   |\nLL |     format!(\"{:+}\", \"foo\"); // warn when the format makes no difference\n   |     ^^^^^^^^^^^^^^^^^^^^^^^ help: consider using .to_string(): `\"foo\".to_string()`\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:29:39] {"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":847,"byte_end":870,"line_start":28,"line_end":28,"column_start":5,"column_end":28,"is_primary":true,"text":[{"text":"    format!(\"{:<}\", \"foo\"); // warn when the format makes no difference","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using .to_string()","code":null,"level":"help","spans":[{"file_name":"<::alloc::macros::format macros>","byte_start":55,"byte_end":86,"line_start":2,"line_end":2,"column_start":28,"column_end":59,"is_primary":true,"text":[{"text":"$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":28,"highlight_end":59}],"label":null,"suggested_replacement":"\"foo\".to_string()","suggestion_applicability":"MachineApplicable","expansion":{"span":{"file_name":"<::alloc::macros::format macros>","byte_start":55,"byte_end":86,"line_start":2,"line_end":2,"column_start":28,"column_end":59,"is_primary":false,"text":[{"text":"$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":28,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/format.rs","byte_start":847,"byte_end":870,"line_start":28,"line_end":28,"column_start":5,"column_end":28,"is_primary":false,"text":[{"text":"    format!(\"{:<}\", \"foo\"); // warn when the format makes no difference","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"format!","def_site_span":{"file_name":"<::alloc::macros::format macros>","byte_start":0,"byte_end":90,"line_start":1,"line_end":2,"column_start":1,"column_end":63,"is_primary":false,"text":[{"text":"( $ ( $ arg : tt ) * ) => (","highlight_start":1,"highlight_end":28},{"text":"$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":1,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"format_args!","def_site_span":null}}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:28:5\n   |\nLL |     format!(\"{:<}\", \"foo\"); // warn when the format makes no difference\n   |     ^^^^^^^^^^^^^^^^^^^^^^^ help: consider using .to_string(): `\"foo\".to_string()`\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:29:39] {"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":1161,"byte_end":1182,"line_start":37,"line_end":37,"column_start":5,"column_end":26,"is_primary":true,"text":[{"text":"    format!(\"{:+}\", arg); // warn when the format makes no difference","highlight_start":5,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using .to_string()","code":null,"level":"help","spans":[{"file_name":"<::alloc::macros::format macros>","byte_start":55,"byte_end":86,"line_start":2,"line_end":2,"column_start":28,"column_end":59,"is_primary":true,"text":[{"text":"$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":28,"highlight_end":59}],"label":null,"suggested_replacement":"arg.to_string()","suggestion_applicability":"MachineApplicable","expansion":{"span":{"file_name":"<::alloc::macros::format macros>","byte_start":55,"byte_end":86,"line_start":2,"line_end":2,"column_start":28,"column_end":59,"is_primary":false,"text":[{"text":"$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":28,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/format.rs","byte_start":1161,"byte_end":1182,"line_start":37,"line_end":37,"column_start":5,"column_end":26,"is_primary":false,"text":[{"text":"    format!(\"{:+}\", arg); // warn when the format makes no difference","highlight_start":5,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"format!","def_site_span":{"file_name":"<::alloc::macros::format macros>","byte_start":0,"byte_end":90,"line_start":1,"line_end":2,"column_start":1,"column_end":63,"is_primary":false,"text":[{"text":"( $ ( $ arg : tt ) * ) => (","highlight_start":1,"highlight_end":28},{"text":"$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":1,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"format_args!","def_site_span":null}}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:37:5\n   |\nLL |     format!(\"{:+}\", arg); // warn when the format makes no difference\n   |     ^^^^^^^^^^^^^^^^^^^^^ help: consider using .to_string(): `arg.to_string()`\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:29:39] {"message":"useless use of `format!`","code":{"code":"clippy::useless_format","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/format.rs","byte_start":1231,"byte_end":1252,"line_start":38,"line_end":38,"column_start":5,"column_end":26,"is_primary":true,"text":[{"text":"    format!(\"{:<}\", arg); // warn when the format makes no difference","highlight_start":5,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider using .to_string()","code":null,"level":"help","spans":[{"file_name":"<::alloc::macros::format macros>","byte_start":55,"byte_end":86,"line_start":2,"line_end":2,"column_start":28,"column_end":59,"is_primary":true,"text":[{"text":"$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":28,"highlight_end":59}],"label":null,"suggested_replacement":"arg.to_string()","suggestion_applicability":"MachineApplicable","expansion":{"span":{"file_name":"<::alloc::macros::format macros>","byte_start":55,"byte_end":86,"line_start":2,"line_end":2,"column_start":28,"column_end":59,"is_primary":false,"text":[{"text":"$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":28,"highlight_end":59}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/format.rs","byte_start":1231,"byte_end":1252,"line_start":38,"line_end":38,"column_start":5,"column_end":26,"is_primary":false,"text":[{"text":"    format!(\"{:<}\", arg); // warn when the format makes no difference","highlight_start":5,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"format!","def_site_span":{"file_name":"<::alloc::macros::format macros>","byte_start":0,"byte_end":90,"line_start":1,"line_end":2,"column_start":1,"column_end":63,"is_primary":false,"text":[{"text":"( $ ( $ arg : tt ) * ) => (","highlight_start":1,"highlight_end":28},{"text":"$ crate :: fmt :: format ( format_args ! ( $ ( $ arg ) * ) ) )","highlight_start":1,"highlight_end":63}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"format_args!","def_site_span":null}}],"children":[],"rendered":null}],"rendered":"error: useless use of `format!`\n  --> tests/ui/format.rs:38:5\n   |\nLL |     format!(\"{:<}\", arg); // warn when the format makes no difference\n   |     ^^^^^^^^^^^^^^^^^^^^^ help: consider using .to_string(): `arg.to_string()`\n   |\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:29:39] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
[01:29:39] ------------------------------------------
[01:29:39] 
[01:29:39] thread '[ui] ui/format.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.18/src/runtest.rs:2632:9
[01:29:39] 
---
travis_time:end:06cdfdc4:start=1547551071792096548,finish=1547551071798712657,duration=6616109
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03a192b9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0179fb92
travis_time:start:0179fb92
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:008ff56b
$ dmesg | grep -i kill
