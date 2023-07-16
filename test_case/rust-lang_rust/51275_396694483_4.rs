\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/trivial-bounds-inconsistent-copy-reborrow.rs","byte_start":777,"byte_end":779,"line_start":20,"line_end":20,"column_start":6,"column_end":8,"is_primary":true,"text":[{"text":"    {*t} //~ ERROR","highlight_start":6,"highlight_end":8}],"label":"cannot borrow as mutable","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"consider changing this to be a mutable reference","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/trivial-bounds-inconsistent-copy-reborrow.rs","byte_start":714,"byte_end":729,"line_start":19,"line_end":19,"column_start":29,"column_end":44,"is_primary":true,"text":[{"text":"fn copy_reborrow_mut<'a>(t: &'a &'a mut i32) -> &'a mut i32 where &'a mut i32: Copy {","highlight_start":29,"highlight_end":44}],"label":null,"suggested_replacement":"&mut &mut i32","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0596]: cannot borrow immutable item `**t` as mutable\n  --> /checkout/src/test/ui/trivial-bounds-inconsistent-copy-reborrow.rs:20:6\n   |\nLL | fn copy_reborrow_mut<'a>(t: &'a &'a mut i32) -> &'a mut i32 where &'a mut i32: Copy {\n   |                             --------------- help: consider changing this to be a mutable reference: `&mut &mut i32`\nLL |     {*t} //~ ERROR\n   |      ^^ cannot borrow as mutable\n\n"}
[00:51:44] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[00:51:44] {"message":"For more information about this error, try `rustc --explain E0596`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0596`.\n"}
[00:51:44] ------------------------------------------
[00:51:44] 
[00:51:44] thread '[ui (nll)] ui/trivial-bounds-inconsistent-copy-reborrow.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3139:9
[00:51:44] 
---
[00:51:44] 
[00:51:44] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:498:22
[00:51:44] 
[00:51:44] 
[00:51:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rusd https://google.com | grep ^Date: | sed 's/Date: //g' || true)
travis_fold:start:after_failure.1
travis_time:start:00b4d0ab
