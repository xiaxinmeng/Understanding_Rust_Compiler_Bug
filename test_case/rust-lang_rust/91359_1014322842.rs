plain
test client_init_with_configuration_camel_case ... ok
test client_infer_lib ... ok
test client_init_with_configuration_mixed_case ... ok
test client_init_with_configuration_snake_case ... ok
{"artifact":"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t6/dependency_typo/target/rls/debug/deps/autocfg-a7f645135509dc5c.d","emit":"dep-info"}
test client_ignore_uninitialized_notification ... ok
test client_reformat ... ok
test client_multiple_binaries ... ok
test client_reformat_with_range ... ok
---

---- compile_test stdout ----
diff of stderr:

 error: using `to_string` in `fmt::Display` implementation might lead to infinite recursion
   --> $DIR/to_string_in_display.rs:25:25
 LL |         write!(f, "{}", self.to_string())
    |                         ^^^^^^^^^^^^^^^^
    |
    |
    = note: `-D clippy::to-string-in-display` implied by `-D warnings`
-error: aborting due to previous error
+error: unnecessary use of `to_string`
+  --> $DIR/to_string_in_display.rs:55:50
+   |
+   |
+LL |             Self::E(string) => write!(f, "E {}", string.to_string()),
+   |
+   |
+   = note: `-D clippy::unnecessary-to-owned` implied by `-D warnings`
+   = note: this error originates in the macro `$crate::format_args` (in Nightly builds, run with -Z macro-backtrace for more info)
+error: aborting due to 2 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/to_string_in_display.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args to_string_in_display.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/to_string_in_display.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/to_string_in_display.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-84536a848ae0c873.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-02ad006c43a59c6d.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-03199b31ca772f2d.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-626b6ae97a618a5f.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/to_string_in_display.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"using `to_string` in `fmt::Display` implementation might lead to infinite recursion","code":{"code":"clippy::to_string_in_display","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/to_string_in_display.rs","byte_start":429,"byte_end":445,"line_start":25,"line_end":25,"column_start":25,"column_end":41,"is_primary":true,"text":[{"text":"        write!(f, \"{}\", self.to_string())","highlight_start":25,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::to-string-in-display` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: using `to_string` in `fmt::Display` implementation might lead to infinite recursion\n  --> tests/ui/to_string_in_display.rs:25:25\n   |\nLL |         write!(f, \"{}\", self.to_string())\n   |                         ^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::to-string-in-display` implied by `-D warnings`\n\n"}
{"message":"unnecessary use of `to_string`","code":{"code":"clippy::unnecessary_to_owned","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/to_string_in_display.rs","byte_start":934,"byte_end":952,"line_start":55,"line_end":55,"column_start":50,"column_end":68,"is_primary":true,"text":[{"text":"            Self::E(string) => write!(f, \"E {}\", string.to_string()),","highlight_start":50,"highlight_end":68}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":17446,"byte_end":17476,"line_start":485,"line_end":485,"column_start":49,"column_end":79,"is_primary":false,"text":[{"text":"    ($dst:expr, $($arg:tt)*) => ($dst.write_fmt($crate::format_args!($($arg)*)))","highlight_start":49,"highlight_end":79}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/to_string_in_display.rs","byte_start":916,"byte_end":953,"line_start":55,"line_end":55,"column_start":32,"column_end":69,"is_primary":false,"text":[{"text":"            Self::E(string) => write!(f, \"E {}\", string.to_string()),","highlight_start":32,"highlight_end":69}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"write!","def_site_span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":17377,"byte_end":17480,"line_start":484,"line_end":486,"column_start":1,"column_end":2,"is_primary":false,"text":[{"text":"macro_rules! write {","highlight_start":1,"highlight_end":1},{"text":"    ($dst:expr, $($arg:tt)*) => ($dst.write_fmt($crate::format_args!($($arg)*)))","highlight_start":1,"highlight_end":1},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"$crate::format_args!","def_site_span":{"file_name":"/checkout/library/core/src/macros/mod.rs","byte_start":29988,"byte_end":30142,"line_start":854,"line_end":857,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    macro_rules! format_args {","highlight_start":5,"highlight_end":31},{"text":"        ($fmt:expr) => {{ /* compiler built-in */ }};","highlight_start":1,"highlight_end":54},{"text":"        ($fmt:expr, $($args:tt)*) => {{ /* compiler built-in */ }};","highlight_start":1,"highlight_end":68},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::unnecessary-to-owned` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: unnecessary use of `to_string`\n  --> tests/ui/to_string_in_display.rs:55:50\n   |\nLL |             Self::E(string) => write!(f, \"E {}\", string.to_string()),\n   |                                                  ^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::unnecessary-to-owned` implied by `-D warnings`\n   = note: this error originates in the macro `$crate::format_args` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/lib.rs:105:22
