plain
2019-07-18T15:33:05.6405459Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-18T15:33:05.6625694Z ##[command]git config gc.auto 0
2019-07-18T15:33:05.6686142Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-18T15:33:05.6746949Z ##[command]git config --get-all http.proxy
2019-07-18T15:33:05.6906358Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62783/merge:refs/remotes/pull/62783/merge
---
2019-07-18T15:33:40.2320474Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-18T15:33:40.2320534Z 
2019-07-18T15:33:40.2320747Z   git checkout -b <new-branch-name>
2019-07-18T15:33:40.2320779Z 
2019-07-18T15:33:40.2320829Z HEAD is now at 880612e07 Merge 39cf14170319a4f53dfc7db84a1bb9d79e02f14f into 21d5b8bf0c26e3ee4c270ce5527df66b1af56513
2019-07-18T15:33:40.2502992Z ##[section]Finishing: Checkout
2019-07-18T15:33:40.2510391Z ##[section]Starting: Decide whether to run this job
2019-07-18T15:33:40.2514274Z Task         : Bash
2019-07-18T15:33:40.2514319Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-18T15:33:40.2514361Z Version      : 3.151.2
2019-07-18T15:33:40.2514419Z Author       : Microsoft Corporation
2019-07-18T15:33:40.2514419Z Author       : Microsoft Corporation
2019-07-18T15:33:40.2514465Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-18T15:33:40.2514532Z ==============================================================================
2019-07-18T15:33:40.3932398Z Generating script.
2019-07-18T15:33:40.3973824Z ========================== Starting Command Output ===========================
2019-07-18T15:33:40.4010918Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/21dc8552-4e38-47df-83fe-3d7ad1099a63.sh
2019-07-18T15:33:40.5229765Z Executing the job since submodules are updated
2019-07-18T15:33:40.5320313Z ##[section]Finishing: Decide whether to run this job
2019-07-18T15:33:40.5330193Z ==============================================================================
2019-07-18T15:33:40.5330302Z Task         : Bash
2019-07-18T15:33:40.5330356Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-18T15:33:40.5330409Z Version      : 3.151.2
---
2019-07-18T18:04:42.3535437Z -args
2019-07-18T18:04:42.3536251Z -
2019-07-18T18:04:42.3536290Z 
2019-07-18T18:04:42.3536344Z The actual stdout differed from the expected stdout.
2019-07-18T18:04:42.3536419Z Actual stdout saved to /tmp/compiletestWXeOcX/args.stdout
2019-07-18T18:04:42.3536535Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-18T18:04:42.3536832Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-18T18:04:42.3536891Z      |
2019-07-18T18:04:42.3536957Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-18T18:04:42.3541359Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:42.3541405Z +
2019-07-18T18:04:42.3541430Z 
2019-07-18T18:04:42.3541473Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:42.3541539Z Actual stderr saved to /tmp/compiletestWXeOcX/args.stderr
2019-07-18T18:04:42.3541586Z To update references, run this command from build directory:
2019-07-18T18:04:42.3541809Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'args.rs'
2019-07-18T18:04:42.3542337Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-18T18:04:42.3542368Z 
2019-07-18T18:04:42.3542427Z error: 2 errors occurred comparing output.
2019-07-18T18:04:42.3542470Z status: exit code: 1
2019-07-18T18:04:42.3542470Z status: exit code: 1
2019-07-18T18:04:42.3543035Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/args.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/args.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/args.stage-id.aux" "-A" "unused"
2019-07-18T18:04:42.3543329Z ------------------------------------------
2019-07-18T18:04:42.3543370Z 
2019-07-18T18:04:42.3543662Z ------------------------------------------
2019-07-18T18:04:42.3543715Z stderr:
---
2019-07-18T18:04:43.3235236Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:43.3235737Z +
2019-07-18T18:04:43.3235777Z 
2019-07-18T18:04:43.3235827Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:43.3235898Z Actual stderr saved to /tmp/compiletestWXeOcX/arrays.stderr
2019-07-18T18:04:43.3235963Z To update references, run this command from build directory:
2019-07-18T18:04:43.3236276Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'arrays.rs'
2019-07-18T18:04:43.3236380Z error: 1 errors occurred comparing output.
2019-07-18T18:04:43.3236428Z status: exit code: 1
2019-07-18T18:04:43.3236428Z status: exit code: 1
2019-07-18T18:04:43.3237075Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/arrays.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/arrays.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/arrays.stage-id.aux" "-A" "unused"
2019-07-18T18:04:43.3237380Z ------------------------------------------
2019-07-18T18:04:43.3237432Z 
2019-07-18T18:04:43.3237651Z ------------------------------------------
2019-07-18T18:04:43.3237709Z stderr:
---
2019-07-18T18:04:43.3251382Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:43.3251431Z +
2019-07-18T18:04:43.3251458Z 
2019-07-18T18:04:43.3251520Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:43.3251912Z Actual stderr saved to /tmp/compiletestWXeOcX/associated-const.stderr
2019-07-18T18:04:43.3251981Z To update references, run this command from build directory:
2019-07-18T18:04:43.3252242Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'associated-const.rs'
2019-07-18T18:04:43.3252322Z error: 1 errors occurred comparing output.
2019-07-18T18:04:43.3252366Z status: exit code: 1
2019-07-18T18:04:43.3252366Z status: exit code: 1
2019-07-18T18:04:43.3252990Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/associated-const.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/associated-const.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/associated-const.stage-id.aux" "-A" "unused"
2019-07-18T18:04:43.3253283Z ------------------------------------------
2019-07-18T18:04:43.3253500Z 
2019-07-18T18:04:43.3253711Z ------------------------------------------
2019-07-18T18:04:43.3253841Z stderr:
---
2019-07-18T18:04:43.3267598Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:43.3267669Z +
2019-07-18T18:04:43.3267699Z 
2019-07-18T18:04:43.3267759Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:43.3267840Z Actual stderr saved to /tmp/compiletestWXeOcX/assume_bug.stderr
2019-07-18T18:04:43.3267897Z To update references, run this command from build directory:
2019-07-18T18:04:43.3268175Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'assume_bug.rs'
2019-07-18T18:04:43.3268283Z error: 1 errors occurred comparing output.
2019-07-18T18:04:43.3268333Z status: exit code: 1
2019-07-18T18:04:43.3268333Z status: exit code: 1
2019-07-18T18:04:43.3269342Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/assume_bug.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/assume_bug.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/assume_bug.stage-id.aux" "-A" "unused"
2019-07-18T18:04:43.3321278Z ------------------------------------------
2019-07-18T18:04:43.3321360Z 
2019-07-18T18:04:43.3321743Z ------------------------------------------
2019-07-18T18:04:43.3321808Z stderr:
---
2019-07-18T18:04:43.3335912Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:43.3335971Z +
2019-07-18T18:04:43.3336016Z 
2019-07-18T18:04:43.3336066Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:43.3336343Z Actual stderr saved to /tmp/compiletestWXeOcX/async-fn.stderr
2019-07-18T18:04:43.3336400Z To update references, run this command from build directory:
2019-07-18T18:04:43.3336665Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'async-fn.rs'
2019-07-18T18:04:43.3336748Z error: 1 errors occurred comparing output.
2019-07-18T18:04:43.3336811Z status: exit code: 1
2019-07-18T18:04:43.3336811Z status: exit code: 1
2019-07-18T18:04:43.3337533Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/async-fn.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/async-fn.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/async-fn.stage-id.aux" "-A" "unused"
2019-07-18T18:04:43.3337895Z ------------------------------------------
2019-07-18T18:04:43.3337930Z 
2019-07-18T18:04:43.3338145Z ------------------------------------------
2019-07-18T18:04:43.3338212Z stderr:
---
2019-07-18T18:04:43.3350901Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:43.3350958Z +
2019-07-18T18:04:43.3350991Z 
2019-07-18T18:04:43.3351051Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:43.3351288Z Actual stderr saved to /tmp/compiletestWXeOcX/atomic-access-bool.stderr
2019-07-18T18:04:43.3351341Z To update references, run this command from build directory:
2019-07-18T18:04:43.3351595Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'atomic-access-bool.rs'
2019-07-18T18:04:43.3351672Z error: 1 errors occurred comparing output.
2019-07-18T18:04:43.3351718Z status: exit code: 1
2019-07-18T18:04:43.3351718Z status: exit code: 1
2019-07-18T18:04:43.3352587Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/atomic-access-bool.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/atomic-access-bool.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/atomic-access-bool.stage-id.aux" "-A" "unused"
2019-07-18T18:04:43.3352923Z ------------------------------------------
2019-07-18T18:04:43.3352955Z 
2019-07-18T18:04:43.3353326Z ------------------------------------------
2019-07-18T18:04:43.3353386Z stderr:
---
2019-07-18T18:04:43.3432239Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:43.3432288Z +
2019-07-18T18:04:43.3432334Z 
2019-07-18T18:04:43.3432381Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:43.3432640Z Actual stderr saved to /tmp/compiletestWXeOcX/atomic-compare_exchange.stderr
2019-07-18T18:04:43.3432711Z To update references, run this command from build directory:
2019-07-18T18:04:43.3432967Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'atomic-compare_exchange.rs'
2019-07-18T18:04:43.3433048Z error: 1 errors occurred comparing output.
2019-07-18T18:04:43.3433108Z status: exit code: 1
2019-07-18T18:04:43.3433108Z status: exit code: 1
2019-07-18T18:04:43.3433890Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/atomic-compare_exchange.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/atomic-compare_exchange.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/atomic-compare_exchange.stage-id.aux" "-A" "unused"
2019-07-18T18:04:43.3434260Z ------------------------------------------
2019-07-18T18:04:43.3434295Z 
2019-07-18T18:04:43.3434527Z ------------------------------------------
2019-07-18T18:04:43.3434572Z stderr:
---
2019-07-18T18:04:43.3448789Z +
2019-07-18T18:04:43.3448819Z 
2019-07-18T18:04:43.3449308Z thread '[ui] run-pass/bad_substs.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-18T18:04:43.3449544Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:43.3449590Z Actual stderr saved to /tmp/compiletestWXeOcX/bad_substs.stderr
2019-07-18T18:04:43.3449654Z To update references, run this command from build directory:
2019-07-18T18:04:43.3449876Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'bad_substs.rs'
2019-07-18T18:04:43.3449958Z error: 1 errors occurred comparing output.
2019-07-18T18:04:43.3450017Z status: exit code: 1
2019-07-18T18:04:43.3450017Z status: exit code: 1
2019-07-18T18:04:43.3450632Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/bad_substs.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/bad_substs.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/bad_substs.stage-id.aux" "-A" "unused"
2019-07-18T18:04:43.3450942Z ------------------------------------------
2019-07-18T18:04:43.3450975Z 
2019-07-18T18:04:43.3451161Z ------------------------------------------
2019-07-18T18:04:43.3451220Z stderr:
---
2019-07-18T18:04:43.3463352Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:43.3463399Z +
2019-07-18T18:04:43.3463425Z 
2019-07-18T18:04:43.3463486Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:43.3463534Z Actual stderr saved to /tmp/compiletestWXeOcX/bools.stderr
2019-07-18T18:04:43.3463584Z To update references, run this command from build directory:
2019-07-18T18:04:43.3463842Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'bools.rs'
2019-07-18T18:04:43.3463920Z error: 1 errors occurred comparing output.
2019-07-18T18:04:43.3463981Z status: exit code: 1
2019-07-18T18:04:43.3463981Z status: exit code: 1
2019-07-18T18:04:43.3464625Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/bools.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/bools.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/bools.stage-id.aux" "-A" "unused"
2019-07-18T18:04:43.3464959Z ------------------------------------------
2019-07-18T18:04:43.3464993Z 
2019-07-18T18:04:43.3465200Z ------------------------------------------
2019-07-18T18:04:43.3465263Z stderr:
---
2019-07-18T18:04:43.3479359Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:43.3479404Z +
2019-07-18T18:04:43.3479429Z 
2019-07-18T18:04:43.3479488Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:43.3479536Z Actual stderr saved to /tmp/compiletestWXeOcX/binops.stderr
2019-07-18T18:04:43.3479584Z To update references, run this command from build directory:
2019-07-18T18:04:43.3479830Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'binops.rs'
2019-07-18T18:04:43.3479996Z error: 1 errors occurred comparing output.
2019-07-18T18:04:43.3480046Z status: exit code: 1
2019-07-18T18:04:43.3480046Z status: exit code: 1
2019-07-18T18:04:43.3480644Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/binops.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/binops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/binops.stage-id.aux" "-A" "unused"
2019-07-18T18:04:43.3480932Z ------------------------------------------
2019-07-18T18:04:43.3480963Z 
2019-07-18T18:04:43.3481156Z ------------------------------------------
2019-07-18T18:04:43.3481215Z stderr:
---
2019-07-18T18:04:43.3490872Z -foo #1 = Foo(1337)
2019-07-18T18:04:43.3491035Z -
2019-07-18T18:04:43.3491062Z 
2019-07-18T18:04:43.3491104Z The actual stdout differed from the expected stdout.
2019-07-18T18:04:43.3491342Z Actual stdout saved to /tmp/compiletestWXeOcX/box-pair-to-vec.stdout
2019-07-18T18:04:43.3491449Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-18T18:04:43.3491680Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-18T18:04:43.3491726Z      |
2019-07-18T18:04:43.3491782Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-18T18:04:43.3498118Z +
2019-07-18T18:04:43.3498456Z thread '[ui] run-pass/box_box_trait.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-18T18:04:43.3498499Z 
2019-07-18T18:04:43.3498562Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:43.3498615Z Actual stderr saved to /tmp/compiletestWXeOcX/box_box_trait.stderr
2019-07-18T18:04:43.3498669Z To update references, run this command from build directory:
2019-07-18T18:04:43.3499098Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'box_box_trait.rs'
2019-07-18T18:04:43.3499172Z error: 1 errors occurred comparing output.
2019-07-18T18:04:43.3499224Z status: exit code: 1
2019-07-18T18:04:43.3499224Z status: exit code: 1
2019-07-18T18:04:43.3499897Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/box_box_trait.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/box_box_trait.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/box_box_trait.stage-id.aux" "-A" "unused"
2019-07-18T18:04:43.3500219Z ------------------------------------------
2019-07-18T18:04:43.3500251Z 
2019-07-18T18:04:43.3500444Z ------------------------------------------
2019-07-18T18:04:43.3500506Z stderr:
---
2019-07-18T18:04:43.3591436Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:43.3591487Z +
2019-07-18T18:04:43.3591533Z 
2019-07-18T18:04:43.3591579Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:43.3591835Z Actual stderr saved to /tmp/compiletestWXeOcX/box-pair-to-vec.stderr
2019-07-18T18:04:43.3591904Z To update references, run this command from build directory:
2019-07-18T18:04:43.3592450Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'box-pair-to-vec.rs'
2019-07-18T18:04:43.3592538Z error: 2 errors occurred comparing output.
2019-07-18T18:04:43.3592596Z status: exit code: 1
2019-07-18T18:04:43.3592596Z status: exit code: 1
2019-07-18T18:04:43.3593165Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/box-pair-to-vec.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/box-pair-to-vec.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/box-pair-to-vec.stage-id.aux" "-A" "unused"
2019-07-18T18:04:43.3593448Z ------------------------------------------
2019-07-18T18:04:43.3593479Z 
2019-07-18T18:04:43.3593684Z ------------------------------------------
2019-07-18T18:04:43.3593726Z stderr:
---
2019-07-18T18:04:43.3974494Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:43.3974542Z +
2019-07-18T18:04:43.3974587Z 
2019-07-18T18:04:43.3974633Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:43.3974683Z Actual stderr saved to /tmp/compiletestWXeOcX/c_enums.stderr
2019-07-18T18:04:43.3974836Z To update references, run this command from build directory:
2019-07-18T18:04:43.3975127Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'c_enums.rs'
2019-07-18T18:04:43.3975209Z error: 1 errors occurred comparing output.
2019-07-18T18:04:43.3975271Z status: exit code: 1
2019-07-18T18:04:43.3975271Z status: exit code: 1
2019-07-18T18:04:43.3976457Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/c_enums.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/c_enums.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/c_enums.stage-id.aux" "-A" "unused"
2019-07-18T18:04:43.3976791Z ------------------------------------------
2019-07-18T18:04:43.3976829Z 
2019-07-18T18:04:43.3977065Z ------------------------------------------
2019-07-18T18:04:43.3977126Z stderr:
---
2019-07-18T18:04:43.4532252Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:43.4532313Z +
2019-07-18T18:04:43.4532337Z 
2019-07-18T18:04:43.4532376Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:43.4532422Z Actual stderr saved to /tmp/compiletestWXeOcX/btreemap.stderr
2019-07-18T18:04:43.4532484Z To update references, run this command from build directory:
2019-07-18T18:04:43.4532698Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'btreemap.rs'
2019-07-18T18:04:43.4532861Z error: 1 errors occurred comparing output.
2019-07-18T18:04:43.4532903Z status: exit code: 1
2019-07-18T18:04:43.4532903Z status: exit code: 1
2019-07-18T18:04:43.4533460Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/btreemap.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/btreemap.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/btreemap.stage-id.aux" "-A" "unused"
2019-07-18T18:04:43.4533737Z ------------------------------------------
2019-07-18T18:04:43.4533767Z 
2019-07-18T18:04:43.4533967Z ------------------------------------------
2019-07-18T18:04:43.4534007Z stderr:
---
2019-07-18T18:04:43.5734428Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:43.5734479Z +
2019-07-18T18:04:43.5734508Z 
2019-07-18T18:04:43.5734554Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:43.5734622Z Actual stderr saved to /tmp/compiletestWXeOcX/call_drop_on_array_elements.stderr
2019-07-18T18:04:43.5734675Z To update references, run this command from build directory:
2019-07-18T18:04:43.5735106Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'call_drop_on_array_elements.rs'
2019-07-18T18:04:43.5735207Z error: 1 errors occurred comparing output.
2019-07-18T18:04:43.5735252Z status: exit code: 1
2019-07-18T18:04:43.5735252Z status: exit code: 1
2019-07-18T18:04:43.5736464Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_array_elements.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/call_drop_on_array_elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/call_drop_on_array_elements.stage-id.aux" "-A" "unused"
2019-07-18T18:04:43.5736796Z ------------------------------------------
2019-07-18T18:04:43.5736851Z 
2019-07-18T18:04:43.5737088Z ------------------------------------------
2019-07-18T18:04:43.5737137Z stderr:
---
2019-07-18T18:04:43.6051291Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:43.6051355Z +
2019-07-18T18:04:43.6051384Z 
2019-07-18T18:04:43.6051430Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:43.6051572Z Actual stderr saved to /tmp/compiletestWXeOcX/call_drop_on_fat_ptr_array_elements.stderr
2019-07-18T18:04:43.6051643Z To update references, run this command from build directory:
2019-07-18T18:04:43.6051950Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'call_drop_on_fat_ptr_array_elements.rs'
2019-07-18T18:04:43.6052051Z error: 1 errors occurred comparing output.
2019-07-18T18:04:43.6052098Z status: exit code: 1
2019-07-18T18:04:43.6052098Z status: exit code: 1
2019-07-18T18:04:43.6052768Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_fat_ptr_array_elements.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/call_drop_on_fat_ptr_array_elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/call_drop_on_fat_ptr_array_elements.stage-id.aux" "-A" "unused"
2019-07-18T18:04:43.6053113Z ------------------------------------------
2019-07-18T18:04:43.6053150Z 
2019-07-18T18:04:43.6053531Z ------------------------------------------
2019-07-18T18:04:43.6053575Z stderr:
---
2019-07-18T18:04:43.7416767Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:43.7416833Z +
2019-07-18T18:04:43.7416864Z 
2019-07-18T18:04:43.7416932Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:43.7416989Z Actual stderr saved to /tmp/compiletestWXeOcX/call_drop_on_zst_array_elements.stderr
2019-07-18T18:04:43.7417045Z To update references, run this command from build directory:
2019-07-18T18:04:43.7417367Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'call_drop_on_zst_array_elements.rs'
2019-07-18T18:04:43.7417456Z error: 1 errors occurred comparing output.
2019-07-18T18:04:43.7417504Z status: exit code: 1
2019-07-18T18:04:43.7417504Z status: exit code: 1
2019-07-18T18:04:43.7418231Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_zst_array_elements.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/call_drop_on_zst_array_elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/call_drop_on_zst_array_elements.stage-id.aux" "-A" "unused"
2019-07-18T18:04:43.7418575Z ------------------------------------------
2019-07-18T18:04:43.7418613Z 
2019-07-18T18:04:43.7418830Z ------------------------------------------
2019-07-18T18:04:43.7418895Z stderr:
---
2019-07-18T18:04:43.7560254Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:43.7560302Z +
2019-07-18T18:04:43.7560328Z 
2019-07-18T18:04:43.7560390Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:43.7560442Z Actual stderr saved to /tmp/compiletestWXeOcX/call_drop_through_owned_slice.stderr
2019-07-18T18:04:43.7560492Z To update references, run this command from build directory:
2019-07-18T18:04:43.7560767Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'call_drop_through_owned_slice.rs'
2019-07-18T18:04:43.7560845Z error: 1 errors occurred comparing output.
2019-07-18T18:04:43.7560888Z status: exit code: 1
2019-07-18T18:04:43.7560888Z status: exit code: 1
2019-07-18T18:04:43.7561536Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_owned_slice.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/call_drop_through_owned_slice.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/call_drop_through_owned_slice.stage-id.aux" "-A" "unused"
2019-07-18T18:04:43.7561848Z ------------------------------------------
2019-07-18T18:04:43.7561880Z 
2019-07-18T18:04:43.7562073Z ------------------------------------------
2019-07-18T18:04:43.7562134Z stderr:
---
2019-07-18T18:04:43.8801935Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:43.8801984Z +
2019-07-18T18:04:43.8802011Z 
2019-07-18T18:04:43.8802073Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:43.8802127Z Actual stderr saved to /tmp/compiletestWXeOcX/call_drop_through_trait_object.stderr
2019-07-18T18:04:43.8802180Z To update references, run this command from build directory:
2019-07-18T18:04:43.8802465Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'call_drop_through_trait_object.rs'
2019-07-18T18:04:43.8802566Z error: 1 errors occurred comparing output.
2019-07-18T18:04:43.8802615Z status: exit code: 1
2019-07-18T18:04:43.8802615Z status: exit code: 1
2019-07-18T18:04:43.8803312Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_trait_object.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/call_drop_through_trait_object.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/call_drop_through_trait_object.stage-id.aux" "-A" "unused"
2019-07-18T18:04:43.8803630Z ------------------------------------------
2019-07-18T18:04:43.8803665Z 
2019-07-18T18:04:43.8803871Z ------------------------------------------
2019-07-18T18:04:43.8803933Z stderr:
---
2019-07-18T18:04:43.8900005Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:43.8900048Z +
2019-07-18T18:04:43.8900072Z 
2019-07-18T18:04:43.8900114Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:43.8900180Z Actual stderr saved to /tmp/compiletestWXeOcX/call_drop_through_trait_object_rc.stderr
2019-07-18T18:04:43.8900243Z To update references, run this command from build directory:
2019-07-18T18:04:43.8900488Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'call_drop_through_trait_object_rc.rs'
2019-07-18T18:04:43.8900588Z error: 1 errors occurred comparing output.
2019-07-18T18:04:43.8900630Z status: exit code: 1
2019-07-18T18:04:43.8900630Z status: exit code: 1
2019-07-18T18:04:43.8901266Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_trait_object_rc.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/call_drop_through_trait_object_rc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/call_drop_through_trait_object_rc.stage-id.aux" "-A" "unused"
2019-07-18T18:04:43.8901561Z ------------------------------------------
2019-07-18T18:04:43.8901592Z 
2019-07-18T18:04:43.8901785Z ------------------------------------------
2019-07-18T18:04:43.8901829Z stderr:
---
2019-07-18T18:04:44.0486916Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:44.0486970Z +
2019-07-18T18:04:44.0487019Z 
2019-07-18T18:04:44.0487069Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:44.0487125Z Actual stderr saved to /tmp/compiletestWXeOcX/calloc.stderr
2019-07-18T18:04:44.0487181Z To update references, run this command from build directory:
2019-07-18T18:04:44.0487480Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'calloc.rs'
2019-07-18T18:04:44.0487566Z error: 1 errors occurred comparing output.
2019-07-18T18:04:44.0487634Z status: exit code: 1
2019-07-18T18:04:44.0487634Z status: exit code: 1
2019-07-18T18:04:44.0488283Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/calloc.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/calloc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/calloc.stage-id.aux" "-A" "unused"
2019-07-18T18:04:44.0488636Z ------------------------------------------
2019-07-18T18:04:44.0488673Z 
2019-07-18T18:04:44.0488905Z ------------------------------------------
2019-07-18T18:04:44.0488974Z stderr:
---
2019-07-18T18:04:44.0814642Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:44.0814703Z +
2019-07-18T18:04:44.0814732Z 
2019-07-18T18:04:44.0814804Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:44.0814855Z Actual stderr saved to /tmp/compiletestWXeOcX/calls.stderr
2019-07-18T18:04:44.0814905Z To update references, run this command from build directory:
2019-07-18T18:04:44.0815193Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'calls.rs'
2019-07-18T18:04:44.0815274Z error: 1 errors occurred comparing output.
2019-07-18T18:04:44.0815318Z status: exit code: 1
2019-07-18T18:04:44.0815318Z status: exit code: 1
2019-07-18T18:04:44.0816368Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/calls.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/calls.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/calls.stage-id.aux" "-A" "unused"
2019-07-18T18:04:44.0816736Z ------------------------------------------
2019-07-18T18:04:44.0816774Z 
2019-07-18T18:04:44.0816993Z ------------------------------------------
2019-07-18T18:04:44.0817057Z stderr:
---
2019-07-18T18:04:44.2177204Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:44.2177265Z +
2019-07-18T18:04:44.2177297Z 
2019-07-18T18:04:44.2177346Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:44.2177422Z Actual stderr saved to /tmp/compiletestWXeOcX/cast_fn_ptr.stderr
2019-07-18T18:04:44.2177478Z To update references, run this command from build directory:
2019-07-18T18:04:44.2177740Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'cast_fn_ptr.rs'
2019-07-18T18:04:44.2177843Z error: 1 errors occurred comparing output.
2019-07-18T18:04:44.2177892Z status: exit code: 1
2019-07-18T18:04:44.2177892Z status: exit code: 1
2019-07-18T18:04:44.2178554Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast_fn_ptr.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/cast_fn_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/cast_fn_ptr.stage-id.aux" "-A" "unused"
2019-07-18T18:04:44.2178889Z ------------------------------------------
2019-07-18T18:04:44.2178924Z 
2019-07-18T18:04:44.2179140Z ------------------------------------------
2019-07-18T18:04:44.2179188Z stderr:
---
2019-07-18T18:04:44.2247687Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:44.2247740Z +
2019-07-18T18:04:44.2247787Z 
2019-07-18T18:04:44.2247836Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:44.2248122Z Actual stderr saved to /tmp/compiletestWXeOcX/cast-rfc0401-vtable-kinds.stderr
2019-07-18T18:04:44.2248199Z To update references, run this command from build directory:
2019-07-18T18:04:44.2248489Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'cast-rfc0401-vtable-kinds.rs'
2019-07-18T18:04:44.2248575Z error: 1 errors occurred comparing output.
2019-07-18T18:04:44.2248642Z status: exit code: 1
2019-07-18T18:04:44.2248642Z status: exit code: 1
2019-07-18T18:04:44.2249495Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast-rfc0401-vtable-kinds.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/cast-rfc0401-vtable-kinds.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/cast-rfc0401-vtable-kinds.stage-id.aux" "-A" "unused"
2019-07-18T18:04:44.2249821Z ------------------------------------------
2019-07-18T18:04:44.2249854Z 
2019-07-18T18:04:44.2250069Z ------------------------------------------
2019-07-18T18:04:44.2250115Z stderr:
---
2019-07-18T18:04:44.3536774Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:44.3536842Z +
2019-07-18T18:04:44.3536874Z 
2019-07-18T18:04:44.3536922Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:44.3536977Z Actual stderr saved to /tmp/compiletestWXeOcX/cast_fn_ptr_unsafe.stderr
2019-07-18T18:04:44.3537074Z To update references, run this command from build directory:
2019-07-18T18:04:44.3537340Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'cast_fn_ptr_unsafe.rs'
2019-07-18T18:04:44.3537442Z error: 1 errors occurred comparing output.
2019-07-18T18:04:44.3537490Z status: exit code: 1
2019-07-18T18:04:44.3537490Z status: exit code: 1
2019-07-18T18:04:44.3538152Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast_fn_ptr_unsafe.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/cast_fn_ptr_unsafe.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/cast_fn_ptr_unsafe.stage-id.aux" "-A" "unused"
2019-07-18T18:04:44.3538484Z ------------------------------------------
2019-07-18T18:04:44.3538520Z 
2019-07-18T18:04:44.3538751Z ------------------------------------------
2019-07-18T18:04:44.3538800Z stderr:
---
2019-07-18T18:04:44.3647355Z -1
2019-07-18T18:04:44.3647545Z -
2019-07-18T18:04:44.3647576Z 
2019-07-18T18:04:44.3647645Z The actual stdout differed from the expected stdout.
2019-07-18T18:04:44.3647701Z Actual stdout saved to /tmp/compiletestWXeOcX/catch.stdout
2019-07-18T18:04:44.3647869Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-18T18:04:44.3648138Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-18T18:04:44.3648191Z      |
2019-07-18T18:04:44.3648262Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-18T18:04:44.3653071Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:44.3653152Z +
2019-07-18T18:04:44.3653182Z 
2019-07-18T18:04:44.3653240Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:44.3653479Z Actual stderr saved to /tmp/compiletestWXeOcX/catch.stderr
2019-07-18T18:04:44.3653726Z To update references, run this command from build directory:
2019-07-18T18:04:44.3653969Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'catch.rs'
2019-07-18T18:04:44.3654067Z error: 2 errors occurred comparing output.
2019-07-18T18:04:44.3654112Z status: exit code: 1
2019-07-18T18:04:44.3654112Z status: exit code: 1
2019-07-18T18:04:44.3654689Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/catch.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/catch.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/catch.stage-id.aux" "-A" "unused"
2019-07-18T18:04:44.3655103Z ------------------------------------------
2019-07-18T18:04:44.3655136Z 
2019-07-18T18:04:44.3655357Z ------------------------------------------
2019-07-18T18:04:44.3655579Z stderr:
---
2019-07-18T18:04:44.4972750Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:44.4972800Z +
2019-07-18T18:04:44.4972830Z 
2019-07-18T18:04:44.4972878Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:44.4973154Z Actual stderr saved to /tmp/compiletestWXeOcX/closure-drop.stderr
2019-07-18T18:04:44.4973211Z To update references, run this command from build directory:
2019-07-18T18:04:44.4973477Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'closure-drop.rs'
2019-07-18T18:04:44.4973580Z error: 1 errors occurred comparing output.
2019-07-18T18:04:44.4973628Z status: exit code: 1
2019-07-18T18:04:44.4973628Z status: exit code: 1
2019-07-18T18:04:44.4974338Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closure-drop.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/closure-drop.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/closure-drop.stage-id.aux" "-A" "unused"
2019-07-18T18:04:44.4974714Z ------------------------------------------
2019-07-18T18:04:44.4974769Z 
2019-07-18T18:04:44.4974997Z ------------------------------------------
2019-07-18T18:04:44.4975046Z stderr:
---
2019-07-18T18:04:44.5254596Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:44.5254645Z +
2019-07-18T18:04:44.5254691Z 
2019-07-18T18:04:44.5254736Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:44.5254785Z Actual stderr saved to /tmp/compiletestWXeOcX/char.stderr
2019-07-18T18:04:44.5254853Z To update references, run this command from build directory:
2019-07-18T18:04:44.5255109Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'char.rs'
2019-07-18T18:04:44.5255187Z error: 1 errors occurred comparing output.
2019-07-18T18:04:44.5255249Z status: exit code: 1
2019-07-18T18:04:44.5255249Z status: exit code: 1
2019-07-18T18:04:44.5256594Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/char.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/char.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/char.stage-id.aux" "-A" "unused"
2019-07-18T18:04:44.5256974Z ------------------------------------------
2019-07-18T18:04:44.5257012Z 
2019-07-18T18:04:44.5257248Z ------------------------------------------
2019-07-18T18:04:44.5257297Z stderr:
---
2019-07-18T18:04:44.6731680Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:44.6731727Z +
2019-07-18T18:04:44.6731756Z 
2019-07-18T18:04:44.6731818Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:44.6732063Z Actual stderr saved to /tmp/compiletestWXeOcX/closure-field-ty.stderr
2019-07-18T18:04:44.6732119Z To update references, run this command from build directory:
2019-07-18T18:04:44.6732390Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'closure-field-ty.rs'
2019-07-18T18:04:44.6732471Z error: 1 errors occurred comparing output.
2019-07-18T18:04:44.6732535Z status: exit code: 1
2019-07-18T18:04:44.6732535Z status: exit code: 1
2019-07-18T18:04:44.6733224Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closure-field-ty.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/closure-field-ty.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/closure-field-ty.stage-id.aux" "-A" "unused"
2019-07-18T18:04:44.6733694Z ------------------------------------------
2019-07-18T18:04:44.6733730Z 
2019-07-18T18:04:44.6733947Z ------------------------------------------
2019-07-18T18:04:44.6734010Z stderr:
---
2019-07-18T18:04:44.7694507Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:44.7694558Z +
2019-07-18T18:04:44.7694587Z 
2019-07-18T18:04:44.7694653Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:44.7694702Z Actual stderr saved to /tmp/compiletestWXeOcX/closures.stderr
2019-07-18T18:04:44.7694752Z To update references, run this command from build directory:
2019-07-18T18:04:44.7695031Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'closures.rs'
2019-07-18T18:04:44.7695111Z error: 1 errors occurred comparing output.
2019-07-18T18:04:44.7695155Z status: exit code: 1
2019-07-18T18:04:44.7695155Z status: exit code: 1
2019-07-18T18:04:44.7696526Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closures.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/closures.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/closures.stage-id.aux" "-A" "unused"
2019-07-18T18:04:44.7696938Z ------------------------------------------
2019-07-18T18:04:44.7696975Z 
2019-07-18T18:04:44.7697191Z ------------------------------------------
2019-07-18T18:04:44.7697258Z stderr:
---
2019-07-18T18:04:44.8037556Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:44.8037606Z +
2019-07-18T18:04:44.8037634Z 
2019-07-18T18:04:44.8037682Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:44.8037934Z Actual stderr saved to /tmp/compiletestWXeOcX/const-vec-of-fns.stderr
2019-07-18T18:04:44.8037991Z To update references, run this command from build directory:
2019-07-18T18:04:44.8038246Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'const-vec-of-fns.rs'
2019-07-18T18:04:44.8038337Z error: 1 errors occurred comparing output.
2019-07-18T18:04:44.8038385Z status: exit code: 1
2019-07-18T18:04:44.8038385Z status: exit code: 1
2019-07-18T18:04:44.8039331Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/const-vec-of-fns.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/const-vec-of-fns.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/const-vec-of-fns.stage-id.aux" "-A" "unused"
2019-07-18T18:04:44.8039806Z ------------------------------------------
2019-07-18T18:04:44.8039852Z 
2019-07-18T18:04:44.8040046Z ------------------------------------------
2019-07-18T18:04:44.8040089Z stderr:
---
2019-07-18T18:04:44.9296275Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:44.9296327Z +
2019-07-18T18:04:44.9296358Z 
2019-07-18T18:04:44.9296425Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:44.9296479Z Actual stderr saved to /tmp/compiletestWXeOcX/constants.stderr
2019-07-18T18:04:44.9296533Z To update references, run this command from build directory:
2019-07-18T18:04:44.9296974Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'constants.rs'
2019-07-18T18:04:44.9297071Z error: 1 errors occurred comparing output.
2019-07-18T18:04:44.9297120Z status: exit code: 1
2019-07-18T18:04:44.9297120Z status: exit code: 1
2019-07-18T18:04:44.9297804Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/constants.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/constants.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/constants.stage-id.aux" "-A" "unused"
2019-07-18T18:04:44.9298131Z ------------------------------------------
2019-07-18T18:04:44.9298166Z 
2019-07-18T18:04:44.9298380Z ------------------------------------------
2019-07-18T18:04:44.9298444Z stderr:
---
2019-07-18T18:04:45.0656279Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:45.0656331Z +
2019-07-18T18:04:45.0656361Z 
2019-07-18T18:04:45.0656427Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:45.0656482Z Actual stderr saved to /tmp/compiletestWXeOcX/drop_empty_slice.stderr
2019-07-18T18:04:45.0656548Z To update references, run this command from build directory:
2019-07-18T18:04:45.0656982Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'drop_empty_slice.rs'
2019-07-18T18:04:45.0657081Z error: 1 errors occurred comparing output.
2019-07-18T18:04:45.0657129Z status: exit code: 1
2019-07-18T18:04:45.0657129Z status: exit code: 1
2019-07-18T18:04:45.0657835Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/drop_empty_slice.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/drop_empty_slice.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/drop_empty_slice.stage-id.aux" "-A" "unused"
2019-07-18T18:04:45.0658155Z ------------------------------------------
2019-07-18T18:04:45.0658192Z 
2019-07-18T18:04:45.0658525Z ------------------------------------------
2019-07-18T18:04:45.0658591Z stderr:
---
2019-07-18T18:04:45.1118747Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:45.1118798Z +
2019-07-18T18:04:45.1118826Z 
2019-07-18T18:04:45.1118890Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:45.1119306Z Actual stderr saved to /tmp/compiletestWXeOcX/deriving-associated-types.stderr
2019-07-18T18:04:45.1119364Z To update references, run this command from build directory:
2019-07-18T18:04:45.1119879Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'deriving-associated-types.rs'
2019-07-18T18:04:45.1119965Z error: 1 errors occurred comparing output.
2019-07-18T18:04:45.1120009Z status: exit code: 1
2019-07-18T18:04:45.1120009Z status: exit code: 1
2019-07-18T18:04:45.1120680Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/deriving-associated-types.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/deriving-associated-types.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/deriving-associated-types.stage-id.aux" "-A" "unused"
2019-07-18T18:04:45.1120984Z ------------------------------------------
2019-07-18T18:04:45.1121016Z 
2019-07-18T18:04:45.1121214Z ------------------------------------------
2019-07-18T18:04:45.1121356Z stderr:
---
2019-07-18T18:04:45.2493955Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:45.2494006Z +
2019-07-18T18:04:45.2494036Z 
2019-07-18T18:04:45.2494100Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:45.2494520Z Actual stderr saved to /tmp/compiletestWXeOcX/dst-irrefutable-bind.stderr
2019-07-18T18:04:45.2494595Z To update references, run this command from build directory:
2019-07-18T18:04:45.2494912Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'dst-irrefutable-bind.rs'
2019-07-18T18:04:45.2495000Z error: 1 errors occurred comparing output.
2019-07-18T18:04:45.2495048Z status: exit code: 1
2019-07-18T18:04:45.2495048Z status: exit code: 1
2019-07-18T18:04:45.2495982Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-irrefutable-bind.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/dst-irrefutable-bind.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/dst-irrefutable-bind.stage-id.aux" "-A" "unused"
2019-07-18T18:04:45.2496486Z ------------------------------------------
2019-07-18T18:04:45.2496534Z 
2019-07-18T18:04:45.2496757Z ------------------------------------------
2019-07-18T18:04:45.2496822Z stderr:
---
2019-07-18T18:04:45.2809495Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:45.2809553Z +
2019-07-18T18:04:45.2809600Z 
2019-07-18T18:04:45.2809648Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:45.2809908Z Actual stderr saved to /tmp/compiletestWXeOcX/dst-field-align.stderr
2019-07-18T18:04:45.2809980Z To update references, run this command from build directory:
2019-07-18T18:04:45.2810224Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'dst-field-align.rs'
2019-07-18T18:04:45.2810306Z error: 1 errors occurred comparing output.
2019-07-18T18:04:45.2810370Z status: exit code: 1
2019-07-18T18:04:45.2810370Z status: exit code: 1
2019-07-18T18:04:45.2811001Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-field-align.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/dst-field-align.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/dst-field-align.stage-id.aux" "-A" "unused"
2019-07-18T18:04:45.2811408Z ------------------------------------------
2019-07-18T18:04:45.2811442Z 
2019-07-18T18:04:45.2811646Z ------------------------------------------
2019-07-18T18:04:45.2811712Z stderr:
---
2019-07-18T18:04:45.4860288Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:45.4860364Z +
2019-07-18T18:04:45.4860391Z 
2019-07-18T18:04:45.4860434Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:45.4860685Z Actual stderr saved to /tmp/compiletestWXeOcX/dst-struct-sole.stderr
2019-07-18T18:04:45.4860755Z To update references, run this command from build directory:
2019-07-18T18:04:45.4860992Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'dst-struct-sole.rs'
2019-07-18T18:04:45.4861085Z error: 1 errors occurred comparing output.
2019-07-18T18:04:45.4861130Z status: exit code: 1
2019-07-18T18:04:45.4861130Z status: exit code: 1
2019-07-18T18:04:45.4861733Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-struct-sole.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/dst-struct-sole.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/dst-struct-sole.stage-id.aux" "-A" "unused"
2019-07-18T18:04:45.4862135Z ------------------------------------------
2019-07-18T18:04:45.4862169Z 
2019-07-18T18:04:45.4862385Z ------------------------------------------
2019-07-18T18:04:45.4862430Z stderr:
---
2019-07-18T18:04:45.5337846Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:45.5337920Z +
2019-07-18T18:04:45.5337954Z 
2019-07-18T18:04:45.5338005Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:45.5338285Z Actual stderr saved to /tmp/compiletestWXeOcX/dst-raw.stderr
2019-07-18T18:04:45.5338362Z To update references, run this command from build directory:
2019-07-18T18:04:45.5338634Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'dst-raw.rs'
2019-07-18T18:04:45.5338740Z error: 1 errors occurred comparing output.
2019-07-18T18:04:45.5338791Z status: exit code: 1
2019-07-18T18:04:45.5338791Z status: exit code: 1
2019-07-18T18:04:45.5339568Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-raw.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/dst-raw.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/dst-raw.stage-id.aux" "-A" "unused"
2019-07-18T18:04:45.5340014Z ------------------------------------------
2019-07-18T18:04:45.5340049Z 
2019-07-18T18:04:45.5340268Z ------------------------------------------
2019-07-18T18:04:45.5340314Z stderr:
---
2019-07-18T18:04:45.6694576Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:45.6694627Z +
2019-07-18T18:04:45.6694676Z 
2019-07-18T18:04:45.6694722Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:45.6694989Z Actual stderr saved to /tmp/compiletestWXeOcX/enum-nullable-const-null-with-fields.stderr
2019-07-18T18:04:45.6695063Z To update references, run this command from build directory:
2019-07-18T18:04:45.6695335Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'enum-nullable-const-null-with-fields.rs'
2019-07-18T18:04:45.6695416Z error: 1 errors occurred comparing output.
2019-07-18T18:04:45.6696020Z status: exit code: 1
2019-07-18T18:04:45.6696020Z status: exit code: 1
2019-07-18T18:04:45.6696796Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/enum-nullable-const-null-with-fields.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/enum-nullable-const-null-with-fields.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/enum-nullable-const-null-with-fields.stage-id.aux" "-A" "unused"
2019-07-18T18:04:45.6697274Z ------------------------------------------
2019-07-18T18:04:45.6697312Z 
2019-07-18T18:04:45.6697546Z ------------------------------------------
2019-07-18T18:04:45.6697594Z stderr:
---
2019-07-18T18:04:45.7810738Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:45.7810805Z +
2019-07-18T18:04:45.7810834Z 
2019-07-18T18:04:45.7810882Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:45.7811128Z Actual stderr saved to /tmp/compiletestWXeOcX/dst-struct.stderr
2019-07-18T18:04:45.7811203Z To update references, run this command from build directory:
2019-07-18T18:04:45.7811566Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'dst-struct.rs'
2019-07-18T18:04:45.7811671Z error: 1 errors occurred comparing output.
2019-07-18T18:04:45.7811718Z status: exit code: 1
2019-07-18T18:04:45.7811718Z status: exit code: 1
2019-07-18T18:04:45.7812349Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-struct.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/dst-struct.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/dst-struct.stage-id.aux" "-A" "unused"
2019-07-18T18:04:45.7812674Z ------------------------------------------
2019-07-18T18:04:45.7812709Z 
2019-07-18T18:04:45.7812951Z ------------------------------------------
2019-07-18T18:04:45.7813009Z stderr:
---
2019-07-18T18:04:45.8444838Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:45.8444906Z +
2019-07-18T18:04:45.8444968Z 
2019-07-18T18:04:45.8445018Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:45.8445070Z Actual stderr saved to /tmp/compiletestWXeOcX/enums.stderr
2019-07-18T18:04:45.8445241Z To update references, run this command from build directory:
2019-07-18T18:04:45.8445978Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'enums.rs'
2019-07-18T18:04:45.8446092Z error: 1 errors occurred comparing output.
2019-07-18T18:04:45.8446141Z status: exit code: 1
2019-07-18T18:04:45.8446141Z status: exit code: 1
2019-07-18T18:04:45.8446777Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/enums.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/enums.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/enums.stage-id.aux" "-A" "unused"
2019-07-18T18:04:45.8447110Z ------------------------------------------
2019-07-18T18:04:45.8447147Z 
2019-07-18T18:04:45.8447385Z ------------------------------------------
2019-07-18T18:04:45.8447445Z stderr:
---
2019-07-18T18:04:45.9516951Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:45.9517021Z +
2019-07-18T18:04:45.9532297Z 
2019-07-18T18:04:45.9533175Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:45.9534098Z Actual stderr saved to /tmp/compiletestWXeOcX/env.stderr
2019-07-18T18:04:45.9534923Z To update references, run this command from build directory:
2019-07-18T18:04:45.9537153Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'env.rs'
2019-07-18T18:04:45.9537284Z error: 1 errors occurred comparing output.
2019-07-18T18:04:45.9537336Z status: exit code: 1
2019-07-18T18:04:45.9537336Z status: exit code: 1
2019-07-18T18:04:45.9538000Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/env.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/env.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/env.stage-id.aux" "-A" "unused"
2019-07-18T18:04:45.9538308Z ------------------------------------------
2019-07-18T18:04:45.9538360Z 
2019-07-18T18:04:45.9538578Z ------------------------------------------
2019-07-18T18:04:45.9538627Z stderr:
---
2019-07-18T18:04:45.9721850Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:45.9721898Z +
2019-07-18T18:04:45.9721944Z 
2019-07-18T18:04:45.9721991Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:45.9722042Z Actual stderr saved to /tmp/compiletestWXeOcX/exit.stderr
2019-07-18T18:04:45.9722110Z To update references, run this command from build directory:
2019-07-18T18:04:45.9722464Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'exit.rs'
2019-07-18T18:04:45.9722555Z error: 1 errors occurred comparing output.
2019-07-18T18:04:45.9722622Z status: exit code: 1
2019-07-18T18:04:45.9722622Z status: exit code: 1
2019-07-18T18:04:45.9723224Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/exit.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/exit.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/exit.stage-id.aux" "-A" "unused"
2019-07-18T18:04:45.9723544Z ------------------------------------------
2019-07-18T18:04:45.9723579Z 
2019-07-18T18:04:45.9723799Z ------------------------------------------
2019-07-18T18:04:45.9723861Z stderr:
---
2019-07-18T18:04:46.1015164Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:46.1015217Z +
2019-07-18T18:04:46.1015247Z 
2019-07-18T18:04:46.1015296Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:46.1015914Z Actual stderr saved to /tmp/compiletestWXeOcX/extern_types.stderr
2019-07-18T18:04:46.1015971Z To update references, run this command from build directory:
2019-07-18T18:04:46.1016310Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'extern_types.rs'
2019-07-18T18:04:46.1016418Z error: 1 errors occurred comparing output.
2019-07-18T18:04:46.1016468Z status: exit code: 1
2019-07-18T18:04:46.1016468Z status: exit code: 1
2019-07-18T18:04:46.1017138Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/extern_types.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/extern_types.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/extern_types.stage-id.aux" "-A" "unused"
2019-07-18T18:04:46.1017497Z ------------------------------------------
2019-07-18T18:04:46.1017553Z 
2019-07-18T18:04:46.1017804Z ------------------------------------------
2019-07-18T18:04:46.1017856Z stderr:
---
2019-07-18T18:04:46.1293024Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:46.1293073Z +
2019-07-18T18:04:46.1293101Z 
2019-07-18T18:04:46.1293163Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:46.1293216Z Actual stderr saved to /tmp/compiletestWXeOcX/float_fast_math.stderr
2019-07-18T18:04:46.1293351Z To update references, run this command from build directory:
2019-07-18T18:04:46.1293655Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'float_fast_math.rs'
2019-07-18T18:04:46.1293738Z error: 1 errors occurred comparing output.
2019-07-18T18:04:46.1293783Z status: exit code: 1
2019-07-18T18:04:46.1293783Z status: exit code: 1
2019-07-18T18:04:46.1294422Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/float_fast_math.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/float_fast_math.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/float_fast_math.stage-id.aux" "-A" "unused"
2019-07-18T18:04:46.1294741Z ------------------------------------------
2019-07-18T18:04:46.1294786Z 
2019-07-18T18:04:46.1295010Z ------------------------------------------
2019-07-18T18:04:46.1295066Z stderr:
---
2019-07-18T18:04:46.2951975Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:46.2952280Z +
2019-07-18T18:04:46.2952470Z 
2019-07-18T18:04:46.2952942Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:46.2953603Z thread '[ui] run-pass/foreign-fn-linkname.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:Actual stderr saved to /tmp/compiletestWXeOcX/foreign-fn-linkname.stderr
2019-07-18T18:04:46.2953902Z To update references, run this command from build directory:
2019-07-18T18:04:46.2954400Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'foreign-fn-linkname.rs'
2019-07-18T18:04:46.2954891Z error: 1 errors occurred comparing output.
2019-07-18T18:04:46.2955272Z status: exit code: 1
2019-07-18T18:04:46.2955272Z status: exit code: 1
2019-07-18T18:04:46.2956792Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/foreign-fn-linkname.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/foreign-fn-linkname.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/foreign-fn-linkname.stage-id.aux" "-A" "unused"
2019-07-18T18:04:46.2957896Z ------------------------------------------
2019-07-18T18:04:46.2958285Z 
2019-07-18T18:04:46.2958735Z ------------------------------------------
2019-07-18T18:04:46.2959003Z stderr:
---
2019-07-18T18:04:46.3813434Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:46.3813481Z +
2019-07-18T18:04:46.3813526Z 
2019-07-18T18:04:46.3813573Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:46.3813624Z Actual stderr saved to /tmp/compiletestWXeOcX/floats.stderr
2019-07-18T18:04:46.3813692Z To update references, run this command from build directory:
2019-07-18T18:04:46.3813946Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'floats.rs'
2019-07-18T18:04:46.3814026Z error: 1 errors occurred comparing output.
2019-07-18T18:04:46.3814089Z status: exit code: 1
2019-07-18T18:04:46.3814089Z status: exit code: 1
2019-07-18T18:04:46.3814695Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/floats.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/floats.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/floats.stage-id.aux" "-A" "unused"
2019-07-18T18:04:46.3815023Z ------------------------------------------
2019-07-18T18:04:46.3815058Z 
2019-07-18T18:04:46.3815851Z ------------------------------------------
2019-07-18T18:04:46.3815911Z stderr:
---
2019-07-18T18:04:46.4410260Z -hello00000
2019-07-18T18:04:46.4410427Z -
2019-07-18T18:04:46.4410456Z 
2019-07-18T18:04:46.4410520Z The actual stdout differed from the expected stdout.
2019-07-18T18:04:46.4410572Z Actual stdout saved to /tmp/compiletestWXeOcX/format.stdout
2019-07-18T18:04:46.4410697Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-18T18:04:46.4411090Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-18T18:04:46.4411154Z      |
2019-07-18T18:04:46.4411220Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-18T18:04:46.4414669Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:46.4414718Z +
2019-07-18T18:04:46.4414745Z 
2019-07-18T18:04:46.4414809Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:46.4414860Z Actual stderr saved to /tmp/compiletestWXeOcX/format.stderr
2019-07-18T18:04:46.4414920Z To update references, run this command from build directory:
2019-07-18T18:04:46.4415207Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'format.rs'
2019-07-18T18:04:46.4415288Z error: 2 errors occurred comparing output.
2019-07-18T18:04:46.4415739Z status: exit code: 1
2019-07-18T18:04:46.4415739Z status: exit code: 1
2019-07-18T18:04:46.4416423Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/format.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/format.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/format.stage-id.aux" "-A" "unused"
2019-07-18T18:04:46.4416746Z ------------------------------------------
2019-07-18T18:04:46.4416784Z 
2019-07-18T18:04:46.4417001Z ------------------------------------------
2019-07-18T18:04:46.4417079Z stderr:
---
2019-07-18T18:04:46.5170424Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:46.5170490Z +
2019-07-18T18:04:46.5170518Z 
2019-07-18T18:04:46.5170564Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:46.5170633Z Actual stderr saved to /tmp/compiletestWXeOcX/from_utf8.stderr
2019-07-18T18:04:46.5170703Z To update references, run this command from build directory:
2019-07-18T18:04:46.5170970Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'from_utf8.rs'
2019-07-18T18:04:46.5171068Z error: 1 errors occurred comparing output.
2019-07-18T18:04:46.5171115Z status: exit code: 1
2019-07-18T18:04:46.5171115Z status: exit code: 1
2019-07-18T18:04:46.5171723Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/from_utf8.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/from_utf8.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/from_utf8.stage-id.aux" "-A" "unused"
2019-07-18T18:04:46.5172042Z ------------------------------------------
2019-07-18T18:04:46.5172104Z 
2019-07-18T18:04:46.5172328Z ------------------------------------------
2019-07-18T18:04:46.5172375Z stderr:
---
2019-07-18T18:04:46.6574044Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:46.6574112Z +
2019-07-18T18:04:46.6574140Z 
2019-07-18T18:04:46.6574186Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:46.6574256Z Actual stderr saved to /tmp/compiletestWXeOcX/function_pointers.stderr
2019-07-18T18:04:46.6574329Z To update references, run this command from build directory:
2019-07-18T18:04:46.6574640Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'function_pointers.rs'
2019-07-18T18:04:46.6574741Z error: 1 errors occurred comparing output.
2019-07-18T18:04:46.6574787Z status: exit code: 1
2019-07-18T18:04:46.6574787Z status: exit code: 1
2019-07-18T18:04:46.6576092Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/function_pointers.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/function_pointers.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/function_pointers.stage-id.aux" "-A" "unused"
2019-07-18T18:04:46.6576490Z ------------------------------------------
2019-07-18T18:04:46.6576527Z 
2019-07-18T18:04:46.6576772Z ------------------------------------------
2019-07-18T18:04:46.6576821Z stderr:
---
2019-07-18T18:04:46.7291810Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:46.7291869Z +
2019-07-18T18:04:46.7291896Z 
2019-07-18T18:04:46.7291966Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:46.7292018Z Actual stderr saved to /tmp/compiletestWXeOcX/generator.stderr
2019-07-18T18:04:46.7292069Z To update references, run this command from build directory:
2019-07-18T18:04:46.7292347Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'generator.rs'
2019-07-18T18:04:46.7292427Z error: 1 errors occurred comparing output.
2019-07-18T18:04:46.7292472Z status: exit code: 1
2019-07-18T18:04:46.7292472Z status: exit code: 1
2019-07-18T18:04:46.7293074Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/generator.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/generator.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/generator.stage-id.aux" "-A" "unused"
2019-07-18T18:04:46.7293410Z ------------------------------------------
2019-07-18T18:04:46.7293445Z 
2019-07-18T18:04:46.7293662Z ------------------------------------------
2019-07-18T18:04:46.7293725Z stderr:
---
2019-07-18T18:04:46.8537616Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:46.8537688Z +
2019-07-18T18:04:46.8537721Z 
2019-07-18T18:04:46.8537771Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:46.8537826Z Actual stderr saved to /tmp/compiletestWXeOcX/heap.stderr
2019-07-18T18:04:46.8537900Z To update references, run this command from build directory:
2019-07-18T18:04:46.8538176Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'heap.rs'
2019-07-18T18:04:46.8538263Z error: 1 errors occurred comparing output.
2019-07-18T18:04:46.8538336Z status: exit code: 1
2019-07-18T18:04:46.8538336Z status: exit code: 1
2019-07-18T18:04:46.8539127Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/heap.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/heap.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/heap.stage-id.aux" "-A" "unused"
2019-07-18T18:04:46.8539544Z ------------------------------------------
2019-07-18T18:04:46.8539578Z 
2019-07-18T18:04:46.8539798Z ------------------------------------------
2019-07-18T18:04:46.8539844Z stderr:
---
2019-07-18T18:04:46.8646728Z -Hello, world!
2019-07-18T18:04:46.8646923Z -
2019-07-18T18:04:46.8646972Z 
2019-07-18T18:04:46.8647023Z The actual stdout differed from the expected stdout.
2019-07-18T18:04:46.8647078Z Actual stdout saved to /tmp/compiletestWXeOcX/hello.stdout
2019-07-18T18:04:46.8647221Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-18T18:04:46.8647480Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-18T18:04:46.8647570Z      |
2019-07-18T18:04:46.8647626Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-18T18:04:46.8651447Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:46.8651496Z +
2019-07-18T18:04:46.8651522Z 
2019-07-18T18:04:46.8651584Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:46.8651636Z Actual stderr saved to /tmp/compiletestWXeOcX/hello.stderr
2019-07-18T18:04:46.8651686Z To update references, run this command from build directory:
2019-07-18T18:04:46.8651964Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'hello.rs'
2019-07-18T18:04:46.8652045Z error: 2 errors occurred comparing output.
2019-07-18T18:04:46.8652090Z status: exit code: 1
2019-07-18T18:04:46.8652090Z status: exit code: 1
2019-07-18T18:04:46.8652716Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/hello.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/hello.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/hello.stage-id.aux" "-A" "unused"
2019-07-18T18:04:46.8653020Z ------------------------------------------
2019-07-18T18:04:46.8653054Z 
2019-07-18T18:04:46.8653261Z ------------------------------------------
2019-07-18T18:04:46.8653323Z stderr:
---
2019-07-18T18:04:47.4947083Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:47.4947153Z +
2019-07-18T18:04:47.4952890Z 
2019-07-18T18:04:47.4952980Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:47.4953386Z Actual stderr saved to /tmp/compiletestWXeOcX/integer-ops.stderr
2019-07-18T18:04:47.4959377Z To update references, run this command from build directory:
2019-07-18T18:04:47.4959797Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'integer-ops.rs'
2019-07-18T18:04:47.4959910Z error: 1 errors occurred comparing output.
2019-07-18T18:04:47.4959954Z status: exit code: 1
2019-07-18T18:04:47.4959954Z status: exit code: 1
2019-07-18T18:04:47.4977843Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/integer-ops.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/integer-ops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/integer-ops.stage-id.aux" "-A" "unused"
2019-07-18T18:04:47.4978225Z ------------------------------------------
2019-07-18T18:04:47.4978263Z 
2019-07-18T18:04:47.4978484Z ------------------------------------------
2019-07-18T18:04:47.4978554Z stderr:
---
2019-07-18T18:04:48.0135964Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:48.0136034Z +
2019-07-18T18:04:48.0136066Z 
2019-07-18T18:04:48.0136141Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:48.0136399Z Actual stderr saved to /tmp/compiletestWXeOcX/intrinsics-math.stderr
2019-07-18T18:04:48.0136457Z To update references, run this command from build directory:
2019-07-18T18:04:48.0136712Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'intrinsics-math.rs'
2019-07-18T18:04:48.0136817Z error: 1 errors occurred comparing output.
2019-07-18T18:04:48.0136866Z status: exit code: 1
2019-07-18T18:04:48.0136866Z status: exit code: 1
2019-07-18T18:04:48.0137538Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics-math.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/intrinsics-math.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/intrinsics-math.stage-id.aux" "-A" "unused"
2019-07-18T18:04:48.0138075Z ------------------------------------------
2019-07-18T18:04:48.0138114Z 
2019-07-18T18:04:48.0138336Z ------------------------------------------
2019-07-18T18:04:48.0138385Z stderr:
---
2019-07-18T18:04:48.2135749Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:48.2135801Z +
2019-07-18T18:04:48.2135830Z 
2019-07-18T18:04:48.2135894Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:48.2135949Z Actual stderr saved to /tmp/compiletestWXeOcX/intrinsics.stderr
2019-07-18T18:04:48.2136002Z To update references, run this command from build directory:
2019-07-18T18:04:48.2136281Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'intrinsics.rs'
2019-07-18T18:04:48.2136365Z error: 1 errors occurred comparing output.
2019-07-18T18:04:48.2136413Z status: exit code: 1
2019-07-18T18:04:48.2136413Z status: exit code: 1
2019-07-18T18:04:48.2137175Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/intrinsics.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/intrinsics.stage-id.aux" "-A" "unused"
2019-07-18T18:04:48.2137546Z ------------------------------------------
2019-07-18T18:04:48.2137584Z 
2019-07-18T18:04:48.2137803Z ------------------------------------------
2019-07-18T18:04:48.2137851Z stderr:
---
2019-07-18T18:04:48.4410425Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:48.4410499Z +
2019-07-18T18:04:48.4410528Z 
2019-07-18T18:04:48.4410582Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:48.4410634Z Actual stderr saved to /tmp/compiletestWXeOcX/ints.stderr
2019-07-18T18:04:48.4410702Z To update references, run this command from build directory:
2019-07-18T18:04:48.4410962Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'ints.rs'
2019-07-18T18:04:48.4411045Z error: 1 errors occurred comparing output.
2019-07-18T18:04:48.4411106Z status: exit code: 1
2019-07-18T18:04:48.4411106Z status: exit code: 1
2019-07-18T18:04:48.4411702Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ints.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/ints.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/ints.stage-id.aux" "-A" "unused"
2019-07-18T18:04:48.4412113Z ------------------------------------------
2019-07-18T18:04:48.4412158Z 
2019-07-18T18:04:48.4412419Z ------------------------------------------
2019-07-18T18:04:48.4412465Z stderr:
---
2019-07-18T18:04:48.5793047Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:48.5793102Z +
2019-07-18T18:04:48.5793150Z 
2019-07-18T18:04:48.5793198Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:48.5793469Z Actual stderr saved to /tmp/compiletestWXeOcX/issue-15063.stderr
2019-07-18T18:04:48.5793547Z To update references, run this command from build directory:
2019-07-18T18:04:48.5793809Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'issue-15063.rs'
2019-07-18T18:04:48.5793892Z error: 1 errors occurred comparing output.
2019-07-18T18:04:48.5793957Z status: exit code: 1
2019-07-18T18:04:48.5793957Z status: exit code: 1
2019-07-18T18:04:48.5794769Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15063.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/issue-15063.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/issue-15063.stage-id.aux" "-A" "unused"
2019-07-18T18:04:48.5795503Z ------------------------------------------
2019-07-18T18:04:48.5795733Z 
2019-07-18T18:04:48.5796017Z ------------------------------------------
2019-07-18T18:04:48.5796069Z stderr:
---
2019-07-18T18:04:48.7638189Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:48.7638262Z +
2019-07-18T18:04:48.7638292Z 
2019-07-18T18:04:48.7638344Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:48.7638636Z Actual stderr saved to /tmp/compiletestWXeOcX/issue-15080.stderr
2019-07-18T18:04:48.7638696Z To update references, run this command from build directory:
2019-07-18T18:04:48.7639113Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'issue-15080.rs'
2019-07-18T18:04:48.7639211Z error: 1 errors occurred comparing output.
2019-07-18T18:04:48.7639257Z status: exit code: 1
2019-07-18T18:04:48.7639257Z status: exit code: 1
2019-07-18T18:04:48.7639946Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15080.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/issue-15080.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/issue-15080.stage-id.aux" "-A" "unused"
2019-07-18T18:04:48.7640298Z ------------------------------------------
2019-07-18T18:04:48.7640333Z 
2019-07-18T18:04:48.7640561Z ------------------------------------------
2019-07-18T18:04:48.7640607Z stderr:
---
2019-07-18T18:04:48.9537586Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:48.9537665Z +
2019-07-18T18:04:48.9537696Z 
2019-07-18T18:04:48.9537747Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:48.9538042Z Actual stderr saved to /tmp/compiletestWXeOcX/issue-15523-big.stderr
2019-07-18T18:04:48.9538121Z To update references, run this command from build directory:
2019-07-18T18:04:48.9538409Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'issue-15523-big.rs'
2019-07-18T18:04:48.9538515Z error: 1 errors occurred comparing output.
2019-07-18T18:04:48.9538565Z status: exit code: 1
2019-07-18T18:04:48.9538565Z status: exit code: 1
2019-07-18T18:04:48.9539590Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15523-big.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/issue-15523-big.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/issue-15523-big.stage-id.aux" "-A" "unused"
2019-07-18T18:04:48.9539961Z ------------------------------------------
2019-07-18T18:04:48.9539996Z 
2019-07-18T18:04:48.9540218Z ------------------------------------------
2019-07-18T18:04:48.9540264Z stderr:
---
2019-07-18T18:04:49.1216987Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:49.1217039Z +
2019-07-18T18:04:49.1217087Z 
2019-07-18T18:04:49.1217136Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:49.1217385Z Actual stderr saved to /tmp/compiletestWXeOcX/issue-17877.stderr
2019-07-18T18:04:49.1217459Z To update references, run this command from build directory:
2019-07-18T18:04:49.1217718Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'issue-17877.rs'
2019-07-18T18:04:49.1217802Z error: 1 errors occurred comparing output.
2019-07-18T18:04:49.1217866Z status: exit code: 1
2019-07-18T18:04:49.1217866Z status: exit code: 1
2019-07-18T18:04:49.1218663Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-17877.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/issue-17877.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/issue-17877.stage-id.aux" "-A" "unused"
2019-07-18T18:04:49.1219196Z ------------------------------------------
2019-07-18T18:04:49.1219231Z 
2019-07-18T18:04:49.1219439Z ------------------------------------------
2019-07-18T18:04:49.1219505Z stderr:
---
2019-07-18T18:04:49.2768739Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:49.2769233Z +
2019-07-18T18:04:49.2769358Z 
2019-07-18T18:04:49.2769498Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:49.2769931Z Actual stderr saved to /tmp/compiletestWXeOcX/issue-20575.stderr
2019-07-18T18:04:49.2770202Z To update references, run this command from build directory:
2019-07-18T18:04:49.2770604Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'issue-20575.rs'
2019-07-18T18:04:49.2770912Z error: 1 errors occurred comparing output.
2019-07-18T18:04:49.2771068Z status: exit code: 1
2019-07-18T18:04:49.2771068Z status: exit code: 1
2019-07-18T18:04:49.2772052Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-20575.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/issue-20575.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/issue-20575.stage-id.aux" "-A" "unused"
2019-07-18T18:04:49.2772770Z ------------------------------------------
2019-07-18T18:04:49.2772938Z 
2019-07-18T18:04:49.2773288Z ------------------------------------------
2019-07-18T18:04:49.2773486Z stderr:
---
2019-07-18T18:04:49.4577908Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:49.4577962Z +
2019-07-18T18:04:49.4577993Z 
2019-07-18T18:04:49.4578045Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:49.4578334Z Actual stderr saved to /tmp/compiletestWXeOcX/issue-23261.stderr
2019-07-18T18:04:49.4578396Z To update references, run this command from build directory:
2019-07-18T18:04:49.4578690Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'issue-23261.rs'
2019-07-18T18:04:49.4578961Z error: 1 errors occurred comparing output.
2019-07-18T18:04:49.4579012Z status: exit code: 1
2019-07-18T18:04:49.4579012Z status: exit code: 1
2019-07-18T18:04:49.4579860Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-23261.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/issue-23261.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/issue-23261.stage-id.aux" "-A" "unused"
2019-07-18T18:04:49.4580161Z ------------------------------------------
2019-07-18T18:04:49.4580213Z 
2019-07-18T18:04:49.4580423Z ------------------------------------------
2019-07-18T18:04:49.4580471Z stderr:
---
2019-07-18T18:04:49.6336400Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:49.6336458Z +
2019-07-18T18:04:49.6336510Z 
2019-07-18T18:04:49.6336562Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:49.6336852Z Actual stderr saved to /tmp/compiletestWXeOcX/issue-26709.stderr
2019-07-18T18:04:49.6336931Z To update references, run this command from build directory:
2019-07-18T18:04:49.6337217Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'issue-26709.rs'
2019-07-18T18:04:49.6337319Z error: 1 errors occurred comparing output.
2019-07-18T18:04:49.6337540Z status: exit code: 1
2019-07-18T18:04:49.6337540Z status: exit code: 1
2019-07-18T18:04:49.6338253Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-26709.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/issue-26709.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/issue-26709.stage-id.aux" "-A" "unused"
2019-07-18T18:04:49.6338601Z ------------------------------------------
2019-07-18T18:04:49.6338639Z 
2019-07-18T18:04:49.6339035Z ------------------------------------------
2019-07-18T18:04:49.6339081Z stderr:
---
2019-07-18T18:04:49.7896762Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:49.7896816Z +
2019-07-18T18:04:49.7896848Z 
2019-07-18T18:04:49.7896917Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:49.7897198Z Actual stderr saved to /tmp/compiletestWXeOcX/issue-27901.stderr
2019-07-18T18:04:49.7897258Z To update references, run this command from build directory:
2019-07-18T18:04:49.7897560Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'issue-27901.rs'
2019-07-18T18:04:49.7897854Z error: 1 errors occurred comparing output.
2019-07-18T18:04:49.7897917Z status: exit code: 1
2019-07-18T18:04:49.7897917Z status: exit code: 1
2019-07-18T18:04:49.7898643Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-27901.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/issue-27901.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/issue-27901.stage-id.aux" "-A" "unused"
2019-07-18T18:04:49.7898999Z ------------------------------------------
2019-07-18T18:04:49.7899039Z 
2019-07-18T18:04:49.7899272Z ------------------------------------------
2019-07-18T18:04:49.7899340Z stderr:
---
2019-07-18T18:04:49.9417582Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:49.9417637Z +
2019-07-18T18:04:49.9417687Z 
2019-07-18T18:04:49.9417739Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:49.9418018Z Actual stderr saved to /tmp/compiletestWXeOcX/intrinsics-integer.stderr
2019-07-18T18:04:49.9418097Z To update references, run this command from build directory:
2019-07-18T18:04:49.9418549Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'intrinsics-integer.rs'
2019-07-18T18:04:49.9418653Z error: 1 errors occurred comparing output.
2019-07-18T18:04:49.9418723Z status: exit code: 1
2019-07-18T18:04:49.9418723Z status: exit code: 1
2019-07-18T18:04:49.9419438Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics-integer.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/intrinsics-integer.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/intrinsics-integer.stage-id.aux" "-A" "unused"
2019-07-18T18:04:49.9419789Z ------------------------------------------
2019-07-18T18:04:49.9419827Z 
2019-07-18T18:04:49.9420082Z ------------------------------------------
2019-07-18T18:04:49.9420238Z stderr:
---
2019-07-18T18:04:49.9730102Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:49.9730153Z +
2019-07-18T18:04:49.9730182Z 
2019-07-18T18:04:49.9730243Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:49.9730510Z Actual stderr saved to /tmp/compiletestWXeOcX/issue-29746.stderr
2019-07-18T18:04:49.9730700Z To update references, run this command from build directory:
2019-07-18T18:04:49.9731008Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'issue-29746.rs'
2019-07-18T18:04:49.9731089Z error: 1 errors occurred comparing output.
2019-07-18T18:04:49.9731153Z status: exit code: 1
2019-07-18T18:04:49.9731153Z status: exit code: 1
2019-07-18T18:04:49.9731758Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-29746.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/issue-29746.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/issue-29746.stage-id.aux" "-A" "unused"
2019-07-18T18:04:49.9732059Z ------------------------------------------
2019-07-18T18:04:49.9732179Z 
2019-07-18T18:04:49.9732414Z ------------------------------------------
2019-07-18T18:04:49.9732479Z stderr:
---
2019-07-18T18:04:50.0976061Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:50.0976119Z +
2019-07-18T18:04:50.0976164Z 
2019-07-18T18:04:50.0976231Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:50.0976653Z Actual stderr saved to /tmp/compiletestWXeOcX/issue-30530.stderr
2019-07-18T18:04:50.0976723Z To update references, run this command from build directory:
2019-07-18T18:04:50.0977031Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'issue-30530.rs'
2019-07-18T18:04:50.0977119Z error: 1 errors occurred comparing output.
2019-07-18T18:04:50.0977167Z status: exit code: 1
2019-07-18T18:04:50.0977167Z status: exit code: 1
2019-07-18T18:04:50.0977828Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-30530.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/issue-30530.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/issue-30530.stage-id.aux" "-A" "unused"
2019-07-18T18:04:50.0978271Z ------------------------------------------
2019-07-18T18:04:50.0978317Z 
2019-07-18T18:04:50.0978540Z ------------------------------------------
2019-07-18T18:04:50.0978605Z stderr:
---
2019-07-18T18:04:50.1371599Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:50.1371657Z +
2019-07-18T18:04:50.1371687Z 
2019-07-18T18:04:50.1371871Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:50.1372187Z Actual stderr saved to /tmp/compiletestWXeOcX/issue-31267-additional.stderr
2019-07-18T18:04:50.1372247Z To update references, run this command from build directory:
2019-07-18T18:04:50.1372501Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'issue-31267-additional.rs'
2019-07-18T18:04:50.1372602Z error: 1 errors occurred comparing output.
2019-07-18T18:04:50.1372648Z status: exit code: 1
2019-07-18T18:04:50.1372648Z status: exit code: 1
2019-07-18T18:04:50.1373390Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-31267-additional.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/issue-31267-additional.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/issue-31267-additional.stage-id.aux" "-A" "unused"
2019-07-18T18:04:50.1373815Z ------------------------------------------
2019-07-18T18:04:50.1373851Z 
2019-07-18T18:04:50.1374060Z ------------------------------------------
2019-07-18T18:04:50.1374107Z stderr:
---
2019-07-18T18:04:50.2373953Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:50.2374003Z +
2019-07-18T18:04:50.2374033Z 
2019-07-18T18:04:50.2374097Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:50.2374345Z Actual stderr saved to /tmp/compiletestWXeOcX/issue-33387.stderr
2019-07-18T18:04:50.2374400Z To update references, run this command from build directory:
2019-07-18T18:04:50.2374675Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'issue-33387.rs'
2019-07-18T18:04:50.2374756Z error: 1 errors occurred comparing output.
2019-07-18T18:04:50.2374818Z status: exit code: 1
2019-07-18T18:04:50.2374818Z status: exit code: 1
2019-07-18T18:04:50.2375906Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-33387.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/issue-33387.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/issue-33387.stage-id.aux" "-A" "unused"
2019-07-18T18:04:50.2376384Z ------------------------------------------
2019-07-18T18:04:50.2376422Z 
2019-07-18T18:04:50.2376638Z ------------------------------------------
2019-07-18T18:04:50.2376704Z stderr:
---
2019-07-18T18:04:50.2566744Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:50.2566797Z +
2019-07-18T18:04:50.2566825Z 
2019-07-18T18:04:50.2566873Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:50.2567139Z Actual stderr saved to /tmp/compiletestWXeOcX/issue-34571.stderr
2019-07-18T18:04:50.2567198Z To update references, run this command from build directory:
2019-07-18T18:04:50.2567454Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'issue-34571.rs'
2019-07-18T18:04:50.2567557Z error: 1 errors occurred comparing output.
2019-07-18T18:04:50.2567683Z status: exit code: 1
2019-07-18T18:04:50.2567683Z status: exit code: 1
2019-07-18T18:04:50.2568498Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-34571.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/issue-34571.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/issue-34571.stage-id.aux" "-A" "unused"
2019-07-18T18:04:50.2568806Z ------------------------------------------
2019-07-18T18:04:50.2568856Z 
2019-07-18T18:04:50.2569058Z ------------------------------------------
2019-07-18T18:04:50.2569103Z stderr:
---
2019-07-18T18:04:50.4091035Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:50.4091088Z +
2019-07-18T18:04:50.4091118Z 
2019-07-18T18:04:50.4091184Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:50.4091453Z Actual stderr saved to /tmp/compiletestWXeOcX/issue-35815.stderr
2019-07-18T18:04:50.4091513Z To update references, run this command from build directory:
2019-07-18T18:04:50.4091782Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'issue-35815.rs'
2019-07-18T18:04:50.4091886Z error: 1 errors occurred comparing output.
2019-07-18T18:04:50.4091934Z status: exit code: 1
2019-07-18T18:04:50.4091934Z status: exit code: 1
2019-07-18T18:04:50.4092601Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-35815.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/issue-35815.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/issue-35815.stage-id.aux" "-A" "unused"
2019-07-18T18:04:50.4093064Z ------------------------------------------
2019-07-18T18:04:50.4093102Z 
2019-07-18T18:04:50.4093330Z ------------------------------------------
2019-07-18T18:04:50.4093380Z stderr:
---
2019-07-18T18:04:50.4249522Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:50.4249591Z +
2019-07-18T18:04:50.4249620Z 
2019-07-18T18:04:50.4249666Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:50.4249935Z Actual stderr saved to /tmp/compiletestWXeOcX/issue-36278-prefix-nesting.stderr
2019-07-18T18:04:50.4249992Z To update references, run this command from build directory:
2019-07-18T18:04:50.4250252Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'issue-36278-prefix-nesting.rs'
2019-07-18T18:04:50.4250461Z error: 1 errors occurred comparing output.
2019-07-18T18:04:50.4250510Z status: exit code: 1
2019-07-18T18:04:50.4250510Z status: exit code: 1
2019-07-18T18:04:50.4251189Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-36278-prefix-nesting.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/issue-36278-prefix-nesting.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/issue-36278-prefix-nesting.stage-id.aux" "-A" "unused"
2019-07-18T18:04:50.4276500Z ------------------------------------------
2019-07-18T18:04:50.4276546Z 
2019-07-18T18:04:50.4276775Z ------------------------------------------
2019-07-18T18:04:50.4276837Z stderr:
---
2019-07-18T18:04:50.5522907Z -S { s: 5 }
2019-07-18T18:04:50.5523097Z -
2019-07-18T18:04:50.5523127Z 
2019-07-18T18:04:50.5523175Z The actual stdout differed from the expected stdout.
2019-07-18T18:04:50.5523413Z Actual stdout saved to /tmp/compiletestWXeOcX/issue-3794.stdout
2019-07-18T18:04:50.5523547Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-18T18:04:50.5523780Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-18T18:04:50.5523848Z      |
2019-07-18T18:04:50.5523895Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-18T18:04:50.5528185Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:50.5528238Z +
2019-07-18T18:04:50.5528286Z 
2019-07-18T18:04:50.5528338Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:50.5528609Z Actual stderr saved to /tmp/compiletestWXeOcX/issue-3794.stderr
2019-07-18T18:04:50.5528668Z To update references, run this command from build directory:
2019-07-18T18:04:50.5528966Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'issue-3794.rs'
2019-07-18T18:04:50.5529054Z error: 2 errors occurred comparing output.
2019-07-18T18:04:50.5529133Z status: exit code: 1
2019-07-18T18:04:50.5529133Z status: exit code: 1
2019-07-18T18:04:50.5529809Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-3794.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/issue-3794.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/issue-3794.stage-id.aux" "-A" "unused"
2019-07-18T18:04:50.5530160Z ------------------------------------------
2019-07-18T18:04:50.5530198Z 
2019-07-18T18:04:50.5530431Z ------------------------------------------
2019-07-18T18:04:50.5530502Z stderr:
---
2019-07-18T18:04:50.5609487Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:50.5609554Z +
2019-07-18T18:04:50.5609583Z 
2019-07-18T18:04:50.5609632Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:50.5609888Z Actual stderr saved to /tmp/compiletestWXeOcX/issue-53728.stderr
2019-07-18T18:04:50.5609962Z To update references, run this command from build directory:
2019-07-18T18:04:50.5610229Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'issue-53728.rs'
2019-07-18T18:04:50.5610329Z error: 1 errors occurred comparing output.
2019-07-18T18:04:50.5610377Z status: exit code: 1
2019-07-18T18:04:50.5610377Z status: exit code: 1
2019-07-18T18:04:50.5611023Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-53728.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/issue-53728.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/issue-53728.stage-id.aux" "-A" "unused"
2019-07-18T18:04:50.5611365Z ------------------------------------------
2019-07-18T18:04:50.5611402Z 
2019-07-18T18:04:50.5611646Z ------------------------------------------
2019-07-18T18:04:50.5611695Z stderr:
---
2019-07-18T18:04:50.6892746Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:50.6892792Z +
2019-07-18T18:04:50.6892820Z 
2019-07-18T18:04:50.6892880Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:50.6893123Z Actual stderr saved to /tmp/compiletestWXeOcX/issue-miri-184.stderr
2019-07-18T18:04:50.6893175Z To update references, run this command from build directory:
2019-07-18T18:04:50.6893442Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'issue-miri-184.rs'
2019-07-18T18:04:50.6893521Z error: 1 errors occurred comparing output.
2019-07-18T18:04:50.6893566Z status: exit code: 1
2019-07-18T18:04:50.6893566Z status: exit code: 1
2019-07-18T18:04:50.6894209Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-miri-184.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/issue-miri-184.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/issue-miri-184.stage-id.aux" "-A" "unused"
2019-07-18T18:04:50.6894524Z ------------------------------------------
2019-07-18T18:04:50.6894558Z 
2019-07-18T18:04:50.6895014Z ------------------------------------------
2019-07-18T18:04:50.6895077Z stderr:
---
2019-07-18T18:04:50.6965701Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:50.6965754Z +
2019-07-18T18:04:50.6965784Z 
2019-07-18T18:04:50.6965849Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:50.6966099Z Actual stderr saved to /tmp/compiletestWXeOcX/issue-5917.stderr
2019-07-18T18:04:50.6966157Z To update references, run this command from build directory:
2019-07-18T18:04:50.6966439Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'issue-5917.rs'
2019-07-18T18:04:50.6966546Z error: 1 errors occurred comparing output.
2019-07-18T18:04:50.6966594Z status: exit code: 1
2019-07-18T18:04:50.6966594Z status: exit code: 1
2019-07-18T18:04:50.6967259Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-5917.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/issue-5917.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/issue-5917.stage-id.aux" "-A" "unused"
2019-07-18T18:04:50.6967586Z ------------------------------------------
2019-07-18T18:04:50.6967623Z 
2019-07-18T18:04:50.6967840Z ------------------------------------------
2019-07-18T18:04:50.6967899Z stderr:
---
2019-07-18T18:04:50.8528672Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:50.8528730Z +
2019-07-18T18:04:50.8528759Z 
2019-07-18T18:04:50.8528828Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:50.8529089Z Actual stderr saved to /tmp/compiletestWXeOcX/last-use-in-cap-clause.stderr
2019-07-18T18:04:50.8529149Z To update references, run this command from build directory:
2019-07-18T18:04:50.8529433Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'last-use-in-cap-clause.rs'
2019-07-18T18:04:50.8529539Z error: 1 errors occurred comparing output.
2019-07-18T18:04:50.8529608Z status: exit code: 1
2019-07-18T18:04:50.8529608Z status: exit code: 1
2019-07-18T18:04:50.8530295Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/last-use-in-cap-clause.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/last-use-in-cap-clause.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/last-use-in-cap-clause.stage-id.aux" "-A" "unused"
2019-07-18T18:04:50.8530624Z ------------------------------------------
2019-07-18T18:04:50.8530660Z 
2019-07-18T18:04:50.8530875Z ------------------------------------------
2019-07-18T18:04:50.8530943Z stderr:
---
2019-07-18T18:04:50.8976964Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:50.8977017Z +
2019-07-18T18:04:50.8977047Z 
2019-07-18T18:04:50.8977112Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:50.8977165Z Actual stderr saved to /tmp/compiletestWXeOcX/iter.stderr
2019-07-18T18:04:50.8977230Z To update references, run this command from build directory:
2019-07-18T18:04:50.8977515Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'iter.rs'
2019-07-18T18:04:50.8977603Z error: 1 errors occurred comparing output.
2019-07-18T18:04:50.8977652Z status: exit code: 1
2019-07-18T18:04:50.8977652Z status: exit code: 1
2019-07-18T18:04:50.8978302Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/iter.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/iter.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/iter.stage-id.aux" "-A" "unused"
2019-07-18T18:04:50.8978779Z ------------------------------------------
2019-07-18T18:04:50.8978814Z 
2019-07-18T18:04:50.8979176Z ------------------------------------------
2019-07-18T18:04:50.8979246Z stderr:
---
2019-07-18T18:04:51.0689863Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:51.0689936Z +
2019-07-18T18:04:51.0689965Z 
2019-07-18T18:04:51.0690012Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:51.0690275Z Actual stderr saved to /tmp/compiletestWXeOcX/linked-list.stderr
2019-07-18T18:04:51.0690352Z To update references, run this command from build directory:
2019-07-18T18:04:51.0690606Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'linked-list.rs'
2019-07-18T18:04:51.0690733Z error: 1 errors occurred comparing output.
2019-07-18T18:04:51.0690781Z status: exit code: 1
2019-07-18T18:04:51.0690781Z status: exit code: 1
2019-07-18T18:04:51.0691399Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/linked-list.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/linked-list.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/linked-list.stage-id.aux" "-A" "unused"
2019-07-18T18:04:51.0691718Z ------------------------------------------
2019-07-18T18:04:51.0691762Z 
2019-07-18T18:04:51.0692010Z ------------------------------------------
2019-07-18T18:04:51.0692057Z stderr:
---
2019-07-18T18:04:51.1733759Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:51.1733807Z +
2019-07-18T18:04:51.1733833Z 
2019-07-18T18:04:51.1733895Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:51.1734145Z Actual stderr saved to /tmp/compiletestWXeOcX/loop-break-value.stderr
2019-07-18T18:04:51.1734200Z To update references, run this command from build directory:
2019-07-18T18:04:51.1734462Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'loop-break-value.rs'
2019-07-18T18:04:51.1734541Z error: 1 errors occurred comparing output.
2019-07-18T18:04:51.1734585Z status: exit code: 1
2019-07-18T18:04:51.1734585Z status: exit code: 1
2019-07-18T18:04:51.1735843Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/loop-break-value.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/loop-break-value.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/loop-break-value.stage-id.aux" "-A" "unused"
2019-07-18T18:04:51.1736196Z ------------------------------------------
2019-07-18T18:04:51.1736233Z 
2019-07-18T18:04:51.1736457Z ------------------------------------------
2019-07-18T18:04:51.1736524Z stderr:
---
2019-07-18T18:04:51.2774684Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:51.2774738Z +
2019-07-18T18:04:51.2774790Z 
2019-07-18T18:04:51.2774841Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:51.2774894Z Actual stderr saved to /tmp/compiletestWXeOcX/loops.stderr
2019-07-18T18:04:51.2775711Z To update references, run this command from build directory:
2019-07-18T18:04:51.2776062Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'loops.rs'
2019-07-18T18:04:51.2776150Z error: 1 errors occurred comparing output.
2019-07-18T18:04:51.2776218Z status: exit code: 1
2019-07-18T18:04:51.2776218Z status: exit code: 1
2019-07-18T18:04:51.2776862Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/loops.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/loops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/loops.stage-id.aux" "-A" "unused"
2019-07-18T18:04:51.2777204Z ------------------------------------------
2019-07-18T18:04:51.2777242Z 
2019-07-18T18:04:51.2777459Z ------------------------------------------
2019-07-18T18:04:51.2777529Z stderr:
---
2019-07-18T18:04:51.3011526Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:51.3011603Z +
2019-07-18T18:04:51.3011631Z 
2019-07-18T18:04:51.3011686Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:51.3011738Z Actual stderr saved to /tmp/compiletestWXeOcX/main_fn.stderr
2019-07-18T18:04:51.3011864Z To update references, run this command from build directory:
2019-07-18T18:04:51.3012131Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'main_fn.rs'
2019-07-18T18:04:51.3012231Z error: 1 errors occurred comparing output.
2019-07-18T18:04:51.3012277Z status: exit code: 1
2019-07-18T18:04:51.3012277Z status: exit code: 1
2019-07-18T18:04:51.3012967Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/main_fn.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/main_fn.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/main_fn.stage-id.aux" "-A" "unused"
2019-07-18T18:04:51.3013312Z ------------------------------------------
2019-07-18T18:04:51.3013349Z 
2019-07-18T18:04:51.3013589Z ------------------------------------------
2019-07-18T18:04:51.3013637Z stderr:
---
2019-07-18T18:04:51.4216093Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:51.4216174Z +
2019-07-18T18:04:51.4216231Z 
2019-07-18T18:04:51.4216282Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:51.4216337Z Actual stderr saved to /tmp/compiletestWXeOcX/many_shr_bor.stderr
2019-07-18T18:04:51.4216410Z To update references, run this command from build directory:
2019-07-18T18:04:51.4216737Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'many_shr_bor.rs'
2019-07-18T18:04:51.4216830Z error: 1 errors occurred comparing output.
2019-07-18T18:04:51.4216897Z status: exit code: 1
2019-07-18T18:04:51.4216897Z status: exit code: 1
2019-07-18T18:04:51.4217653Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/many_shr_bor.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/many_shr_bor.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/many_shr_bor.stage-id.aux" "-A" "unused"
2019-07-18T18:04:51.4218042Z ------------------------------------------
2019-07-18T18:04:51.4218082Z 
2019-07-18T18:04:51.4218345Z ------------------------------------------
2019-07-18T18:04:51.4218397Z stderr:
---
2019-07-18T18:04:51.4354452Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:51.4354503Z +
2019-07-18T18:04:51.4354550Z 
2019-07-18T18:04:51.4354599Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:51.4354653Z Actual stderr saved to /tmp/compiletestWXeOcX/match_slice.stderr
2019-07-18T18:04:51.4354706Z To update references, run this command from build directory:
2019-07-18T18:04:51.4355427Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'match_slice.rs'
2019-07-18T18:04:51.4355523Z error: 1 errors occurred comparing output.
2019-07-18T18:04:51.4355590Z status: exit code: 1
2019-07-18T18:04:51.4355590Z status: exit code: 1
2019-07-18T18:04:51.4356253Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/match_slice.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/match_slice.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/match_slice.stage-id.aux" "-A" "unused"
2019-07-18T18:04:51.4356597Z ------------------------------------------
2019-07-18T18:04:51.4356634Z 
2019-07-18T18:04:51.4356849Z ------------------------------------------
2019-07-18T18:04:51.4356916Z stderr:
---
2019-07-18T18:04:51.6582307Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:51.6582378Z +
2019-07-18T18:04:51.6582407Z 
2019-07-18T18:04:51.6582456Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:51.6582508Z Actual stderr saved to /tmp/compiletestWXeOcX/memchr.stderr
2019-07-18T18:04:51.6582580Z To update references, run this command from build directory:
2019-07-18T18:04:51.6582851Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'memchr.rs'
2019-07-18T18:04:51.6582951Z error: 1 errors occurred comparing output.
2019-07-18T18:04:51.6582999Z status: exit code: 1
2019-07-18T18:04:51.6582999Z status: exit code: 1
2019-07-18T18:04:51.6583608Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/memchr.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/memchr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/memchr.stage-id.aux" "-A" "unused"
2019-07-18T18:04:51.6583947Z ------------------------------------------
2019-07-18T18:04:51.6583983Z 
2019-07-18T18:04:51.6584221Z ------------------------------------------
2019-07-18T18:04:51.6584268Z stderr:
---
2019-07-18T18:04:51.6637627Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:51.6637682Z +
2019-07-18T18:04:51.6637715Z 
2019-07-18T18:04:51.6637766Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:51.6637840Z Actual stderr saved to /tmp/compiletestWXeOcX/mir_coercions.stderr
2019-07-18T18:04:51.6637897Z To update references, run this command from build directory:
2019-07-18T18:04:51.6638180Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'mir_coercions.rs'
2019-07-18T18:04:51.6638286Z error: 1 errors occurred comparing output.
2019-07-18T18:04:51.6638336Z status: exit code: 1
2019-07-18T18:04:51.6638336Z status: exit code: 1
2019-07-18T18:04:51.6639147Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mir_coercions.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/mir_coercions.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/mir_coercions.stage-id.aux" "-A" "unused"
2019-07-18T18:04:51.6639448Z ------------------------------------------
2019-07-18T18:04:51.6639498Z 
2019-07-18T18:04:51.6639701Z ------------------------------------------
2019-07-18T18:04:51.6639748Z stderr:
---
2019-07-18T18:04:51.8304820Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:51.8305571Z +
2019-07-18T18:04:51.8305609Z 
2019-07-18T18:04:51.8305660Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:51.8306023Z Actual stderr saved to /tmp/compiletestWXeOcX/miri-issue-133.stderr
2019-07-18T18:04:51.8306100Z To update references, run this command from build directory:
2019-07-18T18:04:51.8306362Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'miri-issue-133.rs'
2019-07-18T18:04:51.8306452Z error: 1 errors occurred comparing output.
2019-07-18T18:04:51.8306519Z status: exit code: 1
2019-07-18T18:04:51.8306519Z status: exit code: 1
2019-07-18T18:04:51.8307221Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/miri-issue-133.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/miri-issue-133.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/miri-issue-133.stage-id.aux" "-A" "unused"
2019-07-18T18:04:51.8307577Z ------------------------------------------
2019-07-18T18:04:51.8307616Z 
2019-07-18T18:04:51.8307869Z ------------------------------------------
2019-07-18T18:04:51.8307920Z stderr:
---
2019-07-18T18:04:51.8321916Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:51.8321984Z +
2019-07-18T18:04:51.8322012Z 
2019-07-18T18:04:51.8322061Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:51.8322133Z Actual stderr saved to /tmp/compiletestWXeOcX/mir_fat_ptr.stderr
2019-07-18T18:04:51.8322188Z To update references, run this command from build directory:
2019-07-18T18:04:51.8322454Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'mir_fat_ptr.rs'
2019-07-18T18:04:51.8322566Z error: 1 errors occurred comparing output.
2019-07-18T18:04:51.8322616Z status: exit code: 1
2019-07-18T18:04:51.8322616Z status: exit code: 1
2019-07-18T18:04:51.8323250Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mir_fat_ptr.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/mir_fat_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/mir_fat_ptr.stage-id.aux" "-A" "unused"
2019-07-18T18:04:51.8323719Z ------------------------------------------
2019-07-18T18:04:51.8323752Z 
2019-07-18T18:04:51.8323971Z ------------------------------------------
2019-07-18T18:04:51.8324187Z stderr:
---
2019-07-18T18:04:51.9857584Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:51.9857635Z +
2019-07-18T18:04:51.9857673Z 
2019-07-18T18:04:51.9857722Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:51.9857976Z Actual stderr saved to /tmp/compiletestWXeOcX/move-arg-2-unique.stderr
2019-07-18T18:04:51.9858041Z To update references, run this command from build directory:
2019-07-18T18:04:51.9858302Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'move-arg-2-unique.rs'
2019-07-18T18:04:51.9858559Z error: 1 errors occurred comparing output.
2019-07-18T18:04:51.9858613Z status: exit code: 1
2019-07-18T18:04:51.9858613Z status: exit code: 1
2019-07-18T18:04:51.9859212Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-arg-2-unique.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/move-arg-2-unique.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/move-arg-2-unique.stage-id.aux" "-A" "unused"
2019-07-18T18:04:51.9859497Z ------------------------------------------
2019-07-18T18:04:51.9859530Z 
2019-07-18T18:04:51.9859734Z ------------------------------------------
2019-07-18T18:04:51.9859777Z stderr:
---
2019-07-18T18:04:51.9959584Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:51.9959632Z +
2019-07-18T18:04:51.9959659Z 
2019-07-18T18:04:51.9959702Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:51.9959963Z Actual stderr saved to /tmp/compiletestWXeOcX/move-arg-3-unique.stderr
2019-07-18T18:04:51.9960018Z To update references, run this command from build directory:
2019-07-18T18:04:51.9960288Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'move-arg-3-unique.rs'
2019-07-18T18:04:51.9960385Z error: 1 errors occurred comparing output.
2019-07-18T18:04:51.9960430Z status: exit code: 1
2019-07-18T18:04:51.9960430Z status: exit code: 1
2019-07-18T18:04:51.9961063Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-arg-3-unique.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/move-arg-3-unique.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/move-arg-3-unique.stage-id.aux" "-A" "unused"
2019-07-18T18:04:51.9961374Z ------------------------------------------
2019-07-18T18:04:51.9961425Z 
2019-07-18T18:04:51.9961632Z ------------------------------------------
2019-07-18T18:04:51.9961687Z stderr:
---
2019-07-18T18:04:52.1251636Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:52.1251685Z +
2019-07-18T18:04:52.1251713Z 
2019-07-18T18:04:52.1251776Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:52.1252047Z Actual stderr saved to /tmp/compiletestWXeOcX/move-undef-primval.stderr
2019-07-18T18:04:52.1252111Z To update references, run this command from build directory:
2019-07-18T18:04:52.1252391Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'move-undef-primval.rs'
2019-07-18T18:04:52.1252474Z error: 1 errors occurred comparing output.
2019-07-18T18:04:52.1252520Z status: exit code: 1
2019-07-18T18:04:52.1252520Z status: exit code: 1
2019-07-18T18:04:52.1253172Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-undef-primval.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/move-undef-primval.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/move-undef-primval.stage-id.aux" "-A" "unused"
2019-07-18T18:04:52.1253501Z ------------------------------------------
2019-07-18T18:04:52.1253537Z 
2019-07-18T18:04:52.1253828Z ------------------------------------------
2019-07-18T18:04:52.1253902Z stderr:
---
2019-07-18T18:04:52.1833509Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:52.1833748Z +
2019-07-18T18:04:52.1833777Z 
2019-07-18T18:04:52.1833828Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:52.1833877Z Actual stderr saved to /tmp/compiletestWXeOcX/mpsc.stderr
2019-07-18T18:04:52.1833949Z To update references, run this command from build directory:
2019-07-18T18:04:52.1834193Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'mpsc.rs'
2019-07-18T18:04:52.1834290Z error: 1 errors occurred comparing output.
2019-07-18T18:04:52.1834334Z status: exit code: 1
2019-07-18T18:04:52.1834334Z status: exit code: 1
2019-07-18T18:04:52.1835689Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mpsc.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/mpsc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/mpsc.stage-id.aux" "-A" "unused"
2019-07-18T18:04:52.1836154Z ------------------------------------------
2019-07-18T18:04:52.1836325Z 
2019-07-18T18:04:52.1836605Z ------------------------------------------
2019-07-18T18:04:52.1836654Z stderr:
---
2019-07-18T18:04:52.2811414Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:52.2811465Z +
2019-07-18T18:04:52.2811492Z 
2019-07-18T18:04:52.2811555Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:52.2811607Z Actual stderr saved to /tmp/compiletestWXeOcX/multi_arg_closure.stderr
2019-07-18T18:04:52.2811661Z To update references, run this command from build directory:
2019-07-18T18:04:52.2811934Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'multi_arg_closure.rs'
2019-07-18T18:04:52.2812017Z error: 1 errors occurred comparing output.
2019-07-18T18:04:52.2812080Z status: exit code: 1
2019-07-18T18:04:52.2812080Z status: exit code: 1
2019-07-18T18:04:52.2812792Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/multi_arg_closure.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/multi_arg_closure.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/multi_arg_closure.stage-id.aux" "-A" "unused"
2019-07-18T18:04:52.2813148Z ------------------------------------------
2019-07-18T18:04:52.2813183Z 
2019-07-18T18:04:52.2813390Z ------------------------------------------
2019-07-18T18:04:52.2813459Z stderr:
---
2019-07-18T18:04:52.3398649Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:52.3398709Z +
2019-07-18T18:04:52.3398744Z 
2019-07-18T18:04:52.3398795Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:52.3398871Z Actual stderr saved to /tmp/compiletestWXeOcX/negative_discriminant.stderr
2019-07-18T18:04:52.3398928Z To update references, run this command from build directory:
2019-07-18T18:04:52.3399240Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'negative_discriminant.rs'
2019-07-18T18:04:52.3399346Z error: 1 errors occurred comparing output.
2019-07-18T18:04:52.3399397Z status: exit code: 1
2019-07-18T18:04:52.3399397Z status: exit code: 1
2019-07-18T18:04:52.3400238Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/negative_discriminant.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/negative_discriminant.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/negative_discriminant.stage-id.aux" "-A" "unused"
2019-07-18T18:04:52.3400638Z ------------------------------------------
2019-07-18T18:04:52.3400679Z 
2019-07-18T18:04:52.3400920Z ------------------------------------------
2019-07-18T18:04:52.3400971Z stderr:
---
2019-07-18T18:04:52.4650997Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:52.4651065Z +
2019-07-18T18:04:52.4651095Z 
2019-07-18T18:04:52.4651143Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:52.4651199Z Actual stderr saved to /tmp/compiletestWXeOcX/non_capture_closure_to_fn_ptr.stderr
2019-07-18T18:04:52.4651272Z To update references, run this command from build directory:
2019-07-18T18:04:52.4651561Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'non_capture_closure_to_fn_ptr.rs'
2019-07-18T18:04:52.4651664Z error: 1 errors occurred comparing output.
2019-07-18T18:04:52.4651713Z status: exit code: 1
2019-07-18T18:04:52.4651713Z status: exit code: 1
2019-07-18T18:04:52.4652478Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/non_capture_closure_to_fn_ptr.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/non_capture_closure_to_fn_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/non_capture_closure_to_fn_ptr.stage-id.aux" "-A" "unused"
2019-07-18T18:04:52.4652855Z ------------------------------------------
2019-07-18T18:04:52.4652909Z 
2019-07-18T18:04:52.4653138Z ------------------------------------------
2019-07-18T18:04:52.4653188Z stderr:
---
2019-07-18T18:04:52.5007939Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:52.5007998Z +
2019-07-18T18:04:52.5008030Z 
2019-07-18T18:04:52.5008082Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:52.5008159Z Actual stderr saved to /tmp/compiletestWXeOcX/observed_local_mut.stderr
2019-07-18T18:04:52.5008217Z To update references, run this command from build directory:
2019-07-18T18:04:52.5008657Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'observed_local_mut.rs'
2019-07-18T18:04:52.5008755Z error: 1 errors occurred comparing output.
2019-07-18T18:04:52.5008811Z status: exit code: 1
2019-07-18T18:04:52.5008811Z status: exit code: 1
2019-07-18T18:04:52.5009616Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/observed_local_mut.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/observed_local_mut.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletestWXeOcX/observed_local_mut.stage-id.aux" "-A" "unused"
2019-07-18T18:04:52.5009978Z ------------------------------------------
2019-07-18T18:04:52.5010014Z 
2019-07-18T18:04:52.5010217Z ------------------------------------------
2019-07-18T18:04:52.5010262Z stderr:
---
2019-07-18T18:04:52.6170847Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:52.6170911Z +
2019-07-18T18:04:52.6170938Z 
2019-07-18T18:04:52.6170984Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:52.6171038Z Actual stderr saved to /tmp/compiletestWXeOcX/option_box_transmute_ptr.stderr
2019-07-18T18:04:52.6171109Z To update references, run this command from build directory:
2019-07-18T18:04:52.6171382Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'option_box_transmute_ptr.rs'
2019-07-18T18:04:52.6171587Z error: 1 errors occurred comparing output.
2019-07-18T18:04:52.6171653Z status: exit code: 1
2019-07-18T18:04:52.6171653Z status: exit code: 1
2019-07-18T18:04:52.6172345Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/option_box_transmute_ptr.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/option_box_transmute_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/option_box_transmute_ptr.stage-id.aux" "-A" "unused"
2019-07-18T18:04:52.6172669Z ------------------------------------------
2019-07-18T18:04:52.6172705Z 
2019-07-18T18:04:52.6172930Z ------------------------------------------
2019-07-18T18:04:52.6173256Z stderr:
---
2019-07-18T18:04:52.6575419Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:52.6575497Z +
2019-07-18T18:04:52.6575529Z 
2019-07-18T18:04:52.6575578Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:52.6575634Z Actual stderr saved to /tmp/compiletestWXeOcX/option_eq.stderr
2019-07-18T18:04:52.6575873Z To update references, run this command from build directory:
2019-07-18T18:04:52.6576193Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'option_eq.rs'
2019-07-18T18:04:52.6576297Z error: 1 errors occurred comparing output.
2019-07-18T18:04:52.6576345Z status: exit code: 1
2019-07-18T18:04:52.6576345Z status: exit code: 1
2019-07-18T18:04:52.6576987Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/option_eq.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/option_eq.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/option_eq.stage-id.aux" "-A" "unused"
2019-07-18T18:04:52.6577314Z ------------------------------------------
2019-07-18T18:04:52.6577445Z 
2019-07-18T18:04:52.6577713Z ------------------------------------------
2019-07-18T18:04:52.6577763Z stderr:
---
2019-07-18T18:04:52.7662404Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:52.7662563Z +
2019-07-18T18:04:52.7662680Z 
2019-07-18T18:04:52.7662767Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:52.7663162Z Actual stderr saved to /tmp/compiletestWXeOcX/overloaded-calls-simple.stderr
2019-07-18T18:04:52.7663224Z To update references, run this command from build directory:
2019-07-18T18:04:52.7663663Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'overloaded-calls-simple.rs'
2019-07-18T18:04:52.7663953Z error: 1 errors occurred comparing output.
2019-07-18T18:04:52.7664035Z status: exit code: 1
2019-07-18T18:04:52.7664035Z status: exit code: 1
2019-07-18T18:04:52.7711414Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/overloaded-calls-simple.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/overloaded-calls-simple.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/overloaded-calls-simple.stage-id.aux" "-A" "unused"
2019-07-18T18:04:52.7712395Z ------------------------------------------
2019-07-18T18:04:52.7712561Z 
2019-07-18T18:04:52.7712900Z ------------------------------------------
2019-07-18T18:04:52.7712947Z stderr:
---
2019-07-18T18:04:52.8156162Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:52.8156226Z +
2019-07-18T18:04:52.8156316Z 
2019-07-18T18:04:52.8156477Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:52.8156563Z Actual stderr saved to /tmp/compiletestWXeOcX/packed_static.stderr
2019-07-18T18:04:52.8156618Z To update references, run this command from build directory:
2019-07-18T18:04:52.8156920Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'packed_static.rs'
2019-07-18T18:04:52.8157024Z error: 1 errors occurred comparing output.
2019-07-18T18:04:52.8157072Z status: exit code: 1
2019-07-18T18:04:52.8157072Z status: exit code: 1
2019-07-18T18:04:52.8157753Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/packed_static.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/packed_static.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/packed_static.stage-id.aux" "-A" "unused"
2019-07-18T18:04:52.8158180Z ------------------------------------------
2019-07-18T18:04:52.8158217Z 
2019-07-18T18:04:52.8158436Z ------------------------------------------
2019-07-18T18:04:52.8158484Z stderr:
---
2019-07-18T18:04:52.9736937Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:52.9736987Z +
2019-07-18T18:04:52.9737016Z 
2019-07-18T18:04:52.9737080Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:52.9737134Z Actual stderr saved to /tmp/compiletestWXeOcX/packed_struct.stderr
2019-07-18T18:04:52.9737188Z To update references, run this command from build directory:
2019-07-18T18:04:52.9737463Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'packed_struct.rs'
2019-07-18T18:04:52.9737548Z error: 1 errors occurred comparing output.
2019-07-18T18:04:52.9737597Z status: exit code: 1
2019-07-18T18:04:52.9737597Z status: exit code: 1
2019-07-18T18:04:52.9738431Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/packed_struct.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/packed_struct.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/packed_struct.stage-id.aux" "-A" "unused"
2019-07-18T18:04:52.9738993Z ------------------------------------------
2019-07-18T18:04:52.9739026Z 
2019-07-18T18:04:52.9739225Z ------------------------------------------
2019-07-18T18:04:52.9739285Z stderr:
---
2019-07-18T18:04:53.0328633Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:53.0328709Z +
2019-07-18T18:04:53.0328806Z 
2019-07-18T18:04:53.0328855Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:53.0328907Z Actual stderr saved to /tmp/compiletestWXeOcX/pointers.stderr
2019-07-18T18:04:53.0328999Z To update references, run this command from build directory:
2019-07-18T18:04:53.0329700Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'pointers.rs'
2019-07-18T18:04:53.0353196Z thread '[ui] run-pass/pointers.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-18T18:04:53.0353344Z error: 1 errors occurred comparing output.
2019-07-18T18:04:53.0353391Z status: exit code: 1
2019-07-18T18:04:53.0353391Z status: exit code: 1
2019-07-18T18:04:53.0354431Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/pointers.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/pointers.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/pointers.stage-id.aux" "-A" "unused"
2019-07-18T18:04:53.0355352Z ------------------------------------------
2019-07-18T18:04:53.0355402Z 
2019-07-18T18:04:53.0355640Z ------------------------------------------
2019-07-18T18:04:53.0355690Z stderr:
---
2019-07-18T18:04:53.1655693Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:53.1655764Z +
2019-07-18T18:04:53.1655794Z 
2019-07-18T18:04:53.1655843Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:53.1655896Z Actual stderr saved to /tmp/compiletestWXeOcX/products.stderr
2019-07-18T18:04:53.1655969Z To update references, run this command from build directory:
2019-07-18T18:04:53.1656226Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'products.rs'
2019-07-18T18:04:53.1656309Z error: 1 errors occurred comparing output.
2019-07-18T18:04:53.1656486Z status: exit code: 1
2019-07-18T18:04:53.1656486Z status: exit code: 1
2019-07-18T18:04:53.1657150Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/products.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/products.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/products.stage-id.aux" "-A" "unused"
2019-07-18T18:04:53.1657483Z ------------------------------------------
2019-07-18T18:04:53.1657519Z 
2019-07-18T18:04:53.1657754Z ------------------------------------------
2019-07-18T18:04:53.1657801Z stderr:
---
2019-07-18T18:04:53.1767396Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:53.1767448Z +
2019-07-18T18:04:53.1767477Z 
2019-07-18T18:04:53.1767544Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:53.1767600Z Actual stderr saved to /tmp/compiletestWXeOcX/ptr_arith_offset.stderr
2019-07-18T18:04:53.1767656Z To update references, run this command from build directory:
2019-07-18T18:04:53.1767955Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'ptr_arith_offset.rs'
2019-07-18T18:04:53.1768134Z error: 1 errors occurred comparing output.
2019-07-18T18:04:53.1768193Z status: exit code: 1
2019-07-18T18:04:53.1768193Z status: exit code: 1
2019-07-18T18:04:53.1769005Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_arith_offset.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/ptr_arith_offset.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/ptr_arith_offset.stage-id.aux" "-A" "unused"
2019-07-18T18:04:53.1769308Z ------------------------------------------
2019-07-18T18:04:53.1769341Z 
2019-07-18T18:04:53.1769543Z ------------------------------------------
2019-07-18T18:04:53.1769588Z stderr:
---
2019-07-18T18:04:53.3155786Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:53.3155840Z +
2019-07-18T18:04:53.3173116Z 
2019-07-18T18:04:53.3173240Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:53.3173296Z Actual stderr saved to /tmp/compiletestWXeOcX/ptr_arith_offset_overflow.stderr
2019-07-18T18:04:53.3173346Z To update references, run this command from build directory:
2019-07-18T18:04:53.3173786Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'ptr_arith_offset_overflow.rs'
2019-07-18T18:04:53.3173872Z error: 1 errors occurred comparing output.
2019-07-18T18:04:53.3174087Z status: exit code: 1
2019-07-18T18:04:53.3174087Z status: exit code: 1
2019-07-18T18:04:53.3175315Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_arith_offset_overflow.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/ptr_arith_offset_overflow.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/ptr_arith_offset_overflow.stage-id.aux" "-A" "unused"
2019-07-18T18:04:53.3175702Z ------------------------------------------
2019-07-18T18:04:53.3175740Z 
2019-07-18T18:04:53.3175958Z ------------------------------------------
2019-07-18T18:04:53.3176020Z stderr:
---
2019-07-18T18:04:53.3488660Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:53.3488884Z +
2019-07-18T18:04:53.3488912Z 
2019-07-18T18:04:53.3488975Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:53.3489029Z Actual stderr saved to /tmp/compiletestWXeOcX/ptr_int_casts.stderr
2019-07-18T18:04:53.3489080Z To update references, run this command from build directory:
2019-07-18T18:04:53.3489453Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'ptr_int_casts.rs'
2019-07-18T18:04:53.3489546Z error: 1 errors occurred comparing output.
2019-07-18T18:04:53.3489592Z status: exit code: 1
2019-07-18T18:04:53.3489592Z status: exit code: 1
2019-07-18T18:04:53.3490228Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_int_casts.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/ptr_int_casts.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/ptr_int_casts.stage-id.aux" "-A" "unused"
2019-07-18T18:04:53.3490533Z ------------------------------------------
2019-07-18T18:04:53.3490568Z 
2019-07-18T18:04:53.3490775Z ------------------------------------------
2019-07-18T18:04:53.3490822Z stderr:
---
2019-07-18T18:04:53.5016097Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:53.5016151Z +
2019-07-18T18:04:53.5016199Z 
2019-07-18T18:04:53.5016250Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:53.5016305Z Actual stderr saved to /tmp/compiletestWXeOcX/ptr_offset.stderr
2019-07-18T18:04:53.5016444Z To update references, run this command from build directory:
2019-07-18T18:04:53.5016771Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'ptr_offset.rs'
2019-07-18T18:04:53.5016860Z error: 1 errors occurred comparing output.
2019-07-18T18:04:53.5016925Z status: exit code: 1
2019-07-18T18:04:53.5016925Z status: exit code: 1
2019-07-18T18:04:53.5017581Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_offset.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/ptr_offset.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/ptr_offset.stage-id.aux" "-A" "unused"
2019-07-18T18:04:53.5017920Z ------------------------------------------
2019-07-18T18:04:53.5017958Z 
2019-07-18T18:04:53.5018346Z ------------------------------------------
2019-07-18T18:04:53.5018418Z stderr:
---
2019-07-18T18:04:53.5108326Z +
2019-07-18T18:04:53.5109006Z thread '[ui] run-pass/ptr_int_ops.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-18T18:04:53.5110101Z 
2019-07-18T18:04:53.5110346Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:53.5110499Z Actual stderr saved to /tmp/compiletestWXeOcX/ptr_int_ops.stderr
2019-07-18T18:04:53.5110668Z To update references, run this command from build directory:
2019-07-18T18:04:53.5111123Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'ptr_int_ops.rs'
2019-07-18T18:04:53.5111433Z error: 1 errors occurred comparing output.
2019-07-18T18:04:53.5111591Z status: exit code: 1
2019-07-18T18:04:53.5111591Z status: exit code: 1
2019-07-18T18:04:53.5112362Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_int_ops.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/ptr_int_ops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/ptr_int_ops.stage-id.aux" "-A" "unused"
2019-07-18T18:04:53.5112958Z ------------------------------------------
2019-07-18T18:04:53.5113113Z 
2019-07-18T18:04:53.5113443Z ------------------------------------------
2019-07-18T18:04:53.5113627Z stderr:
---
2019-07-18T18:04:53.6786982Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:53.6787054Z +
2019-07-18T18:04:53.6789094Z 
2019-07-18T18:04:53.6790524Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:53.6792508Z Actual stderr saved to /tmp/compiletestWXeOcX/raw.stderr
2019-07-18T18:04:53.6796104Z To update references, run this command from build directory:
2019-07-18T18:04:53.6799009Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'raw.rs'
2019-07-18T18:04:53.6813961Z error: 1 errors occurred comparing output.
2019-07-18T18:04:53.6814220Z status: exit code: 1
2019-07-18T18:04:53.6814220Z status: exit code: 1
2019-07-18T18:04:53.6815701Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/raw.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/raw.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/raw.stage-id.aux" "-A" "unused"
2019-07-18T18:04:53.6816429Z ------------------------------------------
2019-07-18T18:04:53.6816649Z 
2019-07-18T18:04:53.6817032Z ------------------------------------------
2019-07-18T18:04:53.6817219Z stderr:
---
2019-07-18T18:04:53.7772653Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:53.7772712Z +
2019-07-18T18:04:53.7772762Z 
2019-07-18T18:04:53.7772809Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:53.7772861Z Actual stderr saved to /tmp/compiletestWXeOcX/rc.stderr
2019-07-18T18:04:53.7772931Z To update references, run this command from build directory:
2019-07-18T18:04:53.7773187Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'rc.rs'
2019-07-18T18:04:53.7773268Z error: 1 errors occurred comparing output.
2019-07-18T18:04:53.7773334Z status: exit code: 1
2019-07-18T18:04:53.7773334Z status: exit code: 1
2019-07-18T18:04:53.7773924Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rc.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/rc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/rc.stage-id.aux" "-A" "unused"
2019-07-18T18:04:53.7774256Z ------------------------------------------
2019-07-18T18:04:53.7774292Z 
2019-07-18T18:04:53.7774509Z ------------------------------------------
2019-07-18T18:04:53.7774575Z stderr:
---
2019-07-18T18:04:53.8419799Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:53.8419851Z +
2019-07-18T18:04:53.8419880Z 
2019-07-18T18:04:53.8419947Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:53.8420001Z Actual stderr saved to /tmp/compiletestWXeOcX/recursive_static.stderr
2019-07-18T18:04:53.8420053Z To update references, run this command from build directory:
2019-07-18T18:04:53.8420343Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'recursive_static.rs'
2019-07-18T18:04:53.8420426Z error: 1 errors occurred comparing output.
2019-07-18T18:04:53.8420472Z status: exit code: 1
2019-07-18T18:04:53.8420472Z status: exit code: 1
2019-07-18T18:04:53.8421118Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/recursive_static.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/recursive_static.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/recursive_static.stage-id.aux" "-A" "unused"
2019-07-18T18:04:53.8421449Z ------------------------------------------
2019-07-18T18:04:53.8421484Z 
2019-07-18T18:04:53.8421702Z ------------------------------------------
2019-07-18T18:04:53.8421766Z stderr:
---
2019-07-18T18:04:53.9090677Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:53.9090738Z +
2019-07-18T18:04:53.9090774Z 
2019-07-18T18:04:53.9090818Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:53.9091043Z Actual stderr saved to /tmp/compiletestWXeOcX/ref-invalid-ptr.stderr
2019-07-18T18:04:53.9091110Z To update references, run this command from build directory:
2019-07-18T18:04:53.9091341Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'ref-invalid-ptr.rs'
2019-07-18T18:04:53.9091418Z error: 1 errors occurred comparing output.
2019-07-18T18:04:53.9091479Z status: exit code: 1
2019-07-18T18:04:53.9091479Z status: exit code: 1
2019-07-18T18:04:53.9092094Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ref-invalid-ptr.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/ref-invalid-ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletestWXeOcX/ref-invalid-ptr.stage-id.aux" "-A" "unused"
2019-07-18T18:04:53.9092394Z ------------------------------------------
2019-07-18T18:04:53.9092428Z 
2019-07-18T18:04:53.9092637Z ------------------------------------------
2019-07-18T18:04:53.9092680Z stderr:
---
2019-07-18T18:04:53.9890456Z +
2019-07-18T18:04:53.9890827Z thread '[ui] run-pass/refcell.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-18T18:04:53.9890871Z 
2019-07-18T18:04:53.9890920Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:53.9890973Z Actual stderr saved to /tmp/compiletestWXeOcX/refcell.stderr
2019-07-18T18:04:53.9891043Z To update references, run this command from build directory:
2019-07-18T18:04:53.9891305Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'refcell.rs'
2019-07-18T18:04:53.9891517Z error: 1 errors occurred comparing output.
2019-07-18T18:04:53.9891565Z status: exit code: 1
2019-07-18T18:04:53.9891565Z status: exit code: 1
2019-07-18T18:04:53.9892200Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/refcell.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/refcell.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/refcell.stage-id.aux" "-A" "unused"
2019-07-18T18:04:53.9892540Z ------------------------------------------
2019-07-18T18:04:53.9892575Z 
2019-07-18T18:04:53.9892859Z ------------------------------------------
2019-07-18T18:04:53.9892907Z stderr:
---
2019-07-18T18:04:54.0498269Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:54.0498338Z +
2019-07-18T18:04:54.0498528Z 
2019-07-18T18:04:54.0498576Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:54.0498986Z Actual stderr saved to /tmp/compiletestWXeOcX/regions-lifetime-nonfree-late-bound.stderr
2019-07-18T18:04:54.0499057Z To update references, run this command from build directory:
2019-07-18T18:04:54.0499304Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'regions-lifetime-nonfree-late-bound.rs'
2019-07-18T18:04:54.0499397Z error: 1 errors occurred comparing output.
2019-07-18T18:04:54.0499440Z status: exit code: 1
2019-07-18T18:04:54.0499440Z status: exit code: 1
2019-07-18T18:04:54.0500073Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/regions-lifetime-nonfree-late-bound.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/regions-lifetime-nonfree-late-bound.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/regions-lifetime-nonfree-late-bound.stage-id.aux" "-A" "unused"
2019-07-18T18:04:54.0500380Z ------------------------------------------
2019-07-18T18:04:54.0500412Z 
2019-07-18T18:04:54.0500623Z ------------------------------------------
2019-07-18T18:04:54.0500666Z stderr:
---
2019-07-18T18:04:54.1408170Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:54.1408225Z +
2019-07-18T18:04:54.1415352Z 
2019-07-18T18:04:54.1415457Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:54.1415890Z Actual stderr saved to /tmp/compiletestWXeOcX/regions-mock-trans.stderr
2019-07-18T18:04:54.1415974Z To update references, run this command from build directory:
2019-07-18T18:04:54.1416242Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'regions-mock-trans.rs'
2019-07-18T18:04:54.1416360Z error: 1 errors occurred comparing output.
2019-07-18T18:04:54.1416408Z status: exit code: 1
2019-07-18T18:04:54.1416408Z status: exit code: 1
2019-07-18T18:04:54.1417097Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/regions-mock-trans.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/regions-mock-trans.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/regions-mock-trans.stage-id.aux" "-A" "unused"
2019-07-18T18:04:54.1417434Z ------------------------------------------
2019-07-18T18:04:54.1417470Z 
2019-07-18T18:04:54.1417703Z ------------------------------------------
2019-07-18T18:04:54.1417751Z stderr:
---
2019-07-18T18:04:54.2022228Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:54.2022275Z +
2019-07-18T18:04:54.2022303Z 
2019-07-18T18:04:54.2022363Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:54.2022413Z Actual stderr saved to /tmp/compiletestWXeOcX/rfc1623.stderr
2019-07-18T18:04:54.2022463Z To update references, run this command from build directory:
2019-07-18T18:04:54.2022735Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'rfc1623.rs'
2019-07-18T18:04:54.2022828Z error: 1 errors occurred comparing output.
2019-07-18T18:04:54.2022890Z status: exit code: 1
2019-07-18T18:04:54.2022890Z status: exit code: 1
2019-07-18T18:04:54.2023498Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rfc1623.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/rfc1623.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/rfc1623.stage-id.aux" "-A" "unused"
2019-07-18T18:04:54.2023807Z ------------------------------------------
2019-07-18T18:04:54.2023840Z 
2019-07-18T18:04:54.2024055Z ------------------------------------------
2019-07-18T18:04:54.2024116Z stderr:
---
2019-07-18T18:04:54.3243324Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:54.3243388Z +
2019-07-18T18:04:54.3249611Z 
2019-07-18T18:04:54.3249707Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:54.3250120Z Actual stderr saved to /tmp/compiletestWXeOcX/rust-lang-org.stderr
2019-07-18T18:04:54.3250179Z To update references, run this command from build directory:
2019-07-18T18:04:54.3250467Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'rust-lang-org.rs'
2019-07-18T18:04:54.3250566Z error: 1 errors occurred comparing output.
2019-07-18T18:04:54.3250612Z status: exit code: 1
2019-07-18T18:04:54.3250612Z status: exit code: 1
2019-07-18T18:04:54.3251329Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rust-lang-org.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/rust-lang-org.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/rust-lang-org.stage-id.aux" "-A" "unused"
2019-07-18T18:04:54.3251655Z ------------------------------------------
2019-07-18T18:04:54.3251705Z 
2019-07-18T18:04:54.3251919Z ------------------------------------------
2019-07-18T18:04:54.3251975Z stderr:
---
2019-07-18T18:04:54.3732429Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:54.3732490Z +
2019-07-18T18:04:54.3732519Z 
2019-07-18T18:04:54.3732573Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:54.3732866Z Actual stderr saved to /tmp/compiletestWXeOcX/send-is-not-static-par-for.stderr
2019-07-18T18:04:54.3732924Z To update references, run this command from build directory:
2019-07-18T18:04:54.3733204Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'send-is-not-static-par-for.rs'
2019-07-18T18:04:54.3733305Z error: 1 errors occurred comparing output.
2019-07-18T18:04:54.3733353Z status: exit code: 1
2019-07-18T18:04:54.3733353Z status: exit code: 1
2019-07-18T18:04:54.3734066Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/send-is-not-static-par-for.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/send-is-not-static-par-for.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/send-is-not-static-par-for.stage-id.aux" "-A" "unused"
2019-07-18T18:04:54.3734600Z ------------------------------------------
2019-07-18T18:04:54.3734636Z 
2019-07-18T18:04:54.3734857Z ------------------------------------------
2019-07-18T18:04:54.3734907Z stderr:
---
2019-07-18T18:04:54.4776384Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:54.4776442Z +
2019-07-18T18:04:54.4776474Z 
2019-07-18T18:04:54.4776526Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:54.4776829Z Actual stderr saved to /tmp/compiletestWXeOcX/sendable-class.stderr
2019-07-18T18:04:54.4776889Z To update references, run this command from build directory:
2019-07-18T18:04:54.4777172Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'sendable-class.rs'
2019-07-18T18:04:54.4777279Z error: 1 errors occurred comparing output.
2019-07-18T18:04:54.4777329Z status: exit code: 1
2019-07-18T18:04:54.4777329Z status: exit code: 1
2019-07-18T18:04:54.4778176Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sendable-class.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/sendable-class.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/sendable-class.stage-id.aux" "-A" "unused"
2019-07-18T18:04:54.4778489Z ------------------------------------------
2019-07-18T18:04:54.4778523Z 
2019-07-18T18:04:54.4778721Z ------------------------------------------
2019-07-18T18:04:54.4778765Z stderr:
---
2019-07-18T18:04:54.5698663Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:54.5698710Z +
2019-07-18T18:04:54.5698743Z 
2019-07-18T18:04:54.5698807Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:54.5699056Z Actual stderr saved to /tmp/compiletestWXeOcX/simd-intrinsic-generic-elements.stderr
2019-07-18T18:04:54.5699111Z To update references, run this command from build directory:
2019-07-18T18:04:54.5699385Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'simd-intrinsic-generic-elements.rs'
2019-07-18T18:04:54.5699464Z error: 1 errors occurred comparing output.
2019-07-18T18:04:54.5699526Z status: exit code: 1
2019-07-18T18:04:54.5699526Z status: exit code: 1
2019-07-18T18:04:54.5700201Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/simd-intrinsic-generic-elements.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/simd-intrinsic-generic-elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/simd-intrinsic-generic-elements.stage-id.aux" "-A" "unused"
2019-07-18T18:04:54.5700519Z ------------------------------------------
2019-07-18T18:04:54.5700552Z 
2019-07-18T18:04:54.5700754Z ------------------------------------------
2019-07-18T18:04:54.5700816Z stderr:
---
2019-07-18T18:04:54.7087549Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:54.7087620Z +
2019-07-18T18:04:54.7087651Z 
2019-07-18T18:04:54.7087701Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:54.7087758Z Actual stderr saved to /tmp/compiletestWXeOcX/small_enum_size_bug.stderr
2019-07-18T18:04:54.7087833Z To update references, run this command from build directory:
2019-07-18T18:04:54.7088115Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'small_enum_size_bug.rs'
2019-07-18T18:04:54.7088381Z error: 1 errors occurred comparing output.
2019-07-18T18:04:54.7088439Z status: exit code: 1
2019-07-18T18:04:54.7088439Z status: exit code: 1
2019-07-18T18:04:54.7089125Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/small_enum_size_bug.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/small_enum_size_bug.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/small_enum_size_bug.stage-id.aux" "-A" "unused"
2019-07-18T18:04:54.7089443Z ------------------------------------------
2019-07-18T18:04:54.7089495Z 
2019-07-18T18:04:54.7089701Z ------------------------------------------
2019-07-18T18:04:54.7089747Z stderr:
---
2019-07-18T18:04:54.8648655Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:54.8648712Z +
2019-07-18T18:04:54.8648760Z 
2019-07-18T18:04:54.8648811Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:54.8648867Z Actual stderr saved to /tmp/compiletestWXeOcX/specialization.stderr
2019-07-18T18:04:54.8649101Z To update references, run this command from build directory:
2019-07-18T18:04:54.8649359Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'specialization.rs'
2019-07-18T18:04:54.8649445Z error: 1 errors occurred comparing output.
2019-07-18T18:04:54.8649509Z status: exit code: 1
2019-07-18T18:04:54.8649509Z status: exit code: 1
2019-07-18T18:04:54.8650556Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/specialization.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/specialization.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/specialization.stage-id.aux" "-A" "unused"
2019-07-18T18:04:54.8651067Z ------------------------------------------
2019-07-18T18:04:54.8651113Z 
2019-07-18T18:04:54.8651365Z ------------------------------------------
2019-07-18T18:04:54.8651415Z stderr:
---
2019-07-18T18:04:54.8693554Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:54.8693624Z +
2019-07-18T18:04:54.8693653Z 
2019-07-18T18:04:54.8693700Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:54.8693751Z Actual stderr saved to /tmp/compiletestWXeOcX/slices.stderr
2019-07-18T18:04:54.8693819Z To update references, run this command from build directory:
2019-07-18T18:04:54.8694104Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'slices.rs'
2019-07-18T18:04:54.8694186Z error: 1 errors occurred comparing output.
2019-07-18T18:04:54.8694247Z status: exit code: 1
2019-07-18T18:04:54.8694247Z status: exit code: 1
2019-07-18T18:04:54.8695440Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/slices.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/slices.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/slices.stage-id.aux" "-A" "unused"
2019-07-18T18:04:54.8695774Z ------------------------------------------
2019-07-18T18:04:54.8695810Z 
2019-07-18T18:04:54.8696041Z ------------------------------------------
2019-07-18T18:04:54.8696090Z stderr:
---
2019-07-18T18:04:55.0220295Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:55.0220343Z +
2019-07-18T18:04:55.0220370Z 
2019-07-18T18:04:55.0220433Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:55.0220671Z Actual stderr saved to /tmp/compiletestWXeOcX/stacked-borrows/2phase.stderr
2019-07-18T18:04:55.0220727Z To update references, run this command from build directory:
2019-07-18T18:04:55.0220989Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'stacked-borrows/2phase.rs'
2019-07-18T18:04:55.0221084Z error: 1 errors occurred comparing output.
2019-07-18T18:04:55.0221130Z status: exit code: 1
2019-07-18T18:04:55.0221130Z status: exit code: 1
2019-07-18T18:04:55.0221773Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/2phase.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/stacked-borrows/2phase.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/stacked-borrows/2phase.stage-id.aux" "-A" "unused"
2019-07-18T18:04:55.0222074Z ------------------------------------------
2019-07-18T18:04:55.0222107Z 
2019-07-18T18:04:55.0222307Z ------------------------------------------
2019-07-18T18:04:55.0222369Z stderr:
---
2019-07-18T18:04:55.0433411Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:55.0433481Z +
2019-07-18T18:04:55.0433520Z 
2019-07-18T18:04:55.0433567Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:55.0433840Z Actual stderr saved to /tmp/compiletestWXeOcX/stacked-borrows/interior_mutability.stderr
2019-07-18T18:04:55.0433917Z To update references, run this command from build directory:
2019-07-18T18:04:55.0434193Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'stacked-borrows/interior_mutability.rs'
2019-07-18T18:04:55.0434294Z error: 1 errors occurred comparing output.
2019-07-18T18:04:55.0434340Z status: exit code: 1
2019-07-18T18:04:55.0434340Z status: exit code: 1
2019-07-18T18:04:55.0435556Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/interior_mutability.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/stacked-borrows/interior_mutability.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/stacked-borrows/interior_mutability.stage-id.aux" "-A" "unused"
2019-07-18T18:04:55.0435947Z ------------------------------------------
2019-07-18T18:04:55.0436002Z 
2019-07-18T18:04:55.0436222Z ------------------------------------------
2019-07-18T18:04:55.0436272Z stderr:
---
2019-07-18T18:04:55.1970636Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:55.1970824Z +
2019-07-18T18:04:55.1970939Z 
2019-07-18T18:04:55.1971101Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:55.1971244Z Actual stderr saved to /tmp/compiletestWXeOcX/static_memory_modification.stderr
2019-07-18T18:04:55.1971397Z To update references, run this command from build directory:
2019-07-18T18:04:55.1971777Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'static_memory_modification.rs'
2019-07-18T18:04:55.1972082Z error: 1 errors occurred comparing output.
2019-07-18T18:04:55.1972215Z status: exit code: 1
2019-07-18T18:04:55.1972215Z status: exit code: 1
2019-07-18T18:04:55.1973058Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/static_memory_modification.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/static_memory_modification.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/static_memory_modification.stage-id.aux" "-A" "unused"
2019-07-18T18:04:55.1973683Z ------------------------------------------
2019-07-18T18:04:55.1973852Z 
2019-07-18T18:04:55.1974328Z ------------------------------------------
2019-07-18T18:04:55.1975037Z stderr:
---
2019-07-18T18:04:55.2691106Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:55.2691159Z +
2019-07-18T18:04:55.2691207Z 
2019-07-18T18:04:55.2691256Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:55.2691525Z Actual stderr saved to /tmp/compiletestWXeOcX/stacked-borrows/stacked-borrows.stderr
2019-07-18T18:04:55.2691582Z To update references, run this command from build directory:
2019-07-18T18:04:55.2691869Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'stacked-borrows/stacked-borrows.rs'
2019-07-18T18:04:55.2691953Z error: 1 errors occurred comparing output.
2019-07-18T18:04:55.2692016Z status: exit code: 1
2019-07-18T18:04:55.2692016Z status: exit code: 1
2019-07-18T18:04:55.2692772Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/stacked-borrows.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/stacked-borrows/stacked-borrows.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/stacked-borrows/stacked-borrows.stage-id.aux" "-A" "unused"
2019-07-18T18:04:55.2693138Z ------------------------------------------
2019-07-18T18:04:55.2693174Z 
2019-07-18T18:04:55.2693408Z ------------------------------------------
2019-07-18T18:04:55.2693456Z stderr:
---
2019-07-18T18:04:55.3817123Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:55.3817177Z +
2019-07-18T18:04:55.3817230Z 
2019-07-18T18:04:55.3817281Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:55.3817337Z Actual stderr saved to /tmp/compiletestWXeOcX/static_mut.stderr
2019-07-18T18:04:55.3817412Z To update references, run this command from build directory:
2019-07-18T18:04:55.3817691Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'static_mut.rs'
2019-07-18T18:04:55.3817834Z error: 1 errors occurred comparing output.
2019-07-18T18:04:55.3817902Z status: exit code: 1
2019-07-18T18:04:55.3817902Z status: exit code: 1
2019-07-18T18:04:55.3818831Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/static_mut.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/static_mut.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/static_mut.stage-id.aux" "-A" "unused"
2019-07-18T18:04:55.3820217Z ------------------------------------------
2019-07-18T18:04:55.3820254Z 
2019-07-18T18:04:55.3820483Z ------------------------------------------
2019-07-18T18:04:55.3820530Z stderr:
---
2019-07-18T18:04:55.4526635Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:55.4526689Z +
2019-07-18T18:04:55.4526719Z 
2019-07-18T18:04:55.4526786Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:55.4526844Z Actual stderr saved to /tmp/compiletestWXeOcX/strings.stderr
2019-07-18T18:04:55.4526900Z To update references, run this command from build directory:
2019-07-18T18:04:55.4527193Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'strings.rs'
2019-07-18T18:04:55.4527282Z error: 1 errors occurred comparing output.
2019-07-18T18:04:55.4527342Z status: exit code: 1
2019-07-18T18:04:55.4527342Z status: exit code: 1
2019-07-18T18:04:55.4528252Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/strings.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/strings.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/strings.stage-id.aux" "-A" "unused"
2019-07-18T18:04:55.4528587Z ------------------------------------------
2019-07-18T18:04:55.4528620Z 
2019-07-18T18:04:55.4528814Z ------------------------------------------
2019-07-18T18:04:55.4528872Z stderr:
---
2019-07-18T18:04:55.5534410Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:55.5534460Z +
2019-07-18T18:04:55.5534488Z 
2019-07-18T18:04:55.5534555Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:55.5534993Z Actual stderr saved to /tmp/compiletestWXeOcX/subslice_array.stderr
2019-07-18T18:04:55.5535053Z To update references, run this command from build directory:
2019-07-18T18:04:55.5535567Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'subslice_array.rs'
2019-07-18T18:04:55.5535668Z error: 1 errors occurred comparing output.
2019-07-18T18:04:55.5535721Z status: exit code: 1
2019-07-18T18:04:55.5535721Z status: exit code: 1
2019-07-18T18:04:55.5536420Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/subslice_array.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/subslice_array.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/subslice_array.stage-id.aux" "-A" "unused"
2019-07-18T18:04:55.5536747Z ------------------------------------------
2019-07-18T18:04:55.5536784Z 
2019-07-18T18:04:55.5536998Z ------------------------------------------
2019-07-18T18:04:55.5537163Z stderr:
---
2019-07-18T18:04:55.6611347Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:55.6611399Z +
2019-07-18T18:04:55.6611445Z 
2019-07-18T18:04:55.6611494Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:55.6611548Z Actual stderr saved to /tmp/compiletestWXeOcX/sums.stderr
2019-07-18T18:04:55.6611621Z To update references, run this command from build directory:
2019-07-18T18:04:55.6611887Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'sums.rs'
2019-07-18T18:04:55.6612056Z error: 1 errors occurred comparing output.
2019-07-18T18:04:55.6612131Z status: exit code: 1
2019-07-18T18:04:55.6612131Z status: exit code: 1
2019-07-18T18:04:55.6612765Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sums.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/sums.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/sums.stage-id.aux" "-A" "unused"
2019-07-18T18:04:55.6613097Z ------------------------------------------
2019-07-18T18:04:55.6613133Z 
2019-07-18T18:04:55.6613360Z ------------------------------------------
2019-07-18T18:04:55.6613424Z stderr:
---
2019-07-18T18:04:55.6978320Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:55.6978375Z +
2019-07-18T18:04:55.6978407Z 
2019-07-18T18:04:55.6978475Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:55.6978530Z Actual stderr saved to /tmp/compiletestWXeOcX/sync.stderr
2019-07-18T18:04:55.6978585Z To update references, run this command from build directory:
2019-07-18T18:04:55.6978901Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'sync.rs'
2019-07-18T18:04:55.6979149Z error: 1 errors occurred comparing output.
2019-07-18T18:04:55.6979210Z status: exit code: 1
2019-07-18T18:04:55.6979210Z status: exit code: 1
2019-07-18T18:04:55.6980092Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sync.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/sync.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/sync.stage-id.aux" "-A" "unused"
2019-07-18T18:04:55.6980407Z ------------------------------------------
2019-07-18T18:04:55.6980444Z 
2019-07-18T18:04:55.6980814Z ------------------------------------------
2019-07-18T18:04:55.6981051Z stderr:
---
2019-07-18T18:04:55.8020553Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:55.8020607Z +
2019-07-18T18:04:55.8020655Z 
2019-07-18T18:04:55.8020707Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:55.8021088Z Actual stderr saved to /tmp/compiletestWXeOcX/tag-align-dyn-u64.stderr
2019-07-18T18:04:55.8021178Z To update references, run this command from build directory:
2019-07-18T18:04:55.8021486Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'tag-align-dyn-u64.rs'
2019-07-18T18:04:55.8021577Z error: 1 errors occurred comparing output.
2019-07-18T18:04:55.8021646Z status: exit code: 1
2019-07-18T18:04:55.8021646Z status: exit code: 1
2019-07-18T18:04:55.8022318Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tag-align-dyn-u64.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/tag-align-dyn-u64.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/tag-align-dyn-u64.stage-id.aux" "-A" "unused"
2019-07-18T18:04:55.8022771Z ------------------------------------------
2019-07-18T18:04:55.8022809Z 
2019-07-18T18:04:55.8023385Z ------------------------------------------
2019-07-18T18:04:55.8023433Z stderr:
---
2019-07-18T18:04:55.9018892Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:55.9018939Z +
2019-07-18T18:04:55.9024164Z 
2019-07-18T18:04:55.9024253Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:55.9024898Z Actual stderr saved to /tmp/compiletestWXeOcX/thread-local.stderr
2019-07-18T18:04:55.9025119Z To update references, run this command from build directory:
2019-07-18T18:04:55.9025449Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'thread-local.rs'
2019-07-18T18:04:55.9025544Z error: 1 errors occurred comparing output.
2019-07-18T18:04:55.9025611Z status: exit code: 1
2019-07-18T18:04:55.9025611Z status: exit code: 1
2019-07-18T18:04:55.9026294Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/thread-local.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/thread-local.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/thread-local.stage-id.aux" "-A" "unused"
2019-07-18T18:04:55.9026643Z ------------------------------------------
2019-07-18T18:04:55.9042537Z thread '[ui] run-pass/thread-local.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-18T18:04:55.9042625Z 
2019-07-18T18:04:55.9042852Z ------------------------------------------
---
2019-07-18T18:04:55.9433731Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:55.9433790Z +
2019-07-18T18:04:55.9433818Z 
2019-07-18T18:04:55.9433864Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:55.9434210Z Actual stderr saved to /tmp/compiletestWXeOcX/too-large-primval-write-problem.stderr
2019-07-18T18:04:55.9434290Z To update references, run this command from build directory:
2019-07-18T18:04:55.9435011Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'too-large-primval-write-problem.rs'
2019-07-18T18:04:55.9435126Z error: 1 errors occurred comparing output.
2019-07-18T18:04:55.9435175Z status: exit code: 1
2019-07-18T18:04:55.9435175Z status: exit code: 1
2019-07-18T18:04:55.9435883Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/too-large-primval-write-problem.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/too-large-primval-write-problem.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/too-large-primval-write-problem.stage-id.aux" "-A" "unused"
2019-07-18T18:04:55.9436349Z ------------------------------------------
2019-07-18T18:04:55.9436387Z 
2019-07-18T18:04:55.9436627Z ------------------------------------------
2019-07-18T18:04:55.9436676Z stderr:
---
2019-07-18T18:04:56.0934131Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:56.0934192Z +
2019-07-18T18:04:56.0934240Z 
2019-07-18T18:04:56.0934653Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:56.0935083Z Actual stderr saved to /tmp/compiletestWXeOcX/transmute_fat.stderr
2019-07-18T18:04:56.0935248Z To update references, run this command from build directory:
2019-07-18T18:04:56.0935730Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'transmute_fat.rs'
2019-07-18T18:04:56.0936256Z error: 1 errors occurred comparing output.
2019-07-18T18:04:56.0936448Z status: exit code: 1
2019-07-18T18:04:56.0936448Z status: exit code: 1
2019-07-18T18:04:56.0937294Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/transmute_fat.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/transmute_fat.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletestWXeOcX/transmute_fat.stage-id.aux" "-A" "unused"
2019-07-18T18:04:56.0941511Z ------------------------------------------
2019-07-18T18:04:56.0942476Z 
2019-07-18T18:04:56.0943489Z ------------------------------------------
2019-07-18T18:04:56.0945450Z stderr:
---
2019-07-18T18:04:56.1057422Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:56.1057474Z +
2019-07-18T18:04:56.1057504Z 
2019-07-18T18:04:56.1057555Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:56.1057628Z Actual stderr saved to /tmp/compiletestWXeOcX/traits.stderr
2019-07-18T18:04:56.1057684Z To update references, run this command from build directory:
2019-07-18T18:04:56.1057957Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'traits.rs'
2019-07-18T18:04:56.1058062Z error: 1 errors occurred comparing output.
2019-07-18T18:04:56.1058112Z status: exit code: 1
2019-07-18T18:04:56.1058112Z status: exit code: 1
2019-07-18T18:04:56.1059309Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/traits.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/traits.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/traits.stage-id.aux" "-A" "unused"
2019-07-18T18:04:56.1059603Z ------------------------------------------
2019-07-18T18:04:56.1059635Z 
2019-07-18T18:04:56.1059828Z ------------------------------------------
2019-07-18T18:04:56.1059873Z stderr:
---
2019-07-18T18:04:56.2296653Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:56.2296723Z +
2019-07-18T18:04:56.2296755Z 
2019-07-18T18:04:56.2296804Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:56.2296859Z Actual stderr saved to /tmp/compiletestWXeOcX/trivial.stderr
2019-07-18T18:04:56.2296936Z To update references, run this command from build directory:
2019-07-18T18:04:56.2297190Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'trivial.rs'
2019-07-18T18:04:56.2297293Z error: 1 errors occurred comparing output.
2019-07-18T18:04:56.2297444Z status: exit code: 1
2019-07-18T18:04:56.2297444Z status: exit code: 1
2019-07-18T18:04:56.2298103Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/trivial.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/trivial.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/trivial.stage-id.aux" "-A" "unused"
2019-07-18T18:04:56.2298577Z ------------------------------------------
2019-07-18T18:04:56.2298610Z 
2019-07-18T18:04:56.2298827Z ------------------------------------------
2019-07-18T18:04:56.2298872Z stderr:
---
2019-07-18T18:04:56.2401269Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:56.2401319Z +
2019-07-18T18:04:56.2401347Z 
2019-07-18T18:04:56.2401394Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:56.2401667Z Actual stderr saved to /tmp/compiletestWXeOcX/try-operator-custom.stderr
2019-07-18T18:04:56.2401723Z To update references, run this command from build directory:
2019-07-18T18:04:56.2401982Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'try-operator-custom.rs'
2019-07-18T18:04:56.2402078Z error: 1 errors occurred comparing output.
2019-07-18T18:04:56.2402125Z status: exit code: 1
2019-07-18T18:04:56.2402125Z status: exit code: 1
2019-07-18T18:04:56.2402775Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/try-operator-custom.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/try-operator-custom.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/try-operator-custom.stage-id.aux" "-A" "unused"
2019-07-18T18:04:56.2403176Z ------------------------------------------
2019-07-18T18:04:56.2403228Z 
2019-07-18T18:04:56.2403447Z ------------------------------------------
2019-07-18T18:04:56.2403495Z stderr:
---
2019-07-18T18:04:56.4065301Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:56.4065361Z +
2019-07-18T18:04:56.4065392Z 
2019-07-18T18:04:56.4065460Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:56.4065517Z Actual stderr saved to /tmp/compiletestWXeOcX/tuple_like_enum_variant_constructor.stderr
2019-07-18T18:04:56.4065574Z To update references, run this command from build directory:
2019-07-18T18:04:56.4065881Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'tuple_like_enum_variant_constructor.rs'
2019-07-18T18:04:56.4066093Z error: 1 errors occurred comparing output.
2019-07-18T18:04:56.4066141Z status: exit code: 1
2019-07-18T18:04:56.4066141Z status: exit code: 1
2019-07-18T18:04:56.4066908Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/tuple_like_enum_variant_constructor.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/tuple_like_enum_variant_constructor.stage-id.aux" "-A" "unused"
2019-07-18T18:04:56.4067242Z ------------------------------------------
2019-07-18T18:04:56.4067279Z 
2019-07-18T18:04:56.4067496Z ------------------------------------------
2019-07-18T18:04:56.4067562Z stderr:
---
2019-07-18T18:04:56.4169620Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:56.4169673Z +
2019-07-18T18:04:56.4169719Z 
2019-07-18T18:04:56.4169770Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:56.4169832Z Actual stderr saved to /tmp/compiletestWXeOcX/tuple_like_enum_variant_constructor_pointer_opt.stderr
2019-07-18T18:04:56.4169985Z To update references, run this command from build directory:
2019-07-18T18:04:56.4170326Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'tuple_like_enum_variant_constructor_pointer_opt.rs'
2019-07-18T18:04:56.4170418Z error: 1 errors occurred comparing output.
2019-07-18T18:04:56.4170485Z status: exit code: 1
2019-07-18T18:04:56.4170485Z status: exit code: 1
2019-07-18T18:04:56.4171554Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor_pointer_opt.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/tuple_like_enum_variant_constructor_pointer_opt.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/tuple_like_enum_variant_constructor_pointer_opt.stage-id.aux" "-A" "unused"
2019-07-18T18:04:56.4172056Z ------------------------------------------
2019-07-18T18:04:56.4172091Z 
2019-07-18T18:04:56.4173305Z ------------------------------------------
2019-07-18T18:04:56.4173366Z stderr:
---
2019-07-18T18:04:56.5933852Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:56.5934010Z +
2019-07-18T18:04:56.5934050Z 
2019-07-18T18:04:56.5934121Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:56.5934396Z Actual stderr saved to /tmp/compiletestWXeOcX/tuple_like_struct_constructor.stderr
2019-07-18T18:04:56.5934452Z To update references, run this command from build directory:
2019-07-18T18:04:56.5935714Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'tuple_like_struct_constructor.rs'
2019-07-18T18:04:56.5935811Z error: 1 errors occurred comparing output.
2019-07-18T18:04:56.5935860Z status: exit code: 1
2019-07-18T18:04:56.5935860Z status: exit code: 1
2019-07-18T18:04:56.5936585Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_struct_constructor.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/tuple_like_struct_constructor.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/tuple_like_struct_constructor.stage-id.aux" "-A" "unused"
2019-07-18T18:04:56.5936926Z ------------------------------------------
2019-07-18T18:04:56.5936963Z 
2019-07-18T18:04:56.5937177Z ------------------------------------------
2019-07-18T18:04:56.5937243Z stderr:
---
2019-07-18T18:04:56.6211801Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:56.6211852Z +
2019-07-18T18:04:56.6211881Z 
2019-07-18T18:04:56.6211929Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:56.6212003Z Actual stderr saved to /tmp/compiletestWXeOcX/tuple_like_enum_variant_constructor_struct_pointer_opt.stderr
2019-07-18T18:04:56.6212059Z To update references, run this command from build directory:
2019-07-18T18:04:56.6212357Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'tuple_like_enum_variant_constructor_struct_pointer_opt.rs'
2019-07-18T18:04:56.6212461Z error: 1 errors occurred comparing output.
2019-07-18T18:04:56.6212508Z status: exit code: 1
2019-07-18T18:04:56.6212508Z status: exit code: 1
2019-07-18T18:04:56.6213263Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor_struct_pointer_opt.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/tuple_like_enum_variant_constructor_struct_pointer_opt.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/tuple_like_enum_variant_constructor_struct_pointer_opt.stage-id.aux" "-A" "unused"
2019-07-18T18:04:56.6213599Z ------------------------------------------
2019-07-18T18:04:56.6213635Z 
2019-07-18T18:04:56.6214016Z ------------------------------------------
2019-07-18T18:04:56.6214060Z stderr:
---
2019-07-18T18:04:56.7857070Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:56.7857121Z +
2019-07-18T18:04:56.7857149Z 
2019-07-18T18:04:56.7857217Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:56.7857465Z Actual stderr saved to /tmp/compiletestWXeOcX/union-overwrite.stderr
2019-07-18T18:04:56.7857523Z To update references, run this command from build directory:
2019-07-18T18:04:56.7857795Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'union-overwrite.rs'
2019-07-18T18:04:56.7857899Z error: 1 errors occurred comparing output.
2019-07-18T18:04:56.7858106Z status: exit code: 1
2019-07-18T18:04:56.7858106Z status: exit code: 1
2019-07-18T18:04:56.7858875Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/union-overwrite.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/union-overwrite.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/union-overwrite.stage-id.aux" "-A" "unused"
2019-07-18T18:04:56.7859328Z ------------------------------------------
2019-07-18T18:04:56.7859360Z 
2019-07-18T18:04:56.7859553Z ------------------------------------------
2019-07-18T18:04:56.7859614Z stderr:
---
2019-07-18T18:04:56.9774348Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:56.9774399Z +
2019-07-18T18:04:56.9774447Z 
2019-07-18T18:04:56.9774888Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:56.9774966Z Actual stderr saved to /tmp/compiletestWXeOcX/union.stderr
2019-07-18T18:04:56.9775044Z To update references, run this command from build directory:
2019-07-18T18:04:56.9775344Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'union.rs'
2019-07-18T18:04:56.9775430Z error: 1 errors occurred comparing output.
2019-07-18T18:04:56.9775498Z status: exit code: 1
2019-07-18T18:04:56.9775498Z status: exit code: 1
2019-07-18T18:04:56.9776116Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/union.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/union.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/union.stage-id.aux" "-A" "unused"
2019-07-18T18:04:56.9776448Z ------------------------------------------
2019-07-18T18:04:56.9776484Z 
2019-07-18T18:04:56.9776726Z ------------------------------------------
2019-07-18T18:04:56.9776776Z stderr:
---
2019-07-18T18:04:57.0090848Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:57.0090898Z +
2019-07-18T18:04:57.0090925Z 
2019-07-18T18:04:57.0090986Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:57.0091034Z Actual stderr saved to /tmp/compiletestWXeOcX/u128.stderr
2019-07-18T18:04:57.0091093Z To update references, run this command from build directory:
2019-07-18T18:04:57.0091356Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'u128.rs'
2019-07-18T18:04:57.0091453Z error: 1 errors occurred comparing output.
2019-07-18T18:04:57.0091496Z status: exit code: 1
2019-07-18T18:04:57.0091496Z status: exit code: 1
2019-07-18T18:04:57.0092075Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/u128.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/u128.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/u128.stage-id.aux" "-A" "unused"
2019-07-18T18:04:57.0092374Z ------------------------------------------
2019-07-18T18:04:57.0092407Z 
2019-07-18T18:04:57.0092610Z ------------------------------------------
2019-07-18T18:04:57.0092664Z stderr:
---
2019-07-18T18:04:57.1372515Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:57.1372589Z +
2019-07-18T18:04:57.1372618Z 
2019-07-18T18:04:57.1372671Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:57.1372722Z Actual stderr saved to /tmp/compiletestWXeOcX/unops.stderr
2019-07-18T18:04:57.1372790Z To update references, run this command from build directory:
2019-07-18T18:04:57.1373037Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'unops.rs'
2019-07-18T18:04:57.1373133Z error: 1 errors occurred comparing output.
2019-07-18T18:04:57.1373178Z status: exit code: 1
2019-07-18T18:04:57.1373178Z status: exit code: 1
2019-07-18T18:04:57.1373745Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/unops.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/unops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/unops.stage-id.aux" "-A" "unused"
2019-07-18T18:04:57.1374451Z ------------------------------------------
2019-07-18T18:04:57.1374497Z 
2019-07-18T18:04:57.1374790Z ------------------------------------------
2019-07-18T18:04:57.1374841Z stderr:
---
2019-07-18T18:04:57.1849932Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:57.1849982Z +
2019-07-18T18:04:57.1850011Z 
2019-07-18T18:04:57.1850073Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:57.1850327Z Actual stderr saved to /tmp/compiletestWXeOcX/unsized-tuple-impls.stderr
2019-07-18T18:04:57.1850391Z To update references, run this command from build directory:
2019-07-18T18:04:57.1850707Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'unsized-tuple-impls.rs'
2019-07-18T18:04:57.1850791Z error: 1 errors occurred comparing output.
2019-07-18T18:04:57.1850839Z status: exit code: 1
2019-07-18T18:04:57.1850839Z status: exit code: 1
2019-07-18T18:04:57.1851579Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/unsized-tuple-impls.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/unsized-tuple-impls.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/unsized-tuple-impls.stage-id.aux" "-A" "unused"
2019-07-18T18:04:57.1851897Z ------------------------------------------
2019-07-18T18:04:57.1851932Z 
2019-07-18T18:04:57.1852144Z ------------------------------------------
2019-07-18T18:04:57.1852207Z stderr:
---
2019-07-18T18:04:57.2865991Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:57.2866045Z +
2019-07-18T18:04:57.2866208Z 
2019-07-18T18:04:57.2866260Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:57.2866318Z Actual stderr saved to /tmp/compiletestWXeOcX/validation_lifetime_resolution.stderr
2019-07-18T18:04:57.2866378Z To update references, run this command from build directory:
2019-07-18T18:04:57.2866697Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'validation_lifetime_resolution.rs'
2019-07-18T18:04:57.2866788Z error: 1 errors occurred comparing output.
2019-07-18T18:04:57.2866855Z status: exit code: 1
2019-07-18T18:04:57.2866855Z status: exit code: 1
2019-07-18T18:04:57.2867586Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/validation_lifetime_resolution.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/validation_lifetime_resolution.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/validation_lifetime_resolution.stage-id.aux" "-A" "unused"
2019-07-18T18:04:57.2868102Z ------------------------------------------
2019-07-18T18:04:57.2868137Z 
2019-07-18T18:04:57.2868362Z ------------------------------------------
2019-07-18T18:04:57.2868409Z stderr:
---
2019-07-18T18:04:57.3813483Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:57.3813536Z +
2019-07-18T18:04:57.3813566Z 
2019-07-18T18:04:57.3813633Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:57.3813897Z Actual stderr saved to /tmp/compiletestWXeOcX/vec-matching-fold.stderr
2019-07-18T18:04:57.3813953Z To update references, run this command from build directory:
2019-07-18T18:04:57.3814229Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'vec-matching-fold.rs'
2019-07-18T18:04:57.3814314Z error: 1 errors occurred comparing output.
2019-07-18T18:04:57.3814763Z status: exit code: 1
2019-07-18T18:04:57.3814763Z status: exit code: 1
2019-07-18T18:04:57.3815512Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vec-matching-fold.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/vec-matching-fold.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/vec-matching-fold.stage-id.aux" "-A" "unused"
2019-07-18T18:04:57.3815853Z ------------------------------------------
2019-07-18T18:04:57.3815890Z 
2019-07-18T18:04:57.3816105Z ------------------------------------------
2019-07-18T18:04:57.3816173Z stderr:
---
2019-07-18T18:04:57.4800860Z -Iter([], [])
2019-07-18T18:04:57.4801085Z -
2019-07-18T18:04:57.4801116Z 
2019-07-18T18:04:57.4801176Z The actual stdout differed from the expected stdout.
2019-07-18T18:04:57.4801231Z Actual stdout saved to /tmp/compiletestWXeOcX/vecdeque.stdout
2019-07-18T18:04:57.4801358Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-18T18:04:57.4801599Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-18T18:04:57.4801667Z      |
2019-07-18T18:04:57.4801718Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-18T18:04:57.4873584Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:57.4873635Z +
2019-07-18T18:04:57.4873675Z 
2019-07-18T18:04:57.4873723Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:57.4873773Z Actual stderr saved to /tmp/compiletestWXeOcX/vecdeque.stderr
2019-07-18T18:04:57.4873825Z To update references, run this command from build directory:
2019-07-18T18:04:57.4874096Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'vecdeque.rs'
2019-07-18T18:04:57.4874179Z error: 2 errors occurred comparing output.
2019-07-18T18:04:57.4874239Z status: exit code: 1
2019-07-18T18:04:57.4874239Z status: exit code: 1
2019-07-18T18:04:57.4875473Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vecdeque.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/vecdeque.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/vecdeque.stage-id.aux" "-A" "unused"
2019-07-18T18:04:57.4875854Z ------------------------------------------
2019-07-18T18:04:57.4875892Z 
2019-07-18T18:04:57.4876106Z ------------------------------------------
2019-07-18T18:04:57.4876167Z stderr:
---
2019-07-18T18:04:57.6339909Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:57.6340210Z +
2019-07-18T18:04:57.6340403Z 
2019-07-18T18:04:57.6340613Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:57.6340845Z Actual stderr saved to /tmp/compiletestWXeOcX/volatile.stderr
2019-07-18T18:04:57.6341057Z To update references, run this command from build directory:
2019-07-18T18:04:57.6341532Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'volatile.rs'
2019-07-18T18:04:57.6341987Z error: 1 errors occurred comparing output.
2019-07-18T18:04:57.6342193Z status: exit code: 1
2019-07-18T18:04:57.6342193Z status: exit code: 1
2019-07-18T18:04:57.6343376Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/volatile.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/volatile.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/volatile.stage-id.aux" "-A" "unused"
2019-07-18T18:04:57.6344619Z ------------------------------------------
2019-07-18T18:04:57.6344927Z 
2019-07-18T18:04:57.6345445Z ------------------------------------------
2019-07-18T18:04:57.6345713Z stderr:
---
2019-07-18T18:04:57.6960310Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:57.6960356Z +
2019-07-18T18:04:57.6960397Z 
2019-07-18T18:04:57.6960441Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:57.6960489Z Actual stderr saved to /tmp/compiletestWXeOcX/vecs.stderr
2019-07-18T18:04:57.6960552Z To update references, run this command from build directory:
2019-07-18T18:04:57.6960782Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'vecs.rs'
2019-07-18T18:04:57.6960856Z error: 1 errors occurred comparing output.
2019-07-18T18:04:57.6960913Z status: exit code: 1
2019-07-18T18:04:57.6960913Z status: exit code: 1
2019-07-18T18:04:57.6961580Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vecs.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/vecs.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/vecs.stage-id.aux" "-A" "unused"
2019-07-18T18:04:57.6961928Z ------------------------------------------
2019-07-18T18:04:57.6961962Z 
2019-07-18T18:04:57.6962160Z ------------------------------------------
2019-07-18T18:04:57.6962221Z stderr:
---
2019-07-18T18:04:57.7995476Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:57.7995531Z +
2019-07-18T18:04:57.7995579Z 
2019-07-18T18:04:57.7995629Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:57.7995928Z Actual stderr saved to /tmp/compiletestWXeOcX/without-validation.stderr
2019-07-18T18:04:57.7995989Z To update references, run this command from build directory:
2019-07-18T18:04:57.7996285Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'without-validation.rs'
2019-07-18T18:04:57.7996374Z error: 1 errors occurred comparing output.
2019-07-18T18:04:57.7996440Z status: exit code: 1
2019-07-18T18:04:57.7996440Z status: exit code: 1
2019-07-18T18:04:57.7997235Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/without-validation.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/without-validation.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletestWXeOcX/without-validation.stage-id.aux" "-A" "unused"
2019-07-18T18:04:57.7997782Z ------------------------------------------
2019-07-18T18:04:57.7997818Z 
2019-07-18T18:04:57.7998030Z ------------------------------------------
2019-07-18T18:04:57.7998100Z stderr:
---
2019-07-18T18:04:57.9093244Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:57.9093313Z +
2019-07-18T18:04:57.9093342Z 
2019-07-18T18:04:57.9093391Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:57.9093643Z Actual stderr saved to /tmp/compiletestWXeOcX/write-bytes.stderr
2019-07-18T18:04:57.9093719Z To update references, run this command from build directory:
2019-07-18T18:04:57.9093982Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'write-bytes.rs'
2019-07-18T18:04:57.9094083Z error: 1 errors occurred comparing output.
2019-07-18T18:04:57.9094133Z status: exit code: 1
2019-07-18T18:04:57.9094133Z status: exit code: 1
2019-07-18T18:04:57.9095306Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/write-bytes.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/write-bytes.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/write-bytes.stage-id.aux" "-A" "unused"
2019-07-18T18:04:57.9095688Z ------------------------------------------
2019-07-18T18:04:57.9095726Z 
2019-07-18T18:04:57.9095962Z ------------------------------------------
2019-07-18T18:04:57.9096011Z stderr:
---
2019-07-18T18:04:57.9690759Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:57.9690811Z +
2019-07-18T18:04:57.9690841Z 
2019-07-18T18:04:57.9690908Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:57.9691174Z Actual stderr saved to /tmp/compiletestWXeOcX/zero-sized-binary-heap-push.stderr
2019-07-18T18:04:57.9691233Z To update references, run this command from build directory:
2019-07-18T18:04:57.9691518Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'zero-sized-binary-heap-push.rs'
2019-07-18T18:04:57.9691603Z error: 1 errors occurred comparing output.
2019-07-18T18:04:57.9691661Z status: exit code: 1
2019-07-18T18:04:57.9691661Z status: exit code: 1
2019-07-18T18:04:57.9692432Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zero-sized-binary-heap-push.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/zero-sized-binary-heap-push.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/zero-sized-binary-heap-push.stage-id.aux" "-A" "unused"
2019-07-18T18:04:57.9692794Z ------------------------------------------
2019-07-18T18:04:57.9692830Z 
2019-07-18T18:04:57.9693058Z ------------------------------------------
2019-07-18T18:04:57.9693122Z stderr:
---
2019-07-18T18:04:58.0810111Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:58.0810158Z +
2019-07-18T18:04:58.0810185Z 
2019-07-18T18:04:58.0810247Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:58.0810297Z Actual stderr saved to /tmp/compiletestWXeOcX/zst.stderr
2019-07-18T18:04:58.0810358Z To update references, run this command from build directory:
2019-07-18T18:04:58.0810697Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'zst.rs'
2019-07-18T18:04:58.0810788Z error: 1 errors occurred comparing output.
2019-07-18T18:04:58.0810834Z status: exit code: 1
2019-07-18T18:04:58.0810834Z status: exit code: 1
2019-07-18T18:04:58.0811441Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/zst.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/zst.stage-id.aux" "-A" "unused"
2019-07-18T18:04:58.0811751Z ------------------------------------------
2019-07-18T18:04:58.0811785Z 
2019-07-18T18:04:58.0811992Z ------------------------------------------
2019-07-18T18:04:58.0812132Z stderr:
---
2019-07-18T18:04:58.1052775Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:58.1052822Z +
2019-07-18T18:04:58.1052850Z 
2019-07-18T18:04:58.1052914Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:58.1052973Z Actual stderr saved to /tmp/compiletestWXeOcX/zst_box.stderr
2019-07-18T18:04:58.1053146Z To update references, run this command from build directory:
2019-07-18T18:04:58.1053459Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'zst_box.rs'
2019-07-18T18:04:58.1053539Z error: 1 errors occurred comparing output.
2019-07-18T18:04:58.1053583Z status: exit code: 1
2019-07-18T18:04:58.1053583Z status: exit code: 1
2019-07-18T18:04:58.1054158Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst_box.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/zst_box.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/zst_box.stage-id.aux" "-A" "unused"
2019-07-18T18:04:58.1054888Z ------------------------------------------
2019-07-18T18:04:58.1055049Z 
2019-07-18T18:04:58.1055296Z ------------------------------------------
2019-07-18T18:04:58.1055378Z stderr:
---
2019-07-18T18:04:58.2053938Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T18:04:58.2053985Z +
2019-07-18T18:04:58.2054027Z 
2019-07-18T18:04:58.2054072Z The actual stderr differed from the expected stderr.
2019-07-18T18:04:58.2054124Z Actual stderr saved to /tmp/compiletestWXeOcX/zst_variant_drop.stderr
2019-07-18T18:04:58.2054193Z To update references, run this command from build directory:
2019-07-18T18:04:58.2054860Z tests/run-pass/update-references.sh '/tmp/compiletestWXeOcX' 'zst_variant_drop.rs'
2019-07-18T18:04:58.2054956Z error: 1 errors occurred comparing output.
2019-07-18T18:04:58.2055021Z status: exit code: 1
2019-07-18T18:04:58.2055021Z status: exit code: 1
2019-07-18T18:04:58.2055713Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst_variant_drop.rs" "-L" "/tmp/compiletestWXeOcX" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestWXeOcX/zst_variant_drop.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestWXeOcX/zst_variant_drop.stage-id.aux" "-A" "unused"
2019-07-18T18:04:58.2056031Z ------------------------------------------
2019-07-18T18:04:58.2056066Z 
2019-07-18T18:04:58.2056295Z ------------------------------------------
2019-07-18T18:04:58.2056425Z stderr:
---
2019-07-18T18:04:58.2474348Z Verifying status of embedded-book...
2019-07-18T18:04:58.2487167Z Verifying status of rustc-guide...
2019-07-18T18:04:58.2500273Z This PR updated 'src/doc/rustc-guide', verifying if status is 'test-pass'...
2019-07-18T18:04:58.2510529Z 
2019-07-18T18:04:58.2511333Z ⚠️ We detected that this PR updated 'rustc-guide', but its tests failed.
2019-07-18T18:04:58.2511375Z 
2019-07-18T18:04:58.2511810Z If you do intend to update 'rustc-guide', please check the error messages above and
2019-07-18T18:04:58.2511904Z commit another update.
2019-07-18T18:04:58.2511934Z 
2019-07-18T18:04:58.2512214Z If you do NOT intend to update 'rustc-guide', please ensure you did not accidentally
2019-07-18T18:04:58.2512457Z change the submodule at 'src/doc/rustc-guide'. You may ask your reviewer for the
2019-07-18T18:04:58.2512591Z proper steps.
2019-07-18T18:04:58.8780503Z ##[error]Bash exited with code '3'.
2019-07-18T18:04:58.8818086Z ##[section]Starting: Checkout
2019-07-18T18:04:58.8819722Z ==============================================================================
2019-07-18T18:04:58.8819777Z Task         : Get sources
2019-07-18T18:04:58.8819824Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
