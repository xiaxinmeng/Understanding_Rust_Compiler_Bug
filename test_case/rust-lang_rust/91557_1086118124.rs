plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 0677edc86e342f333d4828b0ee1ef395a4e70fe5 and 417df8e8d5df9290c8c14b49e97ab91076c4e451
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
   Compiling rls-data v0.19.1
   Compiling rls-vfs v0.8.0
   Compiling racer v2.2.0
   Compiling toml v0.5.7
error[E0023]: this pattern has 5 fields, but the corresponding tuple variant has 6 fields
    |
    |
227 |             visit::FnKind::Fn(_, _, ref fn_sig, _, _) => &*fn_sig.decl,
    |                               ^  ^  ^^^^^^^^^^  ^  ^ expected 6 fields, found 5
   ::: /checkout/compiler/rustc_ast/src/visit.rs:38:8
    |
    |
38  |     Fn(FnCtxt, Ident, &'a FnSig, &'a Visibility, &'a Generics, Option<&'a Block>),
    |        ------  -----  ---------  --------------  ------------  ----------------- tuple variant has 6 fields
    |
help: use `_` to explicitly ignore each field
    |
227 |             visit::FnKind::Fn(_, _, ref fn_sig, _, _, _) => &*fn_sig.decl,

   Compiling toml_edit v0.13.4
   Compiling toml_edit v0.13.4
error[E0023]: this pattern has 5 fields, but the corresponding tuple variant has 6 fields
     |
     |
1255 |             visit::FnKind::Fn(_, _, ref fn_sig, _, _) => &*fn_sig.decl,
     |                               ^  ^  ^^^^^^^^^^  ^  ^ expected 6 fields, found 5
    ::: /checkout/compiler/rustc_ast/src/visit.rs:38:8
     |
     |
38   |     Fn(FnCtxt, Ident, &'a FnSig, &'a Visibility, &'a Generics, Option<&'a Block>),
     |        ------  -----  ---------  --------------  ------------  ----------------- tuple variant has 6 fields
     |
help: use `_` to explicitly ignore each field
     |
1255 |             visit::FnKind::Fn(_, _, ref fn_sig, _, _, _) => &*fn_sig.decl,

   Compiling openssl-sys v0.9.65
For more information about this error, try `rustc --explain E0023`.
error: could not compile `racer` due to 2 previous errors
---
 error: unneeded unit return type
-  --> $DIR/unused_unit.rs:19:28
+  --> $DIR/unused_unit.rs:19:58
    |
 LL |     pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()
-   |                            ^^^^^^ help: remove the `-> ()`
+   |                                                          ^^^^^^ help: remove the `-> ()`
 note: the lint level is defined here
   --> $DIR/unused_unit.rs:12:9
    |
    |
 LL | #![deny(clippy::unused_unit)]
 
 error: unneeded unit return type
-  --> $DIR/unused_unit.rs:20:18
+  --> $DIR/unused_unit.rs:19:28
+  --> $DIR/unused_unit.rs:19:28
    |
-LL |     where G: Fn() -> () {
-   |                  ^^^^^^ help: remove the `-> ()`
+LL |     pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()
+   |                            ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
-  --> $DIR/unused_unit.rs:19:58
+  --> $DIR/unused_unit.rs:20:18
    |
    |
-LL |     pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()
-   |                                                          ^^^^^^ help: remove the `-> ()`
+LL |     where G: Fn() -> () {
+   |                  ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:21:26
    |
    |
 LL |         let _y: &dyn Fn() -> () = &f;
    |                          ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:28:18
    |
 LL |     fn into(self) -> () {
---
 
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:34:29
    |
 LL |     fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)
    |                             ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:36:19
    |
    |
 LL |         G: FnMut() -> (),
    |                   ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:37:16
    |
    |
 LL |         H: Fn() -> ();
    |                ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:41:29
    |
    |
 LL |     fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)
    |                             ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:43:19
    |
    |
 LL |         G: FnMut() -> (),
    |                   ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:44:16
    |
    |
 LL |         H: Fn() -> () {}
    |                ^^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:47:17
    |
    |
 LL | fn return_unit() -> () { () }
    |                 ^^^^^^ help: remove the `-> ()`
 error: unneeded unit expression
   --> $DIR/unused_unit.rs:47:26
    |
    |
 LL | fn return_unit() -> () { () }
    |                          ^^ help: remove the final `()`
 error: unneeded `()`
   --> $DIR/unused_unit.rs:57:14
    |
 LL |         break();
---
 
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:76:10
    |
 LL | fn test()->(){}
    |          ^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:79:11
    |
    |
 LL | fn test2() ->(){}
    |           ^^^^^ help: remove the `-> ()`
 error: unneeded unit return type
   --> $DIR/unused_unit.rs:82:11
    |
    |
 LL | fn test3()-> (){}
    |           ^^^^^ help: remove the `-> ()`
 error: aborting due to 19 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unused_unit.stage-id.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unused_unit.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/unused_unit.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unused_unit.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-876c59a3e481cb18.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-e6439823b6d16f2c.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unused_unit.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":611,"byte_end":617,"line_start":19,"line_end":19,"column_start":58,"column_end":64,"is_primary":true,"text":[{"text":"    pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()","highlight_start":58,"highlight_end":64}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":413,"byte_end":432,"line_start":12,"line_end":12,"column_start":9,"column_end":28,"is_primary":true,"text":[{"text":"#![deny(clippy::unused_unit)]","highlight_start":9,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":611,"byte_end":617,"line_start":19,"line_end":19,"column_start":58,"column_end":64,"is_primary":true,"text":[{"text":"    pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()","highlight_start":58,"highlight_end":64}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:19:58\n   |\nLL |     pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()\n   |                                                          ^^^^^^ help: remove the `-> ()`\n   |\nnote: the lint level is defined here\n  --> tests/ui/unused_unit.rs:12:9\n   |\nLL | #![deny(clippy::unused_unit)]\n   |         ^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":581,"byte_end":587,"line_start":19,"line_end":19,"column_start":28,"column_end":34,"is_primary":true,"text":[{"text":"    pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()","highlight_start":28,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":581,"byte_end":587,"line_start":19,"line_end":19,"column_start":28,"column_end":34,"is_primary":true,"text":[{"text":"    pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()","highlight_start":28,"highlight_end":34}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:19:28\n   |\nLL |     pub fn get_unit<F: Fn() -> (), G>(&self, f: F, _g: G) -> ()\n   |                            ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":635,"byte_end":641,"line_start":20,"line_end":20,"column_start":18,"column_end":24,"is_primary":true,"text":[{"text":"    where G: Fn() -> () {","highlight_start":18,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":635,"byte_end":641,"line_start":20,"line_end":20,"column_start":18,"column_end":24,"is_primary":true,"text":[{"text":"    where G: Fn() -> () {","highlight_start":18,"highlight_end":24}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:20:18\n   |\nLL |     where G: Fn() -> () {\n   |                  ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":669,"byte_end":675,"line_start":21,"line_end":21,"column_start":26,"column_end":32,"is_primary":true,"text":[{"text":"        let _y: &dyn Fn() -> () = &f;","highlight_start":26,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":669,"byte_end":675,"line_start":21,"line_end":21,"column_start":26,"column_end":32,"is_primary":true,"text":[{"text":"        let _y: &dyn Fn() -> () = &f;","highlight_start":26,"highlight_end":32}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:21:26\n   |\nLL |         let _y: &dyn Fn() -> () = &f;\n   |                          ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":830,"byte_end":836,"line_start":28,"line_end":28,"column_start":18,"column_end":24,"is_primary":true,"text":[{"text":"    fn into(self) -> () {","highlight_start":18,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":830,"byte_end":836,"line_start":28,"line_end":28,"column_start":18,"column_end":24,"is_primary":true,"text":[{"text":"    fn into(self) -> () {","highlight_start":18,"highlight_end":24}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:28:18\n   |\nLL |     fn into(self) -> () {\n   |                  ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit expression","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":847,"byte_end":849,"line_start":29,"line_end":29,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"        ()","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the final `()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":847,"byte_end":849,"line_start":29,"line_end":29,"column_start":9,"column_end":11,"is_primary":true,"text":[{"text":"        ()","highlight_start":9,"highlight_end":11}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit expression\n  --> tests/ui/unused_unit.rs:29:9\n   |\nLL |         ()\n   |         ^^ help: remove the final `()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":901,"byte_end":907,"line_start":34,"line_end":34,"column_start":29,"column_end":35,"is_primary":true,"text":[{"text":"    fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)","highlight_start":29,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":901,"byte_end":907,"line_start":34,"line_end":34,"column_start":29,"column_end":35,"is_primary":true,"text":[{"text":"    fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)","highlight_start":29,"highlight_end":35}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:34:29\n   |\nLL |     fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)\n   |                             ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":971,"byte_end":977,"line_start":36,"line_end":36,"column_start":19,"column_end":25,"is_primary":true,"text":[{"text":"        G: FnMut() -> (),","highlight_start":19,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":971,"byte_end":977,"line_start":36,"line_end":36,"column_start":19,"column_end":25,"is_primary":true,"text":[{"text":"        G: FnMut() -> (),","highlight_start":19,"highlight_end":25}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:36:19\n   |\nLL |         G: FnMut() -> (),\n   |                   ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":994,"byte_end":1000,"line_start":37,"line_end":37,"column_start":16,"column_end":22,"is_primary":true,"text":[{"text":"        H: Fn() -> ();","highlight_start":16,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":994,"byte_end":1000,"line_start":37,"line_end":37,"column_start":16,"column_end":22,"is_primary":true,"text":[{"text":"        H: Fn() -> ();","highlight_start":16,"highlight_end":22}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:37:16\n   |\nLL |         H: Fn() -> ();\n   |                ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1058,"byte_end":1064,"line_start":41,"line_end":41,"column_start":29,"column_end":35,"is_primary":true,"text":[{"text":"    fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)","highlight_start":29,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1058,"byte_end":1064,"line_start":41,"line_end":41,"column_start":29,"column_end":35,"is_primary":true,"text":[{"text":"    fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)","highlight_start":29,"highlight_end":35}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:41:29\n   |\nLL |     fn redundant<F: FnOnce() -> (), G, H>(&self, _f: F, _g: G, _h: H)\n   |                             ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1128,"byte_end":1134,"line_start":43,"line_end":43,"column_start":19,"column_end":25,"is_primary":true,"text":[{"text":"        G: FnMut() -> (),","highlight_start":19,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1128,"byte_end":1134,"line_start":43,"line_end":43,"column_start":19,"column_end":25,"is_primary":true,"text":[{"text":"        G: FnMut() -> (),","highlight_start":19,"highlight_end":25}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:43:19\n   |\nLL |         G: FnMut() -> (),\n   |                   ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1151,"byte_end":1157,"line_start":44,"line_end":44,"column_start":16,"column_end":22,"is_primary":true,"text":[{"text":"        H: Fn() -> () {}","highlight_start":16,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1151,"byte_end":1157,"line_start":44,"line_end":44,"column_start":16,"column_end":22,"is_primary":true,"text":[{"text":"        H: Fn() -> () {}","highlight_start":16,"highlight_end":22}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:44:16\n   |\nLL |         H: Fn() -> () {}\n   |                ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1180,"byte_end":1186,"line_start":47,"line_end":47,"column_start":17,"column_end":23,"is_primary":true,"text":[{"text":"fn return_unit() -> () { () }","highlight_start":17,"highlight_end":23}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1180,"byte_end":1186,"line_start":47,"line_end":47,"column_start":17,"column_end":23,"is_primary":true,"text":[{"text":"fn return_unit() -> () { () }","highlight_start":17,"highlight_end":23}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:47:17\n   |\nLL | fn return_unit() -> () { () }\n   |                 ^^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit expression","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1189,"byte_end":1191,"line_start":47,"line_end":47,"column_start":26,"column_end":28,"is_primary":true,"text":[{"text":"fn return_unit() -> () { () }","highlight_start":26,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the final `()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1189,"byte_end":1191,"line_start":47,"line_end":47,"column_start":26,"column_end":28,"is_primary":true,"text":[{"text":"fn return_unit() -> () { () }","highlight_start":26,"highlight_end":28}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit expression\n  --> tests/ui/unused_unit.rs:47:26\n   |\nLL | fn return_unit() -> () { () }\n   |                          ^^ help: remove the final `()`\n\n"}
{"message":"unneeded `()`","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1419,"byte_end":1421,"line_start":57,"line_end":57,"column_start":14,"column_end":16,"is_primary":true,"text":[{"text":"        break();","highlight_start":14,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1419,"byte_end":1421,"line_start":57,"line_end":57,"column_start":14,"column_end":16,"is_primary":true,"text":[{"text":"        break();","highlight_start":14,"highlight_end":16}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `()`\n  --> tests/ui/unused_unit.rs:57:14\n   |\nLL |         break();\n   |              ^^ help: remove the `()`\n\n"}
{"message":"unneeded `()`","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1439,"byte_end":1441,"line_start":59,"line_end":59,"column_start":11,"column_end":13,"is_primary":true,"text":[{"text":"    return();","highlight_start":11,"highlight_end":13}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1439,"byte_end":1441,"line_start":59,"line_end":59,"column_start":11,"column_end":13,"is_primary":true,"text":[{"text":"    return();","highlight_start":11,"highlight_end":13}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded `()`\n  --> tests/ui/unused_unit.rs:59:11\n   |\nLL |     return();\n   |           ^^ help: remove the `()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1701,"byte_end":1705,"line_start":76,"line_end":76,"column_start":10,"column_end":14,"is_primary":true,"text":[{"text":"fn test()->(){}","highlight_start":10,"highlight_end":14}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1701,"byte_end":1705,"line_start":76,"line_end":76,"column_start":10,"column_end":14,"is_primary":true,"text":[{"text":"fn test()->(){}","highlight_start":10,"highlight_end":14}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:76:10\n   |\nLL | fn test()->(){}\n   |          ^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1736,"byte_end":1741,"line_start":79,"line_end":79,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"fn test2() ->(){}","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1736,"byte_end":1741,"line_start":79,"line_end":79,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"fn test2() ->(){}","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:79:11\n   |\nLL | fn test2() ->(){}\n   |           ^^^^^ help: remove the `-> ()`\n\n"}
{"message":"unneeded unit return type","code":{"code":"clippy::unused_unit","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1772,"byte_end":1777,"line_start":82,"line_end":82,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"fn test3()-> (){}","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove the `-> ()`","code":null,"level":"help","spans":[{"file_name":"tests/ui/unused_unit.rs","byte_start":1772,"byte_end":1777,"line_start":82,"line_end":82,"column_start":11,"column_end":16,"is_primary":true,"text":[{"text":"fn test3()-> (){}","highlight_start":11,"highlight_end":16}],"label":null,"suggested_replacement":"","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unneeded unit return type\n  --> tests/ui/unused_unit.rs:82:11\n   |\nLL | fn test3()-> (){}\n   |           ^^^^^ help: remove the `-> ()`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.7.1/src/lib.rs:105:22
