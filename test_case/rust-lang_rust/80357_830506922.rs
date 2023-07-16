plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---

---- compile_test stdout ----
diff of stderr:

 error: this loop could be written as a `for` loop
    |
    |
 LL |     while let Option::Some(x) = iter.next() {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `for x in iter`
    |
    = note: `-D clippy::while-let-on-iterator` implied by `-D warnings`
 
 error: this loop could be written as a `for` loop
    |
    |
 LL |     while let Some(x) = iter.next() {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `for x in iter`
 
 error: this loop could be written as a `for` loop
    |
    |
 LL |     while let Some(_) = iter.next() {}
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `for _ in iter`
 
 error: this loop could be written as a `for` loop
    |
    |
 LL |         while let Some([..]) = it.next() {}
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `for [..] in it`
 
 error: this loop could be written as a `for` loop
    |
    |
 LL |         while let Some([_x]) = it.next() {}
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `for [_x] in it`
 
 error: this loop could be written as a `for` loop
    |
error: test failed, to rerun pass '--test compile-test'
error: test failed, to rerun pass '--test compile-test'
 LL |         while let Some(x @ [_]) = it.next() {
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `for x @ [_] in it`
 
 error: this loop could be written as a `for` loop
+  --> $DIR/while_let_on_iterator.rs:141:9
    |
    |
+LL |         while let Some(_) = y.next() {}
+   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `for _ in y`
+
+error: this loop could be written as a `for` loop
+   |
+   |
 LL |         while let Some(_) = y.next() {
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `for _ in y`
-error: aborting due to 7 previous errors
+error: aborting due to 8 previous errors
 
 
 

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/while_let_on_iterator.stage-id.stderr
diff of fixed:

 // run-rustfix
 
 #![warn(clippy::while_let_on_iterator)]
 #![allow(clippy::never_loop, unreachable_code, unused_mut)]
 fn base() {
     let mut iter = 1..20;
     for x in iter {
         println!("{}", x);
         println!("{}", x);
     }
 
     let mut iter = 1..20;
     for x in iter {
         println!("{}", x);
     }
 
     let mut iter = 1..20;
     for _ in iter {}
     let mut iter = 1..20;
     let mut iter = 1..20;
     while let None = iter.next() {} // this is fine (if nonsensical)
     let mut iter = 1..20;
     let mut iter = 1..20;
     if let Some(x) = iter.next() {
         // also fine
     }
 
 
     // the following shouldn't warn because it can't be written with a for loop
     let mut iter = 1u32..20;
     while let Some(_) = iter.next() {
         println!("next: {:?}", iter.next())
 
     // neither can this
     let mut iter = 1u32..20;
     let mut iter = 1u32..20;
     while let Some(_) = iter.next() {
         println!("next: {:?}", iter.next());
 
     // or this
     let mut iter = 1u32..20;
     let mut iter = 1u32..20;
     while let Some(_) = iter.next() {
         break;
     }
     println!("Remaining iter {:?}", iter);
     // or this
     let mut iter = 1u32..20;
     let mut iter = 1u32..20;
     while let Some(_) = iter.next() {
         iter = 1..20;
 }
 
 // Issue #1188
 fn refutable() {
 fn refutable() {
     let a = [42, 1337];
     let mut b = a.iter();
     // consume all the 42s
     // consume all the 42s
     while let Some(&42) = b.next() {}
 
     let a = [(1, 2, 3)];
     let mut b = a.iter();
 
     while let Some(&(1, 2, 3)) = b.next() {}
 
     let a = [Some(42)];
     let mut b = a.iter();
 
     while let Some(&None) = b.next() {}
 
     /* This gives “refutable pattern in `for` loop binding: `&_` not covered”
     for &42 in b {}
     for &(1, 2, 3) in b {}
     for &Option::None in b.next() {}
     // */
 
 fn refutable2() {
     // Issue 3780
     {
     {
         let v = vec![1, 2, 3];
         let mut it = v.windows(2);
         while let Some([x, y]) = it.next() {
             println!("x: {}", x);
             println!("y: {}", y);
 
 
         let mut it = v.windows(2);
         while let Some([x, ..]) = it.next() {
             println!("x: {}", x);
 
 
         let mut it = v.windows(2);
         while let Some([.., y]) = it.next() {
             println!("y: {}", y);
 
 
         let mut it = v.windows(2);
         for [..] in it {}
 
         let v = vec![[1], [2], [3]];
         let mut it = v.iter();
         while let Some([1]) = it.next() {}
 
         let mut it = v.iter();
         for [_x] in it {}
 
     // binding
     {
     {
         let v = vec![1, 2, 3];
         let mut it = v.iter();
         while let Some(x @ 1) = it.next() {
         }
 
 
         let v = vec![[1], [2], [3]];
         let mut it = v.iter();
         for x @ [_] in it {
         }
     }
 
     // false negative
     // false negative
     {
         let v = vec![1, 2, 3];
         let mut it = v.iter().map(Some);
         while let Some(Some(_) | None) = it.next() {
             println!("1");
     }
 }
 
 
 fn nested_loops() {
     let a = [42, 1337];
     let mut y = a.iter();
     loop {
         // x is reused, so don't lint here
-        while let Some(_) = y.next() {}
+        for _ in y {}
 
 
     let mut y = a.iter();
     for _ in 0..2 {
         while let Some(_) = y.next() {
             // y is reused, don't lint
     }
 
     loop {
     loop {
         let mut y = a.iter();
         for _ in y {
             // use a for loop here
     }
 }
 
 fn issue1121() {
 fn issue1121() {
     use std::collections::HashSet;
     let mut values = HashSet::new();
     values.insert(1);
 
     while let Some(&value) = values.iter().next() {
         values.remove(&value);
 }
 
 fn issue2965() {
 fn issue2965() {
     // This should not cause an ICE and suggest:
     //
     // for _ in values.iter() {}
     use std::collections::HashSet;
     let mut values = HashSet::new();
     values.insert(1);
 
 
     while let Some(..) = values.iter().next() {}
 
 fn issue3670() {
 fn issue3670() {
     let array = [Some(0), None, Some(1)];
     let mut iter = array.iter();
 
     while let Some(elem) = iter.next() {
         let _ = elem.or_else(|| *iter.next()?);
 }
 
 fn issue1654() {
 fn issue1654() {
     // should not lint if the iterator is generated on every iteration
     use std::collections::HashSet;
     let mut values = HashSet::new();
     values.insert(1);
 
     while let Some(..) = values.iter().next() {
         values.remove(&1);
 
 
     while let Some(..) = values.iter().map(|x| x + 1).next() {}
 
     let chars = "Hello, World!".char_indices();
     while let Some((i, ch)) = chars.clone().next() {
         println!("{}: {}", i, ch);
 }
 
 fn main() {
     base();
---
     issue1654();
 }
 

The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/while_let_on_iterator.stage-id.fixed
To only update this specific test, also pass `--test-args while_let_on_iterator.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/clippy-driver" "tests/ui/while_let_on_iterator.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/while_let_on_iterator.stage-id" "-A" "unused" "--emit=metadata" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-Dwarnings" "-Zui-testing" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-3365d689274e8da9.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-934c285f6858724f.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-7f4531ca9e916653.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-43d16fd8e2fbc291.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-3f3ead7dae58a5a8.rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/test_build_base/while_let_on_iterator.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"this loop could be written as a `for` loop","code":{"code":"clippy::while_let_on_iterator","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/while_let_on_iterator.rs","byte_start":159,"byte_end":198,"line_start":8,"line_end":8,"column_start":5,"column_end":44,"is_primary":true,"text":[{"text":"    while let Option::Some(x) = iter.next() {","highlight_start":5,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D clippy::while-let-on-iterator` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/while_let_on_iterator.rs","byte_start":159,"byte_end":198,"line_start":8,"line_end":8,"column_start":5,"column_end":44,"is_primary":true,"text":[{"text":"    while let Option::Some(x) = iter.next() {","highlight_start":5,"highlight_end":44}],"label":null,"suggested_replacement":"for x in iter","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this loop could be written as a `for` loop\n  --> tests/ui/while_let_on_iterator.rs:8:5\n   |\nLL |     while let Option::Some(x) = iter.next() {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `for x in iter`\n   |\n   = note: `-D clippy::while-let-on-iterator` implied by `-D warnings`\n\n"}
{"message":"this loop could be written as a `for` loop","code":{"code":"clippy::while_let_on_iterator","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/while_let_on_iterator.rs","byte_start":265,"byte_end":296,"line_start":13,"line_end":13,"column_start":5,"column_end":36,"is_primary":true,"text":[{"text":"    while let Some(x) = iter.next() {","highlight_start":5,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/while_let_on_iterator.rs","byte_start":265,"byte_end":296,"line_start":13,"line_end":13,"column_start":5,"column_end":36,"is_primary":true,"text":[{"text":"    while let Some(x) = iter.next() {","highlight_start":5,"highlight_end":36}],"label":null,"suggested_replacement":"for x in iter","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this loop could be written as a `for` loop\n  --> tests/ui/while_let_on_iterator.rs:13:5\n   |\nLL |     while let Some(x) = iter.next() {\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `for x in iter`\n\n"}
{"message":"this loop could be written as a `for` loop","code":{"code":"clippy::while_let_on_iterator","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/while_let_on_iterator.rs","byte_start":363,"byte_end":394,"line_start":18,"line_end":18,"column_start":5,"column_end":36,"is_primary":true,"text":[{"text":"    while let Some(_) = iter.next() {}","highlight_start":5,"highlight_end":36}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/while_let_on_iterator.rs","byte_start":363,"byte_end":394,"line_start":18,"line_end":18,"column_start":5,"column_end":36,"is_primary":true,"text":[{"text":"    while let Some(_) = iter.next() {}","highlight_start":5,"highlight_end":36}],"label":null,"suggested_replacement":"for _ in iter","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this loop could be written as a `for` loop\n  --> tests/ui/while_let_on_iterator.rs:18:5\n   |\nLL |     while let Some(_) = iter.next() {}\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `for _ in iter`\n\n"}
{"message":"this loop could be written as a `for` loop","code":{"code":"clippy::while_let_on_iterator","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/while_let_on_iterator.rs","byte_start":2256,"byte_end":2288,"line_start":101,"line_end":101,"column_start":9,"column_end":41,"is_primary":true,"text":[{"text":"        while let Some([..]) = it.next() {}","highlight_start":9,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/while_let_on_iterator.rs","byte_start":2256,"byte_end":2288,"line_start":101,"line_end":101,"column_start":9,"column_end":41,"is_primary":true,"text":[{"text":"        while let Some([..]) = it.next() {}","highlight_start":9,"highlight_end":41}],"label":null,"suggested_replacement":"for [..] in it","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this loop could be written as a `for` loop\n  --> tests/ui/while_let_on_iterator.rs:101:9\n   |\nLL |         while let Some([..]) = it.next() {}\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `for [..] in it`\n\n"}
{"message":"this loop could be written as a `for` loop","code":{"code":"clippy::while_let_on_iterator","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/while_let_on_iterator.rs","byte_start":2444,"byte_end":2476,"line_start":108,"line_end":108,"column_start":9,"column_end":41,"is_primary":true,"text":[{"text":"        while let Some([_x]) = it.next() {}","highlight_start":9,"highlight_end":41}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/while_let_on_iterator.rs","byte_start":2444,"byte_end":2476,"line_start":108,"line_end":108,"column_start":9,"column_end":41,"is_primary":true,"text":[{"text":"        while let Some([_x]) = it.next() {}","highlight_start":9,"highlight_end":41}],"label":null,"suggested_replacement":"for [_x] in it","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this loop could be written as a `for` loop\n  --> tests/ui/while_let_on_iterator.rs:108:9\n   |\nLL |         while let Some([_x]) = it.next() {}\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `for [_x] in it`\n\n"}
{"message":"this loop could be written as a `for` loop","code":{"code":"clippy::while_let_on_iterator","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/while_let_on_iterator.rs","byte_start":2732,"byte_end":2767,"line_start":121,"line_end":121,"column_start":9,"column_end":44,"is_primary":true,"text":[{"text":"        while let Some(x @ [_]) = it.next() {","highlight_start":9,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/while_let_on_iterator.rs","byte_start":2732,"byte_end":2767,"line_start":121,"line_end":121,"column_start":9,"column_end":44,"is_primary":true,"text":[{"text":"        while let Some(x @ [_]) = it.next() {","highlight_start":9,"highlight_end":44}],"label":null,"suggested_replacement":"for x @ [_] in it","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this loop could be written as a `for` loop\n  --> tests/ui/while_let_on_iterator.rs:121:9\n   |\nLL |         while let Some(x @ [_]) = it.next() {\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `for x @ [_] in it`\n\n"}
{"message":"this loop could be written as a `for` loop","code":{"code":"clippy::while_let_on_iterator","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/while_let_on_iterator.rs","byte_start":3151,"byte_end":3179,"line_start":141,"line_end":141,"column_start":9,"column_end":37,"is_primary":true,"text":[{"text":"        while let Some(_) = y.next() {}","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/while_let_on_iterator.rs","byte_start":3151,"byte_end":3179,"line_start":141,"line_end":141,"column_start":9,"column_end":37,"is_primary":true,"text":[{"text":"        while let Some(_) = y.next() {}","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":"for _ in y","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this loop could be written as a `for` loop\n  --> tests/ui/while_let_on_iterator.rs:141:9\n   |\nLL |         while let Some(_) = y.next() {}\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `for _ in y`\n\n"}
{"message":"this loop could be written as a `for` loop","code":{"code":"clippy::while_let_on_iterator","explanation":null},"level":"error","spans":[{"file_name":"tests/ui/while_let_on_iterator.rs","byte_start":3380,"byte_end":3408,"line_start":153,"line_end":153,"column_start":9,"column_end":37,"is_primary":true,"text":[{"text":"        while let Some(_) = y.next() {","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"try","code":null,"level":"help","spans":[{"file_name":"tests/ui/while_let_on_iterator.rs","byte_start":3380,"byte_end":3408,"line_start":153,"line_end":153,"column_start":9,"column_end":37,"is_primary":true,"text":[{"text":"        while let Some(_) = y.next() {","highlight_start":9,"highlight_end":37}],"label":null,"suggested_replacement":"for _ in y","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: this loop could be written as a `for` loop\n  --> tests/ui/while_let_on_iterator.rs:153:9\n   |\nLL |         while let Some(_) = y.next() {\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: try: `for _ in y`\n\n"}

------------------------------------------

thread 'compile_test' panicked at 'Some tests failed', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.6.0/src/lib.rs:105:22
