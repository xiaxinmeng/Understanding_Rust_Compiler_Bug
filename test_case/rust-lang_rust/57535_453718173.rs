plain
travis_time:end:09f4ea60:start=1547262790484177184,finish=1547262792584946636,duration=2100769452
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:04:47] .................................................................................................... 4900/5297
[01:04:50] .................................................................................................... 5000/5297
[01:04:53] .................................................................................................... 5100/5297
[01:04:56] .................................................................................................... 5200/5297
[01:04:59] ....................................i.....................................................F......
[01:04:59] 
[01:04:59] ---- [ui] ui/while-let.rs stdout ----
[01:04:59] diff of stderr:
[01:04:59] 
[01:04:59] 
[01:04:59] 27    |
[01:04:59] 28 LL | /     while let a = 1 { //~ WARN irrefutable while-let
[01:04:59] 29 LL | |         println!("irrefutable pattern");
[01:04:59] + LL | |         break;
[01:04:59] 30 LL | |     }
[01:04:59] 32 
[01:04:59] 
[01:04:59] 
[01:04:59] The actual stderr differed from the expected stderr.
[01:04:59] The actual stderr differed from the expected stderr.
[01:04:59] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/while-let/while-let.stderr
[01:04:59] To update references, rerun the tests and pass the `--bless` flag
[01:04:59] To only update this specific test, also pass `--test-args while-let.rs`
[01:04:59] error: 1 errors occurred comparing output.
[01:04:59] status: exit code: 0
[01:04:59] status: exit code: 0
[01:04:59] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/while-let.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/while-let/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/while-let/auxiliary" "-A" "unused"
[01:04:59] ------------------------------------------
[01:04:59] 
[01:04:59] ------------------------------------------
[01:04:59] stderr:
[01:04:59] stderr:
[01:04:59] ------------------------------------------
[01:04:59] {"message":"irrefutable while-let pattern","code":{"code":"irrefutable_let_patterns","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/while-let.rs","byte_start":103,"byte_end":108,"line_start":6,"line_end":6,"column_start":13,"column_end":18,"is_primary":true,"text":[{"text":"            while let $p = $e $b","highlight_start":13,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/while-let.rs","byte_start":256,"byte_end":348,"line_start":15,"line_end":17,"column_start":5,"column_end":8,"is_primary":false,"text":[{"text":"    foo!(a, 1, { //~ WARN irrefutable while-let","highlight_start":5,"highlight_end":48},{"text":"        println!(\"irrefutable pattern\");","highlight_start":1,"highlight_end":41},{"text":"    });","highlight_start":1,"highlight_end":8}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"foo!","def_site_span":{"file_name":"/checkout/src/test/ui/while-let.rs","byte_start":31,"byte_end":140,"line_start":4,"line_end":8,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    macro_rules! foo{","highlight_start":5,"highlight_end":22},{"text":"        ($p:pat, $e:expr, $b:block) => {{","highlight_start":1,"highlight_end":42},{"text":"            while let $p = $e $b","highlight_start":1,"highlight_end":33},{"text":"        }}","highlight_start":1,"highlight_end":11},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"#[warn(irrefutable_let_patterns)] on by default","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"warning: irrefutable while-let pattern\n  --> /checkout/src/test/ui/while-let.rs:6:13\n   |\nLL |               while let $p = $e $b\n   |               ^^^^^\n...\nLL | /     foo!(a, 1, { //~ WARN irrefutable while-let\nLL | |         println!(\"irrefutable pattern\");\nLL | |     });\n   | |_______- in this macro invocation\n   |\n   = note: #[warn(irrefutable_let_patterns)] on by default\n\n"}
[01:04:59] {"message":"irrefutable while-let pattern","code":{"code":"irrefutable_let_patterns","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/while-let.rs","byte_start":103,"byte_end":108,"line_start":6,"line_end":6,"column_start":13,"column_end":18,"is_primary":true,"text":[{"text":"            while let $p = $e $b","highlight_start":13,"highlight_end":18}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/while-let.rs","byte_start":217,"byte_end":233,"line_start":11,"line_end":11,"column_start":13,"column_end":29,"is_primary":false,"text":[{"text":"            foo!($p, $e, $b)","highlight_start":13,"highlight_end":29}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/while-let.rs","byte_start":353,"byte_end":445,"line_start":18,"line_end":20,"column_start":5,"column_end":8,"is_primary":false,"text":[{"text":"    bar!(a, 1, { //~ WARN irrefutable while-let","highlight_start":5,"highlight_end":48},{"text":"        println!(\"irrefutable pattern\");","highlight_start":1,"highlight_end":41},{"text":"    });","highlight_start":1,"highlight_end":8}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"bar!","def_site_span":{"file_name":"/checkout/src/test/ui/while-let.rs","byte_start":145,"byte_end":250,"line_start":9,"line_end":13,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    macro_rules! bar{","highlight_start":5,"highlight_end":22},{"text":"        ($p:pat, $e:expr, $b:block) => {{","highlight_start":1,"highlight_end":42},{"text":"            foo!($p, $e, $b)","highlight_start":1,"highlight_end":29},{"text":"        }}","highlight_start":1,"highlight_end":11},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"foo!","def_site_span":{"file_name":"/checkout/src/test/ui/while-let.rs","byte_start":31,"byte_end":140,"line_start":4,"line_end":8,"column_start":5,"column_end":6,"is_primary":false,"text":[{"text":"    macro_rules! foo{","highlight_start":5,"highlight_end":22},{"text":"        ($p:pat, $e:expr, $b:block) => {{","highlight_start":1,"highlight_end":42},{"text":"            while let $p = $e $b","highlight_start":1,"highlight_end":33},{"text":"        }}","highlight_start":1,"highlight_end":11},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[],"rendered":"warning: irrefutable while-let pattern\n  --> /checkout/src/test/ui/while-let.rs:6:13\n   |\nLL |               while let $p = $e $b\n   |               ^^^^^\n...\nLL | /     bar!(a, 1, { //~ WARN irrefutable while-let\nLL | |         println!(\"irrefutable pattern\");\nLL | |     });\n   | |_______- in this macro invocation\n\n"}
[01:04:59] {"message":"irrefutable while-let pattern","code":{"code":"irrefutable_let_patterns","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/while-let.rs","byte_start":469,"byte_end":579,"line_start":24,"line_end":27,"column_start":5,"column_end":6,"is_primary":true,"text":[{"text":"    while let a = 1 { //~ WARN irrefutable while-let","highlight_start":5,"highlight_end":53},{"text":"        println!(\"irrefutable pattern\");","highlight_start":1,"highlight_end":41},{"text":"        break;","highlight_start":1,"highlight_end":15},{"text":"    }","highlight_start":1,"highlight_end":6}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"warning: irrefutable while-let pattern\n  --> /checkout/src/test/ui/while-let.rs:24:5\n   |\nLL | /     while let a = 1 { //~ WARN irrefutable while-let\nLL | |         println!(\"irrefutable pattern\");\nLL | |         break;\nLL | |     }\n   | |_____^\n\n"}
[01:04:59] ------------------------------------------
[01:04:59] 
[01:04:59] thread '[ui] ui/while-let.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3245:9
[01:04:59] 
---
[01:04:59] 
[01:04:59] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:495:22
[01:04:59] 
[01:04:59] 
[01:04:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:59] 
[01:04:59] 
[01:04:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:59] Build completed unsuccessfully in 0:04:13
[01:04:59] Build completed unsuccessfully in 0:04:13
[01:04:59] Makefile:48: recipe for target 'check' failed
[01:04:59] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0bd49ead
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jan 12 04:18:23 UTC 2019
