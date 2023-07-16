plain

-error: empty string literal in `println!`
-  --> $DIR/ice-10148.rs:8:5
-   |
-LL |     println!(with_span!(""something ""));
-   |                         |
-   |                         help: remove the empty string
-   |
error: test failed, to rerun pass `--test compile-test`
error: test failed, to rerun pass `--test compile-test`
-   = note: `-D clippy::println-empty-string` implied by `-D warnings`
-error: aborting due to previous error
-
-


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-10148.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args crashes/ice-10148.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/crashes/ice-10148.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-10148.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-545a5b41a12a98e2.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-48d2da227b3f9b72.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-10148.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------


error: failed to compile fixed code
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/explicit_write.fixed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/explicit_write.stage-id" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-545a5b41a12a98e2.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-48d2da227b3f9b72.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--edition=2021" "--crate-name=fixed" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/explicit_write.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"expected expression, found `$`","code":null,"level":"error","spans":[{"file_name":"tests/ui/explicit_write.fixed","byte_start":402,"byte_end":403,"line_start":26,"line_end":26,"column_start":26,"column_end":27,"is_primary":true,"text":[{"text":"        println!(\"{}\\n\", $crate::format_args!($($arg)*));","highlight_start":26,"highlight_end":27}],"label":"expected expression","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected expression, found `$`\n  --> tests/ui/explicit_write.fixed:26:26\n   |\nLL |         println!(\"{}\\n\", $crate::format_args!($($arg)*));\n   |                          ^ expected expression\n\n"}
{"message":"expected expression, found `$`","code":null,"level":"error","spans":[{"file_name":"tests/ui/explicit_write.fixed","byte_start":461,"byte_end":462,"line_start":27,"line_end":27,"column_start":27,"column_end":28,"is_primary":true,"text":[{"text":"        eprintln!(\"{}\\n\", $crate::format_args!($($arg)*));","highlight_start":27,"highlight_end":28}],"label":"expected expression","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected expression, found `$`\n  --> tests/ui/explicit_write.fixed:27:27\n   |\nLL |         eprintln!(\"{}\\n\", $crate::format_args!($($arg)*));\n   |                           ^ expected expression\n\n"}
{"message":"expected expression, found `$`","code":null,"level":"error","spans":[{"file_name":"tests/ui/explicit_write.fixed","byte_start":599,"byte_end":600,"line_start":32,"line_end":32,"column_start":26,"column_end":27,"is_primary":true,"text":[{"text":"        println!(\"{}\\n\", $crate::format_args!($($arg)*));","highlight_start":26,"highlight_end":27}],"label":"expected expression","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected expression, found `$`\n  --> tests/ui/explicit_write.fixed:32:26\n   |\nLL |         println!(\"{}\\n\", $crate::format_args!($($arg)*));\n   |                          ^ expected expression\n\n"}
{"message":"expected expression, found `$`","code":null,"level":"error","spans":[{"file_name":"tests/ui/explicit_write.fixed","byte_start":658,"byte_end":659,"line_start":33,"line_end":33,"column_start":27,"column_end":28,"is_primary":true,"text":[{"text":"        eprintln!(\"{}\\n\", $crate::format_args!($($arg)*));","highlight_start":27,"highlight_end":28}],"label":"expected expression","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected expression, found `$`\n  --> tests/ui/explicit_write.fixed:33:27\n   |\nLL |         eprintln!(\"{}\\n\", $crate::format_args!($($arg)*));\n   |                           ^ expected expression\n\n"}
{"message":"expected expression, found `$`","code":null,"level":"error","spans":[{"file_name":"tests/ui/explicit_write.fixed","byte_start":741,"byte_end":742,"line_start":36,"line_end":36,"column_start":27,"column_end":28,"is_primary":true,"text":[{"text":"        eprintln!(\"{}\\n\", $crate::format_args!($($arg)*));","highlight_start":27,"highlight_end":28}],"label":"expected expression","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected expression, found `$`\n  --> tests/ui/explicit_write.fixed:36:27\n   |\nLL |         eprintln!(\"{}\\n\", $crate::format_args!($($arg)*));\n   |                           ^ expected expression\n\n"}
{"message":"expected expression, found `$`","code":null,"level":"error","spans":[{"file_name":"tests/ui/explicit_write.fixed","byte_start":800,"byte_end":801,"line_start":37,"line_end":37,"column_start":27,"column_end":28,"is_primary":true,"text":[{"text":"        eprintln!(\"{}\\n\", $crate::format_args!($($arg)*));","highlight_start":27,"highlight_end":28}],"label":"expected expression","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected expression, found `$`\n  --> tests/ui/explicit_write.fixed:37:27\n   |\nLL |         eprintln!(\"{}\\n\", $crate::format_args!($($arg)*));\n   |                           ^ expected expression\n\n"}
{"message":"expected expression, found `$`","code":null,"level":"error","spans":[{"file_name":"tests/ui/explicit_write.fixed","byte_start":859,"byte_end":860,"line_start":38,"line_end":38,"column_start":27,"column_end":28,"is_primary":true,"text":[{"text":"        eprintln!(\"{}\\n\", $crate::format_args!($($arg)*));","highlight_start":27,"highlight_end":28}],"label":"expected expression","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected expression, found `$`\n  --> tests/ui/explicit_write.fixed:38:27\n   |\nLL |         eprintln!(\"{}\\n\", $crate::format_args!($($arg)*));\n   |                           ^ expected expression\n\n"}
{"message":"expected expression, found `$`","code":null,"level":"error","spans":[{"file_name":"tests/ui/explicit_write.fixed","byte_start":918,"byte_end":919,"line_start":39,"line_end":39,"column_start":27,"column_end":28,"is_primary":true,"text":[{"text":"        eprintln!(\"{}\\n\", $crate::format_args!($($arg)*));","highlight_start":27,"highlight_end":28}],"label":"expected expression","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected expression, found `$`\n  --> tests/ui/explicit_write.fixed:39:27\n   |\nLL |         eprintln!(\"{}\\n\", $crate::format_args!($($arg)*));\n   |                           ^ expected expression\n\n"}
{"message":"expected expression, found `$`","code":null,"level":"error","spans":[{"file_name":"tests/ui/explicit_write.fixed","byte_start":1000,"byte_end":1001,"line_start":41,"line_end":41,"column_start":27,"column_end":28,"is_primary":true,"text":[{"text":"        eprintln!(\"{}\\n\", $crate::format_args!($($arg)*));","highlight_start":27,"highlight_end":28}],"label":"expected expression","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: expected expression, found `$`\n  --> tests/ui/explicit_write.fixed:41:27\n   |\nLL |         eprintln!(\"{}\\n\", $crate::format_args!($($arg)*));\n   |                           ^ expected expression\n\n"}
{"message":"unused macro definition: `one`","code":{"code":"unused_macros","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/explicit_write.fixed","byte_start":219,"byte_end":222,"line_start":14,"line_end":14,"column_start":14,"column_end":17,"is_primary":true,"text":[{"text":"macro_rules! one {","highlight_start":14,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D unused-macros` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: unused macro definition: `one`\n  --> tests/ui/explicit_write.fixed:14:14\n   |\nLL | macro_rules! one {\n   |              ^^^\n   |\n   = note: `-D unused-macros` implied by `-D warnings`\n\n"}

------------------------------------------

diff of stderr:
---
-   |     ^^^^^^^^^--^
-   |              |
-   |              help: remove the empty string
-   |
-   = note: `-D clippy::println-empty-string` implied by `-D warnings`
-error: empty string literal in `println!`
-  --> $DIR/println_empty_string.rs:9:14
-   |
-   |
-LL |         _ => println!(""),
-   |              ^^^^^^^^^--^
-   |                       help: remove the empty string
-
-
-error: empty string literal in `eprintln!`
-   |
-LL |     eprintln!("");
-   |     ^^^^^^^^^^--^
-   |               |
-   |               |
-   |               help: remove the empty string
-
-error: empty string literal in `eprintln!`
-   |
-   |
-LL |         _ => eprintln!(""),
-   |              ^^^^^^^^^^--^
-   |                        help: remove the empty string
-
-error: aborting due to 4 previous errors
-
---
To only update this specific test, also pass `--test-args println_empty_string.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/println_empty_string.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/println_empty_string.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-545a5b41a12a98e2.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-48d2da227b3f9b72.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/println_empty_string.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------

Build completed unsuccessfully in 0:01:40
diff of stderr:

-error: variables can be used directly in the `format!` string
-   |
-   |
-LL |     println!("val='{}'", var);
-   |
-   |
-   = note: `-D clippy::uninlined-format-args` implied by `-D warnings`
-   |
-   |
-LL -     println!("val='{}'", var);
-LL +     println!("val='{var}'");
-
-error: aborting due to previous error
-
-
---
To only update this specific test, also pass `--test-args uninlined_format_args_panic.rs`

error in revision `edition2018`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/uninlined_format_args_panic.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--cfg" "edition2018" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/uninlined_format_args_panic.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-545a5b41a12a98e2.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-48d2da227b3f9b72.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/uninlined_format_args_panic.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------

diff of stderr:

-error: format specifiers have no effect on `format_args!()`
-  --> $DIR/unused_format_specs_unfixable.rs:12:15
-   |
-LL |     println!("{:5}.", format_args!(""));
-   |
-   |
-   = note: `-D clippy::unused-format-specs` implied by `-D warnings`
-help: for the width to apply consider using `format!()`
-   |
-LL |     println!("{:5}.", format!(""));
-help: if the current behavior is intentional, remove the format specifiers
-   |
-   |
-LL -     println!("{:5}.", format_args!(""));
-LL +     println!("{}.", format_args!(""));
-
-
-error: format specifiers have no effect on `format_args!()`
-  --> $DIR/unused_format_specs_unfixable.rs:14:15
-   |
-LL |     println!("{:.3}", format_args!("abcde"));
-   |
-   |
-help: for the precision to apply consider using `format!()`
-   |
-LL |     println!("{:.3}", format!("abcde"));
-help: if the current behavior is intentional, remove the format specifiers
-   |
-   |
-LL -     println!("{:.3}", format_args!("abcde"));
-LL +     println!("{}", format_args!("abcde"));
-
-
-error: format specifiers have no effect on `format_args!()`
-  --> $DIR/unused_format_specs_unfixable.rs:16:15
-   |
-LL |     println!("{:5}.", format_args_from_macro!());
-   |
-   |
-   = help: for the width to apply consider using `format!()`
-help: if the current behavior is intentional, remove the format specifiers
-   |
-LL -     println!("{:5}.", format_args_from_macro!());
-LL +     println!("{}.", format_args_from_macro!());
-
-
-error: format specifiers have no effect on `format_args!()`
-  --> $DIR/unused_format_specs_unfixable.rs:19:15
-   |
-LL |     println!("{args:5}");
-   |
-   |
-   = help: for the width to apply consider using `format!()`
-help: if the current behavior is intentional, remove the format specifiers
-   |
-LL -     println!("{args:5}");
-LL +     println!("{args}");
-
-error: aborting due to 4 previous errors
-
-
---
To only update this specific test, also pass `--test-args unused_format_specs_unfixable.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/unused_format_specs_unfixable.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unused_format_specs_unfixable.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-545a5b41a12a98e2.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-48d2da227b3f9b72.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unused_format_specs_unfixable.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---

-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:9:23
-   |
-LL |     writeln!(v, "{}", "{hello}");
-   |
-   |
-   = note: `-D clippy::write-literal` implied by `-D warnings`
-   |
-   |
-LL -     writeln!(v, "{}", "{hello}");
-LL +     writeln!(v, "{{hello}}");
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:10:24
-   |
-   |
-LL |     writeln!(v, r"{}", r"{hello}");
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, r"{}", r"{hello}");
-LL +     writeln!(v, r"{{hello}}");
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:11:23
-   |
-   |
-LL |     writeln!(v, "{}", '/'');
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, "{}", '/'');
-LL +     writeln!(v, "'");
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:12:23
-   |
-   |
-LL |     writeln!(v, "{}", '"');
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, "{}", '"');
-LL +     writeln!(v, "/"");
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:13:24
-   |
-   |
-LL |     writeln!(v, r"{}", '"');
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:14:24
-   |
-   |
-LL |     writeln!(v, r"{}", '/'');
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, r"{}", '/'');
-LL +     writeln!(v, r"'");
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:18:9
-   |
-   |
-LL | /         "hello /
-LL | |         world!"
-   | |_______________^
-   |
-help: try this
-   |
-LL ~         "some hello /
-LL ~         world!"
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:25:9
-   |
-   |
-LL |         "1", "2", "3",
-   |         ^^^
-   |
-help: try this
-   |
-LL ~         "some 1/
-LL ~         {} / {}", "2", "3",
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:25:14
-   |
-   |
-LL |         "1", "2", "3",
-   |              ^^^
-   |
-help: try this
-   |
-LL ~         2 / {}",
-LL ~         "1", "3",
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:25:19
-   |
-   |
-LL |         "1", "2", "3",
-   |                   ^^^
-   |
-help: try this
-   |
-LL ~         {} / 3",
-LL ~         "1", "2",
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:27:23
-   |
-   |
-LL |     writeln!(v, "{}", "/");
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, "{}", "/");
-LL +     writeln!(v, "/");
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:28:24
-   |
-   |
-LL |     writeln!(v, r"{}", "/");
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, r"{}", "/");
-LL +     writeln!(v, r"/");
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:29:26
-   |
-   |
-LL |     writeln!(v, r#"{}"#, "/");
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, r#"{}"#, "/");
-LL +     writeln!(v, r#"/"#);
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:30:23
-   |
-   |
-LL |     writeln!(v, "{}", r"/");
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, "{}", r"/");
-LL +     writeln!(v, "/");
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:31:23
-   |
-   |
-LL |     writeln!(v, "{}", "/r");
-   |
-help: try this
-   |
-   |
-LL -     writeln!(v, "{}", "/r");
-LL +     writeln!(v, "/r");
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:32:28
-   |
-   |
-LL |     writeln!(v, r#"{}{}"#, '#', '"'); // hard mode
-
-error: literal with an empty format string
-  --> $DIR/write_literal_2.rs:32:33
-   |
-   |
-LL |     writeln!(v, r#"{}{}"#, '#', '"'); // hard mode
-
-error: aborting due to 17 previous errors
-
-
---
To only update this specific test, also pass `--test-args write_literal_2.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/write_literal_2.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/write_literal_2.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-545a5b41a12a98e2.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-48d2da227b3f9b72.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/write_literal_2.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
---

-error: empty string literal in `writeln!`
-  --> $DIR/writeln_empty_string.rs:11:5
-   |
-LL |     writeln!(v, "");
-   |               |
-   |               help: remove the empty string
-   |
-   |
-   = note: `-D clippy::writeln-empty-string` implied by `-D warnings`
-error: empty string literal in `writeln!`
-  --> $DIR/writeln_empty_string.rs:14:5
-   |
-   |
-LL |     writeln!(suggestion, "");
-   |                        |
-   |                        help: remove the empty string
-
-error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args writeln_empty_string.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/writeln_empty_string.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/writeln_empty_string.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-e22c295747291f5a.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-545a5b41a12a98e2.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-99ea93d45a2253f6.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-c7bacd82195bfecc.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-9936341359a757f2.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-37e0204bcdda2709.so" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-21e023f2887ebff8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-48d2da227b3f9b72.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/writeln_empty_string.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
