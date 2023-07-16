\n\nFor more information on the rust ownership system, take a look at\nhttps://doc.rust-lang.org/stable/book/references-and-borrowing.html.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/nll/get_default.rs","byte_start":1053,"byte_end":1075,"line_start":33,"line_end":33,"column_start":17,"column_end":39,"is_primary":true,"text":[{"text":"                map.set(String::new()); // Ideally, this would not error.","highlight_start":17,"highlight_end":39}],"label":"mutable borrow occurs here","suggested_replacement":null,"expansion":null},{"file_name":"/checkout/src/test/ui/nll/get_default.rs","byte_start":938,"byte_end":941,"line_start":28,"line_end":28,"column_start":15,"column_end":18,"is_primary":false,"text":[{"text":"        match map.get() {","highlight_start":15,"highlight_end":18}],"label":"immutable borrow occurs here","suggested_replacement":null,"expansion":null}],"children":[{"message":"borrowed value must be valid for the anonymous lifetime #1 defined on the function body at 26:1...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/nll/get_default.rs","byte_start":879,"byte_end":1252,"line_start":26,"line_end":39,"column_start":1,"column_end":2,"is_primary":true,"text":[{"text":"fn ok(map: &mut Map) -> &String {","highlight_start":1,"highlight_end":34},{"text":"    loop {","highlight_starow occurs here\n   |\nnote: borrowed value must be valid for the anonymous lifetime #1 defined on the function body at 41:1...\n  --> /checkout/src/test/ui/nll/get_default.rs:41:1\n   |\nLL | / fn err(map: &mut Map) -> &String {\nLL | |     loop {\nLL | |         match map.get() {\nLL | |             Some(v) => {\n...  |\nLL | |     }\nLL | | }\n   | |_^\n\n"}
[01:03:55] {"message":"aborting due to 6 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 6 previous errors\n\n"}
[01:03:55] {"message":"For more information about this error, try `rustc --explain E0502`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0502`.\n"}
[01:03:55] ------------------------------------------
[01:03:55] 
[01:03:55] thread '[ui (nll)] ui/nll/get_default.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3033:9
[01:03:55] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:03:55] 
[01:03:55] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:492:22
[01:03:55] 
[01:03:55] 
[01:03:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lt=1525964117704774564,finish=1525964117753449699,duration=48675135
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:23470d5d
