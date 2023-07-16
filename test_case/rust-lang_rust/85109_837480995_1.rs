\n\nDelete the offending feature attribute.\n"},"level":"error","spans":[{"file_name":"tests/run-pass/calls.rs","byte_start":11,"byte_end":19,"line_start":1,"line_end":1,"column_start":12,"column_end":20,"is_primary":true,"text":[{"text":"#![feature(const_fn)]","highlight_start":12,"highlight_end":20}],"label":"feature has been removed","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"split into finer-grained feature gates","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0557]: feature has been removed\n --> tests/run-pass/calls.rs:1:12\n  |\n1 | #![feature(const_fn)]\n  |            ^^^^^^^^ feature has been removed\n  |\n  = note: split into finer-grained feature gates\n\n"}
{"message":"For more information about this error, try `rustc --explain E0557`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0557`.\n"}

------------------------------------------

---

---- compile_test stdout ----
diff of stderr:

 error: you should consider adding a `Default` implementation for `Foo`
+  --> $DIR/new_without_default.rs:7:5
    |
    |
 LL | /     pub fn new() -> Foo {
 LL | |         Foo
 LL | |     }
    |
    |
    = note: `-D clippy::new-without-default` implied by `-D warnings`
    |
 LL | impl Default for Foo {
 LL |     fn default() -> Self {
 LL |         Self::new()
 LL |         Self::new()
 LL |     }
 LL | }
    |
 
 error: you should consider adding a `Default` implementation for `Bar`
+  --> $DIR/new_without_default.rs:15:5
    |
    |
 LL | /     pub fn new() -> Self {
 LL | |         Bar
 LL | |     }
    |
 help: try this
    |
    |
 LL | impl Default for Bar {
 LL |     fn default() -> Self {
 LL |     }
 LL | }
    |
 
 
 error: you should consider adding a `Default` implementation for `LtKo<'c>`
+  --> $DIR/new_without_default.rs:79:5
    |
    |
 LL | /     pub fn new() -> LtKo<'c> {
 LL | |         unimplemented!()
 LL | |     }
    |
 help: try this
    |
    |
 LL | impl<'c> Default for LtKo<'c> {
 LL |     fn default() -> Self {
 LL |     }
 LL | }
    |
 
 
 error: you should consider adding a `Default` implementation for `NewNotEqualToDerive`
+  --> $DIR/new_without_default.rs:156:5
    |
    |
 LL | /     pub fn new() -> Self {
 LL | |         NewNotEqualToDerive { foo: 1 }
 LL | |     }
    |
 help: try this
    |
    |
 LL | impl Default for NewNotEqualToDerive {
 LL |     fn default() -> Self {
 LL |     }
 LL | }
    |
 
 
 error: you should consider adding a `Default` implementation for `FooGenerics<T>`
+  --> $DIR/new_without_default.rs:164:5
    |
    |
 LL | /     pub fn new() -> Self {
 LL | |         Self(Default::default())
 LL | |     }
    |
 help: try this
    |
    |
 LL | impl<T> Default for FooGenerics<T> {
 LL |     fn default() -> Self {
 LL |     }
 LL | }
    |
 
 
 error: you should consider adding a `Default` implementation for `BarGenerics<T>`
+  --> $DIR/new_without_default.rs:171:5
    |
    |
 LL | /     pub fn new() -> Self {
 LL | |         Self(Default::default())
 LL | |     }
    |
 help: try this
    |
    |
 LL | impl<T: Copy> Default for BarGenerics<T> {
 LL |     fn default() -> Self {
 LL |     }
 LL | }
    |
 
---
To only update this specific test, also pass `--test-args new_without_default.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/new_without_default.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/new_without_default.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-c9cbd4ed51f0d395.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/new_without_default.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"you should consider adding a `Default` implementation for `Foo`","code":{"code":"clippy::new_without_default","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":120,"byte_end":159,"line_start":7,"line_end":9,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn new() -> Foo {","highlight_start":5,"highlight_end":26},{"text":"        Foo","highlight_start":1,"highlight_end":12},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::new-without-default` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":105,"byte_end":105,"line_start":6,"line_end":6,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"impl Foo {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"impl Default for Foo {\n    fn default() -> Self {\n        Self::new()\n    }\n}\n\n","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you should consider adding a `Default` implementation for `Foo`\n  --> tests/ui/new_without_default.rs:7:5\n   |\nLL | /     pub fn new() -> Foo {\nLL | |         Foo\nLL | |     }\n   | |_____^\n   |\n   = note: `-D clippy::new-without-default` implied by `-D warnings`\nhelp: try this\n   |\nLL | impl Default for Foo {\nLL |     fn default() -> Self {\nLL |         Self::new()\nLL |     }\nLL | }\n   |\n\n"}
{"message":"you should consider adding a `Default` implementation for `Bar`","code":{"code":"clippy::new_without_default","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":195,"byte_end":235,"line_start":15,"line_end":17,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn new() -> Self {","highlight_start":5,"highlight_end":27},{"text":"        Bar","highlight_start":1,"highlight_end":12},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":180,"byte_end":180,"line_start":14,"line_end":14,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"impl Bar {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"impl Default for Bar {\n    fn default() -> Self {\n        Self::new()\n    }\n}\n\n","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you should consider adding a `Default` implementation for `Bar`\n  --> tests/ui/new_without_default.rs:15:5\n   |\nLL | /     pub fn new() -> Self {\nLL | |         Bar\nLL | |     }\n   | |_____^\n   |\nhelp: try this\n   |\nLL | impl Default for Bar {\nLL |     fn default() -> Self {\nLL |         Self::new()\nLL |     }\nLL | }\n   |\n\n"}
{"message":"you should consider adding a `Default` implementation for `LtKo<'c>`","code":{"code":"clippy::new_without_default","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":1001,"byte_end":1058,"line_start":79,"line_end":81,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn new() -> LtKo<'c> {","highlight_start":5,"highlight_end":31},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":977,"byte_end":977,"line_start":78,"line_end":78,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"impl<'c> LtKo<'c> {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"impl<'c> Default for LtKo<'c> {\n    fn default() -> Self {\n        Self::new()\n    }\n}\n\n","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you should consider adding a `Default` implementation for `LtKo<'c>`\n  --> tests/ui/new_without_default.rs:79:5\n   |\nLL | /     pub fn new() -> LtKo<'c> {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\nhelp: try this\n   |\nLL | impl<'c> Default for LtKo<'c> {\nLL |     fn default() -> Self {\nLL |         Self::new()\nLL |     }\nLL | }\n   |\n\n"}
{"message":"you should consider adding a `Default` implementation for `NewNotEqualToDerive`","code":{"code":"clippy::new_without_default","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":2402,"byte_end":2469,"line_start":156,"line_end":158,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn new() -> Self {","highlight_start":5,"highlight_end":27},{"text":"        NewNotEqualToDerive { foo: 1 }","highlight_start":1,"highlight_end":39},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":2273,"byte_end":2273,"line_start":154,"line_end":154,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"impl NewNotEqualToDerive {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"impl Default for NewNotEqualToDerive {\n    fn default() -> Self {\n        Self::new()\n    }\n}\n\n","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you should consider adding a `Default` implementation for `NewNotEqualToDerive`\n  --> tests/ui/new_without_default.rs:156:5\n   |\nLL | /     pub fn new() -> Self {\nLL | |         NewNotEqualToDerive { foo: 1 }\nLL | |     }\n   | |_____^\n   |\nhelp: try this\n   |\nLL | impl Default for NewNotEqualToDerive {\nLL |     fn default() -> Self {\nLL |         Self::new()\nLL |     }\nLL | }\n   |\n\n"}
{"message":"you should consider adding a `Default` implementation for `FooGenerics<T>`","code":{"code":"clippy::new_without_default","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":2571,"byte_end":2632,"line_start":164,"line_end":166,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn new() -> Self {","highlight_start":5,"highlight_end":27},{"text":"        Self(Default::default())","highlight_start":1,"highlight_end":33},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":2542,"byte_end":2542,"line_start":163,"line_end":163,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"impl<T> FooGenerics<T> {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"impl<T> Default for FooGenerics<T> {\n    fn default() -> Self {\n        Self::new()\n    }\n}\n\n","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you should consider adding a `Default` implementation for `FooGenerics<T>`\n  --> tests/ui/new_without_default.rs:164:5\n   |\nLL | /     pub fn new() -> Self {\nLL | |         Self(Default::default())\nLL | |     }\n   | |_____^\n   |\nhelp: try this\n   |\nLL | impl<T> Default for FooGenerics<T> {\nLL |     fn default() -> Self {\nLL |         Self::new()\nLL |     }\nLL | }\n   |\n\n"}
{"message":"you should consider adding a `Default` implementation for `BarGenerics<T>`","code":{"code":"clippy::new_without_default","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":2727,"byte_end":2788,"line_start":171,"line_end":173,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn new() -> Self {","highlight_start":5,"highlight_end":27},{"text":"        Self(Default::default())","highlight_start":1,"highlight_end":33},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":2692,"byte_end":2692,"line_start":170,"line_end":170,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"impl<T: Copy> BarGenerics<T> {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"impl<T: Copy> Default for BarGenerics<T> {\n    fn default() -> Self {\n        Self::new()\n    }\n}\n\n","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you should consider adding a `Default` implementation for `BarGenerics<T>`\n  --> tests/ui/new_without_default.rs:171:5\n   |\nLL | /     pub fn new() -> Self {\nLL | |         Self(Default::default())\nLL | |     }\n   | |_____^\n   |\nhelp: try this\n   |\nLL | impl<T: Copy> Default for BarGenerics<T> {\nLL |     fn default() -> Self {\nLL |         Self::new()\nLL |     }\nLL | }\n   |\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
