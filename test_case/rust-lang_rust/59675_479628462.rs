plain
travis_time:end:04f9a054:start=1554315498485361337,finish=1554315500618617468,duration=2133256131
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_ui
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:11:59] 
[01:11:59] running 5522 tests
[01:12:02] ..........................F......................................................................... 100/5522
[01:12:08] .................................................................................................... 300/5522
[01:12:11] .................................................................................................... 400/5522
[01:12:15] .................................................................................................... 500/5522
[01:12:18] ..........................................i......................................................... 600/5522
[01:12:18] ..........................................i......................................................... 600/5522
[01:12:22] .................................................................................................... 700/5522
[01:12:27] .................................................................................................... 800/5522
[01:12:31] .................................................................................................... 900/5522
[01:12:36] .i...............i.................................................................................. 1000/5522
[01:12:39] ..................................iiiii............................................................. 1100/5522
[01:12:43] .................................................................................................... 1200/5522
[01:12:45] .....................................................F.F............................................ 1300/5522
[01:12:51] .................................................................................................... 1500/5522
[01:12:54] .................................................................................................... 1600/5522
[01:12:57] ..................................................i................................................. 1700/5522
[01:13:00] .................................................................................................... 1800/5522
---
[01:14:32] .................................................................................................... 4300/5522
[01:14:41] .................................................................................................... 4400/5522
[01:14:45] .................................................................................................... 4500/5522
[01:14:48] .................................................................................................... 4600/5522
[01:14:52] ................F................................................................................... 4700/5522
[01:14:58] ........................................................F........................................... 4800/5522
[01:15:05] .................................................................................................... 5000/5522
[01:15:10] .................................................................................................... 5100/5522
[01:15:13] .................................................................................................... 5200/5522
[01:15:16] .................................................................................................... 5300/5522
---
[01:15:23] 1 error: `global_allocator` cannot be used in submodules
[01:15:23] -   --> $DIR/allocator-submodule.rs:27:5
[01:15:23] +   --> $DIR/allocator-submodule.rs:25:5
[01:15:23] 3    |
[01:15:23] 4 LL |     static MY_HEAP: MyAlloc = MyAlloc;
[01:15:23] 
[01:15:23] 
[01:15:23] The actual stderr differed from the expected stderr.
[01:15:23] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator-submodule/allocator-submodule.stderr
[01:15:23] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator-submodule/allocator-submodule.stderr
[01:15:23] To update references, rerun the tests and pass the `--bless` flag
[01:15:23] To only update this specific test, also pass `--test-args allocator-submodule.rs`
[01:15:23] error: 1 errors occurred comparing output.
[01:15:23] status: exit code: 1
[01:15:23] status: exit code: 1
[01:15:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/allocator-submodule.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator-submodule/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/allocator-submodule/auxiliary" "-A" "unused"
[01:15:23] ------------------------------------------
[01:15:23] 
[01:15:23] ------------------------------------------
[01:15:23] stderr:
[01:15:23] stderr:
[01:15:23] ------------------------------------------
[01:15:23] {"message":"`global_allocator` cannot be used in submodules","code":null,"level":"error","spans":[{"file_name":"/checkout/src/test/ui/allocator-submodule.rs","byte_start":461,"byte_end":495,"line_start":25,"line_end":25,"column_start":5,"column_end":39,"is_primary":true,"text":[{"text":"    static MY_HEAP: MyAlloc = MyAlloc; //~ ERROR global_allocator","highlight_start":5,"highlight_end":39}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error: `global_allocator` cannot be used in submodules\n  --> /checkout/src/test/ui/allocator-submodule.rs:25:5\n   |\nLL |     static MY_HEAP: MyAlloc = MyAlloc; //~ ERROR global_allocator\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[01:15:23] 
[01:15:23] ------------------------------------------
[01:15:23] 
[01:15:23] thread '[ui] ui/allocator-submodule.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3422:9
---
[01:15:23] 1 error[E0254]: the name `alloc` is defined multiple times
[01:15:23] -   --> $DIR/E0254.rs:12:5
[01:15:23] +   --> $DIR/E0254.rs:11:5
[01:15:23] 3    |
[01:15:23] 4 LL | extern crate alloc;
[01:15:23] 5    | ------------------- previous import of the extern crate `alloc` here
[01:15:23] 
[01:15:23] The actual stderr differed from the expected stderr.
[01:15:23] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0254/E0254.stderr
[01:15:23] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0254/E0254.stderr
[01:15:23] To update references, rerun the tests and pass the `--bless` flag
[01:15:23] To only update this specific test, also pass `--test-args error-codes/E0254.rs`
[01:15:23] error: 1 errors occurred comparing output.
[01:15:23] status: exit code: 1
[01:15:23] status: exit code: 1
[01:15:23] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0254.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0254/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0254/auxiliary" "-A" "unused"
[01:15:23] ------------------------------------------
[01:15:23] 
[01:15:23] ------------------------------------------
[01:15:23] stderr:
[01:15:23] stderr:
[01:15:23] ------------------------------------------
[01:15:23] {"message":"the name `alloc` is defined multiple times","code":{"code":"E0254","explanation":"\nAttempt was made to import an item whereas an extern crate with this name has\nalready been imported.\n\nErroneous code example:\n\n