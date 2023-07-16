plain
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/03/9d/a0b73320e4b9d776b0b68a67c7dbdc4115eb9eceff992d6b56222ba550d3/awscli-1.15.27-py2.py3-none-any.whl (1.3MB)
    0% |▎                               | 10kB 15.6MB/s eta 0:00:01
    1% |▌                               | 20kB 1.8MB/s eta 0:00:01
    2% |▉                               | 30kB 2.1MB/s eta 0:00:01
    3% |█                               | 40kB 2.0MB/s eta 0:00:01
---
    100% |████████████████████████████████| 552kB 2.2MB/s 
Collecting botocore==1.10.27 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/0a/fc/5dbeb052f4b70346ad3eb1ac291d503c0a28ed7e0f806f7337bbb73e3b76/botocore-1.10.27-py2.py3-none-any.whl (4.2MB)
    0% |                                | 10kB 43.4MB/s eta 0:00:01
    0% |▏                               | 20kB 40.3MB/s eta 0:00:01
    0% |▎                               | 30kB 43.2MB/s eta 0:00:01
    0% |▎                               | 40kB 42.8MB/s eta 0:00:01
---
[01:00:41] ....................................................................................................
[01:00:47] ....................................................................................................
[01:00:54] ..............................................................................i.....................
[01:01:00] .......................................................i............................................
[01:01:06] ........F.F........F.......................................................ii.......................
[01:01:22] .....................................................................................i..............
[01:01:22] .....................................................................................i..............
 concat ! ( $ fmt , \"\\n\" ) , $ ( $ arg ) * ) ) ;","highlight_start":1,"highlight_end":65}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"print!","def_site_span":{"file_name":"<print macros>","byte_start":0,"byte_end":91,"line_start":1,"line_end":2,"column_start":1,"column_end":64,"is_primary":false,"text":[{"text":"( $ ( $ arg : tt ) * ) => (","highlight_start":1,"highlight_end":28},{"text":"$ crate :: io :: _print ( format_args ! ( $ ( $ arg ) * ) ) ) ;","highlight_start":1,"highlight_end":64}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/reachable/expr_loop.rs","byte_start":554,"byte_end":570,"line_start":14,"line_end":14,"column_start":9,"column_end":25,"is_primary":true,"text":[{"text":"#![deny(unreachable_code)]","highlight_start":9,"highlight_end":25}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: unreachable statement\n  --> /checkout/src/test/ui/reachable/expr_loop.rs:18:5\n   |\nLL |     println!(\"I am dead.\");\n   |     ^^^^^^^^^^^^^^^^^^^^^^^\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/reachable/expr_loop.rs:14:9\n   |\nLL | #![deny(unreachable_code)]\n   |         ^^^^^^^^^^^^^^^^\n   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\n\n"}
[01:01:25] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:01:25]   left: `WarnedAlways`,
[01:01:25]  right: `Maybe`', librustc_typeck/check/mod.rs:3956:19
[01:01:25] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:01:25] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:01:25] error: internal compiler error: unexpected panic
[01:01:25] 
[01:01:25] 
[01:01:25] note: the compiler unexpectedly panicked. this is a bug.
[01:01:25] 
[01:01:25] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:01:25] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[01:01:25] 
[01:01:25] 
[01:01:25] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:01:25] 
[01:01:25] ------------------------------------------
[01:01:25] 
[01:01:25] thread '[ui] ui/reachable/expr_loop.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
---
[01:01:25] 20 
[01:01:25] - error: unreachable statement
[01:01:25] -   --> $DIR/expr_match.rs:34:5
[01:01:25] -    |
[01:01:25] - LL |     println!("I am dead");
chable/expr_match.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
[01:01:25] 
[01:01:25] ---- [ui] ui/reachable/expr_while.rs stdout ----
[01:01:25] diff of stderr:
[01:01:25] diff of stderr:
[01:01:25] 
[01:01:25] 11    |         ^^^^^^^^^^^^^^^^
[01:01:25] 12    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:01:25] 13 
[01:01:25] - error: unreachable statement
[01:01:25] -   --> $DIR/expr_while.rs:32:9
[01:01:25] -    |
[01:01:25] - LL |         println!("I am dead.");
[01:01:25] -    |
[01:01:25] -    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:01:25] - 
[01:01:25] - error: unreachable statement
[01:01:25] - error: unreachable statement
[01:01:25] -   --> $DIR/expr_while.rs:34:5
[01:01:25] -    |
[01:01:25] - LL |     println!("I am, too.");
[01:01:25] -    |
[01:01:25] -    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[01:01:25] - 
[01:01:25] - error: aborting due to 3 previous errors
[01:01:25] - error: aborting due to 3 previous errors
[01:01:25] + error: aborting due to previous error
[01:01:25] 31 
[01:01:25] 32 
[01:01:25] 
[01:01:25] 
[01:01:25] The actual stderr differed from the expected stderr.
[01:01:25] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reachable/expr_while/expr_while.stderr
[01:01:25] To update references, rerun the tests and pass the `--bless` fpr , $ ( $ arg : tt ) *","highlight_start":1,"highlight_end":38}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":{"span":{"file_name":"/checkout/src/test/ui/reachable/expr_while.rs","byte_start":614,"byte_end":640,"line_start":18,"line_end":18,"column_start":9,"column_end":35,"is_primary":false,"text":[{"text":"        println!(\"Hello, world!\");","highlight_start":9,"highlight_end":35}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null},"macro_decl_name":"println!","def_site_span":{"file_name":"<println macros>","byte_start":0,"byte_end":195,"line_start":1,"line_end":3,"column_start":1,"column_end":65,"is_primary":false,"text":[{"text":"(  ) => ( print ! ( \"\\n\" ) ) ; ( $ fmt : expr ) => (","highlight_start":1,"highlight_end":53},{"text":"print ! ( concat ! ( $ fmt , \"\\n\" ) ) ) ; ( $ fmt : expr , $ ( $ arg : tt ) *","highlight_start":1,"highlight_end":78},{"text":") => ( print ! ( concat ! ( $ fmt , \"\\n\" ) , $ ( $ arg ) * ) ) ;","highlight_start":1,"highlight_end":65}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"print!","def_site_span":{"file_name":"<print macros>","byte_start":0,"byte_end":91,"line_start":1,"line_end":2,"column_start":1,"column_end":64,"is_primary":false,"text":[{"text":"( $ ( $ arg : tt ) * ) => (","highlight_start":1,"highlight_end":28},{"text":"$ crate :: io :: _print ( format_args ! ( $ ( $ arg ) * ) ) ) ;","highlight_start":1,"highlight_end":64}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}}],"childrG.md#bug-reports
[01:01:25] note: rustc 1.28.0-dev running on x86_64-unknown-linux-gnu
[01:01:25] 
[01:01:25] 
[01:01:25] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[01:01:25] 
[01:01:25] ------------------------------------------
[01:01:25] 
[01:01:25] thread '[ui] ui/reachable/expr_while.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3053:9
---
[01:01:25] test result: FAILED. 1444 passed; 3 failed; 16 ignored; 0 measured; 0 filtered out
[01:01:25] 
[01:01:25] 
[01:01:25] 
[01:01:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-3.9/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" 3847004 .
2574896 ./obj/build
1817616 ./obj/build/x86_64-unknown-linux-gnu
727032 ./src
567104 ./obj/build/bootstrap
---
148912 ./.git/modules/src
140460 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
140456 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
122540 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj
122536 ./obj/build/bootstrap/debug/incremental/bootstrap-1r3bppl29tbrj/s-f1clrrlcn2-182qe1x-33jenrrxh1gwm
106792 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release
104168travis_time:start:0162aedc
$ dmesg | grep -i kill
[   10.501810] init: failsafe main process (1094) killed by TERM signal
