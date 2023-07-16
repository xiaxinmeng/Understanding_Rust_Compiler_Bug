plain
........................................................................................ 10560/13475
........................................................................................ 10648/13475
............................................iiiii....i...i.i............................ 10736/13475
........................................................................................ 10824/13475
.........F.F.i.......................................................................... 10912/13475
........................................................................................ 11088/13475
........................................................................................ 11176/13475
........................................................................................ 11264/13475
........................................................................................ 11352/13475
---
---- [ui] src/test/ui/rfc-2565-param-attrs/param-attrs-pretty.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2565-param-attrs/param-attrs-pretty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2565-param-attrs/param-attrs-pretty" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2565-param-attrs/param-attrs-pretty/auxiliary"
stdout: none
--- stderr -------------------------------
error: custom attribute panicked
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
LL | #[attr_extern]
   |
   = help: message: assertion failed: `(left == right)`
   = help: message: assertion failed: `(left == right)`
             left: `"extern \"C\" { fn ffi(#[a1] arg1 : i32, #[a2]...) ; }"`,
            right: `"extern \"C\" { fn ffi(#[a1] arg1 : i32, #[a2] ...) ; }"`

error: custom attribute panicked
   |
   |
LL | #[attr_extern_cvar]
   |
   = help: message: assertion failed: `(left == right)`
   = help: message: assertion failed: `(left == right)`
             left: `"unsafe extern \"C\" fn cvar(arg1 : i32, #[a1] mut args :...) {}"`,
            right: `"unsafe extern \"C\" fn cvar(arg1 : i32, #[a1] mut args : ...) {}"`

error: custom attribute panicked
   |
LL | #[attr_alias]
   | ^^^^^^^^^^^^^
   |
   |
   = help: message: assertion failed: `(left == right)`
             left: `"type Alias = fn(#[a1] u8, #[a2]...) ;"`,
            right: `"type Alias = fn(#[a1] u8, #[a2] ...) ;"`

error: custom attribute panicked
   |
   |
LL | #[attr_free]
   |
   = help: message: assertion failed: `(left == right)`
   = help: message: assertion failed: `(left == right)`
             left: `"fn free(#[a1] arg1 : u8) { let lam = |#[a2] W(x), #[a3] y | () ; }"`,
            right: `"fn free(#[a1] arg1 : u8) { let lam = | #[a2] W(x), #[a3] y | () ; }"`

error: custom attribute panicked
   |
LL |     #[attr_trait_4]
   |     ^^^^^^^^^^^^^^^
   |
   |
   = help: message: assertion failed: `(left == right)`
             left: `"fn trait4 < 'a > (#[a1] self : Box < Self >, #[a2] arg1 : u8, #[a3] Vec < u8 >) ;"`,
            right: `"fn trait4 < 'a >\n(#[a1] self : Box < Self >, #[a2] arg1 : u8, #[a3] Vec < u8 >) ;"`
error: aborting due to 5 previous errors
------------------------------------------



---- [ui] src/test/ui/rfc-2565-param-attrs/issue-64682-dropping-first-attrs-in-impl-fns.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2565-param-attrs/issue-64682-dropping-first-attrs-in-impl-fns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2565-param-attrs/issue-64682-dropping-first-attrs-in-impl-fns" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2565-param-attrs/issue-64682-dropping-first-attrs-in-impl-fns/auxiliary"
stdout: none
--- stderr -------------------------------
error: custom attribute panicked
   |
   |
LL | #[rename_params(send_help)]
   |
   = help: message: assertion failed: `(left == right)`
   = help: message: assertion failed: `(left == right)`
             left: `"impl Foo { fn hello(#[angery(true)] a : i32, #[a2] b : i32, #[what = \"how\"] c : u32) {} fn hello2(#[a1] #[a2] a : i32, #[what = \"how\"] b : i32, #[angery(true)] c : u32) {} fn hello_self(#[a1] #[a2] & self, #[a1] #[a2] a : i32, #[what = \"how\"] b : i32, #[angery(true)] c : u32) {} }"`,
            right: `"impl Foo\n{\n    fn hello(#[angery(true)] a : i32, #[a2] b : i32, #[what = \"how\"] c : u32)\n    {} fn\n    hello2(#[a1] #[a2] a : i32, #[what = \"how\"] b : i32, #[angery(true)] c :\n    u32) {} fn\n    hello_self(#[a1] #[a2] & self, #[a1] #[a2] a : i32, #[what = \"how\"] b :\n    i32, #[angery(true)] c : u32) {}\n}"`
error: aborting due to previous error
------------------------------------------


