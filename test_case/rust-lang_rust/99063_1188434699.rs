plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between ed9173276a176126150b4c684a4262a135ce51ef and d53ae20f2f0ded4cd677e95a7ff6d52ba2c82725
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---

+error: the feature `lint_reasons` has been stable since 1.64.0 and no longer requires an attribute to enable
+  --> $DIR/let_unit.rs:3:12
+   |
+LL | #![feature(lint_reasons)]
+   |
+   |
+   = note: `-D stable-features` implied by `-D warnings`
 error: this let-binding has unit value
   --> $DIR/let_unit.rs:14:5
    |
 LL |     let _x = println!("x");
 LL |     let _x = println!("x");
    |     ^^^^^^^^^^^^^^^^^^^^^^^ help: omit the `let` binding: `println!("x");`
    |
    = note: `-D clippy::let-unit-value` implied by `-D warnings`
 error: this let-binding has unit value
   --> $DIR/let_unit.rs:18:9
    |
    |
 LL |         let _a = ();
    |         ^^^^^^^^^^^^ help: omit the `let` binding: `();`
 error: this let-binding has unit value
   --> $DIR/let_unit.rs:53:5
    |
 LL | /     let _ = v
 LL | /     let _ = v
 LL | |         .into_iter()
 LL | |         .map(|i| i * 2)
 LL | |         .filter(|i| i % 2 == 0)
 LL | |         .map(|_| ())
 LL | |         .next()
 LL | |         .unwrap();
    |
 help: omit the `let` binding
    |
 LL ~     v
 LL ~     v
 LL +         .into_iter()
 LL +         .map(|i| i * 2)
 LL +         .filter(|i| i % 2 == 0)
 LL +         .map(|_| ())
 LL +         .next()
 LL +         .unwrap();
 
 error: this let-binding has unit value
   --> $DIR/let_unit.rs:80:5
    |
    |
 LL |     let x: () = f(); // Lint.
    |     ^^^^-^^^^^^^^^^^
    |         |
    |         help: use a wild (`_`) binding: `_`
 error: this let-binding has unit value
   --> $DIR/let_unit.rs:83:5
    |
    |
 LL |     let x: () = f2(0i32); // Lint.
    |     ^^^^-^^^^^^^^^^^^^^^^
    |         |
    |         help: use a wild (`_`) binding: `_`
 error: this let-binding has unit value
   --> $DIR/let_unit.rs:85:5
    |
    |
 LL |     let _: () = f3(()); // Lint
    |     ^^^^^^^^^^^^^^^^^^^ help: omit the `let` binding: `f3(());`
 error: this let-binding has unit value
   --> $DIR/let_unit.rs:86:5
    |
    |
 LL |     let x: () = f3(()); // Lint
    |     ^^^^^^^^^^^^^^^^^^^ help: omit the `let` binding: `f3(());`
 error: this let-binding has unit value
   --> $DIR/let_unit.rs:102:5
    |
    |
 LL |     let x: () = if true { f() } else { f2(0) }; // Lint
    |     ^^^^-^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |         |
    |         help: use a wild (`_`) binding: `_`
 error: this let-binding has unit value
   --> $DIR/let_unit.rs:113:5
    |
    |
 LL | /     let _: () = match Some(0) {
 LL | |         None => f2(1),
 LL | |         Some(0) => f(),
 LL | |         Some(1) => f2(3),
 LL | |         Some(_) => (),
 LL | |     };
    |
 help: omit the `let` binding
    |
    |
 LL ~     match Some(0) {
 LL +         None => f2(1),
 LL +         Some(0) => f(),
 LL +         Some(1) => f2(3),
 LL +         Some(_) => (),
 LL +     };
 
 error: this let-binding has unit value
   --> $DIR/let_unit.rs:160:13
    |
    |
 LL |             let _: () = z;
    |             ^^^^^^^^^^^^^^ help: omit the `let` binding: `z;`
-error: aborting due to 10 previous errors
+error: aborting due to 11 previous errors
 
 
---
To only update this specific test, also pass `--test-args let_unit.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/let_unit.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/let_unit.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-7d13fa063f867ef0.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-dcc59fbd39a40970.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-021aec868151835c.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-46430df947b7dad0.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-cdd893c121eb00e4.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-71205fa4273edf27.so" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-5411643be5ff72c2.so" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-aceff80e643e9fe7.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-61ecef333190a996.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-ff530e80824c3a36.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-b92911696ae4394a.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-0c795f7a8756f15a.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/let_unit.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"the feature `lint_reasons` has been stable since 1.64.0 and no longer requires an attribute to enable","code":{"code":"stable_features","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/let_unit.rs","byte_start":27,"byte_end":39,"line_start":3,"line_end":3,"column_start":12,"column_end":24,"is_primary":true,"text":[{"text":"#![feature(lint_reasons)]","highlight_start":12,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D stable-features` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: the feature `lint_reasons` has been stable since 1.64.0 and no longer requires an attribute to enable\n  --> tests/ui/let_unit.rs:3:12\n   |\nLL | #![feature(lint_reasons)]\n   |            ^^^^^^^^^^^^\n   |\n   = note: `-D stable-features` implied by `-D warnings`\n\n"}
{"message":"this let-binding has unit value","code":{"code":"clippy::let_unit_value","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/let_unit.rs","byte_start":257,"byte_end":280,"line_start":14,"line_end":14,"column_start":5,"column_end":28,"is_primary":true,"text":[{"text":"    let _x = println!(\"x\");","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::let-unit-value` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"omit the `let` binding","code":null,"level":"help","spans":[{"file_name":"tests/ui/let_unit.rs","byte_start":257,"byte_end":280,"line_start":14,"line_end":14,"column_start":5,"column_end":28,"is_primary":true,"text":[{"text":"    let _x = println!(\"x\");","highlight_start":5,"highlight_end":28}],"label":null,"suggested_replacement":"println!(\"x\");","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this let-binding has unit value\n  --> tests/ui/let_unit.rs:14:5\n   |\nLL |     let _x = println!(\"x\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^ help: omit the `let` binding: `println!(\"x\");`\n   |\n   = note: `-D clippy::let-unit-value` implied by `-D warnings`\n\n"}
{"message":"this let-binding has unit value","code":{"code":"clippy::let_unit_value","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/let_unit.rs","byte_start":373,"byte_end":385,"line_start":18,"line_end":18,"column_start":9,"column_end":21,"is_primary":true,"text":[{"text":"        let _a = ();","highlight_start":9,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"omit the `let` binding","code":null,"level":"help","spans":[{"file_name":"tests/ui/let_unit.rs","byte_start":373,"byte_end":385,"line_start":18,"line_end":18,"column_start":9,"column_end":21,"is_primary":true,"text":[{"text":"        let _a = ();","highlight_start":9,"highlight_end":21}],"label":null,"suggested_replacement":"();","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this let-binding has unit value\n  --> tests/ui/let_unit.rs:18:9\n   |\nLL |         let _a = ();\n   |         ^^^^^^^^^^^^ help: omit the `let` binding: `();`\n\n"}
{"message":"this let-binding has unit value","code":{"code":"clippy::let_unit_value","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/let_unit.rs","byte_start":1097,"byte_end":1239,"line_start":53,"line_end":59,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    let _ = v","highlight_start":5,"highlight_end":14},{"text":"        .into_iter()","highlight_start":1,"highlight_end":21},{"text":"        .map(|i| i * 2)","highlight_start":1,"highlight_end":24},{"text":"        .filter(|i| i % 2 == 0)","highlight_start":1,"highlight_end":32},{"text":"        .map(|_| ())","highlight_start":1,"highlight_end":21},{"text":"        .next()","highlight_start":1,"highlight_end":16},{"text":"        .unwrap();","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"omit the `let` binding","code":null,"level":"help","spans":[{"file_name":"tests/ui/let_unit.rs","byte_start":1097,"byte_end":1239,"line_start":53,"line_end":59,"column_start":5,"column_end":19,"is_primary":true,"text":[{"text":"    let _ = v","highlight_start":5,"highlight_end":14},{"text":"        .into_iter()","highlight_start":1,"highlight_end":21},{"text":"        .map(|i| i * 2)","highlight_start":1,"highlight_end":24},{"text":"        .filter(|i| i % 2 == 0)","highlight_start":1,"highlight_end":32},{"text":"        .map(|_| ())","highlight_start":1,"highlight_end":21},{"text":"        .next()","highlight_start":1,"highlight_end":16},{"text":"        .unwrap();","highlight_start":1,"highlight_end":19}],"label":null,"suggested_replacement":"v\n        .into_iter()\n        .map(|i| i * 2)\n        .filter(|i| i % 2 == 0)\n        .map(|_| ())\n        .next()\n        .unwrap();","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this let-binding has unit value\n  --> tests/ui/let_unit.rs:53:5\n   |\nLL | /     let _ = v\nLL | |         .into_iter()\nLL | |         .map(|i| i * 2)\nLL | |         .filter(|i| i % 2 == 0)\nLL | |         .map(|_| ())\nLL | |         .next()\nLL | |         .unwrap();\n   | |__________________^\n   |\nhelp: omit the `let` binding\n   |\nLL ~     v\nLL +         .into_iter()\nLL +         .map(|i| i * 2)\nLL +         .filter(|i| i % 2 == 0)\nLL +         .map(|_| ())\nLL +         .next()\nLL +         .unwrap();\n   |\n\n"}
{"message":"this let-binding has unit value","code":{"code":"clippy::let_unit_value","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/let_unit.rs","byte_start":1608,"byte_end":1624,"line_start":80,"line_end":80,"column_start":5,"column_end":21,"is_primary":true,"text":[{"text":"    let x: () = f(); // Lint.","highlight_start":5,"highlight_end":21}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use a wild (`_`) binding","code":null,"level":"help","spans":[{"file_name":"tests/ui/let_unit.rs","byte_start":1612,"byte_end":1613,"line_start":80,"line_end":80,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let x: () = f(); // Lint.","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"_","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this let-binding has unit value\n  --> tests/ui/let_unit.rs:80:5\n   |\nLL |     let x: () = f(); // Lint.\n   |     ^^^^-^^^^^^^^^^^\n   |         |\n   |         help: use a wild (`_`) binding: `_`\n\n"}
{"message":"this let-binding has unit value","code":{"code":"clippy::let_unit_value","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/let_unit.rs","byte_start":1671,"byte_end":1692,"line_start":83,"line_end":83,"column_start":5,"column_end":26,"is_primary":true,"text":[{"text":"    let x: () = f2(0i32); // Lint.","highlight_start":5,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use a wild (`_`) binding","code":null,"level":"help","spans":[{"file_name":"tests/ui/let_unit.rs","byte_start":1675,"byte_end":1676,"line_start":83,"line_end":83,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let x: () = f2(0i32); // Lint.","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"_","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this let-binding has unit value\n  --> tests/ui/let_unit.rs:83:5\n   |\nLL |     let x: () = f2(0i32); // Lint.\n   |     ^^^^-^^^^^^^^^^^^^^^^\n   |         |\n   |         help: use a wild (`_`) binding: `_`\n\n"}
{"message":"this let-binding has unit value","code":{"code":"clippy::let_unit_value","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/let_unit.rs","byte_start":1707,"byte_end":1726,"line_start":85,"line_end":85,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    let _: () = f3(()); // Lint","highlight_start":5,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"omit the `let` binding","code":null,"level":"help","spans":[{"file_name":"tests/ui/let_unit.rs","byte_start":1707,"byte_end":1726,"line_start":85,"line_end":85,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    let _: () = f3(()); // Lint","highlight_start":5,"highlight_end":24}],"label":null,"suggested_replacement":"f3(());","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this let-binding has unit value\n  --> tests/ui/let_unit.rs:85:5\n   |\nLL |     let _: () = f3(()); // Lint\n   |     ^^^^^^^^^^^^^^^^^^^ help: omit the `let` binding: `f3(());`\n\n"}
{"message":"this let-binding has unit value","code":{"code":"clippy::let_unit_value","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/let_unit.rs","byte_start":1739,"byte_end":1758,"line_start":86,"line_end":86,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    let x: () = f3(()); // Lint","highlight_start":5,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"omit the `let` binding","code":null,"level":"help","spans":[{"file_name":"tests/ui/let_unit.rs","byte_start":1739,"byte_end":1758,"line_start":86,"line_end":86,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    let x: () = f3(()); // Lint","highlight_start":5,"highlight_end":24}],"label":null,"suggested_replacement":"f3(());","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this let-binding has unit value\n  --> tests/ui/let_unit.rs:86:5\n   |\nLL |     let x: () = f3(()); // Lint\n   |     ^^^^^^^^^^^^^^^^^^^ help: omit the `let` binding: `f3(());`\n\n"}
{"message":"this let-binding has unit value","code":{"code":"clippy::let_unit_value","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/let_unit.rs","byte_start":2056,"byte_end":2099,"line_start":102,"line_end":102,"column_start":5,"column_end":48,"is_primary":true,"text":[{"text":"    let x: () = if true { f() } else { f2(0) }; // Lint","highlight_start":5,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use a wild (`_`) binding","code":null,"level":"help","spans":[{"file_name":"tests/ui/let_unit.rs","byte_start":2060,"byte_end":2061,"line_start":102,"line_end":102,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    let x: () = if true { f() } else { f2(0) }; // Lint","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":"_","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this let-binding has unit value\n  --> tests/ui/let_unit.rs:102:5\n   |\nLL |     let x: () = if true { f() } else { f2(0) }; // Lint\n   |     ^^^^-^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |         |\n   |         help: use a wild (`_`) binding: `_`\n\n"}
{"message":"this let-binding has unit value","code":{"code":"clippy::let_unit_value","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/let_unit.rs","byte_start":2276,"byte_end":2406,"line_start":113,"line_end":118,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    let _: () = match Some(0) {","highlight_start":5,"highlight_end":32},{"text":"        None => f2(1),","highlight_start":1,"highlight_end":23},{"text":"        Some(0) => f(),","highlight_start":1,"highlight_end":24},{"text":"        Some(1) => f2(3),","highlight_start":1,"highlight_end":26},{"text":"        Some(_) => (),","highlight_start":1,"highlight_end":23},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"omit the `let` binding","code":null,"level":"help","spans":[{"file_name":"tests/ui/let_unit.rs","byte_start":2276,"byte_end":2406,"line_start":113,"line_end":118,"column_start":5,"column_end":7,"is_primary":true,"text":[{"text":"    let _: () = match Some(0) {","highlight_start":5,"highlight_end":32},{"text":"        None => f2(1),","highlight_start":1,"highlight_end":23},{"text":"        Some(0) => f(),","highlight_start":1,"highlight_end":24},{"text":"        Some(1) => f2(3),","highlight_start":1,"highlight_end":26},{"text":"        Some(_) => (),","highlight_start":1,"highlight_end":23},{"text":"    };","highlight_start":1,"highlight_end":7}],"label":null,"suggested_replacement":"match Some(0) {\n        None => f2(1),\n        Some(0) => f(),\n        Some(1) => f2(3),\n        Some(_) => (),\n    };","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this let-binding has unit value\n  --> tests/ui/let_unit.rs:113:5\n   |\nLL | /     let _: () = match Some(0) {\nLL | |         None => f2(1),\nLL | |         Some(0) => f(),\nLL | |         Some(1) => f2(3),\nLL | |         Some(_) => (),\nLL | |     };\n   | |______^\n   |\nhelp: omit the `let` binding\n   |\nLL ~     match Some(0) {\nLL +         None => f2(1),\nLL +         Some(0) => f(),\nLL +         Some(1) => f2(3),\nLL +         Some(_) => (),\nLL +     };\n   |\n\n"}
{"message":"this let-binding has unit value","code":{"code":"clippy::let_unit_value","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/let_unit.rs","byte_start":3195,"byte_end":3209,"line_start":160,"line_end":160,"column_start":13,"column_end":27,"is_primary":true,"text":[{"text":"            let _: () = z;","highlight_start":13,"highlight_end":27}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"omit the `let` binding","code":null,"level":"help","spans":[{"file_name":"tests/ui/let_unit.rs","byte_start":3195,"byte_end":3209,"line_start":160,"line_end":160,"column_start":13,"column_end":27,"is_primary":true,"text":[{"text":"            let _: () = z;","highlight_start":13,"highlight_end":27}],"label":null,"suggested_replacement":"z;","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this let-binding has unit value\n  --> tests/ui/let_unit.rs:160:13\n   |\nLL |             let _: () = z;\n   |             ^^^^^^^^^^^^^^ help: omit the `let` binding: `z;`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.8.0/src/lib.rs:111:22
