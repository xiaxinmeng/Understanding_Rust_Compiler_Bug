\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/conversion-methods.rs","byte_start":553,"byte_end":576,"line_start":15,"line_end":15,"column_start":41,"column_end":62,"is_primary":true,"text":[{"text":"    let _tis_an_instants_play: String = \"'Tis a fond Ambush—\"; //~ ERROR mismatched types","highlight_start":41,"highlight_end":62}],"label":"expected struct `std::string::String`, found reference","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"expected type `std::string::String`\n   found type `&'static str`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try using a conversion method","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/conversion-methods.rs","byte_start":553,"byte_end":576,"line_start":15,"line_end":15,"column_start":41,"column_end":62,"is_primary":true,"text":[{"text":"    let _tis_an_instants_play: String = \"'Tis a fond Ambush—\"; //~ ERROR mismatched types","highlight_start":41,"highlight_end":62}],"label":null,"suggested_replacement":"\"'Tis a fond Ambush—\".to_string()","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0308]: mismatched types\n  --> /checkout/src/test/ui/conversion-methods.rs:15:41\n   |\nLL |     let _tis_an_instants_play: String = \"'Tis a fond Ambush—\"; //~ ERROR mismatched types\n   |                                         ^^^^^^^^^^^^^^^^^^^^^\n   |                                         |\n   |                                         expected struct `std::string::String`, found reference\n   |                                         help: try using a conversion method: `\"'Tis a fond Ambush—\".to_string()`\n   |\n   = note: expected type `std::string::String`\n              found type `&'static str`\n\n"}
[00:53:36] {"message":"mismatched types","code":{"code":"E0308","explanation":"\nThis error occurs when the compiler was unable to infer the concrete type of a\nvariable. It can occur for several cases, the most common of winternal compiler error: unexpected panic
[00:53:36] note: the compiler unexpectedly panicked. this is a bug.
[00:53:36] 
[00:53:36] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:53:36] 
[00:53:36] 
[00:53:36] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:53:36] 
[00:53:36] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:53:36] 
[00:53:36] ------------------------------------------
[00:53:36] 
[00:53:36] thread '[ui] ui/did_you_mean/issue-42764.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] thread '[ui] ui/did_you_mean/issue-42764.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3205:9
[00:53:36] 
[00:53:36] ---- [ui] ui/did_you_mean/issue-53280-expected-float-found-integer-literal.rs stdout ----
[00:53:36] 
[00:53:36] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:53:36] status: exit code: 101
[00:53:36] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-53280-expected-float-found-integer-literal.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-53280-expected-float-found-integer-literal/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-53280-expected-float-found-integer-literal/auxiliary" "-A" "unused"
[00:53:36] stdout:
[00:53:36] ---------------------------------"-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/recursion_limit_deref/auxiliary" "-A" "unused"
[00:53:36] ------------------------------------------
[00:53:36] 
[00:53:36] ------------------------------------------
[00:53:36] stderr:
[00:53:36] stderr:
[00:53:36] ------------------------------------------
[00:53:36] {"message":"reached the recursion limit while auto-dereferencing I","code":{"code":"E0055","explanation":"\nDuring a method call, a value is automatically dereferenced as many times as\nneeded to make the value's type match the method's receiver. The catch is that\nthe compiler will only attempt to dereference a number of times up to the\nrecursion limit (which can be set via the `recursion_limit` attribute).\n\nFor a somewhat artificial example:\n\n