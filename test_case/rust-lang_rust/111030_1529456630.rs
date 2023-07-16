plain

---- compile_test stdout ----
diff of stderr:

 error: assert without any message
    |
    |
 LL |     assert!(foo());
    |
    |
    = help: consider describing why the failing assert is problematic
    = note: `-D clippy::missing-assert-message` implied by `-D warnings`
 
 error: assert without any message
    |
 LL |     assert_eq!(foo(), foo());
    |     ^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = help: consider describing why the failing assert is problematic
 
 
 error: assert without any message
    |
    |
 LL |     assert_ne!(foo(), foo());
    |
    |
    = help: consider describing why the failing assert is problematic
 
 error: assert without any message
    |
    |
 LL |     debug_assert!(foo());
    |
    |
    = help: consider describing why the failing assert is problematic
 
 error: assert without any message
    |
 LL |     debug_assert_eq!(foo(), foo());
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = help: consider describing why the failing assert is problematic
 
 error: assert without any message
    |
    |
 LL |     debug_assert_ne!(foo(), foo());
    |
    |
    = help: consider describing why the failing assert is problematic
 
 error: assert without any message
    |
    |
 LL |     assert!(bar!(true));
    |
    |
    = help: consider describing why the failing assert is problematic
 
 error: assert without any message
    |
    |
 LL |     assert!(bar!(true, false));
    |
    |
    = help: consider describing why the failing assert is problematic
 
 error: assert without any message
    |
    |
 LL |     assert_eq!(bar!(true), foo());
    |
    |
    = help: consider describing why the failing assert is problematic
 
 error: assert without any message
    |
    |
 LL |     assert_ne!(bar!(true, true), bar!(true));
    |
    |
    = help: consider describing why the failing assert is problematic
 
 error: assert without any message
    |
    |
 LL |     assert!(foo(),);
    |
    |
    = help: consider describing why the failing assert is problematic
 
 error: assert without any message
    |
 LL |     assert_eq!(foo(), foo(),);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = help: consider describing why the failing assert is problematic
 
 error: assert without any message
    |
    |
 LL |     assert_ne!(foo(), foo(),);
    |
    |
    = help: consider describing why the failing assert is problematic
 
 error: assert without any message
    |
    |
 LL |     debug_assert!(foo(),);
    |
    |
    = help: consider describing why the failing assert is problematic
 
 error: assert without any message
    |
 LL |     debug_assert_eq!(foo(), foo(),);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    |
    = help: consider describing why the failing assert is problematic
 
 error: assert without any message
    |
    |
 LL |     debug_assert_ne!(foo(), foo(),);
    |
    |
    = help: consider describing why the failing assert is problematic
-error: aborting due to 16 previous errors
-error: aborting due to 16 previous errors
+error: assert without any message
+   |
+   |
+LL |     assert_eq!(bar!(true), foo(), "msg");
+   |
+   |
+   = help: consider describing why the failing assert is problematic
+
+error: assert without any message
+   |
+   |
+LL |     assert_ne!(bar!(true, true), bar!(true), "msg");
+   |
+   |
+   = help: consider describing why the failing assert is problematic
+
+error: assert without any message
+   |
+   |
+LL |     assert_eq!(foo(), foo(), "msg");
+   |
+   |
+   = help: consider describing why the failing assert is problematic
+
+error: assert without any message
+   |
+   |
+LL |     assert_ne!(foo(), foo(), "msg");
+   |
+   |
+   = help: consider describing why the failing assert is problematic
+
+error: assert without any message
+   |
+   |
+LL |     debug_assert_eq!(foo(), foo(), "msg");
+   |
+   |
+   = help: consider describing why the failing assert is problematic
+
+error: assert without any message
+   |
+   |
+LL |     debug_assert_ne!(foo(), foo(), "msg");
+   |
+   |
+   = help: consider describing why the failing assert is problematic
+error: aborting due to 22 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/missing_assert_message.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args missing_assert_message.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/missing_assert_message.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/missing_assert_message.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-48d2da227b3f9b72.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-545a5b41a12a98e2.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/missing_assert_message.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"assert without any message","code":{"code":"clippy::missing_assert_message","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_assert_message.rs","byte_start":204,"byte_end":218,"line_start":14,"line_end":14,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    assert!(foo());","highlight_start":5,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider describing why the failing assert is problematic","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"`-D clippy::missing-assert-message` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: assert without any message\n  --> tests/ui/missing_assert_message.rs:14:5\n   |\nLL |     assert!(foo());\n   |     ^^^^^^^^^^^^^^\n   |\n   = help: consider describing why the failing assert is problematic\n   = note: `-D clippy::missing-assert-message` implied by `-D warnings`\n\n"}
{"message":"assert without any message","code":{"code":"clippy::missing_assert_message","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_assert_message.rs","byte_start":224,"byte_end":248,"line_start":15,"line_end":15,"column_start":5,"column_end":29,"is_primary":true,"text":[{"text":"    assert_eq!(foo(), foo());","highlight_start":5,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider describing why the failing assert is problematic","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: assert without any message\n  --> tests/ui/missing_assert_message.rs:15:5\n   |\nLL |     assert_eq!(foo(), foo());\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider describing why the failing assert is problematic\n\n"}
{"message":"assert without any message","code":{"code":"clippy::missing_assert_message","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_assert_message.rs","byte_start":254,"byte_end":278,"line_start":16,"line_end":16,"column_start":5,"column_end":29,"is_primary":true,"text":[{"text":"    assert_ne!(foo(), foo());","highlight_start":5,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider describing why the failing assert is problematic","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: assert without any message\n  --> tests/ui/missing_assert_message.rs:16:5\n   |\nLL |     assert_ne!(foo(), foo());\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider describing why the failing assert is problematic\n\n"}
{"message":"assert without any message","code":{"code":"clippy::missing_assert_message","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_assert_message.rs","byte_start":284,"byte_end":304,"line_start":17,"line_end":17,"column_start":5,"column_end":25,"is_primary":true,"text":[{"text":"    debug_assert!(foo());","highlight_start":5,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider describing why the failing assert is problematic","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: assert without any message\n  --> tests/ui/missing_assert_message.rs:17:5\n   |\nLL |     debug_assert!(foo());\n   |     ^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider describing why the failing assert is problematic\n\n"}
{"message":"assert without any message","code":{"code":"clippy::missing_assert_message","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_assert_message.rs","byte_start":310,"byte_end":340,"line_start":18,"line_end":18,"column_start":5,"column_end":35,"is_primary":true,"text":[{"text":"    debug_assert_eq!(foo(), foo());","highlight_start":5,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider describing why the failing assert is problematic","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: assert without any message\n  --> tests/ui/missing_assert_message.rs:18:5\n   |\nLL |     debug_assert_eq!(foo(), foo());\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider describing why the failing assert is problematic\n\n"}
{"message":"assert without any message","code":{"code":"clippy::missing_assert_message","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_assert_message.rs","byte_start":346,"byte_end":376,"line_start":19,"line_end":19,"column_start":5,"column_end":35,"is_primary":true,"text":[{"text":"    debug_assert_ne!(foo(), foo());","highlight_start":5,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider describing why the failing assert is problematic","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: assert without any message\n  --> tests/ui/missing_assert_message.rs:19:5\n   |\nLL |     debug_assert_ne!(foo(), foo());\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider describing why the failing assert is problematic\n\n"}
{"message":"assert without any message","code":{"code":"clippy::missing_assert_message","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_assert_message.rs","byte_start":463,"byte_end":482,"line_start":24,"line_end":24,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    assert!(bar!(true));","highlight_start":5,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider describing why the failing assert is problematic","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: assert without any message\n  --> tests/ui/missing_assert_message.rs:24:5\n   |\nLL |     assert!(bar!(true));\n   |     ^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider describing why the failing assert is problematic\n\n"}
{"message":"assert without any message","code":{"code":"clippy::missing_assert_message","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_assert_message.rs","byte_start":488,"byte_end":514,"line_start":25,"line_end":25,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"    assert!(bar!(true, false));","highlight_start":5,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider describing why the failing assert is problematic","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: assert without any message\n  --> tests/ui/missing_assert_message.rs:25:5\n   |\nLL |     assert!(bar!(true, false));\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider describing why the failing assert is problematic\n\n"}
{"message":"assert without any message","code":{"code":"clippy::missing_assert_message","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_assert_message.rs","byte_start":520,"byte_end":549,"line_start":26,"line_end":26,"column_start":5,"column_end":34,"is_primary":true,"text":[{"text":"    assert_eq!(bar!(true), foo());","highlight_start":5,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider describing why the failing assert is problematic","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: assert without any message\n  --> tests/ui/missing_assert_message.rs:26:5\n   |\nLL |     assert_eq!(bar!(true), foo());\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider describing why the failing assert is problematic\n\n"}
{"message":"assert without any message","code":{"code":"clippy::missing_assert_message","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_assert_message.rs","byte_start":555,"byte_end":595,"line_start":27,"line_end":27,"column_start":5,"column_end":45,"is_primary":true,"text":[{"text":"    assert_ne!(bar!(true, true), bar!(true));","highlight_start":5,"highlight_end":45}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider describing why the failing assert is problematic","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: assert without any message\n  --> tests/ui/missing_assert_message.rs:27:5\n   |\nLL |     assert_ne!(bar!(true, true), bar!(true));\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider describing why the failing assert is problematic\n\n"}
{"message":"assert without any message","code":{"code":"clippy::missing_assert_message","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_assert_message.rs","byte_start":666,"byte_end":681,"line_start":32,"line_end":32,"column_start":5,"column_end":20,"is_primary":true,"text":[{"text":"    assert!(foo(),);","highlight_start":5,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider describing why the failing assert is problematic","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: assert without any message\n  --> tests/ui/missing_assert_message.rs:32:5\n   |\nLL |     assert!(foo(),);\n   |     ^^^^^^^^^^^^^^^\n   |\n   = help: consider describing why the failing assert is problematic\n\n"}
{"message":"assert without any message","code":{"code":"clippy::missing_assert_message","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_assert_message.rs","byte_start":687,"byte_end":712,"line_start":33,"line_end":33,"column_start":5,"column_end":30,"is_primary":true,"text":[{"text":"    assert_eq!(foo(), foo(),);","highlight_start":5,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider describing why the failing assert is problematic","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: assert without any message\n  --> tests/ui/missing_assert_message.rs:33:5\n   |\nLL |     assert_eq!(foo(), foo(),);\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider describing why the failing assert is problematic\n\n"}
{"message":"assert without any message","code":{"code":"clippy::missing_assert_message","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_assert_message.rs","byte_start":718,"byte_end":743,"line_start":34,"line_end":34,"column_start":5,"column_end":30,"is_primary":true,"text":[{"text":"    assert_ne!(foo(), foo(),);","highlight_start":5,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider describing why the failing assert is problematic","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: assert without any message\n  --> tests/ui/missing_assert_message.rs:34:5\n   |\nLL |     assert_ne!(foo(), foo(),);\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider describing why the failing assert is problematic\n\n"}
{"message":"assert without any message","code":{"code":"clippy::missing_assert_message","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_assert_message.rs","byte_start":749,"byte_end":770,"line_start":35,"line_end":35,"column_start":5,"column_end":26,"is_primary":true,"text":[{"text":"    debug_assert!(foo(),);","highlight_start":5,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider describing why the failing assert is problematic","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: assert without any message\n  --> tests/ui/missing_assert_message.rs:35:5\n   |\nLL |     debug_assert!(foo(),);\n   |     ^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider describing why the failing assert is problematic\n\n"}
{"message":"assert without any message","code":{"code":"clippy::missing_assert_message","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_assert_message.rs","byte_start":776,"byte_end":807,"line_start":36,"line_end":36,"column_start":5,"column_end":36,"is_primary":true,"text":[{"text":"    debug_assert_eq!(foo(), foo(),);","highlight_start":5,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider describing why the failing assert is problematic","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: assert without any message\n  --> tests/ui/missing_assert_message.rs:36:5\n   |\nLL |     debug_assert_eq!(foo(), foo(),);\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider describing why the failing assert is problematic\n\n"}
{"message":"assert without any message","code":{"code":"clippy::missing_assert_message","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_assert_message.rs","byte_start":813,"byte_end":844,"line_start":37,"line_end":37,"column_start":5,"column_end":36,"is_primary":true,"text":[{"text":"    debug_assert_ne!(foo(), foo(),);","highlight_start":5,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider describing why the failing assert is problematic","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: assert without any message\n  --> tests/ui/missing_assert_message.rs:37:5\n   |\nLL |     debug_assert_ne!(foo(), foo(),);\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider describing why the failing assert is problematic\n\n"}
{"message":"assert without any message","code":{"code":"clippy::missing_assert_message","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_assert_message.rs","byte_start":1003,"byte_end":1039,"line_start":44,"line_end":44,"column_start":5,"column_end":41,"is_primary":true,"text":[{"text":"    assert_eq!(bar!(true), foo(), \"msg\");","highlight_start":5,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider describing why the failing assert is problematic","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: assert without any message\n  --> tests/ui/missing_assert_message.rs:44:5\n   |\nLL |     assert_eq!(bar!(true), foo(), \"msg\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider describing why the failing assert is problematic\n\n"}
{"message":"assert without any message","code":{"code":"clippy::missing_assert_message","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_assert_message.rs","byte_start":1045,"byte_end":1092,"line_start":45,"line_end":45,"column_start":5,"column_end":52,"is_primary":true,"text":[{"text":"    assert_ne!(bar!(true, true), bar!(true), \"msg\");","highlight_start":5,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider describing why the failing assert is problematic","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: assert without any message\n  --> tests/ui/missing_assert_message.rs:45:5\n   |\nLL |     assert_ne!(bar!(true, true), bar!(true), \"msg\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider describing why the failing assert is problematic\n\n"}
{"message":"assert without any message","code":{"code":"clippy::missing_assert_message","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_assert_message.rs","byte_start":1186,"byte_end":1217,"line_start":51,"line_end":51,"column_start":5,"column_end":36,"is_primary":true,"text":[{"text":"    assert_eq!(foo(), foo(), \"msg\");","highlight_start":5,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider describing why the failing assert is problematic","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: assert without any message\n  --> tests/ui/missing_assert_message.rs:51:5\n   |\nLL |     assert_eq!(foo(), foo(), \"msg\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider describing why the failing assert is problematic\n\n"}
{"message":"assert without any message","code":{"code":"clippy::missing_assert_message","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_assert_message.rs","byte_start":1223,"byte_end":1254,"line_start":52,"line_end":52,"column_start":5,"column_end":36,"is_primary":true,"text":[{"text":"    assert_ne!(foo(), foo(), \"msg\");","highlight_start":5,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider describing why the failing assert is problematic","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: assert without any message\n  --> tests/ui/missing_assert_message.rs:52:5\n   |\nLL |     assert_ne!(foo(), foo(), \"msg\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider describing why the failing assert is problematic\n\n"}
{"message":"assert without any message","code":{"code":"clippy::missing_assert_message","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_assert_message.rs","byte_start":1293,"byte_end":1330,"line_start":54,"line_end":54,"column_start":5,"column_end":42,"is_primary":true,"text":[{"text":"    debug_assert_eq!(foo(), foo(), \"msg\");","highlight_start":5,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider describing why the failing assert is problematic","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: assert without any message\n  --> tests/ui/missing_assert_message.rs:54:5\n   |\nLL |     debug_assert_eq!(foo(), foo(), \"msg\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider describing why the failing assert is problematic\n\n"}
{"message":"assert without any message","code":{"code":"clippy::missing_assert_message","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/missing_assert_message.rs","byte_start":1336,"byte_end":1373,"line_start":55,"line_end":55,"column_start":5,"column_end":42,"is_primary":true,"text":[{"text":"    debug_assert_ne!(foo(), foo(), \"msg\");","highlight_start":5,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider describing why the failing assert is problematic","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: assert without any message\n  --> tests/ui/missing_assert_message.rs:55:5\n   |\nLL |     debug_assert_ne!(foo(), foo(), \"msg\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: consider describing why the failing assert is problematic\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/index.crates.io-6f17d22bba15001f/compiletest_rs-0.10.0/src/lib.rs:111:22
