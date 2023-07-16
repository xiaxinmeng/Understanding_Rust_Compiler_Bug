\n"},"level":"error","spans":[{"file_name":"tests/ui/crashes/ice-6252.rs","byte_start":200,"byte_end":244,"line_start":10,"line_end":10,"column_start":1,"column_end":45,"is_primary":true,"text":[{"text":"impl<N, M> TypeVal<usize> for Multiply<N, M> where N: TypeVal<VAL> {}","highlight_start":1,"highlight_end":45}],"label":"missing `VAL` in implementation","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"tests/ui/crashes/ice-6252.rs","byte_start":124,"byte_end":136,"line_start":4,"line_end":4,"column_start":5,"column_end":17,"is_primary":false,"text":[{"text":"    const VAL: T;","highlight_start":5,"highlight_end":17}],"label":"`VAL` from trait","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"implement the missing item: `const VAL: T = value;`","code":null,"level":"help","spans":[{"file_name":"tests/ui/crashes/ice-6252.rs","byte_start":268,"byte_end":268,"line_start":10,"line_end":10,"column_start":69,"column_end":69,"is_primary":true,"text":[{"text":"impl<N, M> TypeVal<usize> for Multiply<N, M> where N: TypeVal<VAL> {}","highlight_start":69,"highlight_end":69}],"label":null,"suggested_replacement":"const VAL: T = value;\n","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0046]: not all trait items implemented, missing: `VAL`\n  --> tests/ui/crashes/ice-6252.rs:10:1\n   |\nLL |     const VAL: T;\n   |     ------------ `VAL` from trait\n...\nLL | impl<N, M> TypeVal<usize> for Multiply<N, M> where N: TypeVal<VAL> {}\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `VAL` in implementation\n\n"}
{"message":"Some errors have detailed explanations: E0046, E0412.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"Some errors have detailed explanations: E0046, E0412.\n"}
{"message":"For more information about an error, try `rustc --explain E0046`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0046`.\n"}

------------------------------------------
------------------------------------------

diff of stderr:

 error: this `impl` can be derived
   --> $DIR/derivable_impls.rs:22:1
    |
 LL | / impl std::default::Default for FooDefault<'_> {
 LL | |     fn default() -> Self {
 LL | |         Self {
 LL | |             a: false,
 LL | |     }
 LL | | }
    | |_^
    |
    |
    = note: `-D clippy::derivable-impls` implied by `-D warnings`
    = help: remove the manual implementation...
 help: ...and instead derive it
-LL | #[derive(Default)]
+LL + #[derive(Default)]
+LL + #[derive(Default)]
+LL | struct FooDefault<'a> {
 
 error: this `impl` can be derived
   --> $DIR/derivable_impls.rs:43:1
    |
    |
 LL | / impl std::default::Default for TupleDefault {
 LL | |     fn default() -> Self {
 LL | |         Self(false, 0, 0u64)
 LL | |     }
 LL | | }
    |
    = help: remove the manual implementation...
    = help: remove the manual implementation...
 help: ...and instead derive it
-LL | #[derive(Default)]
+LL + #[derive(Default)]
+LL + #[derive(Default)]
+LL | struct TupleDefault(bool, i32, u64);
 
 error: this `impl` can be derived
   --> $DIR/derivable_impls.rs:95:1
    |
    |
 LL | / impl Default for StrDefault<'_> {
 LL | |     fn default() -> Self {
 LL | |         Self("")
 LL | |     }
 LL | | }
    |
    = help: remove the manual implementation...
    = help: remove the manual implementation...
 help: ...and instead derive it
-LL | #[derive(Default)]
+LL + #[derive(Default)]
+LL + #[derive(Default)]
+LL | struct StrDefault<'a>(&'a str);
 
 error: this `impl` can be derived
   --> $DIR/derivable_impls.rs:121:1
    |
    |
 LL | / impl Default for Y {
 LL | |     fn default() -> Self {
 LL | |         Self(mac!())
 LL | |     }
 LL | | }
    |
    = help: remove the manual implementation...
    = help: remove the manual implementation...
 help: ...and instead derive it
-LL | #[derive(Default)]
+LL + #[derive(Default)]
+LL + #[derive(Default)]
+LL | struct Y(u32);
 
 error: this `impl` can be derived
   --> $DIR/derivable_impls.rs:160:1
    |
    |
 LL | / impl Default for WithoutSelfCurly {
 LL | |     fn default() -> Self {
 LL | |         WithoutSelfCurly { a: false }
 LL | |     }
 LL | | }
    |
    = help: remove the manual implementation...
    = help: remove the manual implementation...
 help: ...and instead derive it
-LL | #[derive(Default)]
+LL + #[derive(Default)]
+LL + #[derive(Default)]
+LL | struct WithoutSelfCurly {
 
 error: this `impl` can be derived
   --> $DIR/derivable_impls.rs:168:1
    |
    |
 LL | / impl Default for WithoutSelfParan {
 LL | |     fn default() -> Self {
 LL | |         WithoutSelfParan(false)
 LL | |     }
 LL | | }
    |
    = help: remove the manual implementation...
    = help: remove the manual implementation...
 help: ...and instead derive it
-LL | #[derive(Default)]
+LL + #[derive(Default)]
+LL + #[derive(Default)]
+LL | struct WithoutSelfParan(bool);
 
 error: this `impl` can be derived
   --> $DIR/derivable_impls.rs:218:1
    |
    |
 LL | / impl Default for RepeatDefault1 {
 LL | |     fn default() -> Self {
 LL | |         RepeatDefault1 { a: [0; 32] }
 LL | |     }
 LL | | }
    |
    = help: remove the manual implementation...
    = help: remove the manual implementation...
 help: ...and instead derive it
-LL | #[derive(Default)]
+LL + #[derive(Default)]
+LL | pub struct RepeatDefault1 {
    |
    |
 
 error: this `impl` can be derived
   --> $DIR/derivable_impls.rs:252:1
    |
 LL | / impl Default for SimpleEnum {
 LL | |     fn default() -> Self {
 LL | |         SimpleEnum::Bar
 LL | |     }
 LL | | }
    |
    = help: remove the manual implementation...
    = help: remove the manual implementation...
 help: ...and instead derive it...
-LL | #[derive(Default)]
+LL + #[derive(Default)]
+LL | pub enum SimpleEnum {
    |
    |
 help: ...and mark the default variant
    |
 LL ~     #[default]
 LL ~     Bar,
 
 error: aborting due to 8 previous errors
 
 
---
To only update this specific test, also pass `--test-args derivable_impls.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/derivable_impls.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/derivable_impls.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-adddefef864c50e4.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c41789777fdc7dd7.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-2d8deb1fc891b526.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-ce820262823dcf93.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-4d1dc4ef2b9f1742.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-5edcecca416c872f.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-a247279dad6012bb.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/derivable_impls.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this `impl` can be derived","code":{"code":"clippy::derivable_impls","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":308,"byte_end":732,"line_start":22,"line_end":39,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"impl std::default::Default for FooDefault<'_> {","highlight_start":1,"highlight_end":48},{"text":"    fn default() -> Self {","highlight_start":1,"highlight_end":27},{"text":"        Self {","highlight_start":1,"highlight_end":15},{"text":"            a: false,","highlight_start":1,"highlight_end":22},{"text":"            b: 0,","highlight_start":1,"highlight_end":18},{"text":"            c: 0u64,","highlight_start":1,"highlight_end":21},{"text":"            d: vec![],","highlight_start":1,"highlight_end":23},{"text":"            e: Default::default(),","highlight_start":1,"highlight_end":35},{"text":"            f: FooND2::default(),","highlight_start":1,"highlight_end":34},{"text":"            g: HashMap::new(),","highlight_start":1,"highlight_end":31},{"text":"            h: (0, vec![]),","highlight_start":1,"highlight_end":28},{"text":"            i: [vec![], vec![], vec![]],","highlight_start":1,"highlight_end":41},{"text":"            j: [0; 5],","highlight_start":1,"highlight_end":23},{"text":"            k: None,","highlight_start":1,"highlight_end":21},{"text":"            l: &[],","highlight_start":1,"highlight_end":20},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::derivable-impls` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"remove the manual implementation...","code":null,"level":"help","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":308,"byte_end":732,"line_start":22,"line_end":39,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"impl std::default::Default for FooDefault<'_> {","highlight_start":1,"highlight_end":48},{"text":"    fn default() -> Self {","highlight_start":1,"highlight_end":27},{"text":"        Self {","highlight_start":1,"highlight_end":15},{"text":"            a: false,","highlight_start":1,"highlight_end":22},{"text":"            b: 0,","highlight_start":1,"highlight_end":18},{"text":"            c: 0u64,","highlight_start":1,"highlight_end":21},{"text":"            d: vec![],","highlight_start":1,"highlight_end":23},{"text":"            e: Default::default(),","highlight_start":1,"highlight_end":35},{"text":"            f: FooND2::default(),","highlight_start":1,"highlight_end":34},{"text":"            g: HashMap::new(),","highlight_start":1,"highlight_end":31},{"text":"            h: (0, vec![]),","highlight_start":1,"highlight_end":28},{"text":"            i: [vec![], vec![], vec![]],","highlight_start":1,"highlight_end":41},{"text":"            j: [0; 5],","highlight_start":1,"highlight_end":23},{"text":"            k: None,","highlight_start":1,"highlight_end":21},{"text":"            l: &[],","highlight_start":1,"highlight_end":20},{"text":"        }","highlight_start":1,"highlight_end":10},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null},{"message":"...and instead derive it","code":null,"level":"help","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":70,"byte_end":70,"line_start":7,"line_end":7,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"struct FooDefault<'a> {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"#[derive(Default)]\n","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `impl` can be derived\n  --> tests/ui/derivable_impls.rs:22:1\n   |\nLL | / impl std::default::Default for FooDefault<'_> {\nLL | |     fn default() -> Self {\nLL | |         Self {\nLL | |             a: false,\n...  |\nLL | |     }\nLL | | }\n   | |_^\n   |\n   = note: `-D clippy::derivable-impls` implied by `-D warnings`\n   = help: remove the manual implementation...\nhelp: ...and instead derive it\n   |\nLL + #[derive(Default)]\nLL | struct FooDefault<'a> {\n   |\n\n"}
{"message":"this `impl` can be derived","code":{"code":"clippy::derivable_impls","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":772,"byte_end":881,"line_start":43,"line_end":47,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"impl std::default::Default for TupleDefault {","highlight_start":1,"highlight_end":46},{"text":"    fn default() -> Self {","highlight_start":1,"highlight_end":27},{"text":"        Self(false, 0, 0u64)","highlight_start":1,"highlight_end":29},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the manual implementation...","code":null,"level":"help","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":772,"byte_end":881,"line_start":43,"line_end":47,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"impl std::default::Default for TupleDefault {","highlight_start":1,"highlight_end":46},{"text":"    fn default() -> Self {","highlight_start":1,"highlight_end":27},{"text":"        Self(false, 0, 0u64)","highlight_start":1,"highlight_end":29},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null},{"message":"...and instead derive it","code":null,"level":"help","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":734,"byte_end":734,"line_start":41,"line_end":41,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"struct TupleDefault(bool, i32, u64);","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"#[derive(Default)]\n","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `impl` can be derived\n  --> tests/ui/derivable_impls.rs:43:1\n   |\nLL | / impl std::default::Default for TupleDefault {\nLL | |     fn default() -> Self {\nLL | |         Self(false, 0, 0u64)\nLL | |     }\nLL | | }\n   | |_^\n   |\n   = help: remove the manual implementation...\nhelp: ...and instead derive it\n   |\nLL + #[derive(Default)]\nLL | struct TupleDefault(bool, i32, u64);\n   |\n\n"}
{"message":"this `impl` can be derived","code":{"code":"clippy::derivable_impls","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":1487,"byte_end":1572,"line_start":95,"line_end":99,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"impl Default for StrDefault<'_> {","highlight_start":1,"highlight_end":34},{"text":"    fn default() -> Self {","highlight_start":1,"highlight_end":27},{"text":"        Self(\"\")","highlight_start":1,"highlight_end":17},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the manual implementation...","code":null,"level":"help","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":1487,"byte_end":1572,"line_start":95,"line_end":99,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"impl Default for StrDefault<'_> {","highlight_start":1,"highlight_end":34},{"text":"    fn default() -> Self {","highlight_start":1,"highlight_end":27},{"text":"        Self(\"\")","highlight_start":1,"highlight_end":17},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null},{"message":"...and instead derive it","code":null,"level":"help","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":1454,"byte_end":1454,"line_start":93,"line_end":93,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"struct StrDefault<'a>(&'a str);","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"#[derive(Default)]\n","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `impl` can be derived\n  --> tests/ui/derivable_impls.rs:95:1\n   |\nLL | / impl Default for StrDefault<'_> {\nLL | |     fn default() -> Self {\nLL | |         Self(\"\")\nLL | |     }\nLL | | }\n   | |_^\n   |\n   = help: remove the manual implementation...\nhelp: ...and instead derive it\n   |\nLL + #[derive(Default)]\nLL | struct StrDefault<'a>(&'a str);\n   |\n\n"}
{"message":"this `impl` can be derived","code":{"code":"clippy::derivable_impls","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":1866,"byte_end":1942,"line_start":121,"line_end":125,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"impl Default for Y {","highlight_start":1,"highlight_end":21},{"text":"    fn default() -> Self {","highlight_start":1,"highlight_end":27},{"text":"        Self(mac!())","highlight_start":1,"highlight_end":21},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the manual implementation...","code":null,"level":"help","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":1866,"byte_end":1942,"line_start":121,"line_end":125,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"impl Default for Y {","highlight_start":1,"highlight_end":21},{"text":"    fn default() -> Self {","highlight_start":1,"highlight_end":27},{"text":"        Self(mac!())","highlight_start":1,"highlight_end":21},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null},{"message":"...and instead derive it","code":null,"level":"help","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":1851,"byte_end":1851,"line_start":120,"line_end":120,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"struct Y(u32);","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"#[derive(Default)]\n","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `impl` can be derived\n  --> tests/ui/derivable_impls.rs:121:1\n   |\nLL | / impl Default for Y {\nLL | |     fn default() -> Self {\nLL | |         Self(mac!())\nLL | |     }\nLL | | }\n   | |_^\n   |\n   = help: remove the manual implementation...\nhelp: ...and instead derive it\n   |\nLL + #[derive(Default)]\nLL | struct Y(u32);\n   |\n\n"}
{"message":"this `impl` can be derived","code":{"code":"clippy::derivable_impls","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":2711,"byte_end":2819,"line_start":160,"line_end":164,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"impl Default for WithoutSelfCurly {","highlight_start":1,"highlight_end":36},{"text":"    fn default() -> Self {","highlight_start":1,"highlight_end":27},{"text":"        WithoutSelfCurly { a: false }","highlight_start":1,"highlight_end":38},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the manual implementation...","code":null,"level":"help","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":2711,"byte_end":2819,"line_start":160,"line_end":164,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"impl Default for WithoutSelfCurly {","highlight_start":1,"highlight_end":36},{"text":"    fn default() -> Self {","highlight_start":1,"highlight_end":27},{"text":"        WithoutSelfCurly { a: false }","highlight_start":1,"highlight_end":38},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null},{"message":"...and instead derive it","code":null,"level":"help","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":2669,"byte_end":2669,"line_start":156,"line_end":156,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"struct WithoutSelfCurly {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"#[derive(Default)]\n","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `impl` can be derived\n  --> tests/ui/derivable_impls.rs:160:1\n   |\nLL | / impl Default for WithoutSelfCurly {\nLL | |     fn default() -> Self {\nLL | |         WithoutSelfCurly { a: false }\nLL | |     }\nLL | | }\n   | |_^\n   |\n   = help: remove the manual implementation...\nhelp: ...and instead derive it\n   |\nLL + #[derive(Default)]\nLL | struct WithoutSelfCurly {\n   |\n\n"}
{"message":"this `impl` can be derived","code":{"code":"clippy::derivable_impls","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":2853,"byte_end":2955,"line_start":168,"line_end":172,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"impl Default for WithoutSelfParan {","highlight_start":1,"highlight_end":36},{"text":"    fn default() -> Self {","highlight_start":1,"highlight_end":27},{"text":"        WithoutSelfParan(false)","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the manual implementation...","code":null,"level":"help","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":2853,"byte_end":2955,"line_start":168,"line_end":172,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"impl Default for WithoutSelfParan {","highlight_start":1,"highlight_end":36},{"text":"    fn default() -> Self {","highlight_start":1,"highlight_end":27},{"text":"        WithoutSelfParan(false)","highlight_start":1,"highlight_end":32},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null},{"message":"...and instead derive it","code":null,"level":"help","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":2821,"byte_end":2821,"line_start":166,"line_end":166,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"struct WithoutSelfParan(bool);","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"#[derive(Default)]\n","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `impl` can be derived\n  --> tests/ui/derivable_impls.rs:168:1\n   |\nLL | / impl Default for WithoutSelfParan {\nLL | |     fn default() -> Self {\nLL | |         WithoutSelfParan(false)\nLL | |     }\nLL | | }\n   | |_^\n   |\n   = help: remove the manual implementation...\nhelp: ...and instead derive it\n   |\nLL + #[derive(Default)]\nLL | struct WithoutSelfParan(bool);\n   |\n\n"}
{"message":"this `impl` can be derived","code":{"code":"clippy::derivable_impls","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":3640,"byte_end":3746,"line_start":218,"line_end":222,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"impl Default for RepeatDefault1 {","highlight_start":1,"highlight_end":34},{"text":"    fn default() -> Self {","highlight_start":1,"highlight_end":27},{"text":"        RepeatDefault1 { a: [0; 32] }","highlight_start":1,"highlight_end":38},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the manual implementation...","code":null,"level":"help","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":3640,"byte_end":3746,"line_start":218,"line_end":222,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"impl Default for RepeatDefault1 {","highlight_start":1,"highlight_end":34},{"text":"    fn default() -> Self {","highlight_start":1,"highlight_end":27},{"text":"        RepeatDefault1 { a: [0; 32] }","highlight_start":1,"highlight_end":38},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null},{"message":"...and instead derive it","code":null,"level":"help","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":3592,"byte_end":3592,"line_start":214,"line_end":214,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"pub struct RepeatDefault1 {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"#[derive(Default)]\n","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `impl` can be derived\n  --> tests/ui/derivable_impls.rs:218:1\n   |\nLL | / impl Default for RepeatDefault1 {\nLL | |     fn default() -> Self {\nLL | |         RepeatDefault1 { a: [0; 32] }\nLL | |     }\nLL | | }\n   | |_^\n   |\n   = help: remove the manual implementation...\nhelp: ...and instead derive it\n   |\nLL + #[derive(Default)]\nLL | pub struct RepeatDefault1 {\n   |\n\n"}
{"message":"this `impl` can be derived","code":{"code":"clippy::derivable_impls","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":4159,"byte_end":4247,"line_start":252,"line_end":256,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"impl Default for SimpleEnum {","highlight_start":1,"highlight_end":30},{"text":"    fn default() -> Self {","highlight_start":1,"highlight_end":27},{"text":"        SimpleEnum::Bar","highlight_start":1,"highlight_end":24},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the manual implementation...","code":null,"level":"help","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":4159,"byte_end":4247,"line_start":252,"line_end":256,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"impl Default for SimpleEnum {","highlight_start":1,"highlight_end":30},{"text":"    fn default() -> Self {","highlight_start":1,"highlight_end":27},{"text":"        SimpleEnum::Bar","highlight_start":1,"highlight_end":24},{"text":"    }","highlight_start":1,"highlight_end":6},{"text":"}","highlight_start":1,"highlight_end":2}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null},{"message":"...and instead derive it...","code":null,"level":"help","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":4116,"byte_end":4116,"line_start":247,"line_end":247,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"pub enum SimpleEnum {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"#[derive(Default)]\n","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null},{"message":"...and mark the default variant","code":null,"level":"help","spans":[{"file_name":"tests/ui/derivable_impls.rs","byte_start":4151,"byte_end":4151,"line_start":249,"line_end":249,"column_start":5,"column_end":5,"is_primary":true,"text":[{"text":"    Bar,","highlight_start":5,"highlight_end":5}],"label":null,"suggested_replacement":"#[default]\n    ","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `impl` can be derived\n  --> tests/ui/derivable_impls.rs:252:1\n   |\nLL | / impl Default for SimpleEnum {\nLL | |     fn default() -> Self {\nLL | |         SimpleEnum::Bar\nLL | |     }\nLL | | }\n   | |_^\n   |\n   = help: remove the manual implementation...\nhelp: ...and instead derive it...\n   |\nLL + #[derive(Default)]\nLL | pub enum SimpleEnum {\n   |\nhelp: ...and mark the default variant\n   |\nLL ~     #[default]\nLL ~     Bar,\n   |\n\n"}

------------------------------------------

diff of stderr:
---
 LL | |         Foo
 LL | |     }
    | |_____^
    |
    = note: `-D clippy::new-without-default` implied by `-D warnings`
 help: try adding this
 LL + impl Default for Foo {
 LL +     fn default() -> Self {
 LL +         Self::new()
 LL +     }
---
    | |_____^
    |
 help: try adding this
    |
 LL + impl Default for Bar {
 LL +     fn default() -> Self {
 LL +         Self::new()
 LL +     }
 LL + }
+LL | impl Bar {
 
 
 error: you should consider adding a `Default` implementation for `LtKo<'c>`
    |
    |
 LL | /     pub fn new() -> LtKo<'c> {
 LL | |         unimplemented!()
 LL | |     }
    |
 help: try adding this
    |
    |
 LL + impl<'c> Default for LtKo<'c> {
 LL +     fn default() -> Self {
 LL +         Self::new()
 LL +     }
 LL + }
+LL | impl<'c> LtKo<'c> {
 
 
 error: you should consider adding a `Default` implementation for `NewNotEqualToDerive`
    |
 LL | /     pub fn new() -> Self {
 LL | /     pub fn new() -> Self {
 LL | |         NewNotEqualToDerive { foo: 1 }
 LL | |     }
    |
 help: try adding this
    |
    |
 LL + impl Default for NewNotEqualToDerive {
 LL +     fn default() -> Self {
 LL +         Self::new()
 LL +     }
 LL + }
+LL | impl NewNotEqualToDerive {
 
 
 error: you should consider adding a `Default` implementation for `FooGenerics<T>`
    |
 LL | /     pub fn new() -> Self {
 LL | |         Self(Default::default())
 LL | |     }
 LL | |     }
    | |_____^
    |
 help: try adding this
    |
 LL + impl<T> Default for FooGenerics<T> {
 LL +     fn default() -> Self {
 LL +         Self::new()
 LL +     }
 LL + }
+LL | impl<T> FooGenerics<T> {
 
 
 error: you should consider adding a `Default` implementation for `BarGenerics<T>`
    |
 LL | /     pub fn new() -> Self {
 LL | |         Self(Default::default())
 LL | |     }
 LL | |     }
    | |_____^
    |
 help: try adding this
    |
 LL + impl<T: Copy> Default for BarGenerics<T> {
 LL +     fn default() -> Self {
 LL +         Self::new()
 LL +     }
 LL + }
+LL | impl<T: Copy> BarGenerics<T> {
 
 
 error: you should consider adding a `Default` implementation for `Foo<T>`
    |
 LL | /         pub fn new() -> Self {
 LL | |             todo!()
 LL | |         }
 LL | |         }
    | |_________^
    |
 help: try adding this
    |
 LL ~     impl<T> Default for Foo<T> {
 LL +         fn default() -> Self {
 LL +             Self::new()
 LL +         }
 LL +     }
 LL + 
 LL ~     impl<T> Foo<T> {
 
 error: aborting due to 7 previous errors
 
 
---
To only update this specific test, also pass `--test-args new_without_default.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/new_without_default.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/new_without_default.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-adddefef864c50e4.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-c41789777fdc7dd7.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-2d8deb1fc891b526.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-ce820262823dcf93.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-4d1dc4ef2b9f1742.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-5edcecca416c872f.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bad1548ee8a3ddef.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-a247279dad6012bb.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/new_without_default.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"you should consider adding a `Default` implementation for `Foo`","code":{"code":"clippy::new_without_default","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":208,"byte_end":247,"line_start":12,"line_end":14,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn new() -> Foo {","highlight_start":5,"highlight_end":26},{"text":"        Foo","highlight_start":1,"highlight_end":12},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::new-without-default` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try adding this","code":null,"level":"help","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":193,"byte_end":193,"line_start":11,"line_end":11,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"impl Foo {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"impl Default for Foo {\n    fn default() -> Self {\n        Self::new()\n    }\n}\n\n","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you should consider adding a `Default` implementation for `Foo`\n  --> tests/ui/new_without_default.rs:12:5\n   |\nLL | /     pub fn new() -> Foo {\nLL | |         Foo\nLL | |     }\n   | |_____^\n   |\n   = note: `-D clippy::new-without-default` implied by `-D warnings`\nhelp: try adding this\n   |\nLL + impl Default for Foo {\nLL +     fn default() -> Self {\nLL +         Self::new()\nLL +     }\nLL + }\nLL | impl Foo {\n   |\n\n"}
{"message":"you should consider adding a `Default` implementation for `Bar`","code":{"code":"clippy::new_without_default","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":283,"byte_end":323,"line_start":20,"line_end":22,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn new() -> Self {","highlight_start":5,"highlight_end":27},{"text":"        Bar","highlight_start":1,"highlight_end":12},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try adding this","code":null,"level":"help","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":268,"byte_end":268,"line_start":19,"line_end":19,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"impl Bar {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"impl Default for Bar {\n    fn default() -> Self {\n        Self::new()\n    }\n}\n\n","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you should consider adding a `Default` implementation for `Bar`\n  --> tests/ui/new_without_default.rs:20:5\n   |\nLL | /     pub fn new() -> Self {\nLL | |         Bar\nLL | |     }\n   | |_____^\n   |\nhelp: try adding this\n   |\nLL + impl Default for Bar {\nLL +     fn default() -> Self {\nLL +         Self::new()\nLL +     }\nLL + }\nLL | impl Bar {\n   |\n\n"}
{"message":"you should consider adding a `Default` implementation for `LtKo<'c>`","code":{"code":"clippy::new_without_default","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":1089,"byte_end":1146,"line_start":84,"line_end":86,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn new() -> LtKo<'c> {","highlight_start":5,"highlight_end":31},{"text":"        unimplemented!()","highlight_start":1,"highlight_end":25},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try adding this","code":null,"level":"help","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":1065,"byte_end":1065,"line_start":83,"line_end":83,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"impl<'c> LtKo<'c> {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"impl<'c> Default for LtKo<'c> {\n    fn default() -> Self {\n        Self::new()\n    }\n}\n\n","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you should consider adding a `Default` implementation for `LtKo<'c>`\n  --> tests/ui/new_without_default.rs:84:5\n   |\nLL | /     pub fn new() -> LtKo<'c> {\nLL | |         unimplemented!()\nLL | |     }\n   | |_____^\n   |\nhelp: try adding this\n   |\nLL + impl<'c> Default for LtKo<'c> {\nLL +     fn default() -> Self {\nLL +         Self::new()\nLL +     }\nLL + }\nLL | impl<'c> LtKo<'c> {\n   |\n\n"}
{"message":"you should consider adding a `Default` implementation for `NewNotEqualToDerive`","code":{"code":"clippy::new_without_default","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":2810,"byte_end":2877,"line_start":177,"line_end":179,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn new() -> Self {","highlight_start":5,"highlight_end":27},{"text":"        NewNotEqualToDerive { foo: 1 }","highlight_start":1,"highlight_end":39},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try adding this","code":null,"level":"help","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":2681,"byte_end":2681,"line_start":175,"line_end":175,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"impl NewNotEqualToDerive {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"impl Default for NewNotEqualToDerive {\n    fn default() -> Self {\n        Self::new()\n    }\n}\n\n","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you should consider adding a `Default` implementation for `NewNotEqualToDerive`\n  --> tests/ui/new_without_default.rs:177:5\n   |\nLL | /     pub fn new() -> Self {\nLL | |         NewNotEqualToDerive { foo: 1 }\nLL | |     }\n   | |_____^\n   |\nhelp: try adding this\n   |\nLL + impl Default for NewNotEqualToDerive {\nLL +     fn default() -> Self {\nLL +         Self::new()\nLL +     }\nLL + }\nLL | impl NewNotEqualToDerive {\n   |\n\n"}
{"message":"you should consider adding a `Default` implementation for `FooGenerics<T>`","code":{"code":"clippy::new_without_default","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":2979,"byte_end":3040,"line_start":185,"line_end":187,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn new() -> Self {","highlight_start":5,"highlight_end":27},{"text":"        Self(Default::default())","highlight_start":1,"highlight_end":33},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try adding this","code":null,"level":"help","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":2950,"byte_end":2950,"line_start":184,"line_end":184,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"impl<T> FooGenerics<T> {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"impl<T> Default for FooGenerics<T> {\n    fn default() -> Self {\n        Self::new()\n    }\n}\n\n","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you should consider adding a `Default` implementation for `FooGenerics<T>`\n  --> tests/ui/new_without_default.rs:185:5\n   |\nLL | /     pub fn new() -> Self {\nLL | |         Self(Default::default())\nLL | |     }\n   | |_____^\n   |\nhelp: try adding this\n   |\nLL + impl<T> Default for FooGenerics<T> {\nLL +     fn default() -> Self {\nLL +         Self::new()\nLL +     }\nLL + }\nLL | impl<T> FooGenerics<T> {\n   |\n\n"}
{"message":"you should consider adding a `Default` implementation for `BarGenerics<T>`","code":{"code":"clippy::new_without_default","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":3135,"byte_end":3196,"line_start":192,"line_end":194,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    pub fn new() -> Self {","highlight_start":5,"highlight_end":27},{"text":"        Self(Default::default())","highlight_start":1,"highlight_end":33},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try adding this","code":null,"level":"help","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":3100,"byte_end":3100,"line_start":191,"line_end":191,"column_start":1,"column_end":1,"is_primary":true,"text":[{"text":"impl<T: Copy> BarGenerics<T> {","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"impl<T: Copy> Default for BarGenerics<T> {\n    fn default() -> Self {\n        Self::new()\n    }\n}\n\n","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you should consider adding a `Default` implementation for `BarGenerics<T>`\n  --> tests/ui/new_without_default.rs:192:5\n   |\nLL | /     pub fn new() -> Self {\nLL | |         Self(Default::default())\nLL | |     }\n   | |_____^\n   |\nhelp: try adding this\n   |\nLL + impl<T: Copy> Default for BarGenerics<T> {\nLL +     fn default() -> Self {\nLL +         Self::new()\nLL +     }\nLL + }\nLL | impl<T: Copy> BarGenerics<T> {\n   |\n\n"}
{"message":"you should consider adding a `Default` implementation for `Foo<T>`","code":{"code":"clippy::new_without_default","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":3302,"byte_end":3354,"line_start":203,"line_end":205,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"        pub fn new() -> Self {","highlight_start":9,"highlight_end":31},{"text":"            todo!()","highlight_start":1,"highlight_end":20},{"text":"        }","highlight_start":1,"highlight_end":10}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try adding this","code":null,"level":"help","spans":[{"file_name":"tests/ui/new_without_default.rs","byte_start":3277,"byte_end":3277,"line_start":202,"line_end":202,"column_start":5,"column_end":5,"is_primary":true,"text":[{"text":"    impl<T> Foo<T> {","highlight_start":5,"highlight_end":5}],"label":null,"suggested_replacement":"impl<T> Default for Foo<T> {\n        fn default() -> Self {\n            Self::new()\n        }\n    }\n\n    ","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: you should consider adding a `Default` implementation for `Foo<T>`\n  --> tests/ui/new_without_default.rs:203:9\n   |\nLL | /         pub fn new() -> Self {\nLL | |             todo!()\nLL | |         }\n   | |_________^\n   |\nhelp: try adding this\n   |\nLL ~     impl<T> Default for Foo<T> {\nLL +         fn default() -> Self {\nLL +             Self::new()\nLL +         }\nLL +     }\nLL + \nLL ~     impl<T> Foo<T> {\n   |\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/index.crates.io-6f17d22bba15001f/compiletest_rs-0.9.0/src/lib.rs:111:22
