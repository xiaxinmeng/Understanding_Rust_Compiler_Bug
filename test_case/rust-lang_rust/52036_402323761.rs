plain
[01:37:22] 
[01:37:22] ---- builder::__test::build_default stdout ----
[01:37:22] thread 'builder::__test::build_default' panicked at 'assertion failed: `(left == right)`
[01:37:22] 
[01:37:22] Diff < left / right > :
[01:37:22]      Rustc {
[01:37:22]          target: "A",
[01:37:22]          compiler: Compiler {
[01:37:22]              stage: 0,
---
[01:37:22]      },
[01:37:22]      Rustc {
[01:37:22]          target: "B",
[01:37:22]          compiler: Compiler {
[01:37:22] >            stage: 0,
[01:37:22] >            host: "A"
[01:37:22] >        }
[01:37:22] >    },
[01:37:22] >    Rustc {
[01:37:22] >        target: "B",
[01:37:22] >        compiler: Compiler {
[01:37:22]              host: "A"
[01:37:22]          }
[01:37:22]      },
[01:37:22]      Rustc {
---
[01:37:22] 
[01:37:22] ', bootstrap/builder.rs:1604:9
[01:37:22] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:37:22] 
[01:37:22] ---- builder::__test::build_with_target_flag stdout ----
[01:37:22] thread 'builder::__test::build_with_target_flag' panicked at 'assertion failed: `(left == right)`
[01:37:22] 
[01:37:22] Diff < left / right > :
[01:37:22]      Assemble {
[01:37:22]          target_compiler: Compiler {
[01:37:22]              stage: 0,
[01:37:22]              host: "A"
---
[01:37:22] 
[01:37:22] 
[01:37:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:37:22] Build completed unsuccessfully in 1:00:08
[01:37:22] make: *** [check] Error 1
[01:37:22] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17b31f48
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
