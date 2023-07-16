plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 0e1a6fb463e7075572cee841525bf44a864da807 and bc5458670a933f1436d4fe42e8cdaed6c73bca0c
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

 error: use of `unwrap_or` followed by a function call
   --> $DIR/or_fun_call.rs:49:22
    |
 LL |     with_constructor.unwrap_or(make());
    |                      ^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(make)`
    |
    = note: `-D clippy::or-fun-call` implied by `-D warnings`
 
 error: use of `unwrap_or` followed by a call to `new`
   --> $DIR/or_fun_call.rs:52:5
    |
 LL |     with_new.unwrap_or(Vec::new());
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `with_new.unwrap_or_default()`
 error: use of `unwrap_or` followed by a function call
   --> $DIR/or_fun_call.rs:55:21
    |
    |
 LL |     with_const_args.unwrap_or(Vec::with_capacity(12));
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| Vec::with_capacity(12))`
 error: use of `unwrap_or` followed by a function call
   --> $DIR/or_fun_call.rs:58:14
    |
    |
 LL |     with_err.unwrap_or(make());
    |              ^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|_| make())`
 error: use of `unwrap_or` followed by a function call
   --> $DIR/or_fun_call.rs:61:19
    |
    |
 LL |     with_err_args.unwrap_or(Vec::with_capacity(12));
    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|_| Vec::with_capacity(12))`
 
 error: use of `unwrap_or` followed by a call to `default`
   --> $DIR/or_fun_call.rs:64:5
    |
 LL |     with_default_trait.unwrap_or(Default::default());
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `with_default_trait.unwrap_or_default()`
 
 error: use of `unwrap_or` followed by a call to `default`
   --> $DIR/or_fun_call.rs:67:5
    |
 LL |     with_default_type.unwrap_or(u64::default());
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `with_default_type.unwrap_or_default()`
 error: use of `unwrap_or` followed by a function call
   --> $DIR/or_fun_call.rs:70:18
    |
    |
 LL |     self_default.unwrap_or(<FakeDefault>::default());
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(<FakeDefault>::default)`
 
 error: use of `unwrap_or` followed by a call to `default`
   --> $DIR/or_fun_call.rs:73:5
    |
 LL |     real_default.unwrap_or(<FakeDefault as Default>::default());
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `real_default.unwrap_or_default()`
 
 error: use of `unwrap_or` followed by a call to `new`
   --> $DIR/or_fun_call.rs:76:5
    |
 LL |     with_vec.unwrap_or(vec![]);
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `with_vec.unwrap_or_default()`
 error: use of `unwrap_or` followed by a function call
   --> $DIR/or_fun_call.rs:79:21
    |
    |
 LL |     without_default.unwrap_or(Foo::new());
    |                     ^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(Foo::new)`
 error: use of `unwrap_or` followed by a function call
   --> $DIR/or_fun_call.rs:94:21
    |
    |
 LL |     let _ = stringy.unwrap_or("".to_owned());
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| "".to_owned())`
 error: use of `unwrap_or` followed by a function call
   --> $DIR/or_fun_call.rs:102:21
    |
    |
 LL |     let _ = Some(1).unwrap_or(map[&1]);
    |                     ^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| map[&1])`
 error: use of `unwrap_or` followed by a function call
   --> $DIR/or_fun_call.rs:104:21
    |
    |
 LL |     let _ = Some(1).unwrap_or(map[&1]);
    |                     ^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| map[&1])`
 
 error: use of `or` followed by a function call
   --> $DIR/or_fun_call.rs:128:35
    |
 LL |     let _ = Some("a".to_string()).or(Some("b".to_string()));
    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `or_else(|| Some("b".to_string()))`
 error: use of `unwrap_or` followed by a function call
   --> $DIR/or_fun_call.rs:167:14
    |
    |
 LL |         None.unwrap_or(ptr_to_ref(s));
    |              ^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| ptr_to_ref(s))`
 error: use of `unwrap_or` followed by a function call
   --> $DIR/or_fun_call.rs:173:14
    |
    |
 LL |         None.unwrap_or(unsafe { ptr_to_ref(s) });
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| unsafe { ptr_to_ref(s) })`
 error: use of `unwrap_or` followed by a function call
   --> $DIR/or_fun_call.rs:175:14
    |
    |
 LL |         None.unwrap_or( unsafe { ptr_to_ref(s) }    );
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| unsafe { ptr_to_ref(s) })`
 
 error: use of `unwrap_or` followed by a call to `new`
   --> $DIR/or_fun_call.rs:189:14
    |
 LL |             .unwrap_or(String::new());
    |              ^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_default()`
 
 error: use of `unwrap_or` followed by a call to `new`
   --> $DIR/or_fun_call.rs:202:14
    |
 LL |             .unwrap_or(String::new());
    |              ^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_default()`
 
 error: use of `unwrap_or` followed by a call to `new`
   --> $DIR/or_fun_call.rs:208:9
    |
-LL |           iter.map(|f: &String| f.to_lowercase())
-LL |               .reduce(|mut acc, f| {
-LL |                   let _ = "";
-LL |                   acc.push_str(&f);
-LL |                   acc
-LL |               })
-LL |               .unwrap_or(String::new());
-   | /______________^
+LL | /         iter.map(|f: &String| f.to_lowercase())
+LL | |             .reduce(|mut acc, f| {
+LL | |                 let _ = "";
+LL | |                 acc.push_str(&f);
+LL | |                 acc
+LL | |             })
+LL | |             .unwrap_or(String::new());
    |
 help: try this
    |
    |
 LL ~         iter.map(|f: &String| f.to_lowercase())
 LL +             .reduce(|mut acc, f| {
 LL +                 let _ = "";
 LL +                 acc.push_str(&f);
 LL +                 acc
 LL ~             }).unwrap_or_default();
 
 
 error: use of `unwrap_or` followed by a call to `new`
   --> $DIR/or_fun_call.rs:221:9
    |
-LL |           map.reduce(|mut acc, f| {
-LL |               acc.push_str(&f);
-LL |               acc
-LL |           })
-LL |           .unwrap_or(String::new());
-   | /__________^
+LL | /         map.reduce(|mut acc, f| {
+LL | |             acc.push_str(&f);
+LL | |             acc
+LL | |         })
+LL | |         .unwrap_or(String::new());
    |
 help: try this
    |
    |
 LL ~         map.reduce(|mut acc, f| {
 LL +             acc.push_str(&f);
 LL +             acc
 LL ~         }).unwrap_or_default();
 
 error: aborting due to 22 previous errors
 
 
---
To only update this specific test, also pass `--test-args or_fun_call.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/or_fun_call.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/or_fun_call.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-bb39617b33963acb.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-6479f1c58bb283b7.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-0fd25f6e303f34bc.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-03d687731ecf653d.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-5411643be5ff72c2.so" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c495ccdc5de2578f.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-76e15f312bef456e.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-2ea56038ff120115.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-71205fa4273edf27.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3cd37cddf9b6fc4a.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-8f6f6ff006a184c3.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/or_fun_call.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"use of `unwrap_or` followed by a function call","code":{"code":"clippy::or_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":949,"byte_end":966,"line_start":49,"line_end":49,"column_start":22,"column_end":39,"is_primary":true,"text":[{"text":"    with_constructor.unwrap_or(make());","highlight_start":22,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::or-fun-call` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":949,"byte_end":966,"line_start":49,"line_end":49,"column_start":22,"column_end":39,"is_primary":true,"text":[{"text":"    with_constructor.unwrap_or(make());","highlight_start":22,"highlight_end":39}],"label":null,"suggested_replacement":"unwrap_or_else(make)","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `unwrap_or` followed by a function call\n  --> tests/ui/or_fun_call.rs:49:22\n   |\nLL |     with_constructor.unwrap_or(make());\n   |                      ^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(make)`\n   |\n   = note: `-D clippy::or-fun-call` implied by `-D warnings`\n\n"}
{"message":"use of `unwrap_or` followed by a call to `new`","code":{"code":"clippy::or_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":1007,"byte_end":1037,"line_start":52,"line_end":52,"column_start":5,"column_end":35,"is_primary":true,"text":[{"text":"    with_new.unwrap_or(Vec::new());","highlight_start":5,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":1007,"byte_end":1037,"line_start":52,"line_end":52,"column_start":5,"column_end":35,"is_primary":true,"text":[{"text":"    with_new.unwrap_or(Vec::new());","highlight_start":5,"highlight_end":35}],"label":null,"suggested_replacement":"with_new.unwrap_or_default()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `unwrap_or` followed by a call to `new`\n  --> tests/ui/or_fun_call.rs:52:5\n   |\nLL |     with_new.unwrap_or(Vec::new());\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `with_new.unwrap_or_default()`\n\n"}
{"message":"use of `unwrap_or` followed by a function call","code":{"code":"clippy::or_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":1101,"byte_end":1134,"line_start":55,"line_end":55,"column_start":21,"column_end":54,"is_primary":true,"text":[{"text":"    with_const_args.unwrap_or(Vec::with_capacity(12));","highlight_start":21,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":1101,"byte_end":1134,"line_start":55,"line_end":55,"column_start":21,"column_end":54,"is_primary":true,"text":[{"text":"    with_const_args.unwrap_or(Vec::with_capacity(12));","highlight_start":21,"highlight_end":54}],"label":null,"suggested_replacement":"unwrap_or_else(|| Vec::with_capacity(12))","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `unwrap_or` followed by a function call\n  --> tests/ui/or_fun_call.rs:55:21\n   |\nLL |     with_const_args.unwrap_or(Vec::with_capacity(12));\n   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| Vec::with_capacity(12))`\n\n"}
{"message":"use of `unwrap_or` followed by a function call","code":{"code":"clippy::or_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":1197,"byte_end":1214,"line_start":58,"line_end":58,"column_start":14,"column_end":31,"is_primary":true,"text":[{"text":"    with_err.unwrap_or(make());","highlight_start":14,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":1197,"byte_end":1214,"line_start":58,"line_end":58,"column_start":14,"column_end":31,"is_primary":true,"text":[{"text":"    with_err.unwrap_or(make());","highlight_start":14,"highlight_end":31}],"label":null,"suggested_replacement":"unwrap_or_else(|_| make())","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `unwrap_or` followed by a function call\n  --> tests/ui/or_fun_call.rs:58:14\n   |\nLL |     with_err.unwrap_or(make());\n   |              ^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|_| make())`\n\n"}
{"message":"use of `unwrap_or` followed by a function call","code":{"code":"clippy::or_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":1287,"byte_end":1320,"line_start":61,"line_end":61,"column_start":19,"column_end":52,"is_primary":true,"text":[{"text":"    with_err_args.unwrap_or(Vec::with_capacity(12));","highlight_start":19,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":1287,"byte_end":1320,"line_start":61,"line_end":61,"column_start":19,"column_end":52,"is_primary":true,"text":[{"text":"    with_err_args.unwrap_or(Vec::with_capacity(12));","highlight_start":19,"highlight_end":52}],"label":null,"suggested_replacement":"unwrap_or_else(|_| Vec::with_capacity(12))","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `unwrap_or` followed by a function call\n  --> tests/ui/or_fun_call.rs:61:19\n   |\nLL |     with_err_args.unwrap_or(Vec::with_capacity(12));\n   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|_| Vec::with_capacity(12))`\n\n"}
error: test failed, to rerun pass '--test compile-test'
{"message":"use of `unwrap_or` followed by a call to `default`","code":{"code":"clippy::or_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":1365,"byte_end":1413,"line_start":64,"line_end":64,"column_start":5,"column_end":53,"is_primary":true,"text":[{"text":"    with_default_trait.unwrap_or(Default::default());","highlight_start":5,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":1365,"byte_end":1413,"line_start":64,"line_end":64,"column_start":5,"column_end":53,"is_primary":true,"text":[{"text":"    with_default_trait.unwrap_or(Default::default());","highlight_start":5,"highlight_end":53}],"label":null,"suggested_replacement":"with_default_trait.unwrap_or_default()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `unwrap_or` followed by a call to `default`\n  --> tests/ui/or_fun_call.rs:64:5\n   |\nLL |     with_default_trait.unwrap_or(Default::default());\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `with_default_trait.unwrap_or_default()`\n\n"}
{"message":"use of `unwrap_or` followed by a call to `default`","code":{"code":"clippy::or_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":1457,"byte_end":1500,"line_start":67,"line_end":67,"column_start":5,"column_end":48,"is_primary":true,"text":[{"text":"    with_default_type.unwrap_or(u64::default());","highlight_start":5,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":1457,"byte_end":1500,"line_start":67,"line_end":67,"column_start":5,"column_end":48,"is_primary":true,"text":[{"text":"    with_default_type.unwrap_or(u64::default());","highlight_start":5,"highlight_end":48}],"label":null,"suggested_replacement":"with_default_type.unwrap_or_default()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `unwrap_or` followed by a call to `default`\n  --> tests/ui/or_fun_call.rs:67:5\n   |\nLL |     with_default_type.unwrap_or(u64::default());\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `with_default_type.unwrap_or_default()`\n\n"}
{"message":"use of `unwrap_or` followed by a function call","code":{"code":"clippy::or_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":1564,"byte_end":1599,"line_start":70,"line_end":70,"column_start":18,"column_end":53,"is_primary":true,"text":[{"text":"    self_default.unwrap_or(<FakeDefault>::default());","highlight_start":18,"highlight_end":53}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":1564,"byte_end":1599,"line_start":70,"line_end":70,"column_start":18,"column_end":53,"is_primary":true,"text":[{"text":"    self_default.unwrap_or(<FakeDefault>::default());","highlight_start":18,"highlight_end":53}],"label":null,"suggested_replacement":"unwrap_or_else(<FakeDefault>::default)","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `unwrap_or` followed by a function call\n  --> tests/ui/or_fun_call.rs:70:18\n   |\nLL |     self_default.unwrap_or(<FakeDefault>::default());\n   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(<FakeDefault>::default)`\n\n"}
{"message":"use of `unwrap_or` followed by a call to `default`","code":{"code":"clippy::or_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":1650,"byte_end":1709,"line_start":73,"line_end":73,"column_start":5,"column_end":64,"is_primary":true,"text":[{"text":"    real_default.unwrap_or(<FakeDefault as Default>::default());","highlight_start":5,"highlight_end":64}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":1650,"byte_end":1709,"line_start":73,"line_end":73,"column_start":5,"column_end":64,"is_primary":true,"text":[{"text":"    real_default.unwrap_or(<FakeDefault as Default>::default());","highlight_start":5,"highlight_end":64}],"label":null,"suggested_replacement":"real_default.unwrap_or_default()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `unwrap_or` followed by a call to `default`\n  --> tests/ui/or_fun_call.rs:73:5\n   |\nLL |     real_default.unwrap_or(<FakeDefault as Default>::default());\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `real_default.unwrap_or_default()`\n\n"}
{"message":"use of `unwrap_or` followed by a call to `new`","code":{"code":"clippy::or_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":1750,"byte_end":1776,"line_start":76,"line_end":76,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"    with_vec.unwrap_or(vec![]);","highlight_start":5,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":1750,"byte_end":1776,"line_start":76,"line_end":76,"column_start":5,"column_end":31,"is_primary":true,"text":[{"text":"    with_vec.unwrap_or(vec![]);","highlight_start":5,"highlight_end":31}],"label":null,"suggested_replacement":"with_vec.unwrap_or_default()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `unwrap_or` followed by a call to `new`\n  --> tests/ui/or_fun_call.rs:76:5\n   |\nLL |     with_vec.unwrap_or(vec![]);\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `with_vec.unwrap_or_default()`\n\n"}
{"message":"use of `unwrap_or` followed by a function call","code":{"code":"clippy::or_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":1836,"byte_end":1857,"line_start":79,"line_end":79,"column_start":21,"column_end":42,"is_primary":true,"text":[{"text":"    without_default.unwrap_or(Foo::new());","highlight_start":21,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":1836,"byte_end":1857,"line_start":79,"line_end":79,"column_start":21,"column_end":42,"is_primary":true,"text":[{"text":"    without_default.unwrap_or(Foo::new());","highlight_start":21,"highlight_end":42}],"label":null,"suggested_replacement":"unwrap_or_else(Foo::new)","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `unwrap_or` followed by a function call\n  --> tests/ui/or_fun_call.rs:79:21\n   |\nLL |     without_default.unwrap_or(Foo::new());\n   |                     ^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(Foo::new)`\n\n"}
{"message":"use of `unwrap_or` followed by a function call","code":{"code":"clippy::or_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":2314,"byte_end":2338,"line_start":94,"line_end":94,"column_start":21,"column_end":45,"is_primary":true,"text":[{"text":"    let _ = stringy.unwrap_or(\"\".to_owned());","highlight_start":21,"highlight_end":45}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":2314,"byte_end":2338,"line_start":94,"line_end":94,"column_start":21,"column_end":45,"is_primary":true,"text":[{"text":"    let _ = stringy.unwrap_or(\"\".to_owned());","highlight_start":21,"highlight_end":45}],"label":null,"suggested_replacement":"unwrap_or_else(|| \"\".to_owned())","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `unwrap_or` followed by a function call\n  --> tests/ui/or_fun_call.rs:94:21\n   |\nLL |     let _ = stringy.unwrap_or(\"\".to_owned());\n   |                     ^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| \"\".to_owned())`\n\n"}
{"message":"use of `unwrap_or` followed by a function call","code":{"code":"clippy::or_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":2517,"byte_end":2535,"line_start":102,"line_end":102,"column_start":21,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = Some(1).unwrap_or(map[&1]);","highlight_start":21,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":2517,"byte_end":2535,"line_start":102,"line_end":102,"column_start":21,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = Some(1).unwrap_or(map[&1]);","highlight_start":21,"highlight_end":39}],"label":null,"suggested_replacement":"unwrap_or_else(|| map[&1])","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `unwrap_or` followed by a function call\n  --> tests/ui/or_fun_call.rs:102:21\n   |\nLL |     let _ = Some(1).unwrap_or(map[&1]);\n   |                     ^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| map[&1])`\n\n"}
{"message":"use of `unwrap_or` followed by a function call","code":{"code":"clippy::or_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":2600,"byte_end":2618,"line_start":104,"line_end":104,"column_start":21,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = Some(1).unwrap_or(map[&1]);","highlight_start":21,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":2600,"byte_end":2618,"line_start":104,"line_end":104,"column_start":21,"column_end":39,"is_primary":true,"text":[{"text":"    let _ = Some(1).unwrap_or(map[&1]);","highlight_start":21,"highlight_end":39}],"label":null,"suggested_replacement":"unwrap_or_else(|| map[&1])","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `unwrap_or` followed by a function call\n  --> tests/ui/or_fun_call.rs:104:21\n   |\nLL |     let _ = Some(1).unwrap_or(map[&1]);\n   |                     ^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| map[&1])`\n\n"}
{"message":"use of `or` followed by a function call","code":{"code":"clippy::or_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":3258,"byte_end":3283,"line_start":128,"line_end":128,"column_start":35,"column_end":60,"is_primary":true,"text":[{"text":"    let _ = Some(\"a\".to_string()).or(Some(\"b\".to_string()));","highlight_start":35,"highlight_end":60}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":3258,"byte_end":3283,"line_start":128,"line_end":128,"column_start":35,"column_end":60,"is_primary":true,"text":[{"text":"    let _ = Some(\"a\".to_string()).or(Some(\"b\".to_string()));","highlight_start":35,"highlight_end":60}],"label":null,"suggested_replacement":"or_else(|| Some(\"b\".to_string()))","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `or` followed by a function call\n  --> tests/ui/or_fun_call.rs:128:35\n   |\nLL |     let _ = Some(\"a\".to_string()).or(Some(\"b\".to_string()));\n   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `or_else(|| Some(\"b\".to_string()))`\n\n"}
{"message":"use of `unwrap_or` followed by a function call","code":{"code":"clippy::or_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":4146,"byte_end":4170,"line_start":167,"line_end":167,"column_start":14,"column_end":38,"is_primary":true,"text":[{"text":"        None.unwrap_or(ptr_to_ref(s));","highlight_start":14,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":4146,"byte_end":4170,"line_start":167,"line_end":167,"column_start":14,"column_end":38,"is_primary":true,"text":[{"text":"        None.unwrap_or(ptr_to_ref(s));","highlight_start":14,"highlight_end":38}],"label":null,"suggested_replacement":"unwrap_or_else(|| ptr_to_ref(s))","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `unwrap_or` followed by a function call\n  --> tests/ui/or_fun_call.rs:167:14\n   |\nLL |         None.unwrap_or(ptr_to_ref(s));\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| ptr_to_ref(s))`\n\n"}
{"message":"use of `unwrap_or` followed by a function call","code":{"code":"clippy::or_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":4274,"byte_end":4309,"line_start":173,"line_end":173,"column_start":14,"column_end":49,"is_primary":true,"text":[{"text":"        None.unwrap_or(unsafe { ptr_to_ref(s) });","highlight_start":14,"highlight_end":49}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":4274,"byte_end":4309,"line_start":173,"line_end":173,"column_start":14,"column_end":49,"is_primary":true,"text":[{"text":"        None.unwrap_or(unsafe { ptr_to_ref(s) });","highlight_start":14,"highlight_end":49}],"label":null,"suggested_replacement":"unwrap_or_else(|| unsafe { ptr_to_ref(s) })","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `unwrap_or` followed by a function call\n  --> tests/ui/or_fun_call.rs:173:14\n   |\nLL |         None.unwrap_or(unsafe { ptr_to_ref(s) });\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| unsafe { ptr_to_ref(s) })`\n\n"}
{"message":"use of `unwrap_or` followed by a function call","code":{"code":"clippy::or_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":4349,"byte_end":4389,"line_start":175,"line_end":175,"column_start":14,"column_end":54,"is_primary":true,"text":[{"text":"        None.unwrap_or( unsafe { ptr_to_ref(s) }    );","highlight_start":14,"highlight_end":54}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":4349,"byte_end":4389,"line_start":175,"line_end":175,"column_start":14,"column_end":54,"is_primary":true,"text":[{"text":"        None.unwrap_or( unsafe { ptr_to_ref(s) }    );","highlight_start":14,"highlight_end":54}],"label":null,"suggested_replacement":"unwrap_or_else(|| unsafe { ptr_to_ref(s) })","suggestion_applicability":"HasPlaceholders","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `unwrap_or` followed by a function call\n  --> tests/ui/or_fun_call.rs:175:14\n   |\nLL |         None.unwrap_or( unsafe { ptr_to_ref(s) }    );\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_else(|| unsafe { ptr_to_ref(s) })`\n\n"}
{"message":"use of `unwrap_or` followed by a call to `new`","code":{"code":"clippy::or_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":4701,"byte_end":4725,"line_start":189,"line_end":189,"column_start":14,"column_end":38,"is_primary":true,"text":[{"text":"            .unwrap_or(String::new());","highlight_start":14,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":4701,"byte_end":4725,"line_start":189,"line_end":189,"column_start":14,"column_end":38,"is_primary":true,"text":[{"text":"            .unwrap_or(String::new());","highlight_start":14,"highlight_end":38}],"label":null,"suggested_replacement":"unwrap_or_default()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `unwrap_or` followed by a call to `new`\n  --> tests/ui/or_fun_call.rs:189:14\n   |\nLL |             .unwrap_or(String::new());\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_default()`\n\n"}
{"message":"use of `unwrap_or` followed by a call to `new`","code":{"code":"clippy::or_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":5072,"byte_end":5096,"line_start":202,"line_end":202,"column_start":14,"column_end":38,"is_primary":true,"text":[{"text":"            .unwrap_or(String::new());","highlight_start":14,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":5072,"byte_end":5096,"line_start":202,"line_end":202,"column_start":14,"column_end":38,"is_primary":true,"text":[{"text":"            .unwrap_or(String::new());","highlight_start":14,"highlight_end":38}],"label":null,"suggested_replacement":"unwrap_or_default()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `unwrap_or` followed by a call to `new`\n  --> tests/ui/or_fun_call.rs:202:14\n   |\nLL |             .unwrap_or(String::new());\n   |              ^^^^^^^^^^^^^^^^^^^^^^^^ help: try this: `unwrap_or_default()`\n\n"}
{"message":"use of `unwrap_or` followed by a call to `new`","code":{"code":"clippy::or_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":5229,"byte_end":5438,"line_start":208,"line_end":214,"column_start":9,"column_end":38,"is_primary":true,"text":[{"text":"        iter.map(|f: &String| f.to_lowercase())","highlight_start":9,"highlight_end":48},{"text":"            .reduce(|mut acc, f| {","highlight_start":1,"highlight_end":35},{"text":"                let _ = \"\";","highlight_start":1,"highlight_end":28},{"text":"                acc.push_str(&f);","highlight_start":1,"highlight_end":34},{"text":"                acc","highlight_start":1,"highlight_end":20},{"text":"            })","highlight_start":1,"highlight_end":15},{"text":"            .unwrap_or(String::new());","highlight_start":1,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":5229,"byte_end":5438,"line_start":208,"line_end":214,"column_start":9,"column_end":38,"is_primary":true,"text":[{"text":"        iter.map(|f: &String| f.to_lowercase())","highlight_start":9,"highlight_end":48},{"text":"            .reduce(|mut acc, f| {","highlight_start":1,"highlight_end":35},{"text":"                let _ = \"\";","highlight_start":1,"highlight_end":28},{"text":"                acc.push_str(&f);","highlight_start":1,"highlight_end":34},{"text":"                acc","highlight_start":1,"highlight_end":20},{"text":"            })","highlight_start":1,"highlight_end":15},{"text":"            .unwrap_or(String::new());","highlight_start":1,"highlight_end":38}],"label":null,"suggested_replacement":"iter.map(|f: &String| f.to_lowercase())\n            .reduce(|mut acc, f| {\n                let _ = \"\";\n                acc.push_str(&f);\n                acc\n            }).unwrap_or_default()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `unwrap_or` followed by a call to `new`\n  --> tests/ui/or_fun_call.rs:208:9\n   |\nLL | /         iter.map(|f: &String| f.to_lowercase())\nLL | |             .reduce(|mut acc, f| {\nLL | |                 let _ = \"\";\nLL | |                 acc.push_str(&f);\nLL | |                 acc\nLL | |             })\nLL | |             .unwrap_or(String::new());\n   | |_____________________________________^\n   |\nhelp: try this\n   |\nLL ~         iter.map(|f: &String| f.to_lowercase())\nLL +             .reduce(|mut acc, f| {\nLL +                 let _ = \"\";\nLL +                 acc.push_str(&f);\nLL +                 acc\nLL ~             }).unwrap_or_default();\n   |\n\n"}
{"message":"use of `unwrap_or` followed by a call to `new`","code":{"code":"clippy::or_fun_call","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":5631,"byte_end":5747,"line_start":221,"line_end":225,"column_start":9,"column_end":34,"is_primary":true,"text":[{"text":"        map.reduce(|mut acc, f| {","highlight_start":9,"highlight_end":34},{"text":"            acc.push_str(&f);","highlight_start":1,"highlight_end":30},{"text":"            acc","highlight_start":1,"highlight_end":16},{"text":"        })","highlight_start":1,"highlight_end":11},{"text":"        .unwrap_or(String::new());","highlight_start":1,"highlight_end":34}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try this","code":null,"level":"help","spans":[{"file_name":"tests/ui/or_fun_call.rs","byte_start":5631,"byte_end":5747,"line_start":221,"line_end":225,"column_start":9,"column_end":34,"is_primary":true,"text":[{"text":"        map.reduce(|mut acc, f| {","highlight_start":9,"highlight_end":34},{"text":"            acc.push_str(&f);","highlight_start":1,"highlight_end":30},{"text":"            acc","highlight_start":1,"highlight_end":16},{"text":"        })","highlight_start":1,"highlight_end":11},{"text":"        .unwrap_or(String::new());","highlight_start":1,"highlight_end":34}],"label":null,"suggested_replacement":"map.reduce(|mut acc, f| {\n            acc.push_str(&f);\n            acc\n        }).unwrap_or_default()","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: use of `unwrap_or` followed by a call to `new`\n  --> tests/ui/or_fun_call.rs:221:9\n   |\nLL | /         map.reduce(|mut acc, f| {\nLL | |             acc.push_str(&f);\nLL | |             acc\nLL | |         })\nLL | |         .unwrap_or(String::new());\n   | |_________________________________^\n   |\nhelp: try this\n   |\nLL ~         map.reduce(|mut acc, f| {\nLL +             acc.push_str(&f);\nLL +             acc\nLL ~         }).unwrap_or_default();\n   |\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.8.0/src/lib.rs:111:22
