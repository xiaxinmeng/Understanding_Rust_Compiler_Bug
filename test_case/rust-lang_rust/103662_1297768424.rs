plain

---- compile_test stdout ----
diff of stderr:

 error: `Iterator::step_by(0)` will panic at runtime
    |
    |
 LL |     let _ = vec!["A", "B", "B"].iter().step_by(0);
    |
    |
    = note: `-D clippy::iterator-step-by-zero` implied by `-D warnings`
 
 error: `Iterator::step_by(0)` will panic at runtime
    |
    |
 LL |     let _ = "XXX".chars().step_by(0);
 
 
 error: `Iterator::step_by(0)` will panic at runtime
-   |
-   |
-LL |     let _ = (0..1).step_by(0);
-
-
-error: `Iterator::step_by(0)` will panic at runtime
-   |
-   |
-LL |     let _ = (1..).step_by(0);
-
-
-error: `Iterator::step_by(0)` will panic at runtime
-   |
-   |
-LL |     let _ = (1..=2).step_by(0);
-
-
-error: `Iterator::step_by(0)` will panic at runtime
    |
    |
 LL |     let _ = x.step_by(0);
 
 
 error: `Iterator::step_by(0)` will panic at runtime
    |
    |
 LL |     let _ = v1.iter().step_by(2 / 3);
 
-error: aborting due to 7 previous errors
+error: aborting due to 4 previous errors
 
---
To only update this specific test, also pass `--test-args iterator_step_by_zero.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/iterator_step_by_zero.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/iterator_step_by_zero.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-f2da228ff241ec97.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-b9c731f380466bd0.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-97618c8d1e1f91be.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-3ef490519219f110.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-e95902f0f3ef680a.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-5ac9fbce59ab185e.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-f474b2dfd384a595.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-4ace64e065702c69.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-8ab3fb021f549e8b.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/iterator_step_by_zero.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"`Iterator::step_by(0)` will panic at runtime","code":{"code":"clippy::iterator_step_by_zero","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/iterator_step_by_zero.rs","byte_start":63,"byte_end":100,"line_start":3,"line_end":3,"column_start":13,"column_end":50,"is_primary":true,"text":[{"text":"    let _ = vec![\"A\", \"B\", \"B\"].iter().step_by(0);","highlight_start":13,"highlight_end":50}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::iterator-step-by-zero` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: `Iterator::step_by(0)` will panic at runtime\n  --> tests/ui/iterator_step_by_zero.rs:3:13\n   |\nLL |     let _ = vec![\"A\", \"B\", \"B\"].iter().step_by(0);\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D clippy::iterator-step-by-zero` implied by `-D warnings`\n\n"}
{"message":"`Iterator::step_by(0)` will panic at runtime","code":{"code":"clippy::iterator_step_by_zero","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/iterator_step_by_zero.rs","byte_start":114,"byte_end":138,"line_start":4,"line_end":4,"column_start":13,"column_end":37,"is_primary":true,"text":[{"text":"    let _ = \"XXX\".chars().step_by(0);","highlight_start":13,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Iterator::step_by(0)` will panic at runtime\n  --> tests/ui/iterator_step_by_zero.rs:4:13\n   |\nLL |     let _ = \"XXX\".chars().step_by(0);\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"`Iterator::step_by(0)` will panic at runtime","code":{"code":"clippy::iterator_step_by_zero","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/iterator_step_by_zero.rs","byte_start":411,"byte_end":423,"line_start":18,"line_end":18,"column_start":13,"column_end":25,"is_primary":true,"text":[{"text":"    let _ = x.step_by(0);","highlight_start":13,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Iterator::step_by(0)` will panic at runtime\n  --> tests/ui/iterator_step_by_zero.rs:18:13\n   |\nLL |     let _ = x.step_by(0);\n   |             ^^^^^^^^^^^^\n\n"}
{"message":"`Iterator::step_by(0)` will panic at runtime","code":{"code":"clippy::iterator_step_by_zero","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/iterator_step_by_zero.rs","byte_start":490,"byte_end":514,"line_start":22,"line_end":22,"column_start":13,"column_end":37,"is_primary":true,"text":[{"text":"    let _ = v1.iter().step_by(2 / 3);","highlight_start":13,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `Iterator::step_by(0)` will panic at runtime\n  --> tests/ui/iterator_step_by_zero.rs:22:13\n   |\nLL |     let _ = v1.iter().step_by(2 / 3);\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: called `skip(..).next()` on an iterator
    |
    |
 LL |     let _ = some_vec.iter().skip(42).next();
    |                            ^^^^^^^^^^^^^^^^ help: use `nth` instead: `.nth(42)`
    |
    = note: `-D clippy::iter-skip-next` implied by `-D warnings`
 
 error: called `skip(..).next()` on an iterator
    |
    |
 LL |     let _ = some_vec.iter().cycle().skip(42).next();
    |                                    ^^^^^^^^^^^^^^^^ help: use `nth` instead: `.nth(42)`
 
 error: called `skip(..).next()` on an iterator
-   |
-   |
-LL |     let _ = (1..10).skip(10).next();
-   |                    ^^^^^^^^^^^^^^^^ help: use `nth` instead: `.nth(10)`
-
-error: called `skip(..).next()` on an iterator
    |
    |
 LL |     let _ = &some_vec[..].iter().skip(3).next();
    |                                 ^^^^^^^^^^^^^^^ help: use `nth` instead: `.nth(3)`
 
 error: called `skip(..).next()` on an iterator
    |
    |
 LL |     let _: Vec<&str> = sp.skip(1).next().unwrap().split(' ').collect();
    |                          ^^^^^^^^^^^^^^^ help: use `nth` instead: `.nth(1)`
 
 error: called `skip(..).next()` on an iterator
    |
    |
 LL |         let _: Vec<&str> = s.skip(1).next().unwrap().split(' ').collect();
    |                             ^^^^^^^^^^^^^^^ help: use `nth` instead: `.nth(1)`
 
 error: called `skip(..).next()` on an iterator
    |
    |
 LL |         let _: Vec<&str> = s.skip(1).next().unwrap().split(' ').collect();
    |                             ^^^^^^^^^^^^^^^ help: use `nth` instead: `.nth(1)`
-error: aborting due to 7 previous errors
+error: aborting due to 6 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/iter_skip_next.stage-id.stderr
diff of fixed:

 // run-rustfix
 // aux-build:option_helpers.rs
 
 #![warn(clippy::iter_skip_next)]
 #![allow(clippy::disallowed_names)]
 #![allow(clippy::iter_nth)]
 #![allow(unused_mut, dead_code)]
 extern crate option_helpers;
 
 
 use option_helpers::IteratorFalsePositives;
 
 /// Checks implementation of `ITER_SKIP_NEXT` lint
 fn main() {
     let some_vec = vec![0, 1, 2, 3];
     let _ = some_vec.iter().nth(42);
     let _ = some_vec.iter().cycle().nth(42);
-    let _ = (1..10).nth(10);
+    let _ = (1..10).skip(10).next();
     let _ = &some_vec[..].iter().nth(3);
     let foo = IteratorFalsePositives { foo: 0 };
     let _ = foo.skip(42).next();
     let _ = foo.filter().skip(42).next();
     // fix #8128
     let test_string = "1|1 2";
     let test_string = "1|1 2";
     let mut sp = test_string.split('|').map(|s| s.trim());
     let _: Vec<&str> = sp.nth(1).unwrap().split(' ').collect();
     if let Some(mut s) = Some(test_string.split('|').map(|s| s.trim())) {
         let _: Vec<&str> = s.nth(1).unwrap().split(' ').collect();
     };
     fn check<T>(mut s: T)
     where
         T: Iterator<Item = String>,
     {
         let _: Vec<&str> = s.nth(1).unwrap().split(' ').collect();
 }
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/iter_skip_next.stage-id.fixed
To only update this specific test, also pass `--test-args iter_skip_next.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/iter_skip_next.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/iter_skip_next.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-f2da228ff241ec97.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-b9c731f380466bd0.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-97618c8d1e1f91be.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-3ef490519219f110.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-e95902f0f3ef680a.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-5ac9fbce59ab185e.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-f474b2dfd384a595.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-4ace64e065702c69.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-8ab3fb021f549e8b.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/iter_skip_next.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"called `skip(..).next()` on an iterator","code":{"code":"clippy::iter_skip_next","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/iter_skip_next.rs","byte_start":380,"byte_end":396,"line_start":16,"line_end":16,"column_start":28,"column_end":44,"is_primary":true,"text":[{"text":"    let _ = some_vec.iter().skip(42).next();","highlight_start":28,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::iter-skip-next` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"use `nth` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/iter_skip_next.rs","byte_start":380,"byte_end":396,"line_start":16,"line_end":16,"column_start":28,"column_end":44,"is_primary":true,"text":[{"text":"    let _ = some_vec.iter().skip(42).next();","highlight_start":28,"highlight_end":44}],"label":null,"suggested_replacement":".nth(42)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `skip(..).next()` on an iterator\n  --> tests/ui/iter_skip_next.rs:16:28\n   |\nLL |     let _ = some_vec.iter().skip(42).next();\n   |                            ^^^^^^^^^^^^^^^^ help: use `nth` instead: `.nth(42)`\n   |\n   = note: `-D clippy::iter-skip-next` implied by `-D warnings`\n\n"}
{"message":"called `skip(..).next()` on an iterator","code":{"code":"clippy::iter_skip_next","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/iter_skip_next.rs","byte_start":433,"byte_end":449,"line_start":17,"line_end":17,"column_start":36,"column_end":52,"is_primary":true,"text":[{"text":"    let _ = some_vec.iter().cycle().skip(42).next();","highlight_start":36,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `nth` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/iter_skip_next.rs","byte_start":433,"byte_end":449,"line_start":17,"line_end":17,"column_start":36,"column_end":52,"is_primary":true,"text":[{"text":"    let _ = some_vec.iter().cycle().skip(42).next();","highlight_start":36,"highlight_end":52}],"label":null,"suggested_replacement":".nth(42)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `skip(..).next()` on an iterator\n  --> tests/ui/iter_skip_next.rs:17:36\n   |\nLL |     let _ = some_vec.iter().cycle().skip(42).next();\n   |                                    ^^^^^^^^^^^^^^^^ help: use `nth` instead: `.nth(42)`\n\n"}
{"message":"called `skip(..).next()` on an iterator","code":{"code":"clippy::iter_skip_next","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/iter_skip_next.rs","byte_start":520,"byte_end":535,"line_start":19,"line_end":19,"column_start":33,"column_end":48,"is_primary":true,"text":[{"text":"    let _ = &some_vec[..].iter().skip(3).next();","highlight_start":33,"highlight_end":48}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `nth` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/iter_skip_next.rs","byte_start":520,"byte_end":535,"line_start":19,"line_end":19,"column_start":33,"column_end":48,"is_primary":true,"text":[{"text":"    let _ = &some_vec[..].iter().skip(3).next();","highlight_start":33,"highlight_end":48}],"label":null,"suggested_replacement":".nth(3)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `skip(..).next()` on an iterator\n  --> tests/ui/iter_skip_next.rs:19:33\n   |\nLL |     let _ = &some_vec[..].iter().skip(3).next();\n   |                                 ^^^^^^^^^^^^^^^ help: use `nth` instead: `.nth(3)`\n\n"}
{"message":"called `skip(..).next()` on an iterator","code":{"code":"clippy::iter_skip_next","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/iter_skip_next.rs","byte_start":794,"byte_end":809,"line_start":27,"line_end":27,"column_start":26,"column_end":41,"is_primary":true,"text":[{"text":"    let _: Vec<&str> = sp.skip(1).next().unwrap().split(' ').collect();","highlight_start":26,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `nth` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/iter_skip_next.rs","byte_start":794,"byte_end":809,"line_start":27,"line_end":27,"column_start":26,"column_end":41,"is_primary":true,"text":[{"text":"    let _: Vec<&str> = sp.skip(1).next().unwrap().split(' ').collect();","highlight_start":26,"highlight_end":41}],"label":null,"suggested_replacement":".nth(1)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `skip(..).next()` on an iterator\n  --> tests/ui/iter_skip_next.rs:27:26\n   |\nLL |     let _: Vec<&str> = sp.skip(1).next().unwrap().split(' ').collect();\n   |                          ^^^^^^^^^^^^^^^ help: use `nth` instead: `.nth(1)`\n\n"}
{"message":"called `skip(..).next()` on an iterator","code":{"code":"clippy::iter_skip_next","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/iter_skip_next.rs","byte_start":943,"byte_end":958,"line_start":29,"line_end":29,"column_start":29,"column_end":44,"is_primary":true,"text":[{"text":"        let _: Vec<&str> = s.skip(1).next().unwrap().split(' ').collect();","highlight_start":29,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `nth` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/iter_skip_next.rs","byte_start":943,"byte_end":958,"line_start":29,"line_end":29,"column_start":29,"column_end":44,"is_primary":true,"text":[{"text":"        let _: Vec<&str> = s.skip(1).next().unwrap().split(' ').collect();","highlight_start":29,"highlight_end":44}],"label":null,"suggested_replacement":".nth(1)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `skip(..).next()` on an iterator\n  --> tests/ui/iter_skip_next.rs:29:29\n   |\nLL |         let _: Vec<&str> = s.skip(1).next().unwrap().split(' ').collect();\n   |                             ^^^^^^^^^^^^^^^ help: use `nth` instead: `.nth(1)`\n\n"}
{"message":"called `skip(..).next()` on an iterator","code":{"code":"clippy::iter_skip_next","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/iter_skip_next.rs","byte_start":1103,"byte_end":1118,"line_start":35,"line_end":35,"column_start":29,"column_end":44,"is_primary":true,"text":[{"text":"        let _: Vec<&str> = s.skip(1).next().unwrap().split(' ').collect();","highlight_start":29,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"use `nth` instead","code":null,"level":"help","spans":[{"file_name":"tests/ui/iter_skip_next.rs","byte_start":1103,"byte_end":1118,"line_start":35,"line_end":35,"column_start":29,"column_end":44,"is_primary":true,"text":[{"text":"        let _: Vec<&str> = s.skip(1).next().unwrap().split(' ').collect();","highlight_start":29,"highlight_end":44}],"label":null,"suggested_replacement":".nth(1)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: called `skip(..).next()` on an iterator\n  --> tests/ui/iter_skip_next.rs:35:29\n   |\nLL |         let _: Vec<&str> = s.skip(1).next().unwrap().split(' ').collect();\n   |                             ^^^^^^^^^^^^^^^ help: use `nth` instead: `.nth(1)`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

-error: `filter(..).map(..)` can be simplified as `filter_map(..)`
-   |
-   |
-LL |     let _ = (0..).filter(|n| to_opt(*n).is_some()).map(|a| to_opt(a).unwrap());
-   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `filter_map(|a| to_opt(a))`
-   |
-   = note: `-D clippy::manual-filter-map` implied by `-D warnings`
-
-error: `filter(..).map(..)` can be simplified as `filter_map(..)`
-   |
-   |
-LL |     let _ = (0..).filter(|&n| to_opt(n).is_some()).map(|a| to_opt(a).expect("hi"));
-   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `filter_map(|a| to_opt(a))`
-
-error: `filter(..).map(..)` can be simplified as `filter_map(..)`
-   |
-   |
-LL |     let _ = (0..).filter(|&n| to_res(n).is_ok()).map(|a| to_res(a).unwrap_or(1));
-   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `filter_map(|a| to_res(a).ok())`
-
-error: `filter(..).map(..)` can be simplified as `filter_map(..)`
-   |
-   |
-LL |           .filter(|&x| to_ref(to_opt(x)).is_some())
-   |  __________^
-LL | |         .map(|y| to_ref(to_opt(y)).unwrap());
-   | |____________________________________________^ help: try: `filter_map(|y| *to_ref(to_opt(y)))`
-
-error: `filter(..).map(..)` can be simplified as `filter_map(..)`
-   |
-   |
-LL |           .filter(|x| to_ref(to_opt(*x)).is_some())
-   |  __________^
-LL | |         .map(|y| to_ref(to_opt(y)).unwrap());
-   | |____________________________________________^ help: try: `filter_map(|y| *to_ref(to_opt(y)))`
-
-error: `filter(..).map(..)` can be simplified as `filter_map(..)`
-   |
-   |
-LL |           .filter(|&x| to_ref(to_res(x)).is_ok())
-   |  __________^
-LL | |         .map(|y| to_ref(to_res(y)).unwrap());
-   | |____________________________________________^ help: try: `filter_map(|y| to_ref(to_res(y)).ok())`
-
-error: `filter(..).map(..)` can be simplified as `filter_map(..)`
-   |
-   |
-LL |           .filter(|x| to_ref(to_res(*x)).is_ok())
-   |  __________^
-LL | |         .map(|y| to_ref(to_res(y)).unwrap());
-   | |____________________________________________^ help: try: `filter_map(|y| to_ref(to_res(y)).ok())`
-
 error: `find(..).map(..)` can be simplified as `find_map(..)`
    |
    |
 LL |     iter::<Option<&u8>>().find(|x| x.is_some()).map(|x| x.cloned().unwrap());
    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `find_map(|x| x.cloned())`
    |
    = note: `-D clippy::manual-find-map` implied by `-D warnings`
 
 error: `find(..).map(..)` can be simplified as `find_map(..)`
    |
    |
 LL |     iter::<&Option<&u8>>().find(|x| x.is_some()).map(|x| x.cloned().unwrap());
    |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `find_map(|x| x.cloned())`
 
 error: `find(..).map(..)` can be simplified as `find_map(..)`
    |
    |
 LL |     iter::<&Option<String>>().find(|x| x.is_some()).map(|x| x.as_deref().unwrap());
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `find_map(|x| x.as_deref())`
 
 error: `find(..).map(..)` can be simplified as `find_map(..)`
    |
    |
 LL |     iter::<Option<&String>>().find(|&x| to_ref(x).is_some()).map(|y| to_ref(y).cloned().unwrap());
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `find_map(|y| to_ref(y).cloned())`
 
 error: `find(..).map(..)` can be simplified as `find_map(..)`
    |
    |
 LL |     iter::<Result<u8, ()>>().find(|x| x.is_ok()).map(|x| x.unwrap());
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `find_map(|x| x.ok())`
 
 error: `find(..).map(..)` can be simplified as `find_map(..)`
    |
    |
 LL |     iter::<&Result<u8, ()>>().find(|x| x.is_ok()).map(|x| x.unwrap());
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `find_map(|x| x.ok())`
 
 error: `find(..).map(..)` can be simplified as `find_map(..)`
    |
    |
 LL |     iter::<&&Result<u8, ()>>().find(|x| x.is_ok()).map(|x| x.unwrap());
    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `find_map(|x| x.ok())`
 
 error: `find(..).map(..)` can be simplified as `find_map(..)`
    |
    |
 LL |     iter::<Result<&u8, ()>>().find(|x| x.is_ok()).map(|x| x.cloned().unwrap());
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `find_map(|x| x.cloned().ok())`
 
 error: `find(..).map(..)` can be simplified as `find_map(..)`
    |
    |
 LL |     iter::<&Result<&u8, ()>>().find(|x| x.is_ok()).map(|x| x.cloned().unwrap());
    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `find_map(|x| x.cloned().ok())`
 
 error: `find(..).map(..)` can be simplified as `find_map(..)`
    |
    |
 LL |     iter::<&Result<String, ()>>().find(|x| x.is_ok()).map(|x| x.as_deref().unwrap());
    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `find_map(|x| x.as_deref().ok())`
 
 error: `find(..).map(..)` can be simplified as `find_map(..)`
    |
    |
 LL |     iter::<Result<&String, ()>>().find(|&x| to_ref(x).is_ok()).map(|y| to_ref(y).cloned().unwrap());
    |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `find_map(|y| to_ref(y).cloned().ok())`
 
 error: `filter(..).map(..)` can be simplified as `filter_map(..)`
    |
    |
 LL |           .filter(|f| f.option_field.is_some())
    |  __________^
 LL | |         .map(|f| f.option_field.clone().unwrap());
    | |_________________________________________________^ help: try: `filter_map(|f| f.option_field.clone())`
+   |
+   = note: `-D clippy::manual-filter-map` implied by `-D warnings`
 
 error: `filter(..).map(..)` can be simplified as `filter_map(..)`
    |
    |
 LL |           .filter(|f| f.ref_field.is_some())
    |  __________^
 LL | |         .map(|f| f.ref_field.cloned().unwrap());
    | |_______________________________________________^ help: try: `filter_map(|f| f.ref_field.cloned())`
 
 error: `filter(..).map(..)` can be simplified as `filter_map(..)`
    |
    |
 LL |           .filter(|f| f.ref_field.is_some())
    |  __________^
 LL | |         .map(|f| f.ref_field.copied().unwrap());
    | |_______________________________________________^ help: try: `filter_map(|f| f.ref_field.copied())`
 
 error: `filter(..).map(..)` can be simplified as `filter_map(..)`
    |
    |
 LL |           .filter(|f| f.result_field.is_ok())
    |  __________^
 LL | |         .map(|f| f.result_field.clone().unwrap());
    | |_________________________________________________^ help: try: `filter_map(|f| f.result_field.clone().ok())`
 
 error: `filter(..).map(..)` can be simplified as `filter_map(..)`
    |
    |
 LL |           .filter(|f| f.result_field.is_ok())
    |  __________^
 LL | |         .map(|f| f.result_field.as_ref().unwrap());
    | |__________________________________________________^ help: try: `filter_map(|f| f.result_field.as_ref().ok())`
 
 error: `filter(..).map(..)` can be simplified as `filter_map(..)`
    |
    |
 LL |           .filter(|f| f.result_field.is_ok())
    |  __________^
 LL | |         .map(|f| f.result_field.as_deref().unwrap());
    | |____________________________________________________^ help: try: `filter_map(|f| f.result_field.as_deref().ok())`
 
 error: `filter(..).map(..)` can be simplified as `filter_map(..)`
    |
    |
 LL |           .filter(|f| f.result_field.is_ok())
    |  __________^
 LL | |         .map(|f| f.result_field.as_mut().unwrap());
    | |__________________________________________________^ help: try: `filter_map(|f| f.result_field.as_mut().ok())`
 
 error: `filter(..).map(..)` can be simplified as `filter_map(..)`
    |
    |
 LL |           .filter(|f| f.result_field.is_ok())
    |  __________^
 LL | |         .map(|f| f.result_field.as_deref_mut().unwrap());
    | |________________________________________________________^ help: try: `filter_map(|f| f.result_field.as_deref_mut().ok())`
 
 error: `filter(..).map(..)` can be simplified as `filter_map(..)`
    |
    |
 LL |           .filter(|f| f.result_field.is_ok())
    |  __________^
 LL | |         .map(|f| f.result_field.to_owned().unwrap());
    | |____________________________________________________^ help: try: `filter_map(|f| f.result_field.to_owned().ok())`
-error: aborting due to 27 previous errors
+error: aborting due to 20 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/manual_filter_map.stage-id.stderr
diff of fixed:

 // run-rustfix
 #![allow(dead_code)]
 #![warn(clippy::manual_filter_map)]
 #![allow(clippy::redundant_closure)] // FIXME suggestion may have redundant closure
 fn main() {
     // is_some(), unwrap()
     // is_some(), unwrap()
-    let _ = (0..).filter_map(|a| to_opt(a));
+    let _ = (0..).filter(|n| to_opt(*n).is_some()).map(|a| to_opt(a).unwrap());
     // ref pattern, expect()
     // ref pattern, expect()
-    let _ = (0..).filter_map(|a| to_opt(a));
+    let _ = (0..).filter(|&n| to_opt(n).is_some()).map(|a| to_opt(a).expect("hi"));
     // is_ok(), unwrap_or()
     // is_ok(), unwrap_or()
-    let _ = (0..).filter_map(|a| to_res(a).ok());
+    let _ = (0..).filter(|&n| to_res(n).is_ok()).map(|a| to_res(a).unwrap_or(1));
     let _ = (1..5)
     let _ = (1..5)
-        .filter_map(|y| *to_ref(to_opt(y)));
+        .filter(|&x| to_ref(to_opt(x)).is_some())
+        .map(|y| to_ref(to_opt(y)).unwrap());
     let _ = (1..5)
-        .filter_map(|y| *to_ref(to_opt(y)));
+        .filter(|x| to_ref(to_opt(*x)).is_some())
+        .map(|y| to_ref(to_opt(y)).unwrap());
     let _ = (1..5)
     let _ = (1..5)
-        .filter_map(|y| to_ref(to_res(y)).ok());
+        .filter(|&x| to_ref(to_res(x)).is_ok())
+        .map(|y| to_ref(to_res(y)).unwrap());
     let _ = (1..5)
-        .filter_map(|y| to_ref(to_res(y)).ok());
+        .filter(|x| to_ref(to_res(*x)).is_ok())
+        .map(|y| to_ref(to_res(y)).unwrap());
 
 
 #[rustfmt::skip]
 fn simple_equal() {
     iter::<Option<&u8>>().find_map(|x| x.cloned());
     iter::<&Option<&u8>>().find_map(|x| x.cloned());
     iter::<&Option<String>>().find_map(|x| x.as_deref());
     iter::<Option<&String>>().find_map(|y| to_ref(y).cloned());
 
     iter::<Result<u8, ()>>().find_map(|x| x.ok());
     iter::<&Result<u8, ()>>().find_map(|x| x.ok());
     iter::<&&Result<u8, ()>>().find_map(|x| x.ok());
     iter::<Result<&u8, ()>>().find_map(|x| x.cloned().ok());
     iter::<&Result<&u8, ()>>().find_map(|x| x.cloned().ok());
     iter::<&Result<String, ()>>().find_map(|x| x.as_deref().ok());
     iter::<Result<&String, ()>>().find_map(|y| to_ref(y).cloned().ok());
 
 fn no_lint() {
     // no shared code
     // no shared code
     let _ = (0..).filter(|n| *n > 1).map(|n| n + 1);
 
     // very close but different since filter() provides a reference
     let _ = (0..).filter(|n| to_opt(n).is_some()).map(|a| to_opt(a).unwrap());
     // similar but different
     // similar but different
     let _ = (0..).filter(|n| to_opt(n).is_some()).map(|n| to_res(n).unwrap());
     let _ = (0..)
error: test failed, to rerun pass `--test compile-test`
         .filter(|n| to_opt(n).map(|n| n + 1).is_some())
         .map(|a| to_opt(a).unwrap());
 
 
 fn iter<T>() -> impl Iterator<Item = T> {
 }
 
 
 fn to_opt<T>(_: T) -> Option<T> {
     unimplemented!()
 
 
 fn to_res<T>(_: T) -> Result<T, ()> {
     unimplemented!()
 
 
 fn to_ref<'a, T>(_: T) -> &'a T {
     unimplemented!()
 
 struct Issue8920<'a> {
     option_field: Option<String>,
     result_field: Result<String, ()>,
     result_field: Result<String, ()>,
     ref_field: Option<&'a usize>,
 }
 
 fn issue_8920() {
     let mut vec = vec![Issue8920 {
         option_field: Some(String::from("str")),
         result_field: Ok(String::from("str")),
         ref_field: Some(&1),
     }];
     let _ = vec
         .iter()
         .iter()
         .filter_map(|f| f.option_field.clone());
     let _ = vec
         .iter()
         .iter()
         .filter_map(|f| f.ref_field.cloned());
     let _ = vec
         .iter()
         .iter()
         .filter_map(|f| f.ref_field.copied());
     let _ = vec
         .iter()
         .iter()
         .filter_map(|f| f.result_field.clone().ok());
     let _ = vec
         .iter()
         .iter()
         .filter_map(|f| f.result_field.as_ref().ok());
     let _ = vec
         .iter()
         .iter()
         .filter_map(|f| f.result_field.as_deref().ok());
     let _ = vec
         .iter_mut()
         .iter_mut()
         .filter_map(|f| f.result_field.as_mut().ok());
     let _ = vec
         .iter_mut()
         .iter_mut()
         .filter_map(|f| f.result_field.as_deref_mut().ok());
     let _ = vec
         .iter()
         .iter()
         .filter_map(|f| f.result_field.to_owned().ok());
 

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/manual_filter_map.stage-id.fixed
To only update this specific test, also pass `--test-args manual_filter_map.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/manual_filter_map.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/manual_filter_map.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-f2da228ff241ec97.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-b9c731f380466bd0.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-97618c8d1e1f91be.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-3ef490519219f110.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-e95902f0f3ef680a.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-5ac9fbce59ab185e.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-f474b2dfd384a595.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-4ace64e065702c69.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-8ab3fb021f549e8b.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/manual_filter_map.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"`find(..).map(..)` can be simplified as `find_map(..)`","code":{"code":"clippy::manual_find_map","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/manual_filter_map.rs","byte_start":1025,"byte_end":1075,"line_start":33,"line_end":33,"column_start":27,"column_end":77,"is_primary":true,"text":[{"text":"    iter::<Option<&u8>>().find(|x| x.is_some()).map(|x| x.cloned().unwrap());","highlight_start":27,"highlight_end":77}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::manual-find-map` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/manual_filter_map.rs","byte_start":1025,"byte_end":1075,"line_start":33,"line_end":33,"column_start":27,"column_end":77,"is_primary":true,"text":[{"text":"    iter::<Option<&u8>>().find(|x| x.is_some()).map(|x| x.cloned().unwrap());","highlight_start":27,"highlight_end":77}],"label":null,"suggested_replacement":"find_map(|x| x.cloned())","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `find(..).map(..)` can be simplified as `find_map(..)`\n  --> tests/ui/manual_filter_map.rs:33:27\n   |\nLL |     iter::<Option<&u8>>().find(|x| x.is_some()).map(|x| x.cloned().unwrap());\n   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `find_map(|x| x.cloned())`\n   |\n   = note: `-D clippy::manual-find-map` implied by `-D warnings`\n\n"}
{"message":"`find(..).map(..)` can be simplified as `find_map(..)`","code":{"code":"clippy::manual_find_map","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/manual_filter_map.rs","byte_start":1104,"byte_end":1154,"line_start":34,"line_end":34,"column_start":28,"column_end":78,"is_primary":true,"text":[{"text":"    iter::<&Option<&u8>>().find(|x| x.is_some()).map(|x| x.cloned().unwrap());","highlight_start":28,"highlight_end":78}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/manual_filter_map.rs","byte_start":1104,"byte_end":1154,"line_start":34,"line_end":34,"column_start":28,"column_end":78,"is_primary":true,"text":[{"text":"    iter::<&Option<&u8>>().find(|x| x.is_some()).map(|x| x.cloned().unwrap());","highlight_start":28,"highlight_end":78}],"label":null,"suggested_replacement":"find_map(|x| x.cloned())","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `find(..).map(..)` can be simplified as `find_map(..)`\n  --> tests/ui/manual_filter_map.rs:34:28\n   |\nLL |     iter::<&Option<&u8>>().find(|x| x.is_some()).map(|x| x.cloned().unwrap());\n   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `find_map(|x| x.cloned())`\n\n"}
{"message":"`find(..).map(..)` can be simplified as `find_map(..)`","code":{"code":"clippy::manual_find_map","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/manual_filter_map.rs","byte_start":1186,"byte_end":1238,"line_start":35,"line_end":35,"column_start":31,"column_end":83,"is_primary":true,"text":[{"text":"    iter::<&Option<String>>().find(|x| x.is_some()).map(|x| x.as_deref().unwrap());","highlight_start":31,"highlight_end":83}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/manual_filter_map.rs","byte_start":1186,"byte_end":1238,"line_start":35,"line_end":35,"column_start":31,"column_end":83,"is_primary":true,"text":[{"text":"    iter::<&Option<String>>().find(|x| x.is_some()).map(|x| x.as_deref().unwrap());","highlight_start":31,"highlight_end":83}],"label":null,"suggested_replacement":"find_map(|x| x.as_deref())","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `find(..).map(..)` can be simplified as `find_map(..)`\n  --> tests/ui/manual_filter_map.rs:35:31\n   |\nLL |     iter::<&Option<String>>().find(|x| x.is_some()).map(|x| x.as_deref().unwrap());\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `find_map(|x| x.as_deref())`\n\n"}
{"message":"`find(..).map(..)` can be simplified as `find_map(..)`","code":{"code":"clippy::manual_find_map","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/manual_filter_map.rs","byte_start":1270,"byte_end":1337,"line_start":36,"line_end":36,"column_start":31,"column_end":98,"is_primary":true,"text":[{"text":"    iter::<Option<&String>>().find(|&x| to_ref(x).is_some()).map(|y| to_ref(y).cloned().unwrap());","highlight_start":31,"highlight_end":98}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/manual_filter_map.rs","byte_start":1270,"byte_end":1337,"line_start":36,"line_end":36,"column_start":31,"column_end":98,"is_primary":true,"text":[{"text":"    iter::<Option<&String>>().find(|&x| to_ref(x).is_some()).map(|y| to_ref(y).cloned().unwrap());","highlight_start":31,"highlight_end":98}],"label":null,"suggested_replacement":"find_map(|y| to_ref(y).cloned())","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `find(..).map(..)` can be simplified as `find_map(..)`\n  --> tests/ui/manual_filter_map.rs:36:31\n   |\nLL |     iter::<Option<&String>>().find(|&x| to_ref(x).is_some()).map(|y| to_ref(y).cloned().unwrap());\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `find_map(|y| to_ref(y).cloned())`\n\n"}
{"message":"`find(..).map(..)` can be simplified as `find_map(..)`","code":{"code":"clippy::manual_find_map","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/manual_filter_map.rs","byte_start":1369,"byte_end":1408,"line_start":38,"line_end":38,"column_start":30,"column_end":69,"is_primary":true,"text":[{"text":"    iter::<Result<u8, ()>>().find(|x| x.is_ok()).map(|x| x.unwrap());","highlight_start":30,"highlight_end":69}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/manual_filter_map.rs","byte_start":1369,"byte_end":1408,"line_start":38,"line_end":38,"column_start":30,"column_end":69,"is_primary":true,"text":[{"text":"    iter::<Result<u8, ()>>().find(|x| x.is_ok()).map(|x| x.unwrap());","highlight_start":30,"highlight_end":69}],"label":null,"suggested_replacement":"find_map(|x| x.ok())","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `find(..).map(..)` can be simplified as `find_map(..)`\n  --> tests/ui/manual_filter_map.rs:38:30\n   |\nLL |     iter::<Result<u8, ()>>().find(|x| x.is_ok()).map(|x| x.unwrap());\n   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `find_map(|x| x.ok())`\n\n"}
{"message":"`find(..).map(..)` can be simplified as `find_map(..)`","code":{"code":"clippy::manual_find_map","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/manual_filter_map.rs","byte_start":1440,"byte_end":1479,"line_start":39,"line_end":39,"column_start":31,"column_end":70,"is_primary":true,"text":[{"text":"    iter::<&Result<u8, ()>>().find(|x| x.is_ok()).map(|x| x.unwrap());","highlight_start":31,"highlight_end":70}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/manual_filter_map.rs","byte_start":1440,"byte_end":1479,"line_start":39,"line_end":39,"column_start":31,"column_end":70,"is_primary":true,"text":[{"text":"    iter::<&Result<u8, ()>>().find(|x| x.is_ok()).map(|x| x.unwrap());","highlight_start":31,"highlight_end":70}],"label":null,"suggested_replacement":"find_map(|x| x.ok())","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `find(..).map(..)` can be simplified as `find_map(..)`\n  --> tests/ui/manual_filter_map.rs:39:31\n   |\nLL |     iter::<&Result<u8, ()>>().find(|x| x.is_ok()).map(|x| x.unwrap());\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `find_map(|x| x.ok())`\n\n"}
{"message":"`find(..).map(..)` can be simplified as `find_map(..)`","code":{"code":"clippy::manual_find_map","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/manual_filter_map.rs","byte_start":1512,"byte_end":1551,"line_start":40,"line_end":40,"column_start":32,"column_end":71,"is_primary":true,"text":[{"text":"    iter::<&&Result<u8, ()>>().find(|x| x.is_ok()).map(|x| x.unwrap());","highlight_start":32,"highlight_end":71}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/manual_filter_map.rs","byte_start":1512,"byte_end":1551,"line_start":40,"line_end":40,"column_start":32,"column_end":71,"is_primary":true,"text":[{"text":"    iter::<&&Result<u8, ()>>().find(|x| x.is_ok()).map(|x| x.unwrap());","highlight_start":32,"highlight_end":71}],"label":null,"suggested_replacement":"find_map(|x| x.ok())","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `find(..).map(..)` can be simplified as `find_map(..)`\n  --> tests/ui/manual_filter_map.rs:40:32\n   |\nLL |     iter::<&&Result<u8, ()>>().find(|x| x.is_ok()).map(|x| x.unwrap());\n   |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `find_map(|x| x.ok())`\n\n"}
{"message":"`find(..).map(..)` can be simplified as `find_map(..)`","code":{"code":"clippy::manual_find_map","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/manual_filter_map.rs","byte_start":1583,"byte_end":1631,"line_start":41,"line_end":41,"column_start":31,"column_end":79,"is_primary":true,"text":[{"text":"    iter::<Result<&u8, ()>>().find(|x| x.is_ok()).map(|x| x.cloned().unwrap());","highlight_start":31,"highlight_end":79}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/manual_filter_map.rs","byte_start":1583,"byte_end":1631,"line_start":41,"line_end":41,"column_start":31,"column_end":79,"is_primary":true,"text":[{"text":"    iter::<Result<&u8, ()>>().find(|x| x.is_ok()).map(|x| x.cloned().unwrap());","highlight_start":31,"highlight_end":79}],"label":null,"suggested_replacement":"find_map(|x| x.cloned().ok())","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `find(..).map(..)` can be simplified as `find_map(..)`\n  --> tests/ui/manual_filter_map.rs:41:31\n   |\nLL |     iter::<Result<&u8, ()>>().find(|x| x.is_ok()).map(|x| x.cloned().unwrap());\n   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `find_map(|x| x.cloned().ok())`\n\n"}
---
To only update this specific test, also pass `--test-args search_is_some.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/search_is_some.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/search_is_some.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-f2da228ff241ec97.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-b9c731f380466bd0.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-97618c8d1e1f91be.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-3ef490519219f110.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-e95902f0f3ef680a.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-5ac9fbce59ab185e.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-f474b2dfd384a595.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-4ace64e065702c69.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-8ab3fb021f549e8b.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/search_is_some.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"called `is_some()` after searching an `Iterator` with `find`","code":{"code":"clippy::search_is_some","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/search_is_some.rs","byte_start":312,"byte_end":428,"line_start":14,"line_end":17,"column_start":13,"column_end":31,"is_primary":true,"text":[{"text":"    let _ = v.iter().find(|&x| {","highlight_start":13,"highlight_end":33},{"text":"                              *x < 0","highlight_start":1,"highlight_end":37},{"text":"                          }","highlight_start":1,"highlight_end":28},{"text":"                   ).is_some();","highlight_start":1,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this is more succinctly expressed by calling `any()`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"`-D clippy::search-is-some` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: called `is_some()` after searching an `Iterator` with `find`\n  --> tests/ui/search_is_some.rs:14:13\n   |\nLL |       let _ = v.iter().find(|&x| {\n   |  _____________^\nLL | |                               *x < 0\nLL | |                           }\nLL | |                    ).is_some();\n   | |______________________________^\n   |\n   = help: this is more succinctly expressed by calling `any()`\n   = note: `-D clippy::search-is-some` implied by `-D warnings`\n\n"}
{"message":"called `is_some()` after searching an `Iterator` with `position`","code":{"code":"clippy::search_is_some","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/search_is_some.rs","byte_start":497,"byte_end":624,"line_start":20,"line_end":23,"column_start":13,"column_end":31,"is_primary":true,"text":[{"text":"    let _ = v.iter().position(|&x| {","highlight_start":13,"highlight_end":37},{"text":"                                  x < 0","highlight_start":1,"highlight_end":40},{"text":"                              }","highlight_start":1,"highlight_end":32},{"text":"                   ).is_some();","highlight_start":1,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this is more succinctly expressed by calling `any()`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: called `is_some()` after searching an `Iterator` with `position`\n  --> tests/ui/search_is_some.rs:20:13\n   |\nLL |       let _ = v.iter().position(|&x| {\n   |  _____________^\nLL | |                                   x < 0\nLL | |                               }\nLL | |                    ).is_some();\n   | |______________________________^\n   |\n   = help: this is more succinctly expressed by calling `any()`\n\n"}
{"message":"called `is_some()` after searching an `Iterator` with `rposition`","code":{"code":"clippy::search_is_some","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/search_is_some.rs","byte_start":694,"byte_end":824,"line_start":26,"line_end":29,"column_start":13,"column_end":31,"is_primary":true,"text":[{"text":"    let _ = v.iter().rposition(|&x| {","highlight_start":13,"highlight_end":38},{"text":"                                   x < 0","highlight_start":1,"highlight_end":41},{"text":"                               }","highlight_start":1,"highlight_end":33},{"text":"                   ).is_some();","highlight_start":1,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this is more succinctly expressed by calling `any()`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: called `is_some()` after searching an `Iterator` with `rposition`\n  --> tests/ui/search_is_some.rs:26:13\n   |\nLL |       let _ = v.iter().rposition(|&x| {\n   |  _____________^\nLL | |                                    x < 0\nLL | |                                }\nLL | |                    ).is_some();\n   | |______________________________^\n   |\n   = help: this is more succinctly expressed by calling `any()`\n\n"}
{"message":"called `is_none()` after searching an `Iterator` with `find`","code":{"code":"clippy::search_is_some","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/search_is_some.rs","byte_start":1506,"byte_end":1622,"line_start":51,"line_end":54,"column_start":13,"column_end":31,"is_primary":true,"text":[{"text":"    let _ = v.iter().find(|&x| {","highlight_start":13,"highlight_end":33},{"text":"                              *x < 0","highlight_start":1,"highlight_end":37},{"text":"                          }","highlight_start":1,"highlight_end":28},{"text":"                   ).is_none();","highlight_start":1,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this is more succinctly expressed by calling `any()` with negation","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: called `is_none()` after searching an `Iterator` with `find`\n  --> tests/ui/search_is_some.rs:51:13\n   |\nLL |       let _ = v.iter().find(|&x| {\n   |  _____________^\nLL | |                               *x < 0\nLL | |                           }\nLL | |                    ).is_none();\n   | |______________________________^\n   |\n   = help: this is more succinctly expressed by calling `any()` with negation\n\n"}
{"message":"called `is_none()` after searching an `Iterator` with `position`","code":{"code":"clippy::search_is_some","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/search_is_some.rs","byte_start":1691,"byte_end":1818,"line_start":57,"line_end":60,"column_start":13,"column_end":31,"is_primary":true,"text":[{"text":"    let _ = v.iter().position(|&x| {","highlight_start":13,"highlight_end":37},{"text":"                                  x < 0","highlight_start":1,"highlight_end":40},{"text":"                              }","highlight_start":1,"highlight_end":32},{"text":"                   ).is_none();","highlight_start":1,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this is more succinctly expressed by calling `any()` with negation","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: called `is_none()` after searching an `Iterator` with `position`\n  --> tests/ui/search_is_some.rs:57:13\n   |\nLL |       let _ = v.iter().position(|&x| {\n   |  _____________^\nLL | |                                   x < 0\nLL | |                               }\nLL | |                    ).is_none();\n   | |______________________________^\n   |\n   = help: this is more succinctly expressed by calling `any()` with negation\n\n"}
{"message":"called `is_none()` after searching an `Iterator` with `rposition`","code":{"code":"clippy::search_is_some","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/search_is_some.rs","byte_start":1888,"byte_end":2018,"line_start":63,"line_end":66,"column_start":13,"column_end":31,"is_primary":true,"text":[{"text":"    let _ = v.iter().rposition(|&x| {","highlight_start":13,"highlight_end":38},{"text":"                                   x < 0","highlight_start":1,"highlight_end":41},{"text":"                               }","highlight_start":1,"highlight_end":33},{"text":"                   ).is_none();","highlight_start":1,"highlight_end":31}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this is more succinctly expressed by calling `any()` with negation","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: called `is_none()` after searching an `Iterator` with `rposition`\n  --> tests/ui/search_is_some.rs:63:13\n   |\nLL |       let _ = v.iter().rposition(|&x| {\n   |  _____________^\nLL | |                                    x < 0\nLL | |                                }\nLL | |                    ).is_none();\n   | |______________________________^\n   |\n   = help: this is more succinctly expressed by calling `any()` with negation\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 error: called `is_none()` after searching an `Iterator` with `find`
    |
    |
 LL |     let _ = v.iter().find(|&x| *x < 0).is_none();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!v.iter().any(|x| *x < 0)`
    |
    = note: `-D clippy::search-is-some` implied by `-D warnings`
 
 error: called `is_none()` after searching an `Iterator` with `find`
-   |
-   |
-LL |     let _ = (0..1).find(|x| **y == *x).is_none(); // one dereference less
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!(0..1).any(|x| **y == x)`
-
-error: called `is_none()` after searching an `Iterator` with `find`
-   |
-   |
-LL |     let _ = (0..1).find(|x| *x == 0).is_none();
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!(0..1).any(|x| x == 0)`
-
-error: called `is_none()` after searching an `Iterator` with `find`
    |
    |
 LL |     let _ = v.iter().find(|x| **x == 0).is_none();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!v.iter().any(|x| *x == 0)`
 
-error: called `is_none()` after searching an `Iterator` with `find`
-   |
-   |
-LL |     let _ = (4..5).find(|x| *x == 1 || *x == 3 || *x == 5).is_none();
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!(4..5).any(|x| x == 1 || x == 3 || x == 5)`
-
-error: called `is_none()` after searching an `Iterator` with `find`
-   |
-   |
-LL |     let _ = (1..3).find(|x| [1, 2, 3].contains(x)).is_none();
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!(1..3).any(|x| [1, 2, 3].contains(&x))`
-
-error: called `is_none()` after searching an `Iterator` with `find`
-   |
-   |
-LL |     let _ = (1..3).find(|x| *x == 0 || [1, 2, 3].contains(x)).is_none();
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!(1..3).any(|x| x == 0 || [1, 2, 3].contains(&x))`
-
-error: called `is_none()` after searching an `Iterator` with `find`
-   |
-   |
-LL |     let _ = (1..3).find(|x| [1, 2, 3].contains(x) || *x == 0).is_none();
-   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!(1..3).any(|x| [1, 2, 3].contains(&x) || x == 0)`
-
-error: called `is_none()` after searching an `Iterator` with `find`
-   |
-LL |       let _ = (1..3)
-   |  _____________^
-   |  _____________^
-LL | |         .find(|x| [1, 2, 3].contains(x) || *x == 0 || [4, 5, 6].contains(x) || *x == -1)
-LL | |         .is_none();
-   | |__________________^ help: use `!_.any()` instead: `!(1..3).any(|x| [1, 2, 3].contains(&x) || x == 0 || [4, 5, 6].contains(&x) || x == -1)`
-
 error: called `is_none()` after searching an `Iterator` with `position`
    |
    |
 LL |     let _ = v.iter().position(|&x| x < 0).is_none();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!v.iter().any(|&x| x < 0)`
 
 error: called `is_none()` after searching an `Iterator` with `rposition`
    |
    |
 LL |     let _ = v.iter().rposition(|&x| x < 0).is_none();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!v.iter().any(|&x| x < 0)`
 
 error: called `is_none()` after calling `find()` on a string
    |
    |
 LL |     let _ = "hello world".find("world").is_none();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.contains()` instead: `!"hello world".contains("world")`
 
 error: called `is_none()` after calling `find()` on a string
    |
    |
 LL |     let _ = "hello world".find(&s2).is_none();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.contains()` instead: `!"hello world".contains(&s2)`
 
 error: called `is_none()` after calling `find()` on a string
    |
    |
 LL |     let _ = "hello world".find(&s2[2..]).is_none();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.contains()` instead: `!"hello world".contains(&s2[2..])`
 
 error: called `is_none()` after calling `find()` on a string
    |
    |
 LL |     let _ = s1.find("world").is_none();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.contains()` instead: `!s1.contains("world")`
 
 error: called `is_none()` after calling `find()` on a string
    |
    |
 LL |     let _ = s1.find(&s2).is_none();
    |             ^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.contains()` instead: `!s1.contains(&s2)`
 
 error: called `is_none()` after calling `find()` on a string
    |
    |
 LL |     let _ = s1.find(&s2[2..]).is_none();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.contains()` instead: `!s1.contains(&s2[2..])`
 
 error: called `is_none()` after calling `find()` on a string
    |
    |
 LL |     let _ = s1[2..].find("world").is_none();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.contains()` instead: `!s1[2..].contains("world")`
 
 error: called `is_none()` after calling `find()` on a string
    |
    |
 LL |     let _ = s1[2..].find(&s2).is_none();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.contains()` instead: `!s1[2..].contains(&s2)`
 
 error: called `is_none()` after calling `find()` on a string
    |
    |
 LL |     let _ = s1[2..].find(&s2[2..]).is_none();
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.contains()` instead: `!s1[2..].contains(&s2[2..])`
 
 error: called `is_none()` after searching an `Iterator` with `find`
    |
    |
 LL |             .filter(|c| filter_hand.iter().find(|cc| c == cc).is_none())
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!filter_hand.iter().any(|cc| c == &cc)`
 
 error: called `is_none()` after searching an `Iterator` with `find`
    |
    |
 LL |             .filter(|(c, _)| filter_hand.iter().find(|cc| c == *cc).is_none())
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!filter_hand.iter().any(|cc| c == cc)`
 
 error: called `is_none()` after searching an `Iterator` with `find`
    |
    |
 LL |         let _ = vfoo.iter().find(|v| v.foo == 1 && v.bar == 2).is_none();
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!vfoo.iter().any(|v| v.foo == 1 && v.bar == 2)`
 
 error: called `is_none()` after searching an `Iterator` with `find`
    |
 LL |           let _ = vfoo
    |  _________________^
 LL | |             .iter()
 LL | |             .iter()
 LL | |             .find(|(i, v)| *i == 42 && v.foo == 1 && v.bar == 2)
 LL | |             .is_none();
    |
    |
 help: use `!_.any()` instead
    |
 LL ~         let _ = !vfoo
 LL ~             .iter().any(|(i, v)| *i == 42 && v.foo == 1 && v.bar == 2);
 
 
 error: called `is_none()` after searching an `Iterator` with `find`
    |
    |
 LL |         let _ = vfoo.iter().find(|a| a[0] == 42).is_none();
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!vfoo.iter().any(|a| a[0] == 42)`
 
 error: called `is_none()` after searching an `Iterator` with `find`
    |
    |
 LL |         let _ = vfoo.iter().find(|sub| sub[1..4].len() == 3).is_none();
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!vfoo.iter().any(|sub| sub[1..4].len() == 3)`
 
 error: called `is_none()` after searching an `Iterator` with `find`
    |
    |
 LL |         let _ = [ppx].iter().find(|ppp_x: &&&u32| please(**ppp_x)).is_none();
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `![ppx].iter().any(|ppp_x: &&u32| please(ppp_x))`
 
 error: called `is_none()` after searching an `Iterator` with `find`
    |
    |
 LL |         let _ = [String::from("Hey hey")].iter().find(|s| s.len() == 2).is_none();
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `![String::from("Hey hey")].iter().any(|s| s.len() == 2)`
 
 error: called `is_none()` after searching an `Iterator` with `find`
    |
    |
 LL |         let _ = v.iter().find(|x| deref_enough(**x)).is_none();
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!v.iter().any(|x| deref_enough(*x))`
 
 error: called `is_none()` after searching an `Iterator` with `find`
    |
    |
 LL |         let _ = v.iter().find(|x: &&u32| deref_enough(**x)).is_none();
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!v.iter().any(|x: &u32| deref_enough(*x))`
 
 error: called `is_none()` after searching an `Iterator` with `find`
    |
    |
 LL |         let _ = v.iter().find(|x| arg_no_deref(x)).is_none();
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!v.iter().any(|x| arg_no_deref(&x))`
 
 error: called `is_none()` after searching an `Iterator` with `find`
    |
    |
 LL |         let _ = v.iter().find(|x: &&u32| arg_no_deref(x)).is_none();
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!v.iter().any(|x: &u32| arg_no_deref(&x))`
 
 error: called `is_none()` after searching an `Iterator` with `find`
    |
 LL |           let _ = vfoo
    |  _________________^
 LL | |             .iter()
 LL | |             .iter()
 LL | |             .find(|v| v.inner_double.bar[0][0] == 2 && v.inner.bar[0] == 2)
 LL | |             .is_none();
    |
    |
 help: use `!_.any()` instead
    |
 LL ~         let _ = !vfoo
 LL ~             .iter().any(|v| v.inner_double.bar[0][0] == 2 && v.inner.bar[0] == 2);
 
 
 error: called `is_none()` after searching an `Iterator` with `find`
    |
    |
 LL |         let _ = vfoo.iter().find(|v| v.inner[0].bar == 2).is_none();
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!vfoo.iter().any(|v| v.inner[0].bar == 2)`
 
 error: called `is_none()` after searching an `Iterator` with `find`
    |
    |
 LL |         let _ = vfoo.iter().find(|x| (**x)[0] == 9).is_none();
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!vfoo.iter().any(|x| (**x)[0] == 9)`
 
 error: called `is_none()` after searching an `Iterator` with `find`
    |
    |
 LL |         let _ = vfoo.iter().find(|v| v.by_ref(&v.bar)).is_none();
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!vfoo.iter().any(|v| v.by_ref(&v.bar))`
 
 error: called `is_none()` after searching an `Iterator` with `find`
    |
    |
 LL |         let _ = [&(&1, 2), &(&3, 4), &(&5, 4)].iter().find(|(&x, y)| x == *y).is_none();
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `![&(&1, 2), &(&3, 4), &(&5, 4)].iter().any(|(&x, y)| x == *y)`
 
 error: called `is_none()` after searching an `Iterator` with `find`
    |
    |
 LL |         let _ = [&(&1, 2), &(&3, 4), &(&5, 4)].iter().find(|&(&x, y)| x == *y).is_none();
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `![&(&1, 2), &(&3, 4), &(&5, 4)].iter().any(|(&x, y)| x == *y)`
 
 error: called `is_none()` after searching an `Iterator` with `find`
    |
    |
 LL |         let _ = v.iter().find(|s| s[0].is_empty()).is_none();
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!v.iter().any(|s| s[0].is_empty())`
 
 error: called `is_none()` after searching an `Iterator` with `find`
    |
    |
 LL |         let _ = v.iter().find(|s| test_string_1(&s[0])).is_none();
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!v.iter().any(|s| test_string_1(&s[0]))`
 
 error: called `is_none()` after searching an `Iterator` with `find`
    |
    |
 LL |         let _ = v.iter().find(|fp| fp.field.is_power_of_two()).is_none();
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!v.iter().any(|fp| fp.field.is_power_of_two())`
 
 error: called `is_none()` after searching an `Iterator` with `find`
    |
    |
 LL |         let _ = v.iter().find(|fp| test_u32_1(fp.field)).is_none();
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!v.iter().any(|fp| test_u32_1(fp.field))`
 
 error: called `is_none()` after searching an `Iterator` with `find`
    |
    |
 LL |         let _ = v.iter().find(|fp| test_u32_2(*fp.field)).is_none();
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `!_.any()` instead: `!v.iter().any(|fp| test_u32_2(*fp.field))`
-error: aborting due to 43 previous errors
+error: aborting due to 36 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/search_is_some_fixable_none.stage-id.stderr
diff of fixed:

 // run-rustfix
 #![allow(dead_code, clippy::explicit_auto_deref)]
 #![warn(clippy::search_is_some)]
 fn main() {
 fn main() {
     let v = vec![3, 2, 1, 0, -1, -2, -3];
     let y = &&42;
 
     // Check `find().is_none()`, single-line case.
     let _ = !v.iter().any(|x| *x < 0);
-    let _ = !(0..1).any(|x| **y == x); // one dereference less
-    let _ = !(0..1).any(|x| x == 0);
+    let _ = (0..1).find(|x| **y == *x).is_none(); // one dereference less
+    let _ = (0..1).find(|x| *x == 0).is_none();
     let _ = !v.iter().any(|x| *x == 0);
-    let _ = !(4..5).any(|x| x == 1 || x == 3 || x == 5);
-    let _ = !(1..3).any(|x| [1, 2, 3].contains(&x));
-    let _ = !(1..3).any(|x| x == 0 || [1, 2, 3].contains(&x));
-    let _ = !(1..3).any(|x| [1, 2, 3].contains(&x) || x == 0);
-    let _ = !(1..3).any(|x| [1, 2, 3].contains(&x) || x == 0 || [4, 5, 6].contains(&x) || x == -1);
+    let _ = (4..5).find(|x| *x == 1 || *x == 3 || *x == 5).is_none();
+    let _ = (1..3).find(|x| [1, 2, 3].contains(x)).is_none();
+    let _ = (1..3).find(|x| *x == 0 || [1, 2, 3].contains(x)).is_none();
+    let _ = (1..3).find(|x| [1, 2, 3].contains(x) || *x == 0).is_none();
+    let _ = (1..3)
+        .find(|x| [1, 2, 3].contains(x) || *x == 0 || [4, 5, 6].contains(x) || *x == -1)
+        .is_none();
 
     // Check `position().is_none()`, single-line case.
     let _ = !v.iter().any(|&x| x < 0);
 
     // Check `rposition().is_none()`, single-line case.
     let _ = !v.iter().any(|&x| x < 0);
 
     let s1 = String::from("hello world");
     let s2 = String::from("world");
 
     // caller of `find()` is a `&`static str`
     let _ = !"hello world".contains("world");
     let _ = !"hello world".contains(&s2);
     let _ = !"hello world".contains(&s2[2..]);
     // caller of `find()` is a `String`
     let _ = !s1.contains("world");
     let _ = !s1.contains(&s2);
     let _ = !s1.contains(&s2[2..]);
     // caller of `find()` is slice of `String`
     let _ = !s1[2..].contains("world");
     let _ = !s1[2..].contains(&s2);
     let _ = !s1[2..].contains(&s2[2..]);
 
 
 #[allow(clippy::clone_on_copy, clippy::map_clone)]
 mod issue7392 {
     struct Player {
         hand: Vec<usize>,
     fn filter() {
         let p = Player {
         let p = Player {
             hand: vec![1, 2, 3, 4, 5],
         };
         let filter_hand = vec![5];
             .hand
             .iter()
             .iter()
             .filter(|c| !filter_hand.iter().any(|cc| c == &cc))
             .map(|c| c.clone())
             .collect::<Vec<_>>();
 
     struct PlayerTuple {
     struct PlayerTuple {
         hand: Vec<(usize, char)>,
     fn filter_tuple() {
     fn filter_tuple() {
         let p = PlayerTuple {
             hand: vec![(1, 'a'), (2, 'b'), (3, 'c'), (4, 'd'), (5, 'e')],
         };
         let filter_hand = vec![5];
             .hand
             .iter()
             .iter()
             .filter(|(c, _)| !filter_hand.iter().any(|cc| c == cc))
             .map(|c| c.clone())
             .collect::<Vec<_>>();
 
     fn field_projection() {
         struct Foo {
             foo: i32,
             foo: i32,
             bar: u32,
         }
         let vfoo = vec![Foo { foo: 1, bar: 2 }];
         let _ = !vfoo.iter().any(|v| v.foo == 1 && v.bar == 2);
 
         let vfoo = vec![(42, Foo { foo: 1, bar: 2 })];
         let _ = !vfoo
             .iter().any(|(i, v)| *i == 42 && v.foo == 1 && v.bar == 2);
 
     fn index_projection() {
     fn index_projection() {
         let vfoo = vec![[0, 1, 2, 3]];
         let _ = !vfoo.iter().any(|a| a[0] == 42);
 
 
     #[allow(clippy::match_like_matches_macro)]
     fn slice_projection() {
         let vfoo = vec![[0, 1, 2, 3, 0, 1, 2, 3]];
         let _ = !vfoo.iter().any(|sub| sub[1..4].len() == 3);
 
 
     fn please(x: &u32) -> bool {
         *x == 9
 
 
     fn deref_enough(x: u32) -> bool {
         x == 78
 
 
     fn arg_no_deref(x: &&u32) -> bool {
         **x == 78
 
     fn more_projections() {
         let x = 19;
         let x = 19;
         let ppx: &u32 = &x;
         let _ = ![ppx].iter().any(|ppp_x: &&u32| please(ppp_x));
         let _ = ![String::from("Hey hey")].iter().any(|s| s.len() == 2);
 
         let v = vec![3, 2, 1, 0];
         let _ = !v.iter().any(|x| deref_enough(*x));
         let _ = !v.iter().any(|x: &u32| deref_enough(*x));
 
         #[allow(clippy::redundant_closure)]
         let _ = !v.iter().any(|x| arg_no_deref(&x));
         #[allow(clippy::redundant_closure)]
         let _ = !v.iter().any(|x: &u32| arg_no_deref(&x));
 
     fn field_index_projection() {
         struct FooDouble {
         struct FooDouble {
             bar: Vec<Vec<i32>>,
         struct Foo {
         struct Foo {
             bar: Vec<i32>,
         struct FooOuter {
             inner: Foo,
             inner_double: FooDouble,
         }
         }
         let vfoo = vec![FooOuter {
             inner: Foo { bar: vec![0, 1, 2, 3] },
             inner_double: FooDouble {
                 bar: vec![vec![0, 1, 2, 3]],
         }];
         }];
         let _ = !vfoo
             .iter().any(|v| v.inner_double.bar[0][0] == 2 && v.inner.bar[0] == 2);
 
     fn index_field_projection() {
         struct Foo {
             bar: i32,
             bar: i32,
         }
         struct FooOuter {
             inner: Vec<Foo>,
         }
         let vfoo = vec![FooOuter {
             inner: vec![Foo { bar: 0 }],
         }];
         let _ = !vfoo.iter().any(|v| v.inner[0].bar == 2);
 
     fn double_deref_index_projection() {
     fn double_deref_index_projection() {
         let vfoo = vec![&&[0, 1, 2, 3]];
         let _ = !vfoo.iter().any(|x| (**x)[0] == 9);
 
     fn method_call_by_ref() {
         struct Foo {
             bar: u32,
             bar: u32,
         }
         impl Foo {
             pub fn by_ref(&self, x: &u32) -> bool {
                 *x == self.bar
         }
         }
---
To only update this specific test, also pass `--test-args unnecessary_find_map.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/unnecessary_find_map.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unnecessary_find_map.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-f2da228ff241ec97.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-b9c731f380466bd0.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-97618c8d1e1f91be.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-3ef490519219f110.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-e95902f0f3ef680a.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-5ac9fbce59ab185e.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-f474b2dfd384a595.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-4ace64e065702c69.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-8ab3fb021f549e8b.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unnecessary_find_map.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------

------------------------------------------

diff of stderr:

 error: this `.fold` can be written more succinctly using another method
-   |
-   |
-LL |     let _ = (0..3).fold(false, |acc, x| acc || x > 2);
-   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `any(|x| x > 2)`
-   |
-   = note: `-D clippy::unnecessary-fold` implied by `-D warnings`
-
-error: this `.fold` can be written more succinctly using another method
-   |
-   |
-LL |     let _ = (0..3).fold(true, |acc, x| acc && x > 2);
-   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `all(|x| x > 2)`
-
-error: this `.fold` can be written more succinctly using another method
-   |
-   |
-LL |     let _: i32 = (0..3).fold(0, |acc, x| acc + x);
-   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `sum()`
-
-error: this `.fold` can be written more succinctly using another method
-   |
-   |
-LL |     let _: i32 = (0..3).fold(1, |acc, x| acc * x);
-   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `product()`
-
-error: this `.fold` can be written more succinctly using another method
    |
    |
 LL |     let _: bool = (0..3).map(|x| 2 * x).fold(false, |acc, x| acc || x > 2);
    |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `any(|x| x > 2)`
+   |
+   = note: `-D clippy::unnecessary-fold` implied by `-D warnings`
 
 error: this `.fold` can be written more succinctly using another method
    |
    |
 LL |         .fold(false, |acc, x| acc || x > 2);
    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `any(|x| x > 2)`
-error: aborting due to 6 previous errors
+error: aborting due to 2 previous errors
 
 
---
 // run-rustfix
 
 #![allow(dead_code)]
 
 /// Calls which should trigger the `UNNECESSARY_FOLD` lint
 fn unnecessary_fold() {
     // Can be replaced by .any
-    let _ = (0..3).any(|x| x > 2);
+    let _ = (0..3).fold(false, |acc, x| acc || x > 2);
     // Can be replaced by .all
-    let _ = (0..3).all(|x| x > 2);
+    let _ = (0..3).fold(true, |acc, x| acc && x > 2);
     // Can be replaced by .sum
-    let _: i32 = (0..3).sum();
+    let _: i32 = (0..3).fold(0, |acc, x| acc + x);
     // Can be replaced by .product
-    let _: i32 = (0..3).product();
+    let _: i32 = (0..3).fold(1, |acc, x| acc * x);
 
 
 /// Should trigger the `UNNECESSARY_FOLD` lint, with an error span including exactly `.fold(...)`
 fn unnecessary_fold_span_for_multi_element_chain() {
     let _: bool = (0..3).map(|x| 2 * x).any(|x| x > 2);
 
 
 /// Calls which should not trigger the `UNNECESSARY_FOLD` lint
 fn unnecessary_fold_should_ignore() {
     let _ = (0..3).fold(true, |acc, x| acc || x > 2);
     let _ = (0..3).fold(false, |acc, x| acc && x > 2);
     let _ = (0..3).fold(1, |acc, x| acc + x);
     let _ = (0..3).fold(0, |acc, x| acc * x);
     let _ = (0..3).fold(0, |acc, x| 1 + acc + x);
 
     // We only match against an accumulator on the left
     // hand side. We could lint for .sum and .product when
     // it's on the right, but don't for now (and this wouldn't
     // be valid if we extended the lint to cover arbitrary numeric
     // types).
     let _ = (0..3).fold(false, |acc, x| x > 2 || acc);
     let _ = (0..3).fold(true, |acc, x| x > 2 && acc);
     let _ = (0..3).fold(0, |acc, x| x + acc);
     let _ = (0..3).fold(1, |acc, x| x * acc);
 
     let _ = [(0..2), (0..3)].iter().fold(0, |a, b| a + b.len());
     let _ = [(0..2), (0..3)].iter().fold(1, |a, b| a * b.len());
 
 /// Should lint only the line containing the fold
 fn unnecessary_fold_over_multiple_lines() {
     let _ = (0..3)
     let _ = (0..3)
         .map(|x| x + 1)
         .filter(|x| x % 2 == 0)
         .any(|x| x > 2);
 
 fn main() {}
 


The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unnecessary_fold.stage-id.fixed
To only update this specific test, also pass `--test-args unnecessary_fold.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/unnecessary_fold.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unnecessary_fold.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-f2da228ff241ec97.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-b9c731f380466bd0.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-97618c8d1e1f91be.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-3ef490519219f110.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-e95902f0f3ef680a.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-5ac9fbce59ab185e.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-f474b2dfd384a595.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-4ace64e065702c69.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-8ab3fb021f549e8b.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unnecessary_fold.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this `.fold` can be written more succinctly using another method","code":{"code":"clippy::unnecessary_fold","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_fold.rs","byte_start":654,"byte_end":688,"line_start":19,"line_end":19,"column_start":41,"column_end":75,"is_primary":true,"text":[{"text":"    let _: bool = (0..3).map(|x| 2 * x).fold(false, |acc, x| acc || x > 2);","highlight_start":41,"highlight_end":75}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::unnecessary-fold` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_fold.rs","byte_start":654,"byte_end":688,"line_start":19,"line_end":19,"column_start":41,"column_end":75,"is_primary":true,"text":[{"text":"    let _: bool = (0..3).map(|x| 2 * x).fold(false, |acc, x| acc || x > 2);","highlight_start":41,"highlight_end":75}],"label":null,"suggested_replacement":"any(|x| x > 2)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `.fold` can be written more succinctly using another method\n  --> tests/ui/unnecessary_fold.rs:19:41\n   |\nLL |     let _: bool = (0..3).map(|x| 2 * x).fold(false, |acc, x| acc || x > 2);\n   |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `any(|x| x > 2)`\n   |\n   = note: `-D clippy::unnecessary-fold` implied by `-D warnings`\n\n"}
{"message":"this `.fold` can be written more succinctly using another method","code":{"code":"clippy::unnecessary_fold","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/unnecessary_fold.rs","byte_start":1819,"byte_end":1853,"line_start":49,"line_end":49,"column_start":10,"column_end":44,"is_primary":true,"text":[{"text":"        .fold(false, |acc, x| acc || x > 2);","highlight_start":10,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/unnecessary_fold.rs","byte_start":1819,"byte_end":1853,"line_start":49,"line_end":49,"column_start":10,"column_end":44,"is_primary":true,"text":[{"text":"        .fold(false, |acc, x| acc || x > 2);","highlight_start":10,"highlight_end":44}],"label":null,"suggested_replacement":"any(|x| x > 2)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this `.fold` can be written more succinctly using another method\n  --> tests/ui/unnecessary_fold.rs:49:10\n   |\nLL |         .fold(false, |acc, x| acc || x > 2);\n   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `any(|x| x > 2)`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

-error: this `.filter_map` can be written more simply using `.filter`
-   |
-   |
-LL |     let _ = (0..4).filter_map(|x| if x > 1 { Some(x) } else { None });
-   |
-   |
-   = note: `-D clippy::unnecessary-filter-map` implied by `-D warnings`
-
-error: this `.filter_map` can be written more simply using `.filter`
-   |
-   |
-LL |       let _ = (0..4).filter_map(|x| {
-   |  _____________^
-LL | |         if x > 1 {
-LL | |             return Some(x);
-LL | |         };
-LL | |         None
-LL | |     });
-
-
-error: this `.filter_map` can be written more simply using `.filter`
-   |
-   |
-LL |       let _ = (0..4).filter_map(|x| match x {
-LL | |         0 | 1 => None,
-LL | |         0 | 1 => None,
-LL | |         _ => Some(x),
-LL | |     });
-
-
-error: this `.filter_map` can be written more simply using `.map`
-   |
-   |
-LL |     let _ = (0..4).filter_map(|x| Some(x + 1));
-
-error: aborting due to 4 previous errors
-
-
---
To only update this specific test, also pass `--test-args unnecessary_filter_map.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/unnecessary_filter_map.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unnecessary_filter_map.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-f2da228ff241ec97.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-b9c731f380466bd0.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-97618c8d1e1f91be.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-3ef490519219f110.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-e95902f0f3ef680a.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-5ac9fbce59ab185e.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-f474b2dfd384a595.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-4ace64e065702c69.so" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-8ab3fb021f549e8b.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/unnecessary_filter_map.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
