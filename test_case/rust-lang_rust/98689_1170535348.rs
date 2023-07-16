plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between ddcbba036aee08f0709f98a92a342a278eae5c05 and 5c340d4438b32ce6504244419fbd51e5776f3ffa
Rustdoc was updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

 error: sub-expression diverges
   --> $DIR/diverging_sub_expression.rs:19:10
    |
 LL |     b || diverge();
    |
    |
    = note: `-D clippy::diverging-sub-expression` implied by `-D warnings`
 error: sub-expression diverges
   --> $DIR/diverging_sub_expression.rs:20:10
    |
    |
 LL |     b || A.foo();
 
 error: sub-expression diverges
   --> $DIR/diverging_sub_expression.rs:29:26
    |
---
 
 error: sub-expression diverges
   --> $DIR/diverging_sub_expression.rs:33:26
    |
 LL |             3 => true || diverge(),
 
 error: sub-expression diverges
   --> $DIR/diverging_sub_expression.rs:36:30
    |
    |
 LL |                 _ => true || panic!("boo"),
    |
-   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
-   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
+   = note: this error originates in the macro `panic` which expands to macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
 error: sub-expression diverges
   --> $DIR/diverging_sub_expression.rs:38:26
    |
 LL |             _ => true || break,
---
To only update this specific test, also pass `--test-args diverging_sub_expression.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/diverging_sub_expression.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/diverging_sub_expression.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-6479f1c58bb283b7.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-8f6f6ff006a184c3.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-71205fa4273edf27.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-528e210b0f79da7e.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bb39617b33963acb.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-5411643be5ff72c2.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-76e15f312bef456e.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c495ccdc5de2578f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3cd37cddf9b6fc4a.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/diverging_sub_expression.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/diverging_sub_expression.rs","byte_start":363,"byte_end":372,"line_start":19,"line_end":19,"column_start":10,"column_end":19,"is_primary":true,"text":[{"text":"    b || diverge();","highlight_start":10,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::diverging-sub-expression` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: sub-expression diverges\n  --> tests/ui/diverging_sub_expression.rs:19:10\n   |\nLL |     b || diverge();\n   |          ^^^^^^^^^\n   |\n   = note: `-D clippy::diverging-sub-expression` implied by `-D warnings`\n\n"}
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/diverging_sub_expression.rs","byte_start":383,"byte_end":390,"line_start":20,"line_end":20,"column_start":10,"column_end":17,"is_primary":true,"text":[{"text":"    b || A.foo();","highlight_start":10,"highlight_end":17}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: sub-expression diverges\n  --> tests/ui/diverging_sub_expression.rs:20:10\n   |\nLL |     b || A.foo();\n   |          ^^^^^^^\n\n"}
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/diverging_sub_expression.rs","byte_start":561,"byte_end":567,"line_start":29,"line_end":29,"column_start":26,"column_end":32,"is_primary":true,"text":[{"text":"            6 => true || return,","highlight_start":26,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: sub-expression diverges\n  --> tests/ui/diverging_sub_expression.rs:29:26\n   |\nLL |             6 => true || return,\n   |                          ^^^^^^\n\n"}
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/diverging_sub_expression.rs","byte_start":594,"byte_end":602,"line_start":30,"line_end":30,"column_start":26,"column_end":34,"is_primary":true,"text":[{"text":"            7 => true || continue,","highlight_start":26,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: sub-expression diverges\n  --> tests/ui/diverging_sub_expression.rs:30:26\n   |\nLL |             7 => true || continue,\n   |                          ^^^^^^^^\n\n"}
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/diverging_sub_expression.rs","byte_start":681,"byte_end":690,"line_start":33,"line_end":33,"column_start":26,"column_end":35,"is_primary":true,"text":[{"text":"            3 => true || diverge(),","highlight_start":26,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: sub-expression diverges\n  --> tests/ui/diverging_sub_expression.rs:33:26\n   |\nLL |             3 => true || diverge(),\n   |                          ^^^^^^^^^\n\n"}
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"/checkout/library/core/src/panic.rs","byte_start":1873,"byte_end":1937,"line_start":57,"line_end":57,"column_start":9,"column_end":73,"is_primary":true,"text":[{"text":"        $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))","highlight_start":9,"highlight_end":73}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/diverging_sub_expression.rs","byte_start":780,"byte_end":793,"line_start":36,"line_end":36,"column_start":30,"column_end":43,"is_primary":false,"text":[{"text":"                _ => true || panic!(\"boo\"),","highlight_start":30,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/diverging_sub_expression.rs","byte_start":780,"byte_end":793,"line_start":36,"line_end":36,"column_start":30,"column_end":43,"is_primary":false,"text":[{"text":"                _ => true || panic!(\"boo\"),","highlight_start":30,"highlight_end":43}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"panic!","def_site_span":{"file_name":"/checkout/library/std/src/macros.rs","byte_start":463,"byte_end":481,"line_start":13,"line_end":13,"column_start":1,"column_end":19,"is_primary":false,"text":[{"text":"macro_rules! panic {","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"$crate::panic::panic_2021!","def_site_span":{"file_name":"/checkout/library/core/src/panic.rs","byte_start":1601,"byte_end":1621,"line_start":48,"line_end":48,"column_start":1,"column_end":21,"is_primary":false,"text":[{"text":"pub macro panic_2021 {","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: sub-expression diverges\n  --> tests/ui/diverging_sub_expression.rs:36:30\n   |\nLL |                 _ => true || panic!(\"boo\"),\n   |                              ^^^^^^^^^^^^^\n   |\n   = note: this error originates in the macro `panic` which expands to macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
error: test failed, to rerun pass '--test compile-test'
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/diverging_sub_expression.rs","byte_start":835,"byte_end":840,"line_start":38,"line_end":38,"column_start":26,"column_end":31,"is_primary":true,"text":[{"text":"            _ => true || break,","highlight_start":26,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: sub-expression diverges\n  --> tests/ui/diverging_sub_expression.rs:38:26\n   |\nLL |             _ => true || break,\n   |                          ^^^^^\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: consider implementing `TryFrom` instead
    |
    |
 LL | / impl From<String> for Foo {
 LL | |     fn from(s: String) -> Self {
 LL | |         Foo(s.parse().unwrap())
 LL | |     }
 LL | | }
    |
 note: the lint level is defined here
   --> $DIR/fallible_impl_from.rs:1:9
    |
    |
 LL | #![deny(clippy::fallible_impl_from)]
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^
    = help: `From` is intended for infallible conversions only. Use `TryFrom` if there's a possibility for the conversion to fail
 note: potential failure(s)
    |
    |
 LL |         Foo(s.parse().unwrap())
 
 
 error: consider implementing `TryFrom` instead
    |
    |
 LL | / impl From<usize> for Invalid {
 LL | |     fn from(i: usize) -> Invalid {
 LL | |         if i != 42 {
 LL | |             panic!();
 LL | |     }
 LL | | }
    | |_^
    |
    |
    = help: `From` is intended for infallible conversions only. Use `TryFrom` if there's a possibility for the conversion to fail
 note: potential failure(s)
    |
 LL |             panic!();
    |             ^^^^^^^^
-   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
-   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
+   = note: this error originates in the macro `panic` which expands to macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
 
 error: consider implementing `TryFrom` instead
    |
    |
 LL | / impl From<Option<String>> for Invalid {
 LL | |     fn from(s: Option<String>) -> Invalid {
 LL | |         let s = s.unwrap();
 LL | |         if !s.is_empty() {
 LL | |     }
 LL | | }
    | |_^
    |
    |
    = help: `From` is intended for infallible conversions only. Use `TryFrom` if there's a possibility for the conversion to fail
 note: potential failure(s)
    |
 LL |         let s = s.unwrap();
    |                 ^^^^^^^^^^
    |                 ^^^^^^^^^^
 LL |         if !s.is_empty() {
 LL |             panic!("42");
    |             ^^^^^^^^^^^^
 LL |         } else if s.parse::<u32>().unwrap() != 42 {
 LL |             panic!("{:?}", s);
    |             ^^^^^^^^^^^^^^^^^
-   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
-   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
+   = note: this error originates in the macro `panic` which eventually expands to macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
 
 error: consider implementing `TryFrom` instead
    |
    |
 LL | / impl<'a> From<&'a mut <Box<u32> as ProjStrTrait>::ProjString> for Invalid {
 LL | |     fn from(s: &'a mut <Box<u32> as ProjStrTrait>::ProjString) -> Invalid {
 LL | |         if s.parse::<u32>().ok().unwrap() != 42 {
 LL | |             panic!("{:?}", s);
 LL | |     }
 LL | | }
    | |_^
    |
    |
    = help: `From` is intended for infallible conversions only. Use `TryFrom` if there's a possibility for the conversion to fail
 note: potential failure(s)
    |
    |
 LL |         if s.parse::<u32>().ok().unwrap() != 42 {
 LL |             panic!("{:?}", s);
    |             ^^^^^^^^^^^^^^^^^
-   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
-   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
+   = note: this error originates in the macro `panic` which expands to macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
 error: aborting due to 4 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/fallible_impl_from.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fallible_impl_from.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/fallible_impl_from.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/fallible_impl_from.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-6479f1c58bb283b7.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-8f6f6ff006a184c3.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-71205fa4273edf27.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-528e210b0f79da7e.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bb39617b33963acb.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-5411643be5ff72c2.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-76e15f312bef456e.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c495ccdc5de2578f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3cd37cddf9b6fc4a.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/fallible_impl_from.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"consider implementing `TryFrom` instead","code":{"code":"clippy::fallible_impl_from","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/fallible_impl_from.rs","byte_start":71,"byte_end":171,"line_start":5,"line_end":9,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"impl From<String> for Foo {","highlight_start":1,"highlight_end":28},{"text":"    fn from(s: String) -> Self {","highlight_start":1,"highlight_end":33},{"text":"        Foo(s.parse().unwrap())","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/fallible_impl_from.rs","byte_start":8,"byte_end":34,"line_start":1,"line_end":1,"column_start":9,"column_end":35,"is_primary":true,"text":[{"text":"#![deny(clippy::fallible_impl_from)]","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"`From` is intended for infallible conversions only. Use `TryFrom` if there's a possibility for the conversion to fail","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"potential failure(s)","code":null,"level":"note","spans":[{"file_name":"tests/ui/fallible_impl_from.rs","byte_start":144,"byte_end":162,"line_start":7,"line_end":7,"column_start":13,"column_end":31,"is_primary":true,"text":[{"text":"        Foo(s.parse().unwrap())","highlight_start":13,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: consider implementing `TryFrom` instead\n  --> tests/ui/fallible_impl_from.rs:5:1\n   |\nLL | / impl From<String> for Foo {\nLL | |     fn from(s: String) -> Self {\nLL | |         Foo(s.parse().unwrap())\nLL | |     }\nLL | | }\n   | |_^\n   |\nnote: the lint level is defined here\n  --> tests/ui/fallible_impl_from.rs:1:9\n   |\nLL | #![deny(clippy::fallible_impl_from)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^\n   = help: `From` is intended for infallible conversions only. Use `TryFrom` if there's a possibility for the conversion to fail\nnote: potential failure(s)\n  --> tests/ui/fallible_impl_from.rs:7:13\n   |\nLL |         Foo(s.parse().unwrap())\n   |             ^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"consider implementing `TryFrom` instead","code":{"code":"clippy::fallible_impl_from","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/fallible_impl_from.rs","byte_start":441,"byte_end":583,"line_start":26,"line_end":33,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"impl From<usize> for Invalid {","highlight_start":1,"highlight_end":31},{"text":"    fn from(i: usize) -> Invalid {","highlight_start":1,"highlight_end":35},{"text":"        if i != 42 {","highlight_start":1,"highlight_end":21},{"text":"            panic!();","highlight_start":1,"highlight_end":22},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"        Invalid","highlight_start":1,"highlight_end":16},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`From` is intended for infallible conversions only. Use `TryFrom` if there's a possibility for the conversion to fail","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"potential failure(s)","code":null,"level":"note","spans":[{"file_name":"/checkout/library/core/src/panic.rs","byte_start":1644,"byte_end":1686,"line_start":50,"line_end":50,"column_start":9,"column_end":51,"is_primary":true,"text":[{"text":"        $crate::panicking::panic(\"explicit panic\")","highlight_start":9,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/fallible_impl_from.rs","byte_start":540,"byte_end":548,"line_start":29,"line_end":29,"column_start":13,"column_end":21,"is_primary":false,"text":[{"text":"            panic!();","highlight_start":13,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/fallible_impl_from.rs","byte_start":540,"byte_end":548,"line_start":29,"line_end":29,"column_start":13,"column_end":21,"is_primary":false,"text":[{"text":"            panic!();","highlight_start":13,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"panic!","def_site_span":{"file_name":"/checkout/library/std/src/macros.rs","byte_start":463,"byte_end":481,"line_start":13,"line_end":13,"column_start":1,"column_end":19,"is_primary":false,"text":[{"text":"macro_rules! panic {","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"$crate::panic::panic_2021!","def_site_span":{"file_name":"/checkout/library/core/src/panic.rs","byte_start":1601,"byte_end":1621,"line_start":48,"line_end":48,"column_start":1,"column_end":21,"is_primary":false,"text":[{"text":"pub macro panic_2021 {","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: consider implementing `TryFrom` instead\n  --> tests/ui/fallible_impl_from.rs:26:1\n   |\nLL | / impl From<usize> for Invalid {\nLL | |     fn from(i: usize) -> Invalid {\nLL | |         if i != 42 {\nLL | |             panic!();\n...  |\nLL | |     }\nLL | | }\n   | |_^\n   |\n   = help: `From` is intended for infallible conversions only. Use `TryFrom` if there's a possibility for the conversion to fail\nnote: potential failure(s)\n  --> tests/ui/fallible_impl_from.rs:29:13\n   |\nLL |             panic!();\n   |             ^^^^^^^^\n   = note: this error originates in the macro `panic` which expands to macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"consider implementing `TryFrom` instead","code":{"code":"clippy::fallible_impl_from","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/fallible_impl_from.rs","byte_start":585,"byte_end":866,"line_start":35,"line_end":45,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"impl From<Option<String>> for Invalid {","highlight_start":1,"highlight_end":40},{"text":"    fn from(s: Option<String>) -> Invalid {","highlight_start":1,"highlight_end":44},{"text":"        let s = s.unwrap();","highlight_start":1,"highlight_end":28},{"text":"        if !s.is_empty() {","highlight_start":1,"highlight_end":27},{"text":"            panic!(\"42\");","highlight_start":1,"highlight_end":26},{"text":"        } else if s.parse::<u32>().unwrap() != 42 {","highlight_start":1,"highlight_end":52},{"text":"            panic!(\"{:?}\", s);","highlight_start":1,"highlight_end":31},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"        Invalid","highlight_start":1,"highlight_end":16},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`From` is intended for infallible conversions only. Use `TryFrom` if there's a possibility for the conversion to fail","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"potential failure(s)","code":null,"level":"note","spans":[{"file_name":"tests/ui/fallible_impl_from.rs","byte_start":685,"byte_end":695,"line_start":37,"line_end":37,"column_start":17,"column_end":27,"is_primary":true,"text":[{"text":"        let s = s.unwrap();","highlight_start":17,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/fallible_impl_from.rs","byte_start":768,"byte_end":793,"line_start":40,"line_end":40,"column_start":19,"column_end":44,"is_primary":true,"text":[{"text":"        } else if s.parse::<u32>().unwrap() != 42 {","highlight_start":19,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/library/core/src/panic.rs","byte_start":1873,"byte_end":1937,"line_start":57,"line_end":57,"column_start":9,"column_end":73,"is_primary":true,"text":[{"text":"        $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))","highlight_start":9,"highlight_end":73}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/fallible_impl_from.rs","byte_start":736,"byte_end":748,"line_start":39,"line_end":39,"column_start":13,"column_end":25,"is_primary":false,"text":[{"text":"            panic!(\"42\");","highlight_start":13,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/fallible_impl_from.rs","byte_start":736,"byte_end":748,"line_start":39,"line_end":39,"column_start":13,"column_end":25,"is_primary":false,"text":[{"text":"            panic!(\"42\");","highlight_start":13,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"panic!","def_site_span":{"file_name":"/checkout/library/std/src/macros.rs","byte_start":463,"byte_end":481,"line_start":13,"line_end":13,"column_start":1,"column_end":19,"is_primary":false,"text":[{"text":"macro_rules! panic {","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"$crate::panic::panic_2021!","def_site_span":{"file_name":"/checkout/library/core/src/panic.rs","byte_start":1601,"byte_end":1621,"line_start":48,"line_end":48,"column_start":1,"column_end":21,"is_primary":false,"text":[{"text":"pub macro panic_2021 {","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},{"file_name":"/checkout/library/core/src/panic.rs","byte_start":1873,"byte_end":1937,"line_start":57,"line_end":57,"column_start":9,"column_end":73,"is_primary":true,"text":[{"text":"        $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))","highlight_start":9,"highlight_end":73}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/fallible_impl_from.rs","byte_start":814,"byte_end":831,"line_start":41,"line_end":41,"column_start":13,"column_end":30,"is_primary":false,"text":[{"text":"            panic!(\"{:?}\", s);","highlight_start":13,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/fallible_impl_from.rs","byte_start":814,"byte_end":831,"line_start":41,"line_end":41,"column_start":13,"column_end":30,"is_primary":false,"text":[{"text":"            panic!(\"{:?}\", s);","highlight_start":13,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"panic!","def_site_span":{"file_name":"/checkout/library/std/src/macros.rs","byte_start":463,"byte_end":481,"line_start":13,"line_end":13,"column_start":1,"column_end":19,"is_primary":false,"text":[{"text":"macro_rules! panic {","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"$crate::panic::panic_2021!","def_site_span":{"file_name":"/checkout/library/core/src/panic.rs","byte_start":1601,"byte_end":1621,"line_start":48,"line_end":48,"column_start":1,"column_end":21,"is_primary":false,"text":[{"text":"pub macro panic_2021 {","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: consider implementing `TryFrom` instead\n  --> tests/ui/fallible_impl_from.rs:35:1\n   |\nLL | / impl From<Option<String>> for Invalid {\nLL | |     fn from(s: Option<String>) -> Invalid {\nLL | |         let s = s.unwrap();\nLL | |         if !s.is_empty() {\n...  |\nLL | |     }\nLL | | }\n   | |_^\n   |\n   = help: `From` is intended for infallible conversions only. Use `TryFrom` if there's a possibility for the conversion to fail\nnote: potential failure(s)\n  --> tests/ui/fallible_impl_from.rs:37:17\n   |\nLL |         let s = s.unwrap();\n   |                 ^^^^^^^^^^\nLL |         if !s.is_empty() {\nLL |             panic!(\"42\");\n   |             ^^^^^^^^^^^^\nLL |         } else if s.parse::<u32>().unwrap() != 42 {\n   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^\nLL |             panic!(\"{:?}\", s);\n   |             ^^^^^^^^^^^^^^^^^\n   = note: this error originates in the macro `panic` which eventually expands to macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"consider implementing `TryFrom` instead","code":{"code":"clippy::fallible_impl_from","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/fallible_impl_from.rs","byte_start":978,"byte_end":1244,"line_start":53,"line_end":60,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"impl<'a> From<&'a mut <Box<u32> as ProjStrTrait>::ProjString> for Invalid {","highlight_start":1,"highlight_end":76},{"text":"    fn from(s: &'a mut <Box<u32> as ProjStrTrait>::ProjString) -> Invalid {","highlight_start":1,"highlight_end":76},{"text":"        if s.parse::<u32>().ok().unwrap() != 42 {","highlight_start":1,"highlight_end":50},{"text":"            panic!(\"{:?}\", s);","highlight_start":1,"highlight_end":31},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"        Invalid","highlight_start":1,"highlight_end":16},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`From` is intended for infallible conversions only. Use `TryFrom` if there's a possibility for the conversion to fail","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"potential failure(s)","code":null,"level":"note","spans":[{"file_name":"tests/ui/fallible_impl_from.rs","byte_start":1141,"byte_end":1171,"line_start":55,"line_end":55,"column_start":12,"column_end":42,"is_primary":true,"text":[{"text":"        if s.parse::<u32>().ok().unwrap() != 42 {","highlight_start":12,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/library/core/src/panic.rs","byte_start":1873,"byte_end":1937,"line_start":57,"line_end":57,"column_start":9,"column_end":73,"is_primary":true,"text":[{"text":"        $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))","highlight_start":9,"highlight_end":73}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/fallible_impl_from.rs","byte_start":1192,"byte_end":1209,"line_start":56,"line_end":56,"column_start":13,"column_end":30,"is_primary":false,"text":[{"text":"            panic!(\"{:?}\", s);","highlight_start":13,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/fallible_impl_from.rs","byte_start":1192,"byte_end":1209,"line_start":56,"line_end":56,"column_start":13,"column_end":30,"is_primary":false,"text":[{"text":"            panic!(\"{:?}\", s);","highlight_start":13,"highlight_end":30}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"panic!","def_site_span":{"file_name":"/checkout/library/std/src/macros.rs","byte_start":463,"byte_end":481,"line_start":13,"line_end":13,"column_start":1,"column_end":19,"is_primary":false,"text":[{"text":"macro_rules! panic {","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"$crate::panic::panic_2021!","def_site_span":{"file_name":"/checkout/library/core/src/panic.rs","byte_start":1601,"byte_end":1621,"line_start":48,"line_end":48,"column_start":1,"column_end":21,"is_primary":false,"text":[{"text":"pub macro panic_2021 {","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":null}],"rendered":"error: consider implementing `TryFrom` instead\n  --> tests/ui/fallible_impl_from.rs:53:1\n   |\nLL | / impl<'a> From<&'a mut <Box<u32> as ProjStrTrait>::ProjString> for Invalid {\nLL | |     fn from(s: &'a mut <Box<u32> as ProjStrTrait>::ProjString) -> Invalid {\nLL | |         if s.parse::<u32>().ok().unwrap() != 42 {\nLL | |             panic!(\"{:?}\", s);\n...  |\nLL | |     }\nLL | | }\n   | |_^\n   |\n   = help: `From` is intended for infallible conversions only. Use `TryFrom` if there's a possibility for the conversion to fail\nnote: potential failure(s)\n  --> tests/ui/fallible_impl_from.rs:55:12\n   |\nLL |         if s.parse::<u32>().ok().unwrap() != 42 {\n   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\nLL |             panic!(\"{:?}\", s);\n   |             ^^^^^^^^^^^^^^^^^\n   = note: this error originates in the macro `panic` which expands to macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: sub-expression diverges
   --> $DIR/issue-7447.rs:23:15
    |
 LL |     byte_view(panic!());
    |
    |
    = note: `-D clippy::diverging-sub-expression` implied by `-D warnings`
-   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
+   = note: this error originates in the macro `panic` which expands to macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
 error: sub-expression diverges
   --> $DIR/issue-7447.rs:24:19
    |
    |
 LL |     group_entries(panic!());
    |
-   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
-   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
+   = note: this error originates in the macro `panic` which expands to macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
 error: aborting due to 2 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/issue-7447.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issue-7447.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/issue-7447.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/issue-7447.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-6479f1c58bb283b7.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-8f6f6ff006a184c3.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-71205fa4273edf27.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-528e210b0f79da7e.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bb39617b33963acb.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-5411643be5ff72c2.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-76e15f312bef456e.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c495ccdc5de2578f.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3cd37cddf9b6fc4a.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/issue-7447.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"/checkout/library/core/src/panic.rs","byte_start":1644,"byte_end":1686,"line_start":50,"line_end":50,"column_start":9,"column_end":51,"is_primary":true,"text":[{"text":"        $crate::panicking::panic(\"explicit panic\")","highlight_start":9,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/issue-7447.rs","byte_start":436,"byte_end":444,"line_start":23,"line_end":23,"column_start":15,"column_end":23,"is_primary":false,"text":[{"text":"    byte_view(panic!());","highlight_start":15,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/issue-7447.rs","byte_start":436,"byte_end":444,"line_start":23,"line_end":23,"column_start":15,"column_end":23,"is_primary":false,"text":[{"text":"    byte_view(panic!());","highlight_start":15,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"panic!","def_site_span":{"file_name":"/checkout/library/std/src/macros.rs","byte_start":463,"byte_end":481,"line_start":13,"line_end":13,"column_start":1,"column_end":19,"is_primary":false,"text":[{"text":"macro_rules! panic {","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"$crate::panic::panic_2021!","def_site_span":{"file_name":"/checkout/library/core/src/panic.rs","byte_start":1601,"byte_end":1621,"line_start":48,"line_end":48,"column_start":1,"column_end":21,"is_primary":false,"text":[{"text":"pub macro panic_2021 {","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"`-D clippy::diverging-sub-expression` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: sub-expression diverges\n  --> tests/ui/issue-7447.rs:23:15\n   |\nLL |     byte_view(panic!());\n   |               ^^^^^^^^\n   |\n   = note: `-D clippy::diverging-sub-expression` implied by `-D warnings`\n   = note: this error originates in the macro `panic` which expands to macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}
{"message":"sub-expression diverges","code":{"code":"clippy::diverging_sub_expression","explanation":null},"level":"error","spans":[{"file_name":"/checkout/library/core/src/panic.rs","byte_start":1644,"byte_end":1686,"line_start":50,"line_end":50,"column_start":9,"column_end":51,"is_primary":true,"text":[{"text":"        $crate::panicking::panic(\"explicit panic\")","highlight_start":9,"highlight_end":51}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/issue-7447.rs","byte_start":465,"byte_end":473,"line_start":24,"line_end":24,"column_start":19,"column_end":27,"is_primary":false,"text":[{"text":"    group_entries(panic!());","highlight_start":19,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"tests/ui/issue-7447.rs","byte_start":465,"byte_end":473,"line_start":24,"line_end":24,"column_start":19,"column_end":27,"is_primary":false,"text":[{"text":"    group_entries(panic!());","highlight_start":19,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"panic!","def_site_span":{"file_name":"/checkout/library/std/src/macros.rs","byte_start":463,"byte_end":481,"line_start":13,"line_end":13,"column_start":1,"column_end":19,"is_primary":false,"text":[{"text":"macro_rules! panic {","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"$crate::panic::panic_2021!","def_site_span":{"file_name":"/checkout/library/core/src/panic.rs","byte_start":1601,"byte_end":1621,"line_start":48,"line_end":48,"column_start":1,"column_end":21,"is_primary":false,"text":[{"text":"pub macro panic_2021 {","highlight_start":1,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"error: sub-expression diverges\n  --> tests/ui/issue-7447.rs:24:19\n   |\nLL |     group_entries(panic!());\n   |                   ^^^^^^^^\n   |\n   = note: this error originates in the macro `panic` which expands to macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.8.0/src/lib.rs:111:22
