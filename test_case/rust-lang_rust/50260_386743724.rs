plain
[01:06:06] 
[01:06:06] ---- [ui] ui-fulldeps/unnecessary-extern-crate.rs stdout ----
[01:06:06]  diff of stderr:
[01:06:06] 
[01:06:06] 1 error: `extern crate` is unnecessary in the new edition
[01:06:06] -   --> $DIR/unnecessary-extern-crate.rs:4:1
[01:06:06] +   --> $DIR/unnecessary-extern-crate.rs:14:1
[01:06:06] 3    |
[01:06:06] 4 LL | extern crate alloc;
[01:06:06] 5    | ^^^^^^^^^^^^^^^^^^^ help: remove it
[01:06:06] 6    |
[01:06:06] 7 note: lint level defined here
[01:06:06] -   --> $DIR/unnecessary-extern-crate.rs:1:9
[01:06:06] +   --> $DIR/unnecessary-extern-crate.rs:11:9
[01:06:06] +   --> $DIR/unnecessary-extern-crate.rs:11:9
[01:06:06] 9    |
[01:06:06] 10 LL | #![deny(unnecessary_extern_crate)]
[01:06:06] 
[01:06:06] 12 
[01:06:06] 12 
[01:06:06] 13 error: `extern crate` is unnecessary in the new edition
[01:06:06] -   --> $DIR/unnecessary-extern-crate.rs:7:1
[01:06:06] +   --> $DIR/unnecessary-extern-crate.rs:17:1
[01:06:06] 15    |
[01:06:06] 16 LL | extern crate alloc as x;
[01:06:06] 17    | ^^^^^^^^^^^^^^^^^^^^^^^^ help: use `use`: `use alloc as x`
[01:06:06] 18 
[01:06:06] 18 
[01:06:06] 19 error: `extern crate` is unnecessary in the new edition
[01:06:06] -   --> $DIR/unnecessary-extern-crate.rs:13:1
[01:06:06] +   --> $DIR/unnecessary-extern-crate.rs:23:1
[01:06:06] 21    |
[01:06:06] 22 LL | pub extern crate test as y;
[01:06:06] 23    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `pub use`: `pub use test as y`
[01:06:06] 24 
[01:06:06] 24 
[01:06:06] 25 error: `extern crate` is unnecessary in the new edition
[01:06:06] -   --> $DIR/unnecessary-extern-crate.rs:16:1
[01:06:06] +   --> $DIR/unnecessary-extern-crate.rs:26:1
[01:06:06] 27    |
[01:06:06] 28 LL | pub extern crate libc;
[01:06:06] 29    | ^^^^^^^^^^^^^^^^^^^^^^ help: use `pub use`: `pub use libc`
[01:06:06] 30 
[01:06:06] 30 
[01:06:06] 31 error: `extern crate` is unnecessary in the new edition
[01:06:06] -   --> $DIR/unnecessary-extern-crate.rs:22:5
[01:06:06] +   --> $DIR/unnecessary-extern-crate.rs:32:5
[01:06:06] 33    |
[01:06:06] 34 LL |     extern crate alloc;
[01:06:06] 35    |     ^^^^^^^^^^^^^^^^^^^ help: use `use`: `use alloc`
[01:06:06] 36 
[01:06:06] 36 
[01:06:06] 37 error: `extern crate` is unnecessary in the new edition
[01:06:06] -   --> $DIR/unnecessary-extern-crate.rs:25:5
[01:06:06] +   --> $DIR/unnecessary-extern-crate.rs:35:5
[01:06:06] 39    |
[01:06:06] 40 LL |     extern crate alloc as x;
[01:06:06] 41    |     ^^^^^^^^^^^^^^^^^^^^^^^^ help: use `use`: `use alloc as x`
[01:06:06] 42 
[01:06:06] 42 
[01:06:06] 43 error: `extern crate` is unnecessary in the new edition
[01:06:06] -   --> $DIR/unnecessary-extern-crate.rs:28:5
[01:06:06] +   --> $DIR/unnecessary-extern-crate.rs:38:5
[01:06:06] 45    |
[01:06:06] 46 LL |     pub extern crate test;
[01:06:06] 47    |     ^^^^^^^^^^^^^^^^^^^^^^ help: use `pub use`: `pub use test`
[01:06:06] 48 
[01:06:06] 48 
[01:06:06] 49 error: `extern crate` is unnecessary in the new edition
[01:06:06] -   --> $DIR/unnecessary-extern-crate.rs:31:5
[01:06:06] +   --> $DIR/unnecessary-extern-crate.rs:41:5
[01:06:06] 51    |
[01:06:06] 52 LL |     pub extern crate test as y;
[01:06:06] 53    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `pub use`: `pub use test as y`
[01:06:06] 54 
[01:06:06] 54 
[01:06:06] 55 error: `extern crate` is unnecessary in the new edition
[01:06:06] -   --> $DIR/unnecessary-extern-crate.rs:35:9
[01:06:06] +   --> $DIR/unnecessary-extern-crate.rs:45:9
[01:06:06] 57    |
[01:06:06] 58 LL |         extern crate alloc;
[01:06:06] 59    |         ^^^^^^^^^^^^^^^^^^^ help: use `use`: `use alloc`
[01:06:06] 60 
[01:06:06] 60 
[01:06:06] 61 error: `extern crate` is unnecessary in the new edition
[01:06:06] -   --> $DIR/unnecessary-extern-crate.rs:38:9
[01:06:06] +   --> $DIR/unnecessary-extern-crate.rs:48:9
[01:06:06] 63    |
[01:06:06] 64 LL |         extern crate alloc as x;
[01:06:06] 65    |         ^^^^^^^^^^^^^^^^^^^^^^^^ help: use `use`: `use alloc as x`
[01:06:06] 
[01:06:06] The actual stderr differed from the expected stderr.
[01:06:06] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/unnecessary-extern-crate.stderr
[01:06:06] To update references, run this command from build directory:
[01:06:06] To update references, run this command from build directory:
[01:06:06] /checkout/src/test/ui-fulldeps/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps' 'unnecessary-extern-crate.rs'
[01:06:06] error: 1 errors occurred comparing output.
[01:06:06] status: exit code: 101
[01:06:06] status: exit code: 101
[01:06:06] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/unnecessary-extern-crate.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/unnecessary-extern-crate.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[01:06:06] ------------------------------------------
[01:06:06] 
[01:06:06] ------------------------------------------
[01:06:06] stderr:
[01:06:06] stderr:
[01:06:06] ------------------------------------------
[01:06:06] {"message":"`extern crate` is unnecessary in the new edition","code":{"code":"unnecessary_extern_crate","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs","byte_start":534,"byte_end":553,"line_start":14,"line_end":14,"column_start":1,"column_end":20,"is_primary":true,"text":[{"text":"extern crate alloc;","highlight_start":1,"highlight_end":20}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs","byte_start":475,"byte_end":499,"line_start":11,"line_end":11,"column_start":9,"column_end":33,"is_primary":true,"text":[{"text":"#![deny(unnecessary_extern_crate)]","highlight_start":9,"highlight_end":33}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove it","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs","byte_start":534,"byte_end":553,"line_start":14,"line_end":14,"column_start":1,"column_end":20,"is_primary":true,"text":[{"text":"extern crate alloc;","highlight_start":1,"highlight_end":20}],"label":null,"suggested_replacement":"","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `extern crate` is unnecessary in the new edition\n  --> /checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs:14:1\n   |\nLL | extern crate alloc;\n   | ^^^^^^^^^^^^^^^^^^^ help: remove it\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs:11:9\n   |\nLL | #![deny(unnecessary_extern_crate)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:06:06] {"message":"`extern crate` is unnecessary in the new edition","code":{"code":"unnecessary_extern_crate","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs","byte_start":631,"byte_end":655,"line_start":17,"line_end":17,"column_start":1,"column_end":25,"is_primary":true,"text":[{"text":"extern crate alloc as x;","highlight_start":1,"highlight_end":25}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"use `use`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs","byte_start":631,"byte_end":655,"line_start":17,"line_end":17,"column_start":1,"column_end":25,"is_primary":true,"text":[{"text":"extern crate alloc as x;","highlight_start":1,"highlight_end":25}],"label":null,"suggested_replacement":"use alloc as x","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `extern crate` is unnecessary in the new edition\n  --> /checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs:17:1\n   |\nLL | extern crate alloc as x;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^ help: use `use`: `use alloc as x`\n\n"}
[01:06:06] {"message":"`extern crate` is unnecessary in the new edition","code":{"code":"unnecessary_extern_crate","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs","byte_start":769,"byte_end":796,"line_start":23,"line_end":23,"column_start":1,"column_end":28,"is_primary":true,"text":[{"text":"pub extern crate test as y;","highlight_start":1,"highlight_end":28}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"use `pub use`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs","byte_start":769,"byte_end":796,"line_start":23,"line_end":23,"column_start":1,"column_end":28,"is_primary":true,"text":[{"text":"pub extern crate test as y;","highlight_start":1,"highlight_end":28}],"label":null,"suggested_replacement":"pub use test as y","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `extern crate` is unnecessary in the new edition\n  --> /checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs:23:1\n   |\nLL | pub extern crate test as y;\n   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `pub use`: `pub use test as y`\n\n"}
[01:06:06] {"message":"`extern crate` is unnecessary in the new edition","code":{"code":"unnecessary_extern_crate","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs","byte_start":881,"byte_end":903,"line_start":26,"line_end":26,"column_start":1,"column_end":23,"is_primary":true,"text":[{"text":"pub extern crate libc;","highlight_start":1,"highlight_end":23}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"use `pub use`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs","byte_start":881,"byte_end":903,"line_start":26,"line_end":26,"column_start":1,"column_end":23,"is_primary":true,"text":[{"text":"pub extern crate libc;","highlight_start":1,"highlight_end":23}],"label":null,"suggested_replacement":"pub use libc","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `extern crate` is unnecessary in the new edition\n  --> /checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs:26:1\n   |\nLL | pub extern crate libc;\n   | ^^^^^^^^^^^^^^^^^^^^^^ help: use `pub use`: `pub use libc`\n\n"}
[01:06:06] {"message":"`extern crate` is unnecessary in the new edition","code":{"code":"unnecessary_extern_crate","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs","byte_start":1004,"byte_end":1023,"line_start":32,"line_end":32,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    extern crate alloc;","highlight_start":5,"highlight_end":24}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"use `use`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs","byte_start":1004,"byte_end":1023,"line_start":32,"line_end":32,"column_start":5,"column_end":24,"is_primary":true,"text":[{"text":"    extern crate alloc;","highlight_start":5,"highlight_end":24}],"label":null,"suggested_replacement":"use alloc","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `extern crate` is unnecessary in the new edition\n  --> /checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs:32:5\n   |\nLL |     extern crate alloc;\n   |     ^^^^^^^^^^^^^^^^^^^ help: use `use`: `use alloc`\n\n"}
[01:06:06] {"message":"`extern crate` is unnecessary in the new edition","code":{"code":"unnecessary_extern_crate","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs","byte_start":1116,"byte_end":1140,"line_start":35,"line_end":35,"column_start":5,"column_end":29,"is_primary":true,"text":[{"text":"    extern crate alloc as x;","highlight_start":5,"highlight_end":29}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"use `use`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs","byte_start":1116,"byte_end":1140,"line_start":35,"line_end":35,"column_start":5,"column_end":29,"is_primary":true,"text":[{"text":"    extern crate alloc as x;","highlight_start":5,"highlight_end":29}],"label":null,"suggested_replacement":"use alloc as x","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `extern crate` is unnecessary in the new edition\n  --> /checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs:35:5\n   |\nLL |     extern crate alloc as x;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^ help: use `use`: `use alloc as x`\n\n"}
[01:06:06] {"message":"`extern crate` is unnecessary in the new edition","code":{"code":"unnecessary_extern_crate","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs","byte_start":1233,"byte_end":1255,"line_start":38,"line_end":38,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    pub extern crate test;","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"use `pub use`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs","byte_start":1233,"byte_end":1255,"line_start":38,"line_end":38,"column_start":5,"column_end":27,"is_primary":true,"text":[{"text":"    pub extern crate test;","highlight_start":5,"highlight_end":27}],"label":null,"suggested_replacement":"pub use test","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `extern crate` is unnecessary in the new edition\n  --> /checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs:38:5\n   |\nLL |     pub extern crate test;\n   |     ^^^^^^^^^^^^^^^^^^^^^^ help: use `pub use`: `pub use test`\n\n"}
[01:06:06] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[01:06:06] {"message":"`extern crate` is unnecessary in the new edition","code":{"code":"unnecessary_extern_crate","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs","byte_start":1352,"byte_end":1379,"line_start":41,"line_end":41,"column_start":5,"column_end":32,"is_primary":true,"text":[{"text":"    pub extern crate test as y;","highlight_start":5,"highlight_end":32}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"use `pub use`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs","byte_start":1352,"byte_end":1379,"line_start":41,"line_end":41,"column_start":5,"column_end":32,"is_primary":true,"text":[{"text":"    pub extern crate test as y;","highlight_start":5,"highlight_end":32}],"label":null,"suggested_replacement":"pub use test as y","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `extern crate` is unnecessary in the new edition\n  --> /checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs:41:5\n   |\nLL |     pub extern crate test as y;\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `pub use`: `pub use test as y`\n\n"}
[01:06:06] {"message":"`extern crate` is unnecessary in the new edition","code":{"code":"unnecessary_extern_crate","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs","byte_start":1494,"byte_end":1513,"line_start":45,"line_end":45,"column_start":9,"column_end":28,"is_primary":true,"text":[{"text":"        extern crate alloc;","highlight_start":9,"highlight_end":28}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"use `use`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs","byte_start":1494,"byte_end":1513,"line_start":45,"line_end":45,"column_start":9,"column_end":28,"is_primary":true,"text":[{"text":"        extern crate alloc;","highlight_start":9,"highlight_end":28}],"label":null,"suggested_replacement":"use alloc","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `extern crate` is unnecessary in the new edition\n  --> /checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs:45:9\n   |\nLL |         extern crate alloc;\n   |         ^^^^^^^^^^^^^^^^^^^ help: use `use`: `use alloc`\n\n"}
[01:06:06] {"message":"`extern crate` is unnecessary in the new edition","code":{"code":"unnecessary_extern_crate","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs","byte_start":1618,"byte_end":1642,"line_start":48,"line_end":48,"column_start":9,"column_end":33,"is_primary":true,"text":[{"text":"        extern crate alloc as x;","highlight_start":9,"highlight_end":33}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"use `use`","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs","byte_start":1618,"byte_end":1642,"line_start":48,"line_end":48,"column_start":9,"column_end":33,"is_primary":true,"text":[{"text":"        extern crate alloc as x;","highlight_start":9,"highlight_end":33}],"label":null,"suggested_replacement":"use alloc as x","expansion":null}],"children":[],"rendered":null}],"rendered":"error: `extern crate` is unnecessary in the new edition\n  --> /checkout/src/test/ui-fulldeps/unnecessary-extern-crate.rs:48:9\n   |\nLL |         extern crate alloc as x;\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^ help: use `use`: `use alloc as x`\n\n"}
[01:06:06] {"message":"aborting due to 10 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 10 previous errors\n\n"}
[01:06:06] ------------------------------------------
[01:06:06] 
[01:06:06] thread '[ui] ui-fulldeps/unnecessary-extern-crate.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2967:9
[01:06:06] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[01:06:06] test result: FAILED. 12 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
[01:06:06] 
[01:06:06] 
[01:06:06] 
[01:06:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "3.9.1\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:06:06] 
[01:06:06] 
[01:06:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:06:06] Build completed unsuccessfully in 0:20:20
[01:06:06] Build completed unsuccessfully in 0:20:20
[01:06:06] make: *** [check] Error 1
[01:06:06] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:28bc55b0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
