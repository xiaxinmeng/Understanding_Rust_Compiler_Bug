plain
2019-07-18T15:14:18.6652945Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-18T15:14:18.6834456Z ##[command]git config gc.auto 0
2019-07-18T15:14:18.6908023Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-18T15:14:18.6961470Z ##[command]git config --get-all http.proxy
2019-07-18T15:14:18.7087254Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62766/merge:refs/remotes/pull/62766/merge
---
2019-07-18T15:14:53.5993892Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-18T15:14:53.5995509Z 
2019-07-18T15:14:53.5996837Z   git checkout -b <new-branch-name>
2019-07-18T15:14:53.5998110Z 
2019-07-18T15:14:53.5998732Z HEAD is now at 4f2583587 Merge 2e6a06443171c7c08cc03feb47cf8c2b63d27a07 into 21d5b8bf0c26e3ee4c270ce5527df66b1af56513
2019-07-18T15:14:53.6144007Z ##[section]Finishing: Checkout
2019-07-18T15:14:53.6149879Z ##[section]Starting: Decide whether to run this job
2019-07-18T15:14:53.6153131Z Task         : Bash
2019-07-18T15:14:53.6153179Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-18T15:14:53.6153229Z Version      : 3.151.2
2019-07-18T15:14:53.6153294Z Author       : Microsoft Corporation
2019-07-18T15:14:53.6153294Z Author       : Microsoft Corporation
2019-07-18T15:14:53.6153346Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-18T15:14:53.6153401Z ==============================================================================
2019-07-18T15:14:53.7423749Z Generating script.
2019-07-18T15:14:53.7455429Z ========================== Starting Command Output ===========================
2019-07-18T15:14:53.7477375Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8e35d559-c61f-49fe-8ec1-84d6fffa6c54.sh
2019-07-18T15:14:54.0303535Z Executing the job since submodules are updated
2019-07-18T15:14:54.0342136Z ##[section]Finishing: Decide whether to run this job
2019-07-18T15:14:54.0353022Z ==============================================================================
2019-07-18T15:14:54.0353079Z Task         : Bash
2019-07-18T15:14:54.0353171Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-18T15:14:54.0353217Z Version      : 3.151.2
---
2019-07-18T17:43:14.9011344Z -args
2019-07-18T17:43:14.9015775Z -
2019-07-18T17:43:14.9020432Z 
2019-07-18T17:43:14.9024322Z The actual stdout differed from the expected stdout.
2019-07-18T17:43:14.9032842Z Actual stdout saved to /tmp/compiletestLiAEmM/args.stdout
2019-07-18T17:43:14.9040479Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-18T17:43:14.9054192Z normalized stderr:
2019-07-18T17:43:14.9054258Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-18T17:43:14.9054628Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
---
2019-07-18T17:43:14.9057862Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:14.9057902Z +
2019-07-18T17:43:14.9057923Z 
2019-07-18T17:43:14.9057960Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:14.9058013Z Actual stderr saved to /tmp/compiletestLiAEmM/args.stderr
2019-07-18T17:43:14.9058054Z To update references, run this command from build directory:
2019-07-18T17:43:14.9058944Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'args.rs'
2019-07-18T17:43:14.9059064Z error: 2 errors occurred comparing output.
2019-07-18T17:43:14.9059110Z status: exit code: 1
2019-07-18T17:43:14.9059110Z status: exit code: 1
2019-07-18T17:43:14.9060024Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/args.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/args.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/args.stage-id.aux" "-A" "unused"
2019-07-18T17:43:14.9060508Z ------------------------------------------
2019-07-18T17:43:14.9060543Z 
2019-07-18T17:43:14.9060765Z ------------------------------------------
2019-07-18T17:43:14.9060810Z stderr:
---
2019-07-18T17:43:14.9586871Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:14.9586911Z +
2019-07-18T17:43:14.9586949Z 
2019-07-18T17:43:14.9586986Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:14.9587026Z Actual stderr saved to /tmp/compiletestLiAEmM/arrays.stderr
2019-07-18T17:43:14.9587083Z To update references, run this command from build directory:
2019-07-18T17:43:14.9587296Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'arrays.rs'
2019-07-18T17:43:14.9587367Z error: 1 errors occurred comparing output.
2019-07-18T17:43:14.9587418Z status: exit code: 1
2019-07-18T17:43:14.9587418Z status: exit code: 1
2019-07-18T17:43:14.9588346Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/arrays.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/arrays.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/arrays.stage-id.aux" "-A" "unused"
2019-07-18T17:43:14.9588848Z ------------------------------------------
2019-07-18T17:43:14.9588885Z 
2019-07-18T17:43:14.9589125Z ------------------------------------------
2019-07-18T17:43:14.9589172Z stderr:
---
2019-07-18T17:43:15.0663405Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:15.0663470Z +
2019-07-18T17:43:15.0663494Z 
2019-07-18T17:43:15.0663532Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:15.0663741Z Actual stderr saved to /tmp/compiletestLiAEmM/associated-const.stderr
2019-07-18T17:43:15.0663804Z To update references, run this command from build directory:
2019-07-18T17:43:15.0664033Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'associated-const.rs'
2019-07-18T17:43:15.0664103Z error: 1 errors occurred comparing output.
2019-07-18T17:43:15.0664162Z status: exit code: 1
2019-07-18T17:43:15.0664162Z status: exit code: 1
2019-07-18T17:43:15.0664730Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/associated-const.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/associated-const.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/associated-const.stage-id.aux" "-A" "unused"
2019-07-18T17:43:15.0665210Z ------------------------------------------
2019-07-18T17:43:15.0665241Z 
2019-07-18T17:43:15.0665460Z ------------------------------------------
2019-07-18T17:43:15.0665500Z stderr:
---
2019-07-18T17:43:15.0789499Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:15.0789546Z +
2019-07-18T17:43:15.0789574Z 
2019-07-18T17:43:15.0789637Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:15.0789687Z Actual stderr saved to /tmp/compiletestLiAEmM/assume_bug.stderr
2019-07-18T17:43:15.0789738Z To update references, run this command from build directory:
2019-07-18T17:43:15.0790013Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'assume_bug.rs'
2019-07-18T17:43:15.0790101Z error: 1 errors occurred comparing output.
2019-07-18T17:43:15.0790148Z status: exit code: 1
2019-07-18T17:43:15.0790148Z status: exit code: 1
2019-07-18T17:43:15.0790800Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/assume_bug.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/assume_bug.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/assume_bug.stage-id.aux" "-A" "unused"
2019-07-18T17:43:15.0791230Z ------------------------------------------
2019-07-18T17:43:15.0791266Z 
2019-07-18T17:43:15.0791484Z ------------------------------------------
2019-07-18T17:43:15.0791560Z stderr:
---
2019-07-18T17:43:15.2325548Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:15.2325592Z +
2019-07-18T17:43:15.2325613Z 
2019-07-18T17:43:15.2325665Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:15.2325862Z Actual stderr saved to /tmp/compiletestLiAEmM/async-fn.stderr
2019-07-18T17:43:15.2325905Z To update references, run this command from build directory:
2019-07-18T17:43:15.2326141Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'async-fn.rs'
2019-07-18T17:43:15.2326206Z error: 1 errors occurred comparing output.
2019-07-18T17:43:15.2326257Z status: exit code: 1
2019-07-18T17:43:15.2326257Z status: exit code: 1
2019-07-18T17:43:15.2326941Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/async-fn.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/async-fn.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/async-fn.stage-id.aux" "-A" "unused"
2019-07-18T17:43:15.2327473Z ------------------------------------------
2019-07-18T17:43:15.2327503Z 
2019-07-18T17:43:15.2327681Z ------------------------------------------
2019-07-18T17:43:15.2327734Z stderr:
---
2019-07-18T17:43:15.2357774Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:15.2357819Z +
2019-07-18T17:43:15.2357862Z 
2019-07-18T17:43:15.2357912Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:15.2358706Z Actual stderr saved to /tmp/compiletestLiAEmM/atomic-access-bool.stderr
2019-07-18T17:43:15.2358785Z To update references, run this command from build directory:
2019-07-18T17:43:15.2359081Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'atomic-access-bool.rs'
2019-07-18T17:43:15.2359162Z error: 1 errors occurred comparing output.
2019-07-18T17:43:15.2359226Z status: exit code: 1
2019-07-18T17:43:15.2359226Z status: exit code: 1
2019-07-18T17:43:15.2360018Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/atomic-access-bool.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/atomic-access-bool.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/atomic-access-bool.stage-id.aux" "-A" "unused"
2019-07-18T17:43:15.2360357Z ------------------------------------------
2019-07-18T17:43:15.2360391Z 
2019-07-18T17:43:15.2360626Z ------------------------------------------
2019-07-18T17:43:15.2360671Z stderr:
---
2019-07-18T17:43:15.3637156Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:15.3637326Z +
2019-07-18T17:43:15.3637438Z 
2019-07-18T17:43:15.3637591Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:15.3637746Z Actual stderr saved to /tmp/compiletestLiAEmM/bad_substs.stderr
2019-07-18T17:43:15.3637900Z To update references, run this command from build directory:
2019-07-18T17:43:15.3638263Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'bad_substs.rs'
2019-07-18T17:43:15.3639617Z error: 1 errors occurred comparing output.
2019-07-18T17:43:15.3639768Z status: exit code: 1
2019-07-18T17:43:15.3639768Z status: exit code: 1
2019-07-18T17:43:15.3640720Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/bad_substs.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/bad_substs.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/bad_substs.stage-id.aux" "-A" "unused"
2019-07-18T17:43:15.3641340Z ------------------------------------------
2019-07-18T17:43:15.3641500Z 
2019-07-18T17:43:15.3641852Z ------------------------------------------
2019-07-18T17:43:15.3642031Z stderr:
---
2019-07-18T17:43:15.4445151Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:15.4445329Z +
2019-07-18T17:43:15.4445436Z 
2019-07-18T17:43:15.4445746Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:15.4446293Z Actual stderr saved to /tmp/compiletestLiAEmM/atomic-compare_exchange.stderr
2019-07-18T17:43:15.4446458Z To update references, run this command from build directory:
2019-07-18T17:43:15.4446883Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'atomic-compare_exchange.rs'
2019-07-18T17:43:15.4447342Z error: 1 errors occurred comparing output.
2019-07-18T17:43:15.4447467Z status: exit code: 1
2019-07-18T17:43:15.4447467Z status: exit code: 1
2019-07-18T17:43:15.4448906Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/atomic-compare_exchange.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/atomic-compare_exchange.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/atomic-compare_exchange.stage-id.aux" "-A" "unused"
2019-07-18T17:43:15.4449598Z ------------------------------------------
2019-07-18T17:43:15.4449753Z 
2019-07-18T17:43:15.4450101Z ------------------------------------------
2019-07-18T17:43:15.4450293Z stderr:
---
2019-07-18T17:43:15.5843691Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:15.5843752Z +
2019-07-18T17:43:15.5843776Z 
2019-07-18T17:43:15.5843818Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:15.5843861Z Actual stderr saved to /tmp/compiletestLiAEmM/binops.stderr
2019-07-18T17:43:15.5843922Z To update references, run this command from build directory:
2019-07-18T17:43:15.5844161Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'binops.rs'
2019-07-18T17:43:15.5844331Z error: 1 errors occurred comparing output.
2019-07-18T17:43:15.5844371Z status: exit code: 1
2019-07-18T17:43:15.5844371Z status: exit code: 1
2019-07-18T17:43:15.5844953Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/binops.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/binops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/binops.stage-id.aux" "-A" "unused"
2019-07-18T17:43:15.5845253Z ------------------------------------------
2019-07-18T17:43:15.5845284Z 
2019-07-18T17:43:15.5845508Z ------------------------------------------
2019-07-18T17:43:15.5845549Z stderr:
---
2019-07-18T17:43:15.5956847Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:15.5958822Z +
2019-07-18T17:43:15.5958995Z 
2019-07-18T17:43:15.5959149Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:15.5959298Z Actual stderr saved to /tmp/compiletestLiAEmM/bools.stderr
2019-07-18T17:43:15.5959470Z To update references, run this command from build directory:
2019-07-18T17:43:15.5959930Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'bools.rs'
2019-07-18T17:43:15.5960466Z error: 1 errors occurred comparing output.
2019-07-18T17:43:15.5960614Z status: exit code: 1
2019-07-18T17:43:15.5960614Z status: exit code: 1
2019-07-18T17:43:15.5961552Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/bools.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/bools.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/bools.stage-id.aux" "-A" "unused"
2019-07-18T17:43:15.5962388Z ------------------------------------------
2019-07-18T17:43:15.5962560Z 
2019-07-18T17:43:15.5963059Z ------------------------------------------
2019-07-18T17:43:15.5963221Z stderr:
---
2019-07-18T17:43:15.7392797Z -foo #1 = Foo(1337)
2019-07-18T17:43:15.7392939Z -
2019-07-18T17:43:15.7392980Z 
2019-07-18T17:43:15.7393016Z The actual stdout differed from the expected stdout.
2019-07-18T17:43:15.7393216Z Actual stdout saved to /tmp/compiletestLiAEmM/box-pair-to-vec.stdout
2019-07-18T17:43:15.7397826Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-18T17:43:15.7400577Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-18T17:43:15.7400701Z      |
2019-07-18T17:43:15.7400748Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-18T17:43:15.7420613Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:15.7420666Z +
2019-07-18T17:43:15.7420697Z 
2019-07-18T17:43:15.7420760Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:15.7421042Z Actual stderr saved to /tmp/compiletestLiAEmM/box-pair-to-vec.stderr
2019-07-18T17:43:15.7421100Z To update references, run this command from build directory:
2019-07-18T17:43:15.7421408Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'box-pair-to-vec.rs'
2019-07-18T17:43:15.7421495Z error: 2 errors occurred comparing output.
2019-07-18T17:43:15.7421543Z status: exit code: 1
2019-07-18T17:43:15.7421543Z status: exit code: 1
2019-07-18T17:43:15.7422406Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/box-pair-to-vec.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/box-pair-to-vec.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/box-pair-to-vec.stage-id.aux" "-A" "unused"
2019-07-18T17:43:15.7422804Z ------------------------------------------
2019-07-18T17:43:15.7422834Z 
2019-07-18T17:43:15.7423010Z ------------------------------------------
2019-07-18T17:43:15.7423062Z stderr:
---
2019-07-18T17:43:15.7433126Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:15.7433167Z +
2019-07-18T17:43:15.7433206Z 
2019-07-18T17:43:15.7433243Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:15.7433294Z Actual stderr saved to /tmp/compiletestLiAEmM/box_box_trait.stderr
2019-07-18T17:43:15.7433337Z To update references, run this command from build directory:
2019-07-18T17:43:15.7433584Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'box_box_trait.rs'
2019-07-18T17:43:15.7433650Z error: 1 errors occurred comparing output.
2019-07-18T17:43:15.7433702Z status: exit code: 1
2019-07-18T17:43:15.7433702Z status: exit code: 1
2019-07-18T17:43:15.7434240Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/box_box_trait.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/box_box_trait.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/box_box_trait.stage-id.aux" "-A" "unused"
2019-07-18T17:43:15.7434518Z ------------------------------------------
2019-07-18T17:43:15.7434554Z 
2019-07-18T17:43:15.7434748Z ------------------------------------------
2019-07-18T17:43:15.7434804Z stderr:
---
2019-07-18T17:43:15.9093193Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:15.9093428Z +
2019-07-18T17:43:15.9093584Z 
2019-07-18T17:43:15.9093774Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:15.9093947Z Actual stderr saved to /tmp/compiletestLiAEmM/c_enums.stderr
2019-07-18T17:43:15.9094138Z To update references, run this command from build directory:
2019-07-18T17:43:15.9094545Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'c_enums.rs'
2019-07-18T17:43:15.9094919Z error: 1 errors occurred comparing output.
2019-07-18T17:43:15.9095127Z status: exit code: 1
2019-07-18T17:43:15.9095127Z status: exit code: 1
2019-07-18T17:43:15.9095919Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/c_enums.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/c_enums.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/c_enums.stage-id.aux" "-A" "unused"
2019-07-18T17:43:15.9096561Z ------------------------------------------
2019-07-18T17:43:15.9096765Z 
2019-07-18T17:43:15.9097184Z ------------------------------------------
2019-07-18T17:43:15.9097933Z stderr:
---
2019-07-18T17:43:15.9324466Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:15.9324664Z +
2019-07-18T17:43:15.9324772Z 
2019-07-18T17:43:15.9324929Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:15.9379885Z Actual stderr saved to /tmp/compiletestLiAEmM/btreemap.stderr
2019-07-18T17:43:15.9380189Z To update references, run this command from build directory:
2019-07-18T17:43:15.9380887Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'btreemap.rs'
2019-07-18T17:43:15.9381284Z error: 1 errors occurred comparing output.
2019-07-18T17:43:15.9381484Z status: exit code: 1
2019-07-18T17:43:15.9381484Z status: exit code: 1
2019-07-18T17:43:15.9382811Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/btreemap.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/btreemap.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/btreemap.stage-id.aux" "-A" "unused"
2019-07-18T17:43:15.9383592Z ------------------------------------------
2019-07-18T17:43:15.9383755Z 
2019-07-18T17:43:15.9384342Z ------------------------------------------
2019-07-18T17:43:15.9384533Z stderr:
---
2019-07-18T17:43:16.0645453Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:16.0645634Z +
2019-07-18T17:43:16.0645755Z 
2019-07-18T17:43:16.0645896Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:16.0646081Z Actual stderr saved to /tmp/compiletestLiAEmM/call_drop_on_array_elements.stderr
2019-07-18T17:43:16.0646230Z To update references, run this command from build directory:
2019-07-18T17:43:16.0646665Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'call_drop_on_array_elements.rs'
2019-07-18T17:43:16.0647151Z error: 1 errors occurred comparing output.
2019-07-18T17:43:16.0647463Z status: exit code: 1
2019-07-18T17:43:16.0647463Z status: exit code: 1
2019-07-18T17:43:16.0648768Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_array_elements.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/call_drop_on_array_elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/call_drop_on_array_elements.stage-id.aux" "-A" "unused"
2019-07-18T17:43:16.0649469Z ------------------------------------------
2019-07-18T17:43:16.0649635Z 
2019-07-18T17:43:16.0650015Z ------------------------------------------
2019-07-18T17:43:16.0650190Z stderr:
---
2019-07-18T17:43:16.0999512Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:16.0999562Z +
2019-07-18T17:43:16.0999609Z 
2019-07-18T17:43:16.0999655Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:16.0999709Z Actual stderr saved to /tmp/compiletestLiAEmM/call_drop_on_fat_ptr_array_elements.stderr
2019-07-18T17:43:16.0999788Z To update references, run this command from build directory:
2019-07-18T17:43:16.1000077Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'call_drop_on_fat_ptr_array_elements.rs'
2019-07-18T17:43:16.1000160Z error: 1 errors occurred comparing output.
2019-07-18T17:43:16.1000224Z status: exit code: 1
2019-07-18T17:43:16.1000224Z status: exit code: 1
2019-07-18T17:43:16.1000944Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_fat_ptr_array_elements.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/call_drop_on_fat_ptr_array_elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/call_drop_on_fat_ptr_array_elements.stage-id.aux" "-A" "unused"
2019-07-18T17:43:16.1001407Z ------------------------------------------
2019-07-18T17:43:16.1001451Z 
2019-07-18T17:43:16.1001874Z ------------------------------------------
2019-07-18T17:43:16.1001919Z stderr:
---
2019-07-18T17:43:16.2156556Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:16.2156596Z +
2019-07-18T17:43:16.2156618Z 
2019-07-18T17:43:16.2156654Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:16.2156722Z Actual stderr saved to /tmp/compiletestLiAEmM/call_drop_on_zst_array_elements.stderr
2019-07-18T17:43:16.2156764Z To update references, run this command from build directory:
2019-07-18T17:43:16.2157011Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'call_drop_on_zst_array_elements.rs'
2019-07-18T17:43:16.2157093Z error: 1 errors occurred comparing output.
2019-07-18T17:43:16.2157130Z status: exit code: 1
2019-07-18T17:43:16.2157130Z status: exit code: 1
2019-07-18T17:43:16.2157703Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_zst_array_elements.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/call_drop_on_zst_array_elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/call_drop_on_zst_array_elements.stage-id.aux" "-A" "unused"
2019-07-18T17:43:16.2158055Z ------------------------------------------
2019-07-18T17:43:16.2158091Z 
2019-07-18T17:43:16.2158751Z ------------------------------------------
2019-07-18T17:43:16.2158805Z stderr:
---
2019-07-18T17:43:16.2521077Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:16.2521141Z +
2019-07-18T17:43:16.2521171Z 
2019-07-18T17:43:16.2521237Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:16.2521293Z Actual stderr saved to /tmp/compiletestLiAEmM/call_drop_through_owned_slice.stderr
2019-07-18T17:43:16.2521348Z To update references, run this command from build directory:
2019-07-18T17:43:16.2521871Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'call_drop_through_owned_slice.rs'
2019-07-18T17:43:16.2522106Z error: 1 errors occurred comparing output.
2019-07-18T17:43:16.2522140Z status: exit code: 1
2019-07-18T17:43:16.2522140Z status: exit code: 1
2019-07-18T17:43:16.2522790Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_owned_slice.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/call_drop_through_owned_slice.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/call_drop_through_owned_slice.stage-id.aux" "-A" "unused"
2019-07-18T17:43:16.2523083Z ------------------------------------------
2019-07-18T17:43:16.2523111Z 
2019-07-18T17:43:16.2523280Z ------------------------------------------
2019-07-18T17:43:16.2523329Z stderr:
---
2019-07-18T17:43:16.3463419Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:16.3463462Z +
2019-07-18T17:43:16.3463493Z 
2019-07-18T17:43:16.3463532Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:16.3463595Z Actual stderr saved to /tmp/compiletestLiAEmM/call_drop_through_trait_object.stderr
2019-07-18T17:43:16.3463641Z To update references, run this command from build directory:
2019-07-18T17:43:16.3463892Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'call_drop_through_trait_object.rs'
2019-07-18T17:43:16.3463975Z error: 1 errors occurred comparing output.
2019-07-18T17:43:16.3464081Z status: exit code: 1
2019-07-18T17:43:16.3464081Z status: exit code: 1
2019-07-18T17:43:16.3464723Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_trait_object.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/call_drop_through_trait_object.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/call_drop_through_trait_object.stage-id.aux" "-A" "unused"
2019-07-18T17:43:16.3465016Z ------------------------------------------
2019-07-18T17:43:16.3465047Z 
2019-07-18T17:43:16.3465239Z ------------------------------------------
2019-07-18T17:43:16.3465279Z stderr:
---
2019-07-18T17:43:16.3755406Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:16.3755459Z +
2019-07-18T17:43:16.3755480Z 
2019-07-18T17:43:16.3755516Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:16.3755575Z Actual stderr saved to /tmp/compiletestLiAEmM/call_drop_through_trait_object_rc.stderr
2019-07-18T17:43:16.3755617Z To update references, run this command from build directory:
2019-07-18T17:43:16.3755906Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'call_drop_through_trait_object_rc.rs'
2019-07-18T17:43:16.3755996Z error: 1 errors occurred comparing output.
2019-07-18T17:43:16.3756032Z status: exit code: 1
2019-07-18T17:43:16.3756032Z status: exit code: 1
2019-07-18T17:43:16.3756606Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_trait_object_rc.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/call_drop_through_trait_object_rc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/call_drop_through_trait_object_rc.stage-id.aux" "-A" "unused"
2019-07-18T17:43:16.3756886Z ------------------------------------------
2019-07-18T17:43:16.3756930Z 
2019-07-18T17:43:16.3757106Z ------------------------------------------
2019-07-18T17:43:16.3757150Z stderr:
---
2019-07-18T17:43:16.5163245Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:16.5163284Z +
2019-07-18T17:43:16.5163306Z 
2019-07-18T17:43:16.5163341Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:16.5163702Z Actual stderr saved to /tmp/compiletestLiAEmM/calloc.stderr
2019-07-18T17:43:16.5163753Z To update references, run this command from build directory:
2019-07-18T17:43:16.5163990Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'calloc.rs'
2019-07-18T17:43:16.5164075Z error: 1 errors occurred comparing output.
2019-07-18T17:43:16.5164111Z status: exit code: 1
2019-07-18T17:43:16.5164111Z status: exit code: 1
2019-07-18T17:43:16.5164632Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/calloc.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/calloc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/calloc.stage-id.aux" "-A" "unused"
2019-07-18T17:43:16.5164911Z ------------------------------------------
2019-07-18T17:43:16.5164939Z 
2019-07-18T17:43:16.5165126Z ------------------------------------------
2019-07-18T17:43:16.5165164Z stderr:
---
2019-07-18T17:43:16.5402557Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:16.5402592Z +
2019-07-18T17:43:16.5402629Z 
2019-07-18T17:43:16.5402727Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:16.5402770Z Actual stderr saved to /tmp/compiletestLiAEmM/calls.stderr
2019-07-18T17:43:16.5402825Z To update references, run this command from build directory:
2019-07-18T17:43:16.5403031Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'calls.rs'
2019-07-18T17:43:16.5403089Z error: 1 errors occurred comparing output.
2019-07-18T17:43:16.5403141Z status: exit code: 1
2019-07-18T17:43:16.5403141Z status: exit code: 1
2019-07-18T17:43:16.5403599Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/calls.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/calls.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/calls.stage-id.aux" "-A" "unused"
2019-07-18T17:43:16.5403858Z ------------------------------------------
2019-07-18T17:43:16.5403884Z 
2019-07-18T17:43:16.5404065Z ------------------------------------------
2019-07-18T17:43:16.5404100Z stderr:
---
2019-07-18T17:43:16.6599354Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:16.6599419Z +
2019-07-18T17:43:16.6599446Z 
2019-07-18T17:43:16.6599492Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:16.6599561Z Actual stderr saved to /tmp/compiletestLiAEmM/cast_fn_ptr.stderr
2019-07-18T17:43:16.6599720Z To update references, run this command from build directory:
2019-07-18T17:43:16.6600022Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'cast_fn_ptr.rs'
2019-07-18T17:43:16.6600120Z error: 1 errors occurred comparing output.
2019-07-18T17:43:16.6600165Z status: exit code: 1
2019-07-18T17:43:16.6600165Z status: exit code: 1
2019-07-18T17:43:16.6600811Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast_fn_ptr.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/cast_fn_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/cast_fn_ptr.stage-id.aux" "-A" "unused"
2019-07-18T17:43:16.6601151Z ------------------------------------------
2019-07-18T17:43:16.6601202Z 
2019-07-18T17:43:16.6601577Z ------------------------------------------
2019-07-18T17:43:16.6601620Z stderr:
---
2019-07-18T17:43:16.6754189Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:16.6754232Z +
2019-07-18T17:43:16.6754442Z 
2019-07-18T17:43:16.6754478Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:16.6754701Z Actual stderr saved to /tmp/compiletestLiAEmM/cast-rfc0401-vtable-kinds.stderr
2019-07-18T17:43:16.6754745Z To update references, run this command from build directory:
2019-07-18T17:43:16.6754974Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'cast-rfc0401-vtable-kinds.rs'
2019-07-18T17:43:16.6755044Z error: 1 errors occurred comparing output.
2019-07-18T17:43:16.6755097Z status: exit code: 1
2019-07-18T17:43:16.6755097Z status: exit code: 1
2019-07-18T17:43:16.6755636Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast-rfc0401-vtable-kinds.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/cast-rfc0401-vtable-kinds.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/cast-rfc0401-vtable-kinds.stage-id.aux" "-A" "unused"
2019-07-18T17:43:16.6755898Z ------------------------------------------
2019-07-18T17:43:16.6755925Z 
2019-07-18T17:43:16.6756095Z ------------------------------------------
2019-07-18T17:43:16.6756152Z stderr:
---
2019-07-18T17:43:16.7842614Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:16.7842781Z +
2019-07-18T17:43:16.7842812Z 
2019-07-18T17:43:16.7842866Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:16.7842908Z Actual stderr saved to /tmp/compiletestLiAEmM/cast_fn_ptr_unsafe.stderr
2019-07-18T17:43:16.7842948Z To update references, run this command from build directory:
2019-07-18T17:43:16.7843235Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'cast_fn_ptr_unsafe.rs'
2019-07-18T17:43:16.7843310Z error: 1 errors occurred comparing output.
2019-07-18T17:43:16.7843347Z status: exit code: 1
2019-07-18T17:43:16.7843347Z status: exit code: 1
2019-07-18T17:43:16.7843899Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast_fn_ptr_unsafe.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/cast_fn_ptr_unsafe.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/cast_fn_ptr_unsafe.stage-id.aux" "-A" "unused"
2019-07-18T17:43:16.7844177Z ------------------------------------------
2019-07-18T17:43:16.7844207Z 
2019-07-18T17:43:16.7844397Z ------------------------------------------
2019-07-18T17:43:16.7844435Z stderr:
---
2019-07-18T17:43:16.7945170Z -1
2019-07-18T17:43:16.7948270Z -
2019-07-18T17:43:16.7953460Z 
2019-07-18T17:43:16.7957314Z The actual stdout differed from the expected stdout.
2019-07-18T17:43:16.7974593Z Actual stdout saved to /tmp/compiletestLiAEmM/catch.stdout
2019-07-18T17:43:16.7975109Z normalized stderr:
2019-07-18T17:43:16.7975311Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-18T17:43:16.7975568Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-18T17:43:16.7975627Z      |
---
2019-07-18T17:43:16.7980144Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:16.7980199Z +
2019-07-18T17:43:16.7980226Z 
2019-07-18T17:43:16.7980285Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:16.7980353Z Actual stderr saved to /tmp/compiletestLiAEmM/catch.stderr
2019-07-18T17:43:16.7980405Z To update references, run this command from build directory:
2019-07-18T17:43:16.7980685Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'catch.rs'
2019-07-18T17:43:16.7980784Z error: 2 errors occurred comparing output.
2019-07-18T17:43:16.7980832Z status: exit code: 1
2019-07-18T17:43:16.7980832Z status: exit code: 1
2019-07-18T17:43:16.7981487Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/catch.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/catch.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/catch.stage-id.aux" "-A" "unused"
2019-07-18T17:43:16.7982178Z ------------------------------------------
2019-07-18T17:43:16.7982225Z 
2019-07-18T17:43:16.7982402Z ------------------------------------------
2019-07-18T17:43:16.7982438Z stderr:
---
2019-07-18T17:43:16.9285264Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:16.9285303Z +
2019-07-18T17:43:16.9285324Z 
2019-07-18T17:43:16.9285369Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:16.9285602Z Actual stderr saved to /tmp/compiletestLiAEmM/closure-drop.stderr
2019-07-18T17:43:16.9285646Z To update references, run this command from build directory:
2019-07-18T17:43:16.9285859Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'closure-drop.rs'
2019-07-18T17:43:16.9285940Z error: 1 errors occurred comparing output.
2019-07-18T17:43:16.9286052Z status: exit code: 1
2019-07-18T17:43:16.9286052Z status: exit code: 1
2019-07-18T17:43:16.9286605Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closure-drop.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/closure-drop.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/closure-drop.stage-id.aux" "-A" "unused"
2019-07-18T17:43:16.9286890Z ------------------------------------------
2019-07-18T17:43:16.9286919Z 
2019-07-18T17:43:16.9287105Z ------------------------------------------
2019-07-18T17:43:16.9287142Z stderr:
---
2019-07-18T17:43:16.9299169Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:16.9299217Z +
2019-07-18T17:43:16.9299244Z 
2019-07-18T17:43:16.9299288Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:16.9299368Z Actual stderr saved to /tmp/compiletestLiAEmM/char.stderr
2019-07-18T17:43:16.9299589Z To update references, run this command from build directory:
2019-07-18T17:43:16.9299877Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'char.rs'
2019-07-18T17:43:16.9299974Z error: 1 errors occurred comparing output.
2019-07-18T17:43:16.9300020Z status: exit code: 1
2019-07-18T17:43:16.9300020Z status: exit code: 1
2019-07-18T17:43:16.9300805Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/char.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/char.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/char.stage-id.aux" "-A" "unused"
2019-07-18T17:43:16.9301290Z ------------------------------------------
2019-07-18T17:43:16.9301345Z 
2019-07-18T17:43:16.9301899Z ------------------------------------------
2019-07-18T17:43:16.9301939Z stderr:
---
2019-07-18T17:43:17.0962887Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:17.0962929Z +
2019-07-18T17:43:17.0962969Z 
2019-07-18T17:43:17.0963008Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:17.0963223Z Actual stderr saved to /tmp/compiletestLiAEmM/closure-field-ty.stderr
2019-07-18T17:43:17.0963285Z To update references, run this command from build directory:
2019-07-18T17:43:17.0963508Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'closure-field-ty.rs'
2019-07-18T17:43:17.0963651Z error: 1 errors occurred comparing output.
2019-07-18T17:43:17.0963706Z status: exit code: 1
2019-07-18T17:43:17.0963706Z status: exit code: 1
2019-07-18T17:43:17.0964296Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closure-field-ty.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/closure-field-ty.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/closure-field-ty.stage-id.aux" "-A" "unused"
2019-07-18T17:43:17.0964580Z ------------------------------------------
2019-07-18T17:43:17.0964609Z 
2019-07-18T17:43:17.0964793Z ------------------------------------------
2019-07-18T17:43:17.0964851Z stderr:
---
2019-07-18T17:43:17.1717885Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:17.1717921Z +
2019-07-18T17:43:17.1717942Z 
2019-07-18T17:43:17.1717977Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:17.1718035Z Actual stderr saved to /tmp/compiletestLiAEmM/closures.stderr
2019-07-18T17:43:17.1718074Z To update references, run this command from build directory:
2019-07-18T17:43:17.1719122Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'closures.rs'
2019-07-18T17:43:17.1719235Z error: 1 errors occurred comparing output.
2019-07-18T17:43:17.1719280Z status: exit code: 1
2019-07-18T17:43:17.1719280Z status: exit code: 1
2019-07-18T17:43:17.1719956Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closures.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/closures.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/closures.stage-id.aux" "-A" "unused"
2019-07-18T17:43:17.1720274Z ------------------------------------------
2019-07-18T17:43:17.1720331Z 
2019-07-18T17:43:17.1720557Z ------------------------------------------
2019-07-18T17:43:17.1720603Z stderr:
---
2019-07-18T17:43:17.2201726Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:17.2220219Z +
2019-07-18T17:43:17.2220431Z 
2019-07-18T17:43:17.2220628Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:17.2221220Z Actual stderr saved to /tmp/compiletestLiAEmM/const-vec-of-fns.stderr
2019-07-18T17:43:17.2221433Z To update references, run this command from build directory:
2019-07-18T17:43:17.2222049Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'const-vec-of-fns.rs'
2019-07-18T17:43:17.2222482Z error: 1 errors occurred comparing output.
2019-07-18T17:43:17.2222591Z status: exit code: 1
2019-07-18T17:43:17.2222591Z status: exit code: 1
2019-07-18T17:43:17.2223240Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/const-vec-of-fns.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/const-vec-of-fns.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/const-vec-of-fns.stage-id.aux" "-A" "unused"
2019-07-18T17:43:17.2223695Z ------------------------------------------
2019-07-18T17:43:17.2223838Z 
2019-07-18T17:43:17.2224104Z ------------------------------------------
2019-07-18T17:43:17.2224240Z stderr:
---
2019-07-18T17:43:17.3042191Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:17.3042350Z +
2019-07-18T17:43:17.3042472Z 
2019-07-18T17:43:17.3042592Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:17.3042736Z Actual stderr saved to /tmp/compiletestLiAEmM/constants.stderr
2019-07-18T17:43:17.3042862Z To update references, run this command from build directory:
2019-07-18T17:43:17.3043418Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'constants.rs'
2019-07-18T17:43:17.3044029Z error: 1 errors occurred comparing output.
2019-07-18T17:43:17.3044138Z status: exit code: 1
2019-07-18T17:43:17.3044138Z status: exit code: 1
2019-07-18T17:43:17.3044849Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/constants.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/constants.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/constants.stage-id.aux" "-A" "unused"
2019-07-18T17:43:17.3045319Z ------------------------------------------
2019-07-18T17:43:17.3045444Z 
2019-07-18T17:43:17.3045722Z ------------------------------------------
2019-07-18T17:43:17.3045862Z stderr:
---
2019-07-18T17:43:17.4199829Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:17.4199889Z +
2019-07-18T17:43:17.4199918Z 
2019-07-18T17:43:17.4199968Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:17.4200043Z Actual stderr saved to /tmp/compiletestLiAEmM/drop_empty_slice.stderr
2019-07-18T17:43:17.4200219Z To update references, run this command from build directory:
2019-07-18T17:43:17.4200567Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'drop_empty_slice.rs'
2019-07-18T17:43:17.4200684Z error: 1 errors occurred comparing output.
2019-07-18T17:43:17.4200733Z status: exit code: 1
2019-07-18T17:43:17.4200733Z status: exit code: 1
2019-07-18T17:43:17.4201796Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/drop_empty_slice.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/drop_empty_slice.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/drop_empty_slice.stage-id.aux" "-A" "unused"
2019-07-18T17:43:17.4202096Z ------------------------------------------
2019-07-18T17:43:17.4202151Z 
2019-07-18T17:43:17.4202552Z ------------------------------------------
2019-07-18T17:43:17.4202598Z stderr:
---
2019-07-18T17:43:17.4911329Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:17.4911386Z +
2019-07-18T17:43:17.4911600Z 
2019-07-18T17:43:17.4911648Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:17.4912021Z Actual stderr saved to /tmp/compiletestLiAEmM/deriving-associated-types.stderr
2019-07-18T17:43:17.4912101Z To update references, run this command from build directory:
2019-07-18T17:43:17.4912715Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'deriving-associated-types.rs'
2019-07-18T17:43:17.4912788Z error: 1 errors occurred comparing output.
2019-07-18T17:43:17.4912849Z status: exit code: 1
2019-07-18T17:43:17.4912849Z status: exit code: 1
2019-07-18T17:43:17.4913474Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/deriving-associated-types.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/deriving-associated-types.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/deriving-associated-types.stage-id.aux" "-A" "unused"
2019-07-18T17:43:17.4913799Z ------------------------------------------
2019-07-18T17:43:17.4913834Z 
2019-07-18T17:43:17.4914062Z ------------------------------------------
2019-07-18T17:43:17.4914107Z stderr:
---
2019-07-18T17:43:17.6199334Z +
2019-07-18T17:43:17.6199750Z thread '[ui] run-pass/dst-irrefutable-bind.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-18T17:43:17.6199817Z 
2019-07-18T17:43:17.6199866Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:17.6200142Z Actual stderr saved to /tmp/compiletestLiAEmM/dst-irrefutable-bind.stderr
2019-07-18T17:43:17.6200229Z To update references, run this command from build directory:
2019-07-18T17:43:17.6200516Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'dst-irrefutable-bind.rs'
2019-07-18T17:43:17.6200620Z error: 1 errors occurred comparing output.
2019-07-18T17:43:17.6200669Z status: exit code: 1
2019-07-18T17:43:17.6200669Z status: exit code: 1
2019-07-18T17:43:17.6201529Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-irrefutable-bind.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/dst-irrefutable-bind.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/dst-irrefutable-bind.stage-id.aux" "-A" "unused"
2019-07-18T17:43:17.6201991Z ------------------------------------------
2019-07-18T17:43:17.6202018Z 
2019-07-18T17:43:17.6202214Z ------------------------------------------
2019-07-18T17:43:17.6202251Z stderr:
---
2019-07-18T17:43:17.6431709Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:17.6431759Z +
2019-07-18T17:43:17.6431788Z 
2019-07-18T17:43:17.6432028Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:17.6432470Z Actual stderr saved to /tmp/compiletestLiAEmM/dst-field-align.stderr
2019-07-18T17:43:17.6432511Z To update references, run this command from build directory:
2019-07-18T17:43:17.6432718Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'dst-field-align.rs'
2019-07-18T17:43:17.6432777Z error: 1 errors occurred comparing output.
2019-07-18T17:43:17.6432810Z status: exit code: 1
2019-07-18T17:43:17.6432810Z status: exit code: 1
2019-07-18T17:43:17.6433308Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-field-align.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/dst-field-align.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/dst-field-align.stage-id.aux" "-A" "unused"
2019-07-18T17:43:17.6433564Z ------------------------------------------
2019-07-18T17:43:17.6433590Z 
2019-07-18T17:43:17.6433751Z ------------------------------------------
2019-07-18T17:43:17.6433800Z stderr:
---
2019-07-18T17:43:17.8242399Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:17.8242435Z +
2019-07-18T17:43:17.8242472Z 
2019-07-18T17:43:17.8242507Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:17.8242699Z Actual stderr saved to /tmp/compiletestLiAEmM/dst-struct-sole.stderr
2019-07-18T17:43:17.8242766Z To update references, run this command from build directory:
2019-07-18T17:43:17.8242969Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'dst-struct-sole.rs'
2019-07-18T17:43:17.8243030Z error: 1 errors occurred comparing output.
2019-07-18T17:43:17.8243082Z status: exit code: 1
2019-07-18T17:43:17.8243082Z status: exit code: 1
2019-07-18T17:43:17.8243596Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-struct-sole.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/dst-struct-sole.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/dst-struct-sole.stage-id.aux" "-A" "unused"
2019-07-18T17:43:17.8243862Z ------------------------------------------
2019-07-18T17:43:17.8243888Z 
2019-07-18T17:43:17.8244082Z ------------------------------------------
2019-07-18T17:43:17.8244117Z stderr:
---
2019-07-18T17:43:17.8306499Z +
2019-07-18T17:43:17.8306813Z thread '[ui] run-pass/dst-raw.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-18T17:43:17.8306874Z 
2019-07-18T17:43:17.8306914Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:17.8307144Z Actual stderr saved to /tmp/compiletestLiAEmM/dst-raw.stderr
2019-07-18T17:43:17.8307213Z To update references, run this command from build directory:
2019-07-18T17:43:17.8307450Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'dst-raw.rs'
2019-07-18T17:43:17.8307531Z error: 1 errors occurred comparing output.
2019-07-18T17:43:17.8307590Z status: exit code: 1
2019-07-18T17:43:17.8307590Z status: exit code: 1
2019-07-18T17:43:17.8308625Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-raw.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/dst-raw.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/dst-raw.stage-id.aux" "-A" "unused"
2019-07-18T17:43:17.8309053Z ------------------------------------------
2019-07-18T17:43:17.8309095Z 
2019-07-18T17:43:17.8309351Z ------------------------------------------
2019-07-18T17:43:17.8309427Z stderr:
---
2019-07-18T17:43:17.9528945Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:17.9528996Z +
2019-07-18T17:43:17.9529024Z 
2019-07-18T17:43:17.9529099Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:17.9529380Z Actual stderr saved to /tmp/compiletestLiAEmM/enum-nullable-const-null-with-fields.stderr
2019-07-18T17:43:17.9529437Z To update references, run this command from build directory:
2019-07-18T17:43:17.9529728Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'enum-nullable-const-null-with-fields.rs'
2019-07-18T17:43:17.9529810Z error: 1 errors occurred comparing output.
2019-07-18T17:43:17.9529866Z status: exit code: 1
2019-07-18T17:43:17.9529866Z status: exit code: 1
2019-07-18T17:43:17.9530617Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/enum-nullable-const-null-with-fields.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/enum-nullable-const-null-with-fields.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/enum-nullable-const-null-with-fields.stage-id.aux" "-A" "unused"
2019-07-18T17:43:17.9530951Z ------------------------------------------
2019-07-18T17:43:17.9530986Z 
2019-07-18T17:43:17.9531206Z ------------------------------------------
2019-07-18T17:43:17.9531267Z stderr:
---
2019-07-18T17:43:18.1082115Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:18.1082175Z +
2019-07-18T17:43:18.1082199Z 
2019-07-18T17:43:18.1082239Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:18.1082454Z Actual stderr saved to /tmp/compiletestLiAEmM/dst-struct.stderr
2019-07-18T17:43:18.1082521Z To update references, run this command from build directory:
2019-07-18T17:43:18.1082753Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'dst-struct.rs'
2019-07-18T17:43:18.1082824Z error: 1 errors occurred comparing output.
2019-07-18T17:43:18.1082880Z status: exit code: 1
2019-07-18T17:43:18.1082880Z status: exit code: 1
2019-07-18T17:43:18.1083606Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-struct.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/dst-struct.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/dst-struct.stage-id.aux" "-A" "unused"
2019-07-18T17:43:18.1083887Z ------------------------------------------
2019-07-18T17:43:18.1083918Z 
2019-07-18T17:43:18.1084119Z ------------------------------------------
2019-07-18T17:43:18.1084167Z stderr:
---
2019-07-18T17:43:18.1192340Z +
2019-07-18T17:43:18.1256155Z 
2019-07-18T17:43:18.1263050Z thread '[ui] run-pass/enums.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-18T17:43:18.1263317Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:18.1263455Z Actual stderr saved to /tmp/compiletestLiAEmM/enums.stderr
2019-07-18T17:43:18.1263606Z To update references, run this command from build directory:
2019-07-18T17:43:18.1263972Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'enums.rs'
2019-07-18T17:43:18.1264275Z error: 1 errors occurred comparing output.
2019-07-18T17:43:18.1264396Z status: exit code: 1
2019-07-18T17:43:18.1264396Z status: exit code: 1
2019-07-18T17:43:18.1265205Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/enums.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/enums.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/enums.stage-id.aux" "-A" "unused"
2019-07-18T17:43:18.1265716Z ------------------------------------------
2019-07-18T17:43:18.1266051Z 
2019-07-18T17:43:18.1266362Z ------------------------------------------
2019-07-18T17:43:18.1266854Z stderr:
---
2019-07-18T17:43:18.2498447Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:18.2498513Z +
2019-07-18T17:43:18.2498541Z 
2019-07-18T17:43:18.2498587Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:18.2498660Z Actual stderr saved to /tmp/compiletestLiAEmM/exit.stderr
2019-07-18T17:43:18.2498710Z To update references, run this command from build directory:
2019-07-18T17:43:18.2498992Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'exit.rs'
2019-07-18T17:43:18.2499093Z error: 1 errors occurred comparing output.
2019-07-18T17:43:18.2499147Z status: exit code: 1
2019-07-18T17:43:18.2499147Z status: exit code: 1
2019-07-18T17:43:18.2500168Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/exit.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/exit.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/exit.stage-id.aux" "-A" "unused"
2019-07-18T17:43:18.2500508Z ------------------------------------------
2019-07-18T17:43:18.2500565Z 
2019-07-18T17:43:18.2500790Z ------------------------------------------
2019-07-18T17:43:18.2500837Z stderr:
---
2019-07-18T17:43:18.2513294Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:18.2513348Z +
2019-07-18T17:43:18.2513372Z 
2019-07-18T17:43:18.2513528Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:18.2513573Z Actual stderr saved to /tmp/compiletestLiAEmM/env.stderr
2019-07-18T17:43:18.2513618Z To update references, run this command from build directory:
2019-07-18T17:43:18.2513865Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'env.rs'
2019-07-18T17:43:18.2513944Z error: 1 errors occurred comparing output.
2019-07-18T17:43:18.2513985Z status: exit code: 1
2019-07-18T17:43:18.2513985Z status: exit code: 1
2019-07-18T17:43:18.2514554Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/env.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/env.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/env.stage-id.aux" "-A" "unused"
2019-07-18T17:43:18.2515324Z ------------------------------------------
2019-07-18T17:43:18.2515352Z 
2019-07-18T17:43:18.2515864Z ------------------------------------------
2019-07-18T17:43:18.2515900Z stderr:
---
2019-07-18T17:43:18.3680287Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:18.3680371Z +
2019-07-18T17:43:18.3680401Z 
2019-07-18T17:43:18.3680450Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:18.3680525Z Actual stderr saved to /tmp/compiletestLiAEmM/extern_types.stderr
2019-07-18T17:43:18.3680581Z To update references, run this command from build directory:
2019-07-18T17:43:18.3680892Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'extern_types.rs'
2019-07-18T17:43:18.3681022Z error: 1 errors occurred comparing output.
2019-07-18T17:43:18.3681073Z status: exit code: 1
2019-07-18T17:43:18.3681073Z status: exit code: 1
2019-07-18T17:43:18.3682134Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/extern_types.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/extern_types.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/extern_types.stage-id.aux" "-A" "unused"
2019-07-18T17:43:18.3682453Z ------------------------------------------
2019-07-18T17:43:18.3682485Z 
2019-07-18T17:43:18.3682701Z ------------------------------------------
2019-07-18T17:43:18.3682742Z stderr:
---
2019-07-18T17:43:18.4124855Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:18.4124918Z +
2019-07-18T17:43:18.4124942Z 
2019-07-18T17:43:18.4125002Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:18.4125046Z Actual stderr saved to /tmp/compiletestLiAEmM/float_fast_math.stderr
2019-07-18T17:43:18.4125090Z To update references, run this command from build directory:
2019-07-18T17:43:18.4125381Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'float_fast_math.rs'
2019-07-18T17:43:18.4125458Z error: 1 errors occurred comparing output.
2019-07-18T17:43:18.4125524Z status: exit code: 1
2019-07-18T17:43:18.4125524Z status: exit code: 1
2019-07-18T17:43:18.4126272Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/float_fast_math.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/float_fast_math.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/float_fast_math.stage-id.aux" "-A" "unused"
2019-07-18T17:43:18.4126790Z ------------------------------------------
2019-07-18T17:43:18.4126827Z 
2019-07-18T17:43:18.4127210Z ------------------------------------------
2019-07-18T17:43:18.4127274Z stderr:
---
2019-07-18T17:43:18.5684357Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:18.5684403Z +
2019-07-18T17:43:18.5684428Z 
2019-07-18T17:43:18.5684485Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:18.5684714Z Actual stderr saved to /tmp/compiletestLiAEmM/foreign-fn-linkname.stderr
2019-07-18T17:43:18.5684763Z To update references, run this command from build directory:
2019-07-18T17:43:18.5685007Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'foreign-fn-linkname.rs'
2019-07-18T17:43:18.5685087Z error: 1 errors occurred comparing output.
2019-07-18T17:43:18.5685127Z status: exit code: 1
2019-07-18T17:43:18.5685127Z status: exit code: 1
2019-07-18T17:43:18.5685716Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/foreign-fn-linkname.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/foreign-fn-linkname.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/foreign-fn-linkname.stage-id.aux" "-A" "unused"
2019-07-18T17:43:18.5686010Z ------------------------------------------
2019-07-18T17:43:18.5686041Z 
2019-07-18T17:43:18.5686391Z ------------------------------------------
2019-07-18T17:43:18.5686449Z stderr:
---
2019-07-18T17:43:18.6121107Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:18.6121168Z +
2019-07-18T17:43:18.6121197Z 
2019-07-18T17:43:18.6121246Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:18.6121325Z Actual stderr saved to /tmp/compiletestLiAEmM/floats.stderr
2019-07-18T17:43:18.6121391Z To update references, run this command from build directory:
2019-07-18T17:43:18.6121700Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'floats.rs'
2019-07-18T17:43:18.6121817Z error: 1 errors occurred comparing output.
2019-07-18T17:43:18.6121866Z status: exit code: 1
2019-07-18T17:43:18.6121866Z status: exit code: 1
2019-07-18T17:43:18.6122748Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/floats.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/floats.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/floats.stage-id.aux" "-A" "unused"
2019-07-18T17:43:18.6123038Z ------------------------------------------
2019-07-18T17:43:18.6123089Z 
2019-07-18T17:43:18.6123274Z ------------------------------------------
2019-07-18T17:43:18.6123314Z stderr:
---
2019-07-18T17:43:18.7194471Z -hello00000
2019-07-18T17:43:18.7194633Z -
2019-07-18T17:43:18.7194680Z 
2019-07-18T17:43:18.7194722Z The actual stdout differed from the expected stdout.
2019-07-18T17:43:18.7194766Z Actual stdout saved to /tmp/compiletestLiAEmM/format.stdout
2019-07-18T17:43:18.7194891Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-18T17:43:18.7195125Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-18T17:43:18.7195191Z      |
2019-07-18T17:43:18.7195230Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-18T17:43:18.7198894Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:18.7198978Z +
2019-07-18T17:43:18.7199020Z 
2019-07-18T17:43:18.7199092Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:18.7199143Z Actual stderr saved to /tmp/compiletestLiAEmM/format.stderr
2019-07-18T17:43:18.7199195Z To update references, run this command from build directory:
2019-07-18T17:43:18.7199555Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'format.rs'
2019-07-18T17:43:18.7199644Z error: 2 errors occurred comparing output.
2019-07-18T17:43:18.7199692Z status: exit code: 1
2019-07-18T17:43:18.7199692Z status: exit code: 1
2019-07-18T17:43:18.7200476Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/format.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/format.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/format.stage-id.aux" "-A" "unused"
2019-07-18T17:43:18.7200894Z ------------------------------------------
2019-07-18T17:43:18.7200938Z 
2019-07-18T17:43:18.7201186Z ------------------------------------------
2019-07-18T17:43:18.7201260Z stderr:
---
2019-07-18T17:43:18.7361336Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:18.7361430Z +
2019-07-18T17:43:18.7361461Z 
2019-07-18T17:43:18.7361512Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:18.7361567Z Actual stderr saved to /tmp/compiletestLiAEmM/from_utf8.stderr
2019-07-18T17:43:18.7361642Z To update references, run this command from build directory:
2019-07-18T17:43:18.7361957Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'from_utf8.rs'
2019-07-18T17:43:18.7362234Z error: 1 errors occurred comparing output.
2019-07-18T17:43:18.7362390Z status: exit code: 1
2019-07-18T17:43:18.7362390Z status: exit code: 1
2019-07-18T17:43:18.7363030Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/from_utf8.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/from_utf8.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/from_utf8.stage-id.aux" "-A" "unused"
2019-07-18T17:43:18.7363379Z ------------------------------------------
2019-07-18T17:43:18.7363416Z 
2019-07-18T17:43:18.7363835Z ------------------------------------------
2019-07-18T17:43:18.7363883Z stderr:
---
2019-07-18T17:43:18.9342242Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:18.9342284Z +
2019-07-18T17:43:18.9342305Z 
2019-07-18T17:43:18.9342344Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:18.9342404Z Actual stderr saved to /tmp/compiletestLiAEmM/generator.stderr
2019-07-18T17:43:18.9342534Z To update references, run this command from build directory:
2019-07-18T17:43:18.9342782Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'generator.rs'
2019-07-18T17:43:18.9342944Z error: 1 errors occurred comparing output.
2019-07-18T17:43:18.9342983Z status: exit code: 1
2019-07-18T17:43:18.9342983Z status: exit code: 1
2019-07-18T17:43:18.9343546Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/generator.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/generator.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/generator.stage-id.aux" "-A" "unused"
2019-07-18T17:43:18.9343990Z ------------------------------------------
2019-07-18T17:43:18.9344019Z 
2019-07-18T17:43:18.9344204Z ------------------------------------------
2019-07-18T17:43:18.9344251Z stderr:
---
2019-07-18T17:43:18.9355596Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:18.9355638Z +
2019-07-18T17:43:18.9355660Z 
2019-07-18T17:43:18.9355706Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:18.9355750Z Actual stderr saved to /tmp/compiletestLiAEmM/function_pointers.stderr
2019-07-18T17:43:18.9355792Z To update references, run this command from build directory:
2019-07-18T17:43:18.9356101Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'function_pointers.rs'
2019-07-18T17:43:18.9356176Z error: 1 errors occurred comparing output.
2019-07-18T17:43:18.9356213Z status: exit code: 1
2019-07-18T17:43:18.9356213Z status: exit code: 1
2019-07-18T17:43:18.9356776Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/function_pointers.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/function_pointers.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/function_pointers.stage-id.aux" "-A" "unused"
2019-07-18T17:43:18.9357067Z ------------------------------------------
2019-07-18T17:43:18.9357097Z 
2019-07-18T17:43:18.9357290Z ------------------------------------------
2019-07-18T17:43:18.9357328Z stderr:
---
2019-07-18T17:43:19.0607508Z -Hello, world!
2019-07-18T17:43:19.0607691Z -
2019-07-18T17:43:19.0614478Z 
2019-07-18T17:43:19.0614588Z The actual stdout differed from the expected stdout.
2019-07-18T17:43:19.0620139Z Actual stdout saved to /tmp/compiletestLiAEmM/hello.stdout
2019-07-18T17:43:19.0625706Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-18T17:43:19.0626309Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-18T17:43:19.0626381Z      |
2019-07-18T17:43:19.0626426Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-18T17:43:19.0647568Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:19.0647608Z +
2019-07-18T17:43:19.0647631Z 
2019-07-18T17:43:19.0647688Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:19.0647740Z Actual stderr saved to /tmp/compiletestLiAEmM/hello.stderr
2019-07-18T17:43:19.0647781Z To update references, run this command from build directory:
2019-07-18T17:43:19.0648022Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'hello.rs'
2019-07-18T17:43:19.0648089Z error: 2 errors occurred comparing output.
2019-07-18T17:43:19.0648126Z status: exit code: 1
2019-07-18T17:43:19.0648126Z status: exit code: 1
2019-07-18T17:43:19.0648662Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/hello.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/hello.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/hello.stage-id.aux" "-A" "unused"
2019-07-18T17:43:19.0648944Z ------------------------------------------
2019-07-18T17:43:19.0649056Z 
2019-07-18T17:43:19.0649271Z ------------------------------------------
2019-07-18T17:43:19.0649311Z stderr:
---
2019-07-18T17:43:19.1077670Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:19.1077725Z +
2019-07-18T17:43:19.1077929Z 
2019-07-18T17:43:19.1077972Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:19.1078286Z Actual stderr saved to /tmp/compiletestLiAEmM/heap.stderr
2019-07-18T17:43:19.1078336Z To update references, run this command from build directory:
2019-07-18T17:43:19.1078847Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'heap.rs'
2019-07-18T17:43:19.1078987Z error: 1 errors occurred comparing output.
2019-07-18T17:43:19.1079036Z status: exit code: 1
2019-07-18T17:43:19.1079036Z status: exit code: 1
2019-07-18T17:43:19.1079699Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/heap.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/heap.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/heap.stage-id.aux" "-A" "unused"
2019-07-18T17:43:19.1080165Z ------------------------------------------
2019-07-18T17:43:19.1080222Z 
2019-07-18T17:43:19.1080465Z ------------------------------------------
2019-07-18T17:43:19.1080514Z stderr:
---
2019-07-18T17:43:19.6763087Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:19.6763130Z +
2019-07-18T17:43:19.6763171Z 
2019-07-18T17:43:19.6763218Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:19.6763446Z Actual stderr saved to /tmp/compiletestLiAEmM/integer-ops.stderr
2019-07-18T17:43:19.6763491Z To update references, run this command from build directory:
2019-07-18T17:43:19.6763727Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'integer-ops.rs'
2019-07-18T17:43:19.6763794Z error: 1 errors occurred comparing output.
2019-07-18T17:43:19.6763846Z status: exit code: 1
2019-07-18T17:43:19.6763846Z status: exit code: 1
2019-07-18T17:43:19.6764384Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/integer-ops.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/integer-ops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/integer-ops.stage-id.aux" "-A" "unused"
2019-07-18T17:43:19.6764776Z ------------------------------------------
2019-07-18T17:43:19.6764807Z 
2019-07-18T17:43:19.6764997Z ------------------------------------------
2019-07-18T17:43:19.6765051Z stderr:
---
2019-07-18T17:43:20.1764977Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:20.1765041Z +
2019-07-18T17:43:20.1765066Z 
2019-07-18T17:43:20.1765104Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:20.1765380Z Actual stderr saved to /tmp/compiletestLiAEmM/intrinsics-math.stderr
2019-07-18T17:43:20.1765433Z To update references, run this command from build directory:
2019-07-18T17:43:20.1765673Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'intrinsics-math.rs'
2019-07-18T17:43:20.1765766Z error: 1 errors occurred comparing output.
2019-07-18T17:43:20.1765812Z status: exit code: 1
2019-07-18T17:43:20.1765812Z status: exit code: 1
2019-07-18T17:43:20.1766382Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics-math.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/intrinsics-math.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/intrinsics-math.stage-id.aux" "-A" "unused"
2019-07-18T17:43:20.1766826Z ------------------------------------------
2019-07-18T17:43:20.1766861Z 
2019-07-18T17:43:20.1767079Z ------------------------------------------
2019-07-18T17:43:20.1767122Z stderr:
---
2019-07-18T17:43:20.3565891Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:20.3565942Z +
2019-07-18T17:43:20.3565967Z 
2019-07-18T17:43:20.3566008Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:20.3566070Z Actual stderr saved to /tmp/compiletestLiAEmM/intrinsics.stderr
2019-07-18T17:43:20.3566114Z To update references, run this command from build directory:
2019-07-18T17:43:20.3566371Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'intrinsics.rs'
2019-07-18T17:43:20.3566474Z error: 1 errors occurred comparing output.
2019-07-18T17:43:20.3566513Z status: exit code: 1
2019-07-18T17:43:20.3566513Z status: exit code: 1
2019-07-18T17:43:20.3567072Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/intrinsics.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/intrinsics.stage-id.aux" "-A" "unused"
2019-07-18T17:43:20.3567509Z ------------------------------------------
2019-07-18T17:43:20.3567563Z 
2019-07-18T17:43:20.3567787Z ------------------------------------------
2019-07-18T17:43:20.3567832Z stderr:
---
2019-07-18T17:43:20.5686481Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:20.5686526Z +
2019-07-18T17:43:20.5686549Z 
2019-07-18T17:43:20.5686586Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:20.5686645Z Actual stderr saved to /tmp/compiletestLiAEmM/ints.stderr
2019-07-18T17:43:20.5686685Z To update references, run this command from build directory:
2019-07-18T17:43:20.5686929Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'ints.rs'
2019-07-18T17:43:20.5687019Z error: 1 errors occurred comparing output.
2019-07-18T17:43:20.5687057Z status: exit code: 1
2019-07-18T17:43:20.5687057Z status: exit code: 1
2019-07-18T17:43:20.5687581Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ints.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/ints.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/ints.stage-id.aux" "-A" "unused"
2019-07-18T17:43:20.5687998Z ------------------------------------------
2019-07-18T17:43:20.5688033Z 
2019-07-18T17:43:20.5688238Z ------------------------------------------
2019-07-18T17:43:20.5688280Z stderr:
---
2019-07-18T17:43:20.6980865Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:20.6981221Z +
2019-07-18T17:43:20.6981479Z 
2019-07-18T17:43:20.6981738Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:20.6982289Z Actual stderr saved to /tmp/compiletestLiAEmM/issue-15063.stderr
2019-07-18T17:43:20.6983005Z To update references, run this command from build directory:
2019-07-18T17:43:20.6983487Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'issue-15063.rs'
2019-07-18T17:43:20.6984186Z error: 1 errors occurred comparing output.
2019-07-18T17:43:20.6984393Z status: exit code: 1
2019-07-18T17:43:20.6984393Z status: exit code: 1
2019-07-18T17:43:20.6985214Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15063.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/issue-15063.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/issue-15063.stage-id.aux" "-A" "unused"
2019-07-18T17:43:20.6986246Z ------------------------------------------
2019-07-18T17:43:20.6986925Z 
2019-07-18T17:43:20.6987527Z ------------------------------------------
2019-07-18T17:43:20.6987744Z stderr:
---
2019-07-18T17:43:20.8823622Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:20.8823859Z +
2019-07-18T17:43:20.8824116Z 
2019-07-18T17:43:20.8824493Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:20.8824937Z Actual stderr saved to /tmp/compiletestLiAEmM/issue-15080.stderr
2019-07-18T17:43:20.8825400Z To update references, run this command from build directory:
2019-07-18T17:43:20.8827154Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'issue-15080.rs'
2019-07-18T17:43:20.8827243Z error: 1 errors occurred comparing output.
2019-07-18T17:43:20.8827283Z status: exit code: 1
2019-07-18T17:43:20.8827283Z status: exit code: 1
2019-07-18T17:43:20.8827886Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15080.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/issue-15080.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/issue-15080.stage-id.aux" "-A" "unused"
2019-07-18T17:43:20.8828377Z ------------------------------------------
2019-07-18T17:43:20.8828414Z 
2019-07-18T17:43:20.8829248Z ------------------------------------------
2019-07-18T17:43:20.8829313Z stderr:
---
2019-07-18T17:43:21.0667936Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:21.0668008Z +
2019-07-18T17:43:21.0668033Z 
2019-07-18T17:43:21.0668075Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:21.0668832Z Actual stderr saved to /tmp/compiletestLiAEmM/issue-15523-big.stderr
2019-07-18T17:43:21.0668925Z To update references, run this command from build directory:
2019-07-18T17:43:21.0669217Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'issue-15523-big.rs'
2019-07-18T17:43:21.0669469Z error: 1 errors occurred comparing output.
2019-07-18T17:43:21.0669518Z status: exit code: 1
2019-07-18T17:43:21.0669518Z status: exit code: 1
2019-07-18T17:43:21.0670242Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15523-big.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/issue-15523-big.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/issue-15523-big.stage-id.aux" "-A" "unused"
2019-07-18T17:43:21.0670616Z ------------------------------------------
2019-07-18T17:43:21.0670656Z 
2019-07-18T17:43:21.0670929Z ------------------------------------------
2019-07-18T17:43:21.0670984Z stderr:
---
2019-07-18T17:43:21.2298059Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:21.2298120Z +
2019-07-18T17:43:21.2298151Z 
2019-07-18T17:43:21.2298189Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:21.2299079Z Actual stderr saved to /tmp/compiletestLiAEmM/issue-17877.stderr
2019-07-18T17:43:21.2299144Z To update references, run this command from build directory:
2019-07-18T17:43:21.2299743Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'issue-17877.rs'
2019-07-18T17:43:21.2299854Z error: 1 errors occurred comparing output.
2019-07-18T17:43:21.2300044Z status: exit code: 1
2019-07-18T17:43:21.2300044Z status: exit code: 1
2019-07-18T17:43:21.2300758Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-17877.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/issue-17877.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/issue-17877.stage-id.aux" "-A" "unused"
2019-07-18T17:43:21.2301094Z ------------------------------------------
2019-07-18T17:43:21.2301131Z 
2019-07-18T17:43:21.2301374Z ------------------------------------------
2019-07-18T17:43:21.2301420Z stderr:
---
2019-07-18T17:43:21.3723795Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:21.3723854Z +
2019-07-18T17:43:21.3723876Z 
2019-07-18T17:43:21.3723913Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:21.3724128Z Actual stderr saved to /tmp/compiletestLiAEmM/issue-20575.stderr
2019-07-18T17:43:21.3724188Z To update references, run this command from build directory:
2019-07-18T17:43:21.3724398Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'issue-20575.rs'
2019-07-18T17:43:21.3724566Z error: 1 errors occurred comparing output.
2019-07-18T17:43:21.3724600Z status: exit code: 1
2019-07-18T17:43:21.3724600Z status: exit code: 1
2019-07-18T17:43:21.3725110Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-20575.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/issue-20575.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/issue-20575.stage-id.aux" "-A" "unused"
2019-07-18T17:43:21.3725370Z ------------------------------------------
2019-07-18T17:43:21.3725399Z 
2019-07-18T17:43:21.3725591Z ------------------------------------------
2019-07-18T17:43:21.3725627Z stderr:
---
2019-07-18T17:43:21.5513717Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:21.5513762Z +
2019-07-18T17:43:21.5513783Z 
2019-07-18T17:43:21.5513839Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:21.5514055Z Actual stderr saved to /tmp/compiletestLiAEmM/issue-23261.stderr
2019-07-18T17:43:21.5514102Z To update references, run this command from build directory:
2019-07-18T17:43:21.5514446Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'issue-23261.rs'
2019-07-18T17:43:21.5514516Z error: 1 errors occurred comparing output.
2019-07-18T17:43:21.5514553Z status: exit code: 1
2019-07-18T17:43:21.5514553Z status: exit code: 1
2019-07-18T17:43:21.5515106Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-23261.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/issue-23261.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/issue-23261.stage-id.aux" "-A" "unused"
2019-07-18T17:43:21.5515415Z ------------------------------------------
2019-07-18T17:43:21.5515447Z 
2019-07-18T17:43:21.5515643Z ------------------------------------------
2019-07-18T17:43:21.5515704Z stderr:
---
2019-07-18T17:43:21.7244990Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:21.7245048Z +
2019-07-18T17:43:21.7245069Z 
2019-07-18T17:43:21.7245106Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:21.7245327Z Actual stderr saved to /tmp/compiletestLiAEmM/issue-26709.stderr
2019-07-18T17:43:21.7245462Z To update references, run this command from build directory:
2019-07-18T17:43:21.7245691Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'issue-26709.rs'
2019-07-18T17:43:21.7245770Z error: 1 errors occurred comparing output.
2019-07-18T17:43:21.7245805Z status: exit code: 1
2019-07-18T17:43:21.7245805Z status: exit code: 1
2019-07-18T17:43:21.7246326Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-26709.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/issue-26709.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/issue-26709.stage-id.aux" "-A" "unused"
2019-07-18T17:43:21.7246590Z ------------------------------------------
2019-07-18T17:43:21.7246631Z 
2019-07-18T17:43:21.7246810Z ------------------------------------------
2019-07-18T17:43:21.7246854Z stderr:
---
2019-07-18T17:43:21.8730190Z +
2019-07-18T17:43:21.8730581Z thread '[ui] run-pass/issue-27901.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-18T17:43:21.8730738Z 
2019-07-18T17:43:21.8730806Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:21.8731111Z Actual stderr saved to /tmp/compiletestLiAEmM/issue-27901.stderr
2019-07-18T17:43:21.8731170Z To update references, run this command from build directory:
2019-07-18T17:43:21.8731453Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'issue-27901.rs'
2019-07-18T17:43:21.8731559Z error: 1 errors occurred comparing output.
2019-07-18T17:43:21.8731616Z status: exit code: 1
2019-07-18T17:43:21.8731616Z status: exit code: 1
2019-07-18T17:43:21.8732529Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-27901.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/issue-27901.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/issue-27901.stage-id.aux" "-A" "unused"
2019-07-18T17:43:21.8732786Z ------------------------------------------
2019-07-18T17:43:21.8732812Z 
2019-07-18T17:43:21.8732978Z ------------------------------------------
2019-07-18T17:43:21.8733012Z stderr:
---
2019-07-18T17:43:21.8787242Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:21.8787279Z +
2019-07-18T17:43:21.8787299Z 
2019-07-18T17:43:21.8787350Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:21.8787637Z Actual stderr saved to /tmp/compiletestLiAEmM/intrinsics-integer.stderr
2019-07-18T17:43:21.8787680Z To update references, run this command from build directory:
2019-07-18T17:43:21.8787908Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'intrinsics-integer.rs'
2019-07-18T17:43:21.8787969Z error: 1 errors occurred comparing output.
2019-07-18T17:43:21.8788021Z status: exit code: 1
2019-07-18T17:43:21.8788021Z status: exit code: 1
2019-07-18T17:43:21.8788540Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics-integer.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/intrinsics-integer.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/intrinsics-integer.stage-id.aux" "-A" "unused"
2019-07-18T17:43:21.8789270Z ------------------------------------------
2019-07-18T17:43:21.8789307Z 
2019-07-18T17:43:21.8789534Z ------------------------------------------
2019-07-18T17:43:21.8789599Z stderr:
---
2019-07-18T17:43:22.0081006Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:22.0081171Z +
2019-07-18T17:43:22.0081202Z 
2019-07-18T17:43:22.0081267Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:22.0081595Z Actual stderr saved to /tmp/compiletestLiAEmM/issue-30530.stderr
2019-07-18T17:43:22.0081653Z To update references, run this command from build directory:
2019-07-18T17:43:22.0081966Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'issue-30530.rs'
2019-07-18T17:43:22.0082061Z error: 1 errors occurred comparing output.
2019-07-18T17:43:22.0082266Z status: exit code: 1
2019-07-18T17:43:22.0082266Z status: exit code: 1
2019-07-18T17:43:22.0083017Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-30530.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/issue-30530.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/issue-30530.stage-id.aux" "-A" "unused"
2019-07-18T17:43:22.0083312Z ------------------------------------------
2019-07-18T17:43:22.0083342Z 
2019-07-18T17:43:22.0083532Z ------------------------------------------
2019-07-18T17:43:22.0083587Z stderr:
---
2019-07-18T17:43:22.0478981Z +
2019-07-18T17:43:22.0479370Z thread '[ui] run-pass/issue-29746.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-18T17:43:22.0479415Z 
2019-07-18T17:43:22.0479477Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:22.0479728Z Actual stderr saved to /tmp/compiletestLiAEmM/issue-29746.stderr
2019-07-18T17:43:22.0479796Z To update references, run this command from build directory:
2019-07-18T17:43:22.0480080Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'issue-29746.rs'
2019-07-18T17:43:22.0480160Z error: 1 errors occurred comparing output.
2019-07-18T17:43:22.0480206Z status: exit code: 1
2019-07-18T17:43:22.0480206Z status: exit code: 1
2019-07-18T17:43:22.0480877Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-29746.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/issue-29746.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/issue-29746.stage-id.aux" "-A" "unused"
2019-07-18T17:43:22.0481218Z ------------------------------------------
2019-07-18T17:43:22.0481253Z 
2019-07-18T17:43:22.0481485Z ------------------------------------------
2019-07-18T17:43:22.0481531Z stderr:
---
2019-07-18T17:43:22.1599332Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:22.1599380Z +
2019-07-18T17:43:22.1599407Z 
2019-07-18T17:43:22.1599452Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:22.1599734Z Actual stderr saved to /tmp/compiletestLiAEmM/issue-31267-additional.stderr
2019-07-18T17:43:22.1599791Z To update references, run this command from build directory:
2019-07-18T17:43:22.1600073Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'issue-31267-additional.rs'
2019-07-18T17:43:22.1600172Z error: 1 errors occurred comparing output.
2019-07-18T17:43:22.1600218Z status: exit code: 1
2019-07-18T17:43:22.1600218Z status: exit code: 1
2019-07-18T17:43:22.1600922Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-31267-additional.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/issue-31267-additional.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/issue-31267-additional.stage-id.aux" "-A" "unused"
2019-07-18T17:43:22.1601246Z ------------------------------------------
2019-07-18T17:43:22.1601298Z 
2019-07-18T17:43:22.1601524Z ------------------------------------------
2019-07-18T17:43:22.1601578Z stderr:
---
2019-07-18T17:43:22.1695561Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:22.1695603Z +
2019-07-18T17:43:22.1695676Z 
2019-07-18T17:43:22.1695738Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:22.1695982Z Actual stderr saved to /tmp/compiletestLiAEmM/issue-33387.stderr
2019-07-18T17:43:22.1696030Z To update references, run this command from build directory:
2019-07-18T17:43:22.1696282Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'issue-33387.rs'
2019-07-18T17:43:22.1696353Z error: 1 errors occurred comparing output.
2019-07-18T17:43:22.1696407Z status: exit code: 1
2019-07-18T17:43:22.1696407Z status: exit code: 1
2019-07-18T17:43:22.1697436Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-33387.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/issue-33387.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/issue-33387.stage-id.aux" "-A" "unused"
2019-07-18T17:43:22.1697708Z ------------------------------------------
2019-07-18T17:43:22.1697742Z 
2019-07-18T17:43:22.1697916Z ------------------------------------------
2019-07-18T17:43:22.1698145Z stderr:
---
2019-07-18T17:43:22.2821917Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:22.2859971Z +
2019-07-18T17:43:22.2861665Z 
2019-07-18T17:43:22.2862311Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:22.2863053Z Actual stderr saved to /tmp/compiletestLiAEmM/issue-34571.stderr
2019-07-18T17:43:22.2863297Z To update references, run this command from build directory:
2019-07-18T17:43:22.2863704Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'issue-34571.rs'
2019-07-18T17:43:22.2864013Z error: 1 errors occurred comparing output.
2019-07-18T17:43:22.2864176Z status: exit code: 1
2019-07-18T17:43:22.2864176Z status: exit code: 1
2019-07-18T17:43:22.2865061Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-34571.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/issue-34571.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/issue-34571.stage-id.aux" "-A" "unused"
2019-07-18T17:43:22.2865890Z ------------------------------------------
2019-07-18T17:43:22.2866082Z 
2019-07-18T17:43:22.2866435Z ------------------------------------------
2019-07-18T17:43:22.2866622Z stderr:
---
2019-07-18T17:43:22.3136309Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:22.3136699Z +
2019-07-18T17:43:22.3138125Z 
2019-07-18T17:43:22.3139171Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:22.3151694Z Actual stderr saved to /tmp/compiletestLiAEmM/issue-35815.stderr
2019-07-18T17:43:22.3152000Z To update references, run this command from build directory:
2019-07-18T17:43:22.3152743Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'issue-35815.rs'
2019-07-18T17:43:22.3153096Z error: 1 errors occurred comparing output.
2019-07-18T17:43:22.3153232Z status: exit code: 1
2019-07-18T17:43:22.3153232Z status: exit code: 1
2019-07-18T17:43:22.3153877Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-35815.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/issue-35815.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/issue-35815.stage-id.aux" "-A" "unused"
2019-07-18T17:43:22.3154358Z ------------------------------------------
2019-07-18T17:43:22.3154485Z 
2019-07-18T17:43:22.3154787Z ------------------------------------------
2019-07-18T17:43:22.3154920Z stderr:
---
2019-07-18T17:43:22.4429235Z -S { s: 5 }
2019-07-18T17:43:22.4429421Z -
2019-07-18T17:43:22.4429450Z 
2019-07-18T17:43:22.4429497Z The actual stdout differed from the expected stdout.
2019-07-18T17:43:22.4429765Z Actual stdout saved to /tmp/compiletestLiAEmM/issue-3794.stdout
2019-07-18T17:43:22.4429876Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-18T17:43:22.4430157Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-18T17:43:22.4430205Z      |
2019-07-18T17:43:22.4430251Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-18T17:43:22.4433951Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:22.4433994Z +
2019-07-18T17:43:22.4434033Z 
2019-07-18T17:43:22.4434074Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:22.4434304Z Actual stderr saved to /tmp/compiletestLiAEmM/issue-3794.stderr
2019-07-18T17:43:22.4434426Z To update references, run this command from build directory:
2019-07-18T17:43:22.4434700Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'issue-3794.rs'
2019-07-18T17:43:22.4434770Z error: 2 errors occurred comparing output.
2019-07-18T17:43:22.4434827Z status: exit code: 1
2019-07-18T17:43:22.4434827Z status: exit code: 1
2019-07-18T17:43:22.4435401Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-3794.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/issue-3794.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/issue-3794.stage-id.aux" "-A" "unused"
2019-07-18T17:43:22.4435697Z ------------------------------------------
2019-07-18T17:43:22.4435735Z 
2019-07-18T17:43:22.4435942Z ------------------------------------------
2019-07-18T17:43:22.4436000Z stderr:
---
2019-07-18T17:43:22.4459800Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:22.4460030Z +
2019-07-18T17:43:22.4460157Z 
2019-07-18T17:43:22.4460315Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:22.4460743Z Actual stderr saved to /tmp/compiletestLiAEmM/issue-36278-prefix-nesting.stderr
2019-07-18T17:43:22.4460929Z To update references, run this command from build directory:
2019-07-18T17:43:22.4461330Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'issue-36278-prefix-nesting.rs'
2019-07-18T17:43:22.4461691Z error: 1 errors occurred comparing output.
2019-07-18T17:43:22.4462058Z status: exit code: 1
2019-07-18T17:43:22.4462058Z status: exit code: 1
2019-07-18T17:43:22.4462879Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-36278-prefix-nesting.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/issue-36278-prefix-nesting.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/issue-36278-prefix-nesting.stage-id.aux" "-A" "unused"
2019-07-18T17:43:22.4464382Z ------------------------------------------
2019-07-18T17:43:22.4464415Z 
2019-07-18T17:43:22.4464600Z ------------------------------------------
2019-07-18T17:43:22.4464658Z stderr:
---
2019-07-18T17:43:22.5636452Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:22.5636488Z +
2019-07-18T17:43:22.5636508Z 
2019-07-18T17:43:22.5636558Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:22.5636756Z Actual stderr saved to /tmp/compiletestLiAEmM/issue-53728.stderr
2019-07-18T17:43:22.5636797Z To update references, run this command from build directory:
2019-07-18T17:43:22.5637029Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'issue-53728.rs'
2019-07-18T17:43:22.5637091Z error: 1 errors occurred comparing output.
2019-07-18T17:43:22.5637141Z status: exit code: 1
2019-07-18T17:43:22.5637141Z status: exit code: 1
2019-07-18T17:43:22.5637702Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-53728.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/issue-53728.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/issue-53728.stage-id.aux" "-A" "unused"
2019-07-18T17:43:22.5637991Z ------------------------------------------
2019-07-18T17:43:22.5638019Z 
2019-07-18T17:43:22.5638762Z ------------------------------------------
2019-07-18T17:43:22.5638850Z stderr:
---
2019-07-18T17:43:22.5736131Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:22.5736167Z +
2019-07-18T17:43:22.5736367Z 
2019-07-18T17:43:22.5736403Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:22.5736616Z Actual stderr saved to /tmp/compiletestLiAEmM/issue-5917.stderr
2019-07-18T17:43:22.5736667Z To update references, run this command from build directory:
2019-07-18T17:43:22.5736873Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'issue-5917.rs'
2019-07-18T17:43:22.5736954Z error: 1 errors occurred comparing output.
2019-07-18T17:43:22.5736990Z status: exit code: 1
2019-07-18T17:43:22.5736990Z status: exit code: 1
2019-07-18T17:43:22.5737604Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-5917.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/issue-5917.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/issue-5917.stage-id.aux" "-A" "unused"
2019-07-18T17:43:22.5737903Z ------------------------------------------
2019-07-18T17:43:22.5737932Z 
2019-07-18T17:43:22.5738118Z ------------------------------------------
2019-07-18T17:43:22.5738155Z stderr:
---
2019-07-18T17:43:22.6794508Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:22.6794566Z +
2019-07-18T17:43:22.6794589Z 
2019-07-18T17:43:22.6794626Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:22.6794851Z Actual stderr saved to /tmp/compiletestLiAEmM/issue-miri-184.stderr
2019-07-18T17:43:22.6794914Z To update references, run this command from build directory:
2019-07-18T17:43:22.6795134Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'issue-miri-184.rs'
2019-07-18T17:43:22.6795198Z error: 1 errors occurred comparing output.
2019-07-18T17:43:22.6795256Z status: exit code: 1
2019-07-18T17:43:22.6795256Z status: exit code: 1
2019-07-18T17:43:22.6795842Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-miri-184.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/issue-miri-184.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/issue-miri-184.stage-id.aux" "-A" "unused"
2019-07-18T17:43:22.6796152Z ------------------------------------------
2019-07-18T17:43:22.6796181Z 
2019-07-18T17:43:22.6796385Z ------------------------------------------
2019-07-18T17:43:22.6796423Z stderr:
---
2019-07-18T17:43:22.7887890Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:22.7887933Z +
2019-07-18T17:43:22.7887973Z 
2019-07-18T17:43:22.7888012Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:22.7888055Z Actual stderr saved to /tmp/compiletestLiAEmM/iter.stderr
2019-07-18T17:43:22.7888105Z To update references, run this command from build directory:
2019-07-18T17:43:22.7888348Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'iter.rs'
2019-07-18T17:43:22.7888416Z error: 1 errors occurred comparing output.
2019-07-18T17:43:22.7889070Z status: exit code: 1
2019-07-18T17:43:22.7889070Z status: exit code: 1
2019-07-18T17:43:22.7889912Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/iter.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/iter.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/iter.stage-id.aux" "-A" "unused"
2019-07-18T17:43:22.7890280Z ------------------------------------------
2019-07-18T17:43:22.7890318Z 
2019-07-18T17:43:22.7890539Z ------------------------------------------
2019-07-18T17:43:22.7890616Z stderr:
---
2019-07-18T17:43:22.8321811Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:22.8321880Z +
2019-07-18T17:43:22.8321909Z 
2019-07-18T17:43:22.8321955Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:22.8322428Z Actual stderr saved to /tmp/compiletestLiAEmM/last-use-in-cap-clause.stderr
2019-07-18T17:43:22.8322483Z To update references, run this command from build directory:
2019-07-18T17:43:22.8322738Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'last-use-in-cap-clause.rs'
2019-07-18T17:43:22.8322912Z error: 1 errors occurred comparing output.
2019-07-18T17:43:22.8322964Z status: exit code: 1
2019-07-18T17:43:22.8322964Z status: exit code: 1
2019-07-18T17:43:22.8323910Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/last-use-in-cap-clause.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/last-use-in-cap-clause.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/last-use-in-cap-clause.stage-id.aux" "-A" "unused"
2019-07-18T17:43:22.8324267Z ------------------------------------------
2019-07-18T17:43:22.8324305Z 
2019-07-18T17:43:22.8324526Z ------------------------------------------
2019-07-18T17:43:22.8324572Z stderr:
---
2019-07-18T17:43:23.0233609Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:23.0233657Z +
2019-07-18T17:43:23.0233678Z 
2019-07-18T17:43:23.0233730Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:23.0233943Z Actual stderr saved to /tmp/compiletestLiAEmM/linked-list.stderr
2019-07-18T17:43:23.0233984Z To update references, run this command from build directory:
2019-07-18T17:43:23.0234210Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'linked-list.rs'
2019-07-18T17:43:23.0234270Z error: 1 errors occurred comparing output.
2019-07-18T17:43:23.0234402Z status: exit code: 1
2019-07-18T17:43:23.0234402Z status: exit code: 1
2019-07-18T17:43:23.0234931Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/linked-list.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/linked-list.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/linked-list.stage-id.aux" "-A" "unused"
2019-07-18T17:43:23.0235193Z ------------------------------------------
2019-07-18T17:43:23.0235219Z 
2019-07-18T17:43:23.0257704Z ------------------------------------------
2019-07-18T17:43:23.0257803Z stderr:
---
2019-07-18T17:43:23.0793355Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:23.0793400Z +
2019-07-18T17:43:23.0793419Z 
2019-07-18T17:43:23.0793454Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:23.0793665Z Actual stderr saved to /tmp/compiletestLiAEmM/loop-break-value.stderr
2019-07-18T17:43:23.0793707Z To update references, run this command from build directory:
2019-07-18T17:43:23.0793908Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'loop-break-value.rs'
2019-07-18T17:43:23.0794051Z error: 1 errors occurred comparing output.
2019-07-18T17:43:23.0794091Z status: exit code: 1
2019-07-18T17:43:23.0794091Z status: exit code: 1
2019-07-18T17:43:23.0794631Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/loop-break-value.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/loop-break-value.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/loop-break-value.stage-id.aux" "-A" "unused"
2019-07-18T17:43:23.0794897Z ------------------------------------------
2019-07-18T17:43:23.0794924Z 
2019-07-18T17:43:23.0795097Z ------------------------------------------
2019-07-18T17:43:23.0795133Z stderr:
---
2019-07-18T17:43:23.2077405Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:23.2077464Z +
2019-07-18T17:43:23.2077488Z 
2019-07-18T17:43:23.2077528Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:23.2077571Z Actual stderr saved to /tmp/compiletestLiAEmM/main_fn.stderr
2019-07-18T17:43:23.2077632Z To update references, run this command from build directory:
2019-07-18T17:43:23.2077943Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'main_fn.rs'
2019-07-18T17:43:23.2078037Z error: 1 errors occurred comparing output.
2019-07-18T17:43:23.2078077Z status: exit code: 1
2019-07-18T17:43:23.2078077Z status: exit code: 1
2019-07-18T17:43:23.2078647Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/main_fn.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/main_fn.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/main_fn.stage-id.aux" "-A" "unused"
2019-07-18T17:43:23.2079387Z ------------------------------------------
2019-07-18T17:43:23.2079429Z 
2019-07-18T17:43:23.2079678Z ------------------------------------------
2019-07-18T17:43:23.2079725Z stderr:
---
2019-07-18T17:43:23.2183615Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:23.2183913Z +
2019-07-18T17:43:23.2184128Z 
2019-07-18T17:43:23.2184470Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:23.2184772Z Actual stderr saved to /tmp/compiletestLiAEmM/loops.stderr
2019-07-18T17:43:23.2185028Z To update references, run this command from build directory:
2019-07-18T17:43:23.2185535Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'loops.rs'
2019-07-18T17:43:23.2186062Z error: 1 errors occurred comparing output.
2019-07-18T17:43:23.2186273Z status: exit code: 1
2019-07-18T17:43:23.2186273Z status: exit code: 1
2019-07-18T17:43:23.2187257Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/loops.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/loops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/loops.stage-id.aux" "-A" "unused"
2019-07-18T17:43:23.2188076Z ------------------------------------------
2019-07-18T17:43:23.2188347Z 
2019-07-18T17:43:23.2189602Z ------------------------------------------
2019-07-18T17:43:23.2190871Z stderr:
---
2019-07-18T17:43:23.3360980Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:23.3361039Z +
2019-07-18T17:43:23.3361063Z 
2019-07-18T17:43:23.3361879Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:23.3362049Z Actual stderr saved to /tmp/compiletestLiAEmM/many_shr_bor.stderr
2019-07-18T17:43:23.3362120Z To update references, run this command from build directory:
2019-07-18T17:43:23.3362456Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'many_shr_bor.rs'
2019-07-18T17:43:23.3362555Z error: 1 errors occurred comparing output.
2019-07-18T17:43:23.3362612Z status: exit code: 1
2019-07-18T17:43:23.3362612Z status: exit code: 1
2019-07-18T17:43:23.3363254Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/many_shr_bor.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/many_shr_bor.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/many_shr_bor.stage-id.aux" "-A" "unused"
2019-07-18T17:43:23.3363593Z ------------------------------------------
2019-07-18T17:43:23.3363627Z 
2019-07-18T17:43:23.3363860Z ------------------------------------------
2019-07-18T17:43:23.3363906Z stderr:
---
2019-07-18T17:43:23.3523953Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:23.3523999Z +
2019-07-18T17:43:23.3524042Z 
2019-07-18T17:43:23.3524165Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:23.3524217Z Actual stderr saved to /tmp/compiletestLiAEmM/match_slice.stderr
2019-07-18T17:43:23.3524258Z To update references, run this command from build directory:
2019-07-18T17:43:23.3524546Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'match_slice.rs'
2019-07-18T17:43:23.3524617Z error: 1 errors occurred comparing output.
2019-07-18T17:43:23.3524676Z status: exit code: 1
2019-07-18T17:43:23.3524676Z status: exit code: 1
2019-07-18T17:43:23.3525224Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/match_slice.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/match_slice.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/match_slice.stage-id.aux" "-A" "unused"
2019-07-18T17:43:23.3525530Z ------------------------------------------
2019-07-18T17:43:23.3525563Z 
2019-07-18T17:43:23.3525768Z ------------------------------------------
2019-07-18T17:43:23.3525832Z stderr:
---
2019-07-18T17:43:23.5512891Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:23.5512935Z +
2019-07-18T17:43:23.5512959Z 
2019-07-18T17:43:23.5513074Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:23.5513146Z Actual stderr saved to /tmp/compiletestLiAEmM/memchr.stderr
2019-07-18T17:43:23.5513192Z To update references, run this command from build directory:
2019-07-18T17:43:23.5513452Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'memchr.rs'
2019-07-18T17:43:23.5513709Z error: 1 errors occurred comparing output.
2019-07-18T17:43:23.5513747Z status: exit code: 1
2019-07-18T17:43:23.5513747Z status: exit code: 1
2019-07-18T17:43:23.5514626Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/memchr.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/memchr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/memchr.stage-id.aux" "-A" "unused"
2019-07-18T17:43:23.5514892Z ------------------------------------------
2019-07-18T17:43:23.5514939Z 
2019-07-18T17:43:23.5515114Z ------------------------------------------
2019-07-18T17:43:23.5515151Z stderr:
---
2019-07-18T17:43:23.5841885Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:23.5842314Z +
2019-07-18T17:43:23.5842717Z 
2019-07-18T17:43:23.5842904Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:23.5843036Z Actual stderr saved to /tmp/compiletestLiAEmM/mir_coercions.stderr
2019-07-18T17:43:23.5843193Z To update references, run this command from build directory:
2019-07-18T17:43:23.5843575Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'mir_coercions.rs'
2019-07-18T17:43:23.5843904Z error: 1 errors occurred comparing output.
2019-07-18T17:43:23.5844045Z status: exit code: 1
2019-07-18T17:43:23.5844045Z status: exit code: 1
2019-07-18T17:43:23.5844842Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mir_coercions.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/mir_coercions.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/mir_coercions.stage-id.aux" "-A" "unused"
2019-07-18T17:43:23.5845396Z ------------------------------------------
2019-07-18T17:43:23.5845570Z 
2019-07-18T17:43:23.5845886Z ------------------------------------------
2019-07-18T17:43:23.5846055Z stderr:
---
2019-07-18T17:43:23.7124231Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:23.7124281Z +
2019-07-18T17:43:23.7124321Z 
2019-07-18T17:43:23.7124358Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:23.7124397Z Actual stderr saved to /tmp/compiletestLiAEmM/mir_fat_ptr.stderr
2019-07-18T17:43:23.7124453Z To update references, run this command from build directory:
2019-07-18T17:43:23.7124690Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'mir_fat_ptr.rs'
2019-07-18T17:43:23.7124761Z error: 1 errors occurred comparing output.
2019-07-18T17:43:23.7124813Z status: exit code: 1
2019-07-18T17:43:23.7124813Z status: exit code: 1
2019-07-18T17:43:23.7125322Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mir_fat_ptr.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/mir_fat_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/mir_fat_ptr.stage-id.aux" "-A" "unused"
2019-07-18T17:43:23.7125585Z ------------------------------------------
2019-07-18T17:43:23.7125612Z 
2019-07-18T17:43:23.7125787Z ------------------------------------------
2019-07-18T17:43:23.7125840Z stderr:
---
2019-07-18T17:43:23.7357504Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:23.7357546Z +
2019-07-18T17:43:23.7357568Z 
2019-07-18T17:43:23.7357606Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:23.7357844Z Actual stderr saved to /tmp/compiletestLiAEmM/miri-issue-133.stderr
2019-07-18T17:43:23.7357890Z To update references, run this command from build directory:
2019-07-18T17:43:23.7358124Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'miri-issue-133.rs'
2019-07-18T17:43:23.7358207Z error: 1 errors occurred comparing output.
2019-07-18T17:43:23.7358245Z status: exit code: 1
2019-07-18T17:43:23.7358245Z status: exit code: 1
2019-07-18T17:43:23.7359446Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/miri-issue-133.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/miri-issue-133.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/miri-issue-133.stage-id.aux" "-A" "unused"
2019-07-18T17:43:23.7359773Z ------------------------------------------
2019-07-18T17:43:23.7359826Z 
2019-07-18T17:43:23.7360053Z ------------------------------------------
2019-07-18T17:43:23.7360099Z stderr:
---
2019-07-18T17:43:23.8524667Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:23.8524704Z +
2019-07-18T17:43:23.8524734Z 
2019-07-18T17:43:23.8524830Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:23.8525037Z Actual stderr saved to /tmp/compiletestLiAEmM/move-arg-2-unique.stderr
2019-07-18T17:43:23.8525078Z To update references, run this command from build directory:
2019-07-18T17:43:23.8525295Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'move-arg-2-unique.rs'
2019-07-18T17:43:23.8525357Z error: 1 errors occurred comparing output.
2019-07-18T17:43:23.8525412Z status: exit code: 1
2019-07-18T17:43:23.8525412Z status: exit code: 1
2019-07-18T17:43:23.8525921Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-arg-2-unique.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/move-arg-2-unique.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/move-arg-2-unique.stage-id.aux" "-A" "unused"
2019-07-18T17:43:23.8526274Z ------------------------------------------
2019-07-18T17:43:23.8526301Z 
2019-07-18T17:43:23.8526471Z ------------------------------------------
2019-07-18T17:43:23.8526521Z stderr:
---
2019-07-18T17:43:23.8839357Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:23.8839423Z +
2019-07-18T17:43:23.8839450Z 
2019-07-18T17:43:23.8839496Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:23.8839765Z Actual stderr saved to /tmp/compiletestLiAEmM/move-arg-3-unique.stderr
2019-07-18T17:43:23.8839837Z To update references, run this command from build directory:
2019-07-18T17:43:23.8840097Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'move-arg-3-unique.rs'
2019-07-18T17:43:23.8840195Z error: 1 errors occurred comparing output.
2019-07-18T17:43:23.8840242Z status: exit code: 1
2019-07-18T17:43:23.8840242Z status: exit code: 1
2019-07-18T17:43:23.8840903Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-arg-3-unique.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/move-arg-3-unique.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/move-arg-3-unique.stage-id.aux" "-A" "unused"
2019-07-18T17:43:23.8841354Z ------------------------------------------
2019-07-18T17:43:23.8841389Z 
2019-07-18T17:43:23.8841625Z ------------------------------------------
2019-07-18T17:43:23.8841670Z stderr:
---
2019-07-18T17:43:23.9864872Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:23.9864931Z +
2019-07-18T17:43:23.9864955Z 
2019-07-18T17:43:23.9864994Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:23.9865218Z Actual stderr saved to /tmp/compiletestLiAEmM/move-undef-primval.stderr
2019-07-18T17:43:23.9865282Z To update references, run this command from build directory:
2019-07-18T17:43:23.9865518Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'move-undef-primval.rs'
2019-07-18T17:43:23.9865605Z error: 1 errors occurred comparing output.
2019-07-18T17:43:23.9865645Z status: exit code: 1
2019-07-18T17:43:23.9865645Z status: exit code: 1
2019-07-18T17:43:23.9866220Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-undef-primval.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/move-undef-primval.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/move-undef-primval.stage-id.aux" "-A" "unused"
2019-07-18T17:43:23.9866640Z ------------------------------------------
2019-07-18T17:43:23.9866671Z 
2019-07-18T17:43:23.9866884Z ------------------------------------------
2019-07-18T17:43:23.9866933Z stderr:
---
2019-07-18T17:43:24.0604272Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:24.0604313Z +
2019-07-18T17:43:24.0604352Z 
2019-07-18T17:43:24.0604390Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:24.0604440Z Actual stderr saved to /tmp/compiletestLiAEmM/mpsc.stderr
2019-07-18T17:43:24.0604500Z To update references, run this command from build directory:
2019-07-18T17:43:24.0604733Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'mpsc.rs'
2019-07-18T17:43:24.0604801Z error: 1 errors occurred comparing output.
2019-07-18T17:43:24.0604854Z status: exit code: 1
2019-07-18T17:43:24.0604854Z status: exit code: 1
2019-07-18T17:43:24.0605374Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mpsc.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/mpsc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/mpsc.stage-id.aux" "-A" "unused"
2019-07-18T17:43:24.0605758Z ------------------------------------------
2019-07-18T17:43:24.0605790Z 
2019-07-18T17:43:24.0606172Z ------------------------------------------
2019-07-18T17:43:24.0606229Z stderr:
---
2019-07-18T17:43:24.1565931Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:24.1565976Z +
2019-07-18T17:43:24.1566002Z 
2019-07-18T17:43:24.1566063Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:24.1566120Z Actual stderr saved to /tmp/compiletestLiAEmM/multi_arg_closure.stderr
2019-07-18T17:43:24.1566166Z To update references, run this command from build directory:
2019-07-18T17:43:24.1566440Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'multi_arg_closure.rs'
2019-07-18T17:43:24.1566515Z error: 1 errors occurred comparing output.
2019-07-18T17:43:24.1566572Z status: exit code: 1
2019-07-18T17:43:24.1566572Z status: exit code: 1
2019-07-18T17:43:24.1567157Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/multi_arg_closure.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/multi_arg_closure.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/multi_arg_closure.stage-id.aux" "-A" "unused"
2019-07-18T17:43:24.1567595Z ------------------------------------------
2019-07-18T17:43:24.1567628Z 
2019-07-18T17:43:24.1567843Z ------------------------------------------
2019-07-18T17:43:24.1567901Z stderr:
---
2019-07-18T17:43:24.2002156Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:24.2002225Z +
2019-07-18T17:43:24.2002420Z 
2019-07-18T17:43:24.2002458Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:24.2002510Z Actual stderr saved to /tmp/compiletestLiAEmM/negative_discriminant.stderr
2019-07-18T17:43:24.2002572Z To update references, run this command from build directory:
2019-07-18T17:43:24.2002961Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'negative_discriminant.rs'
2019-07-18T17:43:24.2003044Z error: 1 errors occurred comparing output.
2019-07-18T17:43:24.2003081Z status: exit code: 1
2019-07-18T17:43:24.2003081Z status: exit code: 1
2019-07-18T17:43:24.2003627Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/negative_discriminant.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/negative_discriminant.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/negative_discriminant.stage-id.aux" "-A" "unused"
2019-07-18T17:43:24.2003996Z ------------------------------------------
2019-07-18T17:43:24.2004033Z 
2019-07-18T17:43:24.2004233Z ------------------------------------------
2019-07-18T17:43:24.2004270Z stderr:
---
2019-07-18T17:43:24.3386887Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:24.3386977Z +
2019-07-18T17:43:24.3387005Z 
2019-07-18T17:43:24.3387056Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:24.3387106Z Actual stderr saved to /tmp/compiletestLiAEmM/non_capture_closure_to_fn_ptr.stderr
2019-07-18T17:43:24.3387196Z To update references, run this command from build directory:
2019-07-18T17:43:24.3387644Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'non_capture_closure_to_fn_ptr.rs'
2019-07-18T17:43:24.3387770Z error: 1 errors occurred comparing output.
2019-07-18T17:43:24.3388117Z status: exit code: 1
2019-07-18T17:43:24.3388117Z status: exit code: 1
2019-07-18T17:43:24.3389521Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/non_capture_closure_to_fn_ptr.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/non_capture_closure_to_fn_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/non_capture_closure_to_fn_ptr.stage-id.aux" "-A" "unused"
2019-07-18T17:43:24.3389914Z ------------------------------------------
2019-07-18T17:43:24.3389950Z 
2019-07-18T17:43:24.3390220Z ------------------------------------------
2019-07-18T17:43:24.3390267Z stderr:
---
2019-07-18T17:43:24.3529127Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:24.3529203Z +
2019-07-18T17:43:24.3529233Z 
2019-07-18T17:43:24.3529278Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:24.3529331Z Actual stderr saved to /tmp/compiletestLiAEmM/observed_local_mut.stderr
2019-07-18T17:43:24.3531005Z To update references, run this command from build directory:
2019-07-18T17:43:24.3531556Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'observed_local_mut.rs'
2019-07-18T17:43:24.3531661Z error: 1 errors occurred comparing output.
2019-07-18T17:43:24.3531707Z status: exit code: 1
2019-07-18T17:43:24.3531707Z status: exit code: 1
2019-07-18T17:43:24.3532646Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/observed_local_mut.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/observed_local_mut.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletestLiAEmM/observed_local_mut.stage-id.aux" "-A" "unused"
2019-07-18T17:43:24.3533152Z ------------------------------------------
2019-07-18T17:43:24.3533187Z 
2019-07-18T17:43:24.3533423Z ------------------------------------------
2019-07-18T17:43:24.3533480Z stderr:
---
2019-07-18T17:43:24.4836587Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:24.4836628Z +
2019-07-18T17:43:24.4836651Z 
2019-07-18T17:43:24.4836709Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:24.4836753Z Actual stderr saved to /tmp/compiletestLiAEmM/option_box_transmute_ptr.stderr
2019-07-18T17:43:24.4836872Z To update references, run this command from build directory:
2019-07-18T17:43:24.4837148Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'option_box_transmute_ptr.rs'
2019-07-18T17:43:24.4837217Z error: 1 errors occurred comparing output.
2019-07-18T17:43:24.4837272Z status: exit code: 1
2019-07-18T17:43:24.4837272Z status: exit code: 1
2019-07-18T17:43:24.4837848Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/option_box_transmute_ptr.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/option_box_transmute_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/option_box_transmute_ptr.stage-id.aux" "-A" "unused"
2019-07-18T17:43:24.4838145Z ------------------------------------------
2019-07-18T17:43:24.4838175Z 
2019-07-18T17:43:24.4838370Z ------------------------------------------
2019-07-18T17:43:24.4838431Z stderr:
---
2019-07-18T17:43:24.5124468Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:24.5124602Z +
2019-07-18T17:43:24.5124643Z 
2019-07-18T17:43:24.5124681Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:24.5124899Z Actual stderr saved to /tmp/compiletestLiAEmM/option_eq.stderr
2019-07-18T17:43:24.5124956Z To update references, run this command from build directory:
2019-07-18T17:43:24.5125196Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'option_eq.rs'
2019-07-18T17:43:24.5125260Z error: 1 errors occurred comparing output.
2019-07-18T17:43:24.5125320Z status: exit code: 1
2019-07-18T17:43:24.5125320Z status: exit code: 1
2019-07-18T17:43:24.5126021Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/option_eq.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/option_eq.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/option_eq.stage-id.aux" "-A" "unused"
2019-07-18T17:43:24.5126486Z ------------------------------------------
2019-07-18T17:43:24.5126515Z 
2019-07-18T17:43:24.5126716Z ------------------------------------------
2019-07-18T17:43:24.5126755Z stderr:
---
2019-07-18T17:43:24.6402958Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:24.6403003Z +
2019-07-18T17:43:24.6403027Z 
2019-07-18T17:43:24.6403067Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:24.6403320Z Actual stderr saved to /tmp/compiletestLiAEmM/overloaded-calls-simple.stderr
2019-07-18T17:43:24.6403381Z To update references, run this command from build directory:
2019-07-18T17:43:24.6403621Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'overloaded-calls-simple.rs'
2019-07-18T17:43:24.6403708Z error: 1 errors occurred comparing output.
2019-07-18T17:43:24.6403748Z status: exit code: 1
2019-07-18T17:43:24.6403748Z status: exit code: 1
2019-07-18T17:43:24.6404376Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/overloaded-calls-simple.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/overloaded-calls-simple.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/overloaded-calls-simple.stage-id.aux" "-A" "unused"
2019-07-18T17:43:24.6404845Z ------------------------------------------
2019-07-18T17:43:24.6404894Z 
2019-07-18T17:43:24.6405105Z ------------------------------------------
2019-07-18T17:43:24.6405147Z stderr:
---
2019-07-18T17:43:24.6554560Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:24.6554624Z +
2019-07-18T17:43:24.6554650Z 
2019-07-18T17:43:24.6554694Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:24.6554759Z Actual stderr saved to /tmp/compiletestLiAEmM/packed_static.stderr
2019-07-18T17:43:24.6554818Z To update references, run this command from build directory:
2019-07-18T17:43:24.6555082Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'packed_static.rs'
2019-07-18T17:43:24.6555176Z error: 1 errors occurred comparing output.
2019-07-18T17:43:24.6555220Z status: exit code: 1
2019-07-18T17:43:24.6555220Z status: exit code: 1
2019-07-18T17:43:24.6555839Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/packed_static.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/packed_static.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/packed_static.stage-id.aux" "-A" "unused"
2019-07-18T17:43:24.6556171Z ------------------------------------------
2019-07-18T17:43:24.6556222Z 
2019-07-18T17:43:24.6556446Z ------------------------------------------
2019-07-18T17:43:24.6556490Z stderr:
---
2019-07-18T17:43:24.8407236Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:24.8407426Z +
2019-07-18T17:43:24.8407541Z 
2019-07-18T17:43:24.8407671Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:24.8407815Z Actual stderr saved to /tmp/compiletestLiAEmM/packed_struct.stderr
2019-07-18T17:43:24.8407945Z To update references, run this command from build directory:
2019-07-18T17:43:24.8408289Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'packed_struct.rs'
2019-07-18T17:43:24.8409130Z error: 1 errors occurred comparing output.
2019-07-18T17:43:24.8419903Z status: exit code: 1
2019-07-18T17:43:24.8419903Z status: exit code: 1
2019-07-18T17:43:24.8420975Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/packed_struct.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/packed_struct.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/packed_struct.stage-id.aux" "-A" "unused"
2019-07-18T17:43:24.8421625Z ------------------------------------------
2019-07-18T17:43:24.8421788Z 
2019-07-18T17:43:24.8422148Z ------------------------------------------
2019-07-18T17:43:24.8422345Z stderr:
---
2019-07-18T17:43:24.8504821Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:24.8504877Z +
2019-07-18T17:43:24.8504901Z 
2019-07-18T17:43:24.8504947Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:24.8504990Z Actual stderr saved to /tmp/compiletestLiAEmM/pointers.stderr
2019-07-18T17:43:24.8505050Z To update references, run this command from build directory:
2019-07-18T17:43:24.8505282Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'pointers.rs'
2019-07-18T17:43:24.8505349Z error: 1 errors occurred comparing output.
2019-07-18T17:43:24.8505402Z status: exit code: 1
2019-07-18T17:43:24.8505402Z status: exit code: 1
2019-07-18T17:43:24.8505949Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/pointers.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/pointers.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/pointers.stage-id.aux" "-A" "unused"
2019-07-18T17:43:24.8506238Z ------------------------------------------
2019-07-18T17:43:24.8506268Z 
2019-07-18T17:43:24.8506482Z ------------------------------------------
2019-07-18T17:43:24.8506521Z stderr:
---
2019-07-18T17:43:25.0022025Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:25.0022249Z +
2019-07-18T17:43:25.0022381Z 
2019-07-18T17:43:25.0022698Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:25.0022849Z Actual stderr saved to /tmp/compiletestLiAEmM/ptr_arith_offset.stderr
2019-07-18T17:43:25.0022975Z To update references, run this command from build directory:
2019-07-18T17:43:25.0023356Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'ptr_arith_offset.rs'
2019-07-18T17:43:25.0023626Z error: 1 errors occurred comparing output.
2019-07-18T17:43:25.0023758Z status: exit code: 1
2019-07-18T17:43:25.0023758Z status: exit code: 1
2019-07-18T17:43:25.0024515Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_arith_offset.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/ptr_arith_offset.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/ptr_arith_offset.stage-id.aux" "-A" "unused"
2019-07-18T17:43:25.0025032Z ------------------------------------------
2019-07-18T17:43:25.0025163Z 
2019-07-18T17:43:25.0025449Z ------------------------------------------
2019-07-18T17:43:25.0025634Z stderr:
---
2019-07-18T17:43:25.0096932Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:25.0097090Z +
2019-07-18T17:43:25.0097214Z 
2019-07-18T17:43:25.0097332Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:25.0097453Z Actual stderr saved to /tmp/compiletestLiAEmM/products.stderr
2019-07-18T17:43:25.0097597Z To update references, run this command from build directory:
2019-07-18T17:43:25.0097937Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'products.rs'
2019-07-18T17:43:25.0098226Z error: 1 errors occurred comparing output.
2019-07-18T17:43:25.0098724Z status: exit code: 1
2019-07-18T17:43:25.0098724Z status: exit code: 1
2019-07-18T17:43:25.0100021Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/products.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/products.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/products.stage-id.aux" "-A" "unused"
2019-07-18T17:43:25.0100690Z ------------------------------------------
2019-07-18T17:43:25.0100847Z 
2019-07-18T17:43:25.0101191Z ------------------------------------------
2019-07-18T17:43:25.0101379Z stderr:
---
2019-07-18T17:43:25.1512637Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:25.1512874Z +
2019-07-18T17:43:25.1513033Z 
2019-07-18T17:43:25.1513232Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:25.1513418Z Actual stderr saved to /tmp/compiletestLiAEmM/ptr_arith_offset_overflow.stderr
2019-07-18T17:43:25.1513765Z To update references, run this command from build directory:
2019-07-18T17:43:25.1514232Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'ptr_arith_offset_overflow.rs'
2019-07-18T17:43:25.1514634Z error: 1 errors occurred comparing output.
2019-07-18T17:43:25.1514842Z status: exit code: 1
2019-07-18T17:43:25.1514842Z status: exit code: 1
2019-07-18T17:43:25.1515638Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_arith_offset_overflow.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/ptr_arith_offset_overflow.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/ptr_arith_offset_overflow.stage-id.aux" "-A" "unused"
2019-07-18T17:43:25.1516318Z ------------------------------------------
2019-07-18T17:43:25.1516528Z 
2019-07-18T17:43:25.1516943Z ------------------------------------------
2019-07-18T17:43:25.1517183Z stderr:
---
2019-07-18T17:43:25.1840485Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:25.1840563Z +
2019-07-18T17:43:25.1840592Z 
2019-07-18T17:43:25.1840639Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:25.1840712Z Actual stderr saved to /tmp/compiletestLiAEmM/ptr_int_casts.stderr
2019-07-18T17:43:25.1840768Z To update references, run this command from build directory:
2019-07-18T17:43:25.1841050Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'ptr_int_casts.rs'
2019-07-18T17:43:25.1841163Z error: 1 errors occurred comparing output.
2019-07-18T17:43:25.1841210Z status: exit code: 1
2019-07-18T17:43:25.1841210Z status: exit code: 1
2019-07-18T17:43:25.1841884Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_int_casts.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/ptr_int_casts.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/ptr_int_casts.stage-id.aux" "-A" "unused"
2019-07-18T17:43:25.1842244Z ------------------------------------------
2019-07-18T17:43:25.1842298Z 
2019-07-18T17:43:25.1842910Z ------------------------------------------
2019-07-18T17:43:25.1842949Z stderr:
---
2019-07-18T17:43:25.3300749Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:25.3300800Z +
2019-07-18T17:43:25.3300828Z 
2019-07-18T17:43:25.3300893Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:25.3300947Z Actual stderr saved to /tmp/compiletestLiAEmM/ptr_offset.stderr
2019-07-18T17:43:25.3301000Z To update references, run this command from build directory:
2019-07-18T17:43:25.3301410Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'ptr_offset.rs'
2019-07-18T17:43:25.3301494Z error: 1 errors occurred comparing output.
2019-07-18T17:43:25.3301541Z status: exit code: 1
2019-07-18T17:43:25.3301541Z status: exit code: 1
2019-07-18T17:43:25.3302233Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_offset.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/ptr_offset.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/ptr_offset.stage-id.aux" "-A" "unused"
2019-07-18T17:43:25.3302707Z ------------------------------------------
2019-07-18T17:43:25.3302734Z 
2019-07-18T17:43:25.3302907Z ------------------------------------------
2019-07-18T17:43:25.3302943Z stderr:
---
2019-07-18T17:43:25.3344649Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:25.3344687Z +
2019-07-18T17:43:25.3344707Z 
2019-07-18T17:43:25.3344742Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:25.3344797Z Actual stderr saved to /tmp/compiletestLiAEmM/ptr_int_ops.stderr
2019-07-18T17:43:25.3344834Z To update references, run this command from build directory:
2019-07-18T17:43:25.3345049Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'ptr_int_ops.rs'
2019-07-18T17:43:25.3345128Z error: 1 errors occurred comparing output.
2019-07-18T17:43:25.3345162Z status: exit code: 1
2019-07-18T17:43:25.3345162Z status: exit code: 1
2019-07-18T17:43:25.3345662Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_int_ops.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/ptr_int_ops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/ptr_int_ops.stage-id.aux" "-A" "unused"
2019-07-18T17:43:25.3345927Z ------------------------------------------
2019-07-18T17:43:25.3345954Z 
2019-07-18T17:43:25.3346131Z ------------------------------------------
2019-07-18T17:43:25.3346166Z stderr:
---
2019-07-18T17:43:25.4958566Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:25.4958974Z +
2019-07-18T17:43:25.4959019Z 
2019-07-18T17:43:25.4959067Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:25.4959118Z Actual stderr saved to /tmp/compiletestLiAEmM/raw.stderr
2019-07-18T17:43:25.4959198Z To update references, run this command from build directory:
2019-07-18T17:43:25.4959499Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'raw.rs'
2019-07-18T17:43:25.4959578Z error: 1 errors occurred comparing output.
2019-07-18T17:43:25.4959641Z status: exit code: 1
2019-07-18T17:43:25.4959641Z status: exit code: 1
2019-07-18T17:43:25.4960276Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/raw.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/raw.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/raw.stage-id.aux" "-A" "unused"
2019-07-18T17:43:25.4960613Z ------------------------------------------
2019-07-18T17:43:25.4960648Z 
2019-07-18T17:43:25.4960885Z ------------------------------------------
2019-07-18T17:43:25.4960931Z stderr:
---
2019-07-18T17:43:25.5720286Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:25.5720361Z +
2019-07-18T17:43:25.5720389Z 
2019-07-18T17:43:25.5720437Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:25.5720507Z Actual stderr saved to /tmp/compiletestLiAEmM/rc.stderr
2019-07-18T17:43:25.5720560Z To update references, run this command from build directory:
2019-07-18T17:43:25.5720833Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'rc.rs'
2019-07-18T17:43:25.5720945Z error: 1 errors occurred comparing output.
2019-07-18T17:43:25.5720995Z status: exit code: 1
2019-07-18T17:43:25.5720995Z status: exit code: 1
2019-07-18T17:43:25.5721641Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rc.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/rc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/rc.stage-id.aux" "-A" "unused"
2019-07-18T17:43:25.5721999Z ------------------------------------------
2019-07-18T17:43:25.5722036Z 
2019-07-18T17:43:25.5722299Z ------------------------------------------
2019-07-18T17:43:25.5722348Z stderr:
---
2019-07-18T17:43:25.6482889Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:25.6485173Z +
2019-07-18T17:43:25.6485312Z 
2019-07-18T17:43:25.6485425Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:25.6485576Z Actual stderr saved to /tmp/compiletestLiAEmM/recursive_static.stderr
2019-07-18T17:43:25.6485693Z To update references, run this command from build directory:
2019-07-18T17:43:25.6486106Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'recursive_static.rs'
2019-07-18T17:43:25.6486349Z error: 1 errors occurred comparing output.
2019-07-18T17:43:25.6486473Z status: exit code: 1
2019-07-18T17:43:25.6486473Z status: exit code: 1
2019-07-18T17:43:25.6487071Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/recursive_static.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/recursive_static.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/recursive_static.stage-id.aux" "-A" "unused"
2019-07-18T17:43:25.6487573Z ------------------------------------------
2019-07-18T17:43:25.6487812Z 
2019-07-18T17:43:25.6488191Z ------------------------------------------
2019-07-18T17:43:25.6488335Z stderr:
---
2019-07-18T17:43:25.6959173Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:25.6959227Z +
2019-07-18T17:43:25.6959255Z 
2019-07-18T17:43:25.6959300Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:25.6959597Z Actual stderr saved to /tmp/compiletestLiAEmM/ref-invalid-ptr.stderr
2019-07-18T17:43:25.6959666Z To update references, run this command from build directory:
2019-07-18T17:43:25.6959929Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'ref-invalid-ptr.rs'
2019-07-18T17:43:25.6960026Z error: 1 errors occurred comparing output.
2019-07-18T17:43:25.6960071Z status: exit code: 1
2019-07-18T17:43:25.6960071Z status: exit code: 1
2019-07-18T17:43:25.6960786Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ref-invalid-ptr.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/ref-invalid-ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletestLiAEmM/ref-invalid-ptr.stage-id.aux" "-A" "unused"
2019-07-18T17:43:25.6961110Z ------------------------------------------
2019-07-18T17:43:25.6961160Z 
2019-07-18T17:43:25.6961481Z ------------------------------------------
2019-07-18T17:43:25.6961537Z stderr:
---
2019-07-18T17:43:25.7956270Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:25.7956316Z +
2019-07-18T17:43:25.7956341Z 
2019-07-18T17:43:25.7956382Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:25.7956455Z Actual stderr saved to /tmp/compiletestLiAEmM/refcell.stderr
2019-07-18T17:43:25.7956499Z To update references, run this command from build directory:
2019-07-18T17:43:25.7956738Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'refcell.rs'
2019-07-18T17:43:25.7956827Z error: 1 errors occurred comparing output.
2019-07-18T17:43:25.7956869Z status: exit code: 1
2019-07-18T17:43:25.7956869Z status: exit code: 1
2019-07-18T17:43:25.7957736Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/refcell.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/refcell.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/refcell.stage-id.aux" "-A" "unused"
2019-07-18T17:43:25.7958007Z ------------------------------------------
2019-07-18T17:43:25.7958134Z 
2019-07-18T17:43:25.7958339Z ------------------------------------------
2019-07-18T17:43:25.7958375Z stderr:
---
2019-07-18T17:43:25.8279553Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:25.8279617Z +
2019-07-18T17:43:25.8279646Z 
2019-07-18T17:43:25.8279706Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:25.8280029Z Actual stderr saved to /tmp/compiletestLiAEmM/regions-lifetime-nonfree-late-bound.stderr
2019-07-18T17:43:25.8280090Z To update references, run this command from build directory:
2019-07-18T17:43:25.8280396Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'regions-lifetime-nonfree-late-bound.rs'
2019-07-18T17:43:25.8280507Z error: 1 errors occurred comparing output.
2019-07-18T17:43:25.8280566Z status: exit code: 1
2019-07-18T17:43:25.8280566Z status: exit code: 1
2019-07-18T17:43:25.8281452Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/regions-lifetime-nonfree-late-bound.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/regions-lifetime-nonfree-late-bound.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/regions-lifetime-nonfree-late-bound.stage-id.aux" "-A" "unused"
2019-07-18T17:43:25.8281863Z ------------------------------------------
2019-07-18T17:43:25.8281904Z 
2019-07-18T17:43:25.8282154Z ------------------------------------------
2019-07-18T17:43:25.8282207Z stderr:
---
2019-07-18T17:43:25.9446507Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:25.9446554Z +
2019-07-18T17:43:25.9446577Z 
2019-07-18T17:43:25.9446616Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:25.9446871Z Actual stderr saved to /tmp/compiletestLiAEmM/regions-mock-trans.stderr
2019-07-18T17:43:25.9446920Z To update references, run this command from build directory:
2019-07-18T17:43:25.9447175Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'regions-mock-trans.rs'
2019-07-18T17:43:25.9447268Z error: 1 errors occurred comparing output.
2019-07-18T17:43:25.9447305Z status: exit code: 1
2019-07-18T17:43:25.9447305Z status: exit code: 1
2019-07-18T17:43:25.9447964Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/regions-mock-trans.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/regions-mock-trans.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/regions-mock-trans.stage-id.aux" "-A" "unused"
2019-07-18T17:43:25.9448315Z ------------------------------------------
2019-07-18T17:43:25.9448778Z 
2019-07-18T17:43:25.9449096Z ------------------------------------------
2019-07-18T17:43:25.9449167Z stderr:
---
2019-07-18T17:43:25.9680017Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:25.9680089Z +
2019-07-18T17:43:25.9680119Z 
2019-07-18T17:43:25.9680169Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:25.9680242Z Actual stderr saved to /tmp/compiletestLiAEmM/rfc1623.stderr
2019-07-18T17:43:25.9680296Z To update references, run this command from build directory:
2019-07-18T17:43:25.9680608Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'rfc1623.rs'
2019-07-18T17:43:25.9680719Z error: 1 errors occurred comparing output.
2019-07-18T17:43:25.9680876Z status: exit code: 1
2019-07-18T17:43:25.9680876Z status: exit code: 1
2019-07-18T17:43:25.9681621Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rfc1623.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/rfc1623.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/rfc1623.stage-id.aux" "-A" "unused"
2019-07-18T17:43:25.9682149Z ------------------------------------------
2019-07-18T17:43:25.9682202Z 
2019-07-18T17:43:25.9682402Z ------------------------------------------
2019-07-18T17:43:25.9682445Z stderr:
---
2019-07-18T17:43:26.1296705Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:26.1296754Z +
2019-07-18T17:43:26.1296779Z 
2019-07-18T17:43:26.1296839Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:26.1297113Z Actual stderr saved to /tmp/compiletestLiAEmM/send-is-not-static-par-for.stderr
2019-07-18T17:43:26.1297166Z To update references, run this command from build directory:
2019-07-18T17:43:26.1297444Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'send-is-not-static-par-for.rs'
2019-07-18T17:43:26.1297519Z error: 1 errors occurred comparing output.
2019-07-18T17:43:26.1297560Z status: exit code: 1
2019-07-18T17:43:26.1297560Z status: exit code: 1
2019-07-18T17:43:26.1298705Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/send-is-not-static-par-for.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/send-is-not-static-par-for.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/send-is-not-static-par-for.stage-id.aux" "-A" "unused"
2019-07-18T17:43:26.1299207Z ------------------------------------------
2019-07-18T17:43:26.1299249Z 
2019-07-18T17:43:26.1299721Z ------------------------------------------
2019-07-18T17:43:26.1299805Z stderr:
---
2019-07-18T17:43:26.1457412Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:26.1457475Z +
2019-07-18T17:43:26.1457499Z 
2019-07-18T17:43:26.1457540Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:26.1457819Z Actual stderr saved to /tmp/compiletestLiAEmM/rust-lang-org.stderr
2019-07-18T17:43:26.1457874Z To update references, run this command from build directory:
2019-07-18T17:43:26.1458134Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'rust-lang-org.rs'
2019-07-18T17:43:26.1458311Z error: 1 errors occurred comparing output.
2019-07-18T17:43:26.1458959Z status: exit code: 1
2019-07-18T17:43:26.1458959Z status: exit code: 1
2019-07-18T17:43:26.1460032Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rust-lang-org.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/rust-lang-org.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/rust-lang-org.stage-id.aux" "-A" "unused"
2019-07-18T17:43:26.1460427Z ------------------------------------------
2019-07-18T17:43:26.1460495Z 
2019-07-18T17:43:26.1460750Z ------------------------------------------
2019-07-18T17:43:26.1460803Z stderr:
---
2019-07-18T17:43:26.2561658Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:26.2561716Z +
2019-07-18T17:43:26.2561767Z 
2019-07-18T17:43:26.2561818Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:26.2562235Z Actual stderr saved to /tmp/compiletestLiAEmM/sendable-class.stderr
2019-07-18T17:43:26.2562305Z To update references, run this command from build directory:
2019-07-18T17:43:26.2562632Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'sendable-class.rs'
2019-07-18T17:43:26.2562720Z error: 1 errors occurred comparing output.
2019-07-18T17:43:26.2562780Z status: exit code: 1
2019-07-18T17:43:26.2562780Z status: exit code: 1
2019-07-18T17:43:26.2563735Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sendable-class.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/sendable-class.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/sendable-class.stage-id.aux" "-A" "unused"
2019-07-18T17:43:26.2564061Z ------------------------------------------
2019-07-18T17:43:26.2564096Z 
2019-07-18T17:43:26.2564305Z ------------------------------------------
2019-07-18T17:43:26.2564370Z stderr:
---
2019-07-18T17:43:26.3324867Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:26.3324917Z +
2019-07-18T17:43:26.3324942Z 
2019-07-18T17:43:26.3325003Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:26.3325268Z Actual stderr saved to /tmp/compiletestLiAEmM/simd-intrinsic-generic-elements.stderr
2019-07-18T17:43:26.3325462Z To update references, run this command from build directory:
2019-07-18T17:43:26.3325816Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'simd-intrinsic-generic-elements.rs'
2019-07-18T17:43:26.3325894Z error: 1 errors occurred comparing output.
2019-07-18T17:43:26.3325935Z status: exit code: 1
2019-07-18T17:43:26.3325935Z status: exit code: 1
2019-07-18T17:43:26.3326748Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/simd-intrinsic-generic-elements.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/simd-intrinsic-generic-elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/simd-intrinsic-generic-elements.stage-id.aux" "-A" "unused"
2019-07-18T17:43:26.3327264Z ------------------------------------------
2019-07-18T17:43:26.3327308Z 
2019-07-18T17:43:26.3327519Z ------------------------------------------
2019-07-18T17:43:26.3327582Z stderr:
---
2019-07-18T17:43:26.4717544Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:26.4717605Z +
2019-07-18T17:43:26.4717629Z 
2019-07-18T17:43:26.4717966Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:26.4718022Z Actual stderr saved to /tmp/compiletestLiAEmM/small_enum_size_bug.stderr
2019-07-18T17:43:26.4718079Z To update references, run this command from build directory:
2019-07-18T17:43:26.4718328Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'small_enum_size_bug.rs'
2019-07-18T17:43:26.4718410Z error: 1 errors occurred comparing output.
2019-07-18T17:43:26.4718455Z status: exit code: 1
2019-07-18T17:43:26.4718455Z status: exit code: 1
2019-07-18T17:43:26.4719634Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/small_enum_size_bug.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/small_enum_size_bug.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/small_enum_size_bug.stage-id.aux" "-A" "unused"
2019-07-18T17:43:26.4719997Z ------------------------------------------
2019-07-18T17:43:26.4720033Z 
2019-07-18T17:43:26.4720277Z ------------------------------------------
2019-07-18T17:43:26.4720324Z stderr:
---
2019-07-18T17:43:26.5964748Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:26.5964802Z +
2019-07-18T17:43:26.5964844Z 
2019-07-18T17:43:26.5964886Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:26.5964935Z Actual stderr saved to /tmp/compiletestLiAEmM/slices.stderr
2019-07-18T17:43:26.5964983Z To update references, run this command from build directory:
2019-07-18T17:43:26.5965264Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'slices.rs'
2019-07-18T17:43:26.5965350Z error: 1 errors occurred comparing output.
2019-07-18T17:43:26.5965408Z status: exit code: 1
2019-07-18T17:43:26.5965408Z status: exit code: 1
2019-07-18T17:43:26.5966031Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/slices.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/slices.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/slices.stage-id.aux" "-A" "unused"
2019-07-18T17:43:26.5966340Z ------------------------------------------
2019-07-18T17:43:26.5966373Z 
2019-07-18T17:43:26.5966579Z ------------------------------------------
2019-07-18T17:43:26.5966639Z stderr:
---
2019-07-18T17:43:26.6116077Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:26.6116121Z +
2019-07-18T17:43:26.6116146Z 
2019-07-18T17:43:26.6116188Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:26.6116252Z Actual stderr saved to /tmp/compiletestLiAEmM/specialization.stderr
2019-07-18T17:43:26.6116299Z To update references, run this command from build directory:
2019-07-18T17:43:26.6116563Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'specialization.rs'
2019-07-18T17:43:26.6116654Z error: 1 errors occurred comparing output.
2019-07-18T17:43:26.6116696Z status: exit code: 1
2019-07-18T17:43:26.6116696Z status: exit code: 1
2019-07-18T17:43:26.6117310Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/specialization.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/specialization.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/specialization.stage-id.aux" "-A" "unused"
2019-07-18T17:43:26.6117620Z ------------------------------------------
2019-07-18T17:43:26.6117668Z 
2019-07-18T17:43:26.6117882Z ------------------------------------------
2019-07-18T17:43:26.6117926Z stderr:
---
2019-07-18T17:43:26.7441305Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:26.7441371Z +
2019-07-18T17:43:26.7445670Z 
2019-07-18T17:43:26.7445746Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:26.7450583Z Actual stderr saved to /tmp/compiletestLiAEmM/stacked-borrows/2phase.stderr
2019-07-18T17:43:26.7450700Z To update references, run this command from build directory:
2019-07-18T17:43:26.7455581Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'stacked-borrows/2phase.rs'
2019-07-18T17:43:26.7459316Z error: 1 errors occurred comparing output.
2019-07-18T17:43:26.7459405Z status: exit code: 1
2019-07-18T17:43:26.7459405Z status: exit code: 1
2019-07-18T17:43:26.7464007Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/2phase.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/stacked-borrows/2phase.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/stacked-borrows/2phase.stage-id.aux" "-A" "unused"
2019-07-18T17:43:26.7464634Z ------------------------------------------
2019-07-18T17:43:26.7467229Z 
2019-07-18T17:43:26.7468180Z ------------------------------------------
2019-07-18T17:43:26.7468489Z stderr:
---
2019-07-18T17:43:26.7920282Z +
2019-07-18T17:43:26.7920311Z 
2019-07-18T17:43:26.7920749Z thread '[ui] run-pass/stacked-borrows/interior_mutability.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-18T17:43:26.7920817Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:26.7921108Z Actual stderr saved to /tmp/compiletestLiAEmM/stacked-borrows/interior_mutability.stderr
2019-07-18T17:43:26.7921186Z To update references, run this command from build directory:
2019-07-18T17:43:26.7921498Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'stacked-borrows/interior_mutability.rs'
2019-07-18T17:43:26.7921605Z error: 1 errors occurred comparing output.
2019-07-18T17:43:26.7921653Z status: exit code: 1
2019-07-18T17:43:26.7921653Z status: exit code: 1
2019-07-18T17:43:26.7922519Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/interior_mutability.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/stacked-borrows/interior_mutability.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/stacked-borrows/interior_mutability.stage-id.aux" "-A" "unused"
2019-07-18T17:43:26.7923110Z ------------------------------------------
2019-07-18T17:43:26.7923157Z 
2019-07-18T17:43:26.7923341Z ------------------------------------------
2019-07-18T17:43:26.7923386Z stderr:
---
2019-07-18T17:43:26.9551490Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:26.9551720Z +
2019-07-18T17:43:26.9551873Z 
2019-07-18T17:43:26.9552241Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:26.9552408Z Actual stderr saved to /tmp/compiletestLiAEmM/static_memory_modification.stderr
2019-07-18T17:43:26.9552587Z To update references, run this command from build directory:
2019-07-18T17:43:26.9552997Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'static_memory_modification.rs'
2019-07-18T17:43:26.9553324Z error: 1 errors occurred comparing output.
2019-07-18T17:43:26.9553464Z status: exit code: 1
2019-07-18T17:43:26.9553464Z status: exit code: 1
2019-07-18T17:43:26.9554244Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/static_memory_modification.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/static_memory_modification.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/static_memory_modification.stage-id.aux" "-A" "unused"
2019-07-18T17:43:26.9555088Z ------------------------------------------
2019-07-18T17:43:26.9555261Z 
2019-07-18T17:43:26.9555617Z ------------------------------------------
2019-07-18T17:43:26.9555830Z stderr:
---
2019-07-18T17:43:26.9781437Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:26.9781496Z +
2019-07-18T17:43:26.9781525Z 
2019-07-18T17:43:26.9781597Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:26.9781884Z Actual stderr saved to /tmp/compiletestLiAEmM/stacked-borrows/stacked-borrows.stderr
2019-07-18T17:43:26.9781942Z To update references, run this command from build directory:
2019-07-18T17:43:26.9782373Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'stacked-borrows/stacked-borrows.rs'
2019-07-18T17:43:26.9782608Z error: 1 errors occurred comparing output.
2019-07-18T17:43:26.9782812Z status: exit code: 1
2019-07-18T17:43:26.9782812Z status: exit code: 1
2019-07-18T17:43:26.9783460Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/stacked-borrows.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/stacked-borrows/stacked-borrows.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/stacked-borrows/stacked-borrows.stage-id.aux" "-A" "unused"
2019-07-18T17:43:26.9783750Z ------------------------------------------
2019-07-18T17:43:26.9783780Z 
2019-07-18T17:43:26.9783970Z ------------------------------------------
2019-07-18T17:43:26.9784030Z stderr:
---
2019-07-18T17:43:27.1241524Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:27.1241601Z +
2019-07-18T17:43:27.1241632Z 
2019-07-18T17:43:27.1241680Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:27.1241750Z Actual stderr saved to /tmp/compiletestLiAEmM/static_mut.stderr
2019-07-18T17:43:27.1241805Z To update references, run this command from build directory:
2019-07-18T17:43:27.1242110Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'static_mut.rs'
2019-07-18T17:43:27.1242324Z error: 1 errors occurred comparing output.
2019-07-18T17:43:27.1242370Z status: exit code: 1
2019-07-18T17:43:27.1242370Z status: exit code: 1
2019-07-18T17:43:27.1243339Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/static_mut.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/static_mut.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/static_mut.stage-id.aux" "-A" "unused"
2019-07-18T17:43:27.1243611Z ------------------------------------------
2019-07-18T17:43:27.1243639Z 
2019-07-18T17:43:27.1243844Z ------------------------------------------
2019-07-18T17:43:27.1243880Z stderr:
---
2019-07-18T17:43:27.1480369Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:27.1480436Z +
2019-07-18T17:43:27.1480465Z 
2019-07-18T17:43:27.1480513Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:27.1480566Z Actual stderr saved to /tmp/compiletestLiAEmM/strings.stderr
2019-07-18T17:43:27.1480745Z To update references, run this command from build directory:
2019-07-18T17:43:27.1481057Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'strings.rs'
2019-07-18T17:43:27.1481160Z error: 1 errors occurred comparing output.
2019-07-18T17:43:27.1481206Z status: exit code: 1
2019-07-18T17:43:27.1481206Z status: exit code: 1
2019-07-18T17:43:27.1481865Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/strings.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/strings.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/strings.stage-id.aux" "-A" "unused"
2019-07-18T17:43:27.1482210Z ------------------------------------------
2019-07-18T17:43:27.1482254Z 
2019-07-18T17:43:27.1482519Z ------------------------------------------
2019-07-18T17:43:27.1482568Z stderr:
---
2019-07-18T17:43:27.2883738Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:27.2883780Z +
2019-07-18T17:43:27.2883803Z 
2019-07-18T17:43:27.2883856Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:27.2883899Z Actual stderr saved to /tmp/compiletestLiAEmM/subslice_array.stderr
2019-07-18T17:43:27.2883940Z To update references, run this command from build directory:
2019-07-18T17:43:27.2884481Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'subslice_array.rs'
2019-07-18T17:43:27.2884546Z error: 1 errors occurred comparing output.
2019-07-18T17:43:27.2884802Z status: exit code: 1
2019-07-18T17:43:27.2884802Z status: exit code: 1
2019-07-18T17:43:27.2885360Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/subslice_array.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/subslice_array.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/subslice_array.stage-id.aux" "-A" "unused"
2019-07-18T17:43:27.2885631Z ------------------------------------------
2019-07-18T17:43:27.2885660Z 
2019-07-18T17:43:27.2885839Z ------------------------------------------
2019-07-18T17:43:27.2885885Z stderr:
---
2019-07-18T17:43:27.3443728Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:27.3443792Z +
2019-07-18T17:43:27.3443813Z 
2019-07-18T17:43:27.3443848Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:27.3443887Z Actual stderr saved to /tmp/compiletestLiAEmM/sums.stderr
2019-07-18T17:43:27.3444020Z To update references, run this command from build directory:
2019-07-18T17:43:27.3444240Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'sums.rs'
2019-07-18T17:43:27.3444398Z error: 1 errors occurred comparing output.
2019-07-18T17:43:27.3444434Z status: exit code: 1
2019-07-18T17:43:27.3444434Z status: exit code: 1
2019-07-18T17:43:27.3445533Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sums.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/sums.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/sums.stage-id.aux" "-A" "unused"
2019-07-18T17:43:27.3445994Z ------------------------------------------
2019-07-18T17:43:27.3446024Z 
2019-07-18T17:43:27.3446405Z ------------------------------------------
2019-07-18T17:43:27.3446454Z stderr:
---
2019-07-18T17:43:27.4204783Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:27.4205193Z +
2019-07-18T17:43:27.4209540Z 
2019-07-18T17:43:27.4211910Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:27.4212687Z Actual stderr saved to /tmp/compiletestLiAEmM/sync.stderr
2019-07-18T17:43:27.4231309Z To update references, run this command from build directory:
2019-07-18T17:43:27.4231811Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'sync.rs'
2019-07-18T17:43:27.4237578Z error: 1 errors occurred comparing output.
2019-07-18T17:43:27.4237618Z status: exit code: 1
2019-07-18T17:43:27.4237618Z status: exit code: 1
2019-07-18T17:43:27.4238352Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sync.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/sync.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/sync.stage-id.aux" "-A" "unused"
2019-07-18T17:43:27.4239146Z ------------------------------------------
2019-07-18T17:43:27.4239185Z 
2019-07-18T17:43:27.4239429Z ------------------------------------------
2019-07-18T17:43:27.4239477Z stderr:
---
2019-07-18T17:43:27.4759763Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:27.4759839Z +
2019-07-18T17:43:27.4759866Z 
2019-07-18T17:43:27.4759912Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:27.4760306Z Actual stderr saved to /tmp/compiletestLiAEmM/tag-align-dyn-u64.stderr
2019-07-18T17:43:27.4760382Z To update references, run this command from build directory:
2019-07-18T17:43:27.4760653Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'tag-align-dyn-u64.rs'
2019-07-18T17:43:27.4760752Z error: 1 errors occurred comparing output.
2019-07-18T17:43:27.4760798Z status: exit code: 1
2019-07-18T17:43:27.4760798Z status: exit code: 1
2019-07-18T17:43:27.4761475Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tag-align-dyn-u64.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/tag-align-dyn-u64.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/tag-align-dyn-u64.stage-id.aux" "-A" "unused"
2019-07-18T17:43:27.4761818Z ------------------------------------------
2019-07-18T17:43:27.4761853Z 
2019-07-18T17:43:27.4762099Z ------------------------------------------
2019-07-18T17:43:27.4762146Z stderr:
---
2019-07-18T17:43:27.6073354Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:27.6073700Z +
2019-07-18T17:43:27.6073729Z 
2019-07-18T17:43:27.6073774Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:27.6074069Z Actual stderr saved to /tmp/compiletestLiAEmM/too-large-primval-write-problem.stderr
2019-07-18T17:43:27.6074149Z To update references, run this command from build directory:
2019-07-18T17:43:27.6074448Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'too-large-primval-write-problem.rs'
2019-07-18T17:43:27.6074553Z error: 1 errors occurred comparing output.
2019-07-18T17:43:27.6074937Z status: exit code: 1
2019-07-18T17:43:27.6074937Z status: exit code: 1
2019-07-18T17:43:27.6077459Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/too-large-primval-write-problem.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/too-large-primval-write-problem.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/too-large-primval-write-problem.stage-id.aux" "-A" "unused"
2019-07-18T17:43:27.6078164Z ------------------------------------------
2019-07-18T17:43:27.6078228Z 
2019-07-18T17:43:27.6078591Z ------------------------------------------
2019-07-18T17:43:27.6080853Z stderr:
---
2019-07-18T17:43:27.6115577Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:27.6115793Z +
2019-07-18T17:43:27.6116111Z 
2019-07-18T17:43:27.6116259Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:27.6116641Z Actual stderr saved to /tmp/compiletestLiAEmM/thread-local.stderr
2019-07-18T17:43:27.6116854Z To update references, run this command from build directory:
2019-07-18T17:43:27.6117247Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'thread-local.rs'
2019-07-18T17:43:27.6117597Z error: 1 errors occurred comparing output.
2019-07-18T17:43:27.6117741Z status: exit code: 1
2019-07-18T17:43:27.6117741Z status: exit code: 1
2019-07-18T17:43:27.6118489Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/thread-local.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/thread-local.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/thread-local.stage-id.aux" "-A" "unused"
2019-07-18T17:43:27.6119856Z ------------------------------------------
2019-07-18T17:43:27.6120062Z 
2019-07-18T17:43:27.6120499Z ------------------------------------------
2019-07-18T17:43:27.6120721Z stderr:
---
2019-07-18T17:43:27.7486057Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:27.7486105Z +
2019-07-18T17:43:27.7486130Z 
2019-07-18T17:43:27.7486195Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:27.7486245Z Actual stderr saved to /tmp/compiletestLiAEmM/transmute_fat.stderr
2019-07-18T17:43:27.7486302Z To update references, run this command from build directory:
2019-07-18T17:43:27.7486911Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'transmute_fat.rs'
2019-07-18T17:43:27.7486983Z error: 1 errors occurred comparing output.
2019-07-18T17:43:27.7487023Z status: exit code: 1
2019-07-18T17:43:27.7487023Z status: exit code: 1
2019-07-18T17:43:27.7487643Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/transmute_fat.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/transmute_fat.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletestLiAEmM/transmute_fat.stage-id.aux" "-A" "unused"
2019-07-18T17:43:27.7487940Z ------------------------------------------
2019-07-18T17:43:27.7488133Z 
2019-07-18T17:43:27.7488485Z ------------------------------------------
2019-07-18T17:43:27.7488541Z stderr:
---
2019-07-18T17:43:27.7916908Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:27.7916962Z +
2019-07-18T17:43:27.7916982Z 
2019-07-18T17:43:27.7917024Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:27.7917063Z Actual stderr saved to /tmp/compiletestLiAEmM/traits.stderr
2019-07-18T17:43:27.7917119Z To update references, run this command from build directory:
2019-07-18T17:43:27.7917333Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'traits.rs'
2019-07-18T17:43:27.7917409Z error: 1 errors occurred comparing output.
2019-07-18T17:43:27.7917444Z status: exit code: 1
2019-07-18T17:43:27.7917444Z status: exit code: 1
2019-07-18T17:43:27.7917927Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/traits.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/traits.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/traits.stage-id.aux" "-A" "unused"
2019-07-18T17:43:27.7918192Z ------------------------------------------
2019-07-18T17:43:27.7918220Z 
2019-07-18T17:43:27.7918417Z ------------------------------------------
2019-07-18T17:43:27.7918452Z stderr:
---
2019-07-18T17:43:27.8760773Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:27.8760822Z +
2019-07-18T17:43:27.8760849Z 
2019-07-18T17:43:27.8760894Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:27.8760962Z Actual stderr saved to /tmp/compiletestLiAEmM/trivial.stderr
2019-07-18T17:43:27.8761013Z To update references, run this command from build directory:
2019-07-18T17:43:27.8761278Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'trivial.rs'
2019-07-18T17:43:27.8761383Z error: 1 errors occurred comparing output.
2019-07-18T17:43:27.8761428Z status: exit code: 1
2019-07-18T17:43:27.8761428Z status: exit code: 1
2019-07-18T17:43:27.8762087Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/trivial.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/trivial.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/trivial.stage-id.aux" "-A" "unused"
2019-07-18T17:43:27.8762394Z ------------------------------------------
2019-07-18T17:43:27.8762445Z 
2019-07-18T17:43:27.8762667Z ------------------------------------------
2019-07-18T17:43:27.8762712Z stderr:
---
2019-07-18T17:43:27.9265741Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:27.9265795Z +
2019-07-18T17:43:27.9265820Z 
2019-07-18T17:43:27.9265880Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:27.9266129Z Actual stderr saved to /tmp/compiletestLiAEmM/try-operator-custom.stderr
2019-07-18T17:43:27.9266180Z To update references, run this command from build directory:
2019-07-18T17:43:27.9266452Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'try-operator-custom.rs'
2019-07-18T17:43:27.9266532Z error: 1 errors occurred comparing output.
2019-07-18T17:43:27.9266573Z status: exit code: 1
2019-07-18T17:43:27.9266573Z status: exit code: 1
2019-07-18T17:43:27.9267204Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/try-operator-custom.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/try-operator-custom.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/try-operator-custom.stage-id.aux" "-A" "unused"
2019-07-18T17:43:27.9267508Z ------------------------------------------
2019-07-18T17:43:27.9267540Z 
2019-07-18T17:43:27.9267750Z ------------------------------------------
2019-07-18T17:43:27.9267812Z stderr:
---
2019-07-18T17:43:28.0442388Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:28.0442440Z +
2019-07-18T17:43:28.0442469Z 
2019-07-18T17:43:28.0442516Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:28.0442590Z Actual stderr saved to /tmp/compiletestLiAEmM/tuple_like_enum_variant_constructor.stderr
2019-07-18T17:43:28.0442810Z To update references, run this command from build directory:
2019-07-18T17:43:28.0443061Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'tuple_like_enum_variant_constructor.rs'
2019-07-18T17:43:28.0443147Z error: 1 errors occurred comparing output.
2019-07-18T17:43:28.0443185Z status: exit code: 1
2019-07-18T17:43:28.0443185Z status: exit code: 1
2019-07-18T17:43:28.0443813Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/tuple_like_enum_variant_constructor.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/tuple_like_enum_variant_constructor.stage-id.aux" "-A" "unused"
2019-07-18T17:43:28.0444097Z ------------------------------------------
2019-07-18T17:43:28.0444127Z 
2019-07-18T17:43:28.0444323Z ------------------------------------------
2019-07-18T17:43:28.0444362Z stderr:
---
2019-07-18T17:43:28.0974068Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:28.0974202Z +
2019-07-18T17:43:28.0974226Z 
2019-07-18T17:43:28.0974266Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:28.0974340Z Actual stderr saved to /tmp/compiletestLiAEmM/tuple_like_enum_variant_constructor_pointer_opt.stderr
2019-07-18T17:43:28.0974386Z To update references, run this command from build directory:
2019-07-18T17:43:28.0974662Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'tuple_like_enum_variant_constructor_pointer_opt.rs'
2019-07-18T17:43:28.0974752Z error: 1 errors occurred comparing output.
2019-07-18T17:43:28.0974798Z status: exit code: 1
2019-07-18T17:43:28.0974798Z status: exit code: 1
2019-07-18T17:43:28.0975631Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor_pointer_opt.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/tuple_like_enum_variant_constructor_pointer_opt.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/tuple_like_enum_variant_constructor_pointer_opt.stage-id.aux" "-A" "unused"
2019-07-18T17:43:28.0975925Z ------------------------------------------
2019-07-18T17:43:28.0975955Z 
2019-07-18T17:43:28.0976147Z ------------------------------------------
2019-07-18T17:43:28.0976203Z stderr:
---
2019-07-18T17:43:28.2524261Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:28.2524305Z +
2019-07-18T17:43:28.2524329Z 
2019-07-18T17:43:28.2524398Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:28.2524448Z Actual stderr saved to /tmp/compiletestLiAEmM/tuple_like_enum_variant_constructor_struct_pointer_opt.stderr
2019-07-18T17:43:28.2524497Z To update references, run this command from build directory:
2019-07-18T17:43:28.2550505Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'tuple_like_enum_variant_constructor_struct_pointer_opt.rs'
2019-07-18T17:43:28.2550632Z error: 1 errors occurred comparing output.
2019-07-18T17:43:28.2550700Z status: exit code: 1
2019-07-18T17:43:28.2550700Z status: exit code: 1
2019-07-18T17:43:28.2551687Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor_struct_pointer_opt.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/tuple_like_enum_variant_constructor_struct_pointer_opt.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/tuple_like_enum_variant_constructor_struct_pointer_opt.stage-id.aux" "-A" "unused"
2019-07-18T17:43:28.2552070Z ------------------------------------------
2019-07-18T17:43:28.2552107Z 
2019-07-18T17:43:28.2552342Z ------------------------------------------
2019-07-18T17:43:28.2552391Z stderr:
---
2019-07-18T17:43:28.2677463Z +
2019-07-18T17:43:28.2678033Z thread '[ui] run-pass/tuple_like_struct_constructor.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-18T17:43:28.2678403Z 
2019-07-18T17:43:28.2679431Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:28.2679642Z Actual stderr saved to /tmp/compiletestLiAEmM/tuple_like_struct_constructor.stderr
2019-07-18T17:43:28.2679857Z To update references, run this command from build directory:
2019-07-18T17:43:28.2680384Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'tuple_like_struct_constructor.rs'
2019-07-18T17:43:28.2680711Z error: 1 errors occurred comparing output.
2019-07-18T17:43:28.2680982Z status: exit code: 1
2019-07-18T17:43:28.2680982Z status: exit code: 1
2019-07-18T17:43:28.2682256Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_struct_constructor.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/tuple_like_struct_constructor.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/tuple_like_struct_constructor.stage-id.aux" "-A" "unused"
2019-07-18T17:43:28.2683093Z ------------------------------------------
2019-07-18T17:43:28.2683180Z 
2019-07-18T17:43:28.2683377Z ------------------------------------------
2019-07-18T17:43:28.2683416Z stderr:
---
2019-07-18T17:43:28.4243284Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:28.4243348Z +
2019-07-18T17:43:28.4243370Z 
2019-07-18T17:43:28.4243407Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:28.4243610Z Actual stderr saved to /tmp/compiletestLiAEmM/union-overwrite.stderr
2019-07-18T17:43:28.4243671Z To update references, run this command from build directory:
2019-07-18T17:43:28.4243889Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'union-overwrite.rs'
2019-07-18T17:43:28.4244155Z error: 1 errors occurred comparing output.
2019-07-18T17:43:28.4244323Z status: exit code: 1
2019-07-18T17:43:28.4244323Z status: exit code: 1
2019-07-18T17:43:28.4245102Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/union-overwrite.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/union-overwrite.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/union-overwrite.stage-id.aux" "-A" "unused"
2019-07-18T17:43:28.4245396Z ------------------------------------------
2019-07-18T17:43:28.4245427Z 
2019-07-18T17:43:28.4245636Z ------------------------------------------
2019-07-18T17:43:28.4245676Z stderr:
---
2019-07-18T17:43:28.5842103Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:28.5842156Z +
2019-07-18T17:43:28.5842187Z 
2019-07-18T17:43:28.5842448Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:28.5842491Z Actual stderr saved to /tmp/compiletestLiAEmM/union.stderr
2019-07-18T17:43:28.5842529Z To update references, run this command from build directory:
2019-07-18T17:43:28.5842775Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'union.rs'
2019-07-18T17:43:28.5842838Z error: 1 errors occurred comparing output.
2019-07-18T17:43:28.5842874Z status: exit code: 1
2019-07-18T17:43:28.5842874Z status: exit code: 1
2019-07-18T17:43:28.5843824Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/union.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/union.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/union.stage-id.aux" "-A" "unused"
2019-07-18T17:43:28.5844302Z ------------------------------------------
2019-07-18T17:43:28.5844331Z 
2019-07-18T17:43:28.5844509Z ------------------------------------------
2019-07-18T17:43:28.5844563Z stderr:
---
2019-07-18T17:43:28.6239566Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:28.6239636Z +
2019-07-18T17:43:28.6239675Z 
2019-07-18T17:43:28.6239722Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:28.6239773Z Actual stderr saved to /tmp/compiletestLiAEmM/u128.stderr
2019-07-18T17:43:28.6239844Z To update references, run this command from build directory:
2019-07-18T17:43:28.6240135Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'u128.rs'
2019-07-18T17:43:28.6240235Z error: 1 errors occurred comparing output.
2019-07-18T17:43:28.6240283Z status: exit code: 1
2019-07-18T17:43:28.6240283Z status: exit code: 1
2019-07-18T17:43:28.6241050Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/u128.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/u128.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/u128.stage-id.aux" "-A" "unused"
2019-07-18T17:43:28.6241445Z ------------------------------------------
2019-07-18T17:43:28.6241482Z 
2019-07-18T17:43:28.6241745Z ------------------------------------------
2019-07-18T17:43:28.6241794Z stderr:
---
2019-07-18T17:43:28.7442879Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:28.7442927Z +
2019-07-18T17:43:28.7442969Z 
2019-07-18T17:43:28.7443005Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:28.7443043Z Actual stderr saved to /tmp/compiletestLiAEmM/unops.stderr
2019-07-18T17:43:28.7443101Z To update references, run this command from build directory:
2019-07-18T17:43:28.7443321Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'unops.rs'
2019-07-18T17:43:28.7443383Z error: 1 errors occurred comparing output.
2019-07-18T17:43:28.7443575Z status: exit code: 1
2019-07-18T17:43:28.7443575Z status: exit code: 1
2019-07-18T17:43:28.7444104Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/unops.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/unops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/unops.stage-id.aux" "-A" "unused"
2019-07-18T17:43:28.7444378Z ------------------------------------------
2019-07-18T17:43:28.7444406Z 
2019-07-18T17:43:28.7444583Z ------------------------------------------
2019-07-18T17:43:28.7444639Z stderr:
---
2019-07-18T17:43:28.7799401Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:28.7799463Z +
2019-07-18T17:43:28.7799489Z 
2019-07-18T17:43:28.7799535Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:28.7799820Z Actual stderr saved to /tmp/compiletestLiAEmM/unsized-tuple-impls.stderr
2019-07-18T17:43:28.7799875Z To update references, run this command from build directory:
2019-07-18T17:43:28.7800144Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'unsized-tuple-impls.rs'
2019-07-18T17:43:28.7800352Z error: 1 errors occurred comparing output.
2019-07-18T17:43:28.7800407Z status: exit code: 1
2019-07-18T17:43:28.7800407Z status: exit code: 1
2019-07-18T17:43:28.7801120Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/unsized-tuple-impls.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/unsized-tuple-impls.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/unsized-tuple-impls.stage-id.aux" "-A" "unused"
2019-07-18T17:43:28.7801444Z ------------------------------------------
2019-07-18T17:43:28.7801495Z 
2019-07-18T17:43:28.7801722Z ------------------------------------------
2019-07-18T17:43:28.7801768Z stderr:
---
2019-07-18T17:43:28.8884730Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:28.8884772Z +
2019-07-18T17:43:28.8884796Z 
2019-07-18T17:43:28.8884852Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:28.8884895Z Actual stderr saved to /tmp/compiletestLiAEmM/validation_lifetime_resolution.stderr
2019-07-18T17:43:28.8884937Z To update references, run this command from build directory:
2019-07-18T17:43:28.8885350Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'validation_lifetime_resolution.rs'
2019-07-18T17:43:28.8885430Z error: 1 errors occurred comparing output.
2019-07-18T17:43:28.8885484Z status: exit code: 1
2019-07-18T17:43:28.8885484Z status: exit code: 1
2019-07-18T17:43:28.8886080Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/validation_lifetime_resolution.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/validation_lifetime_resolution.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/validation_lifetime_resolution.stage-id.aux" "-A" "unused"
2019-07-18T17:43:28.8886372Z ------------------------------------------
2019-07-18T17:43:28.8886401Z 
2019-07-18T17:43:28.8886592Z ------------------------------------------
2019-07-18T17:43:28.8886657Z stderr:
---
2019-07-18T17:43:28.9520150Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:28.9520199Z +
2019-07-18T17:43:28.9520226Z 
2019-07-18T17:43:28.9520287Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:28.9520698Z Actual stderr saved to /tmp/compiletestLiAEmM/vec-matching-fold.stderr
2019-07-18T17:43:28.9520763Z To update references, run this command from build directory:
2019-07-18T17:43:28.9521082Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'vec-matching-fold.rs'
2019-07-18T17:43:28.9521164Z error: 1 errors occurred comparing output.
2019-07-18T17:43:28.9521210Z status: exit code: 1
2019-07-18T17:43:28.9521210Z status: exit code: 1
2019-07-18T17:43:28.9521910Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vec-matching-fold.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/vec-matching-fold.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/vec-matching-fold.stage-id.aux" "-A" "unused"
2019-07-18T17:43:28.9522401Z ------------------------------------------
2019-07-18T17:43:28.9522431Z 
2019-07-18T17:43:28.9522618Z ------------------------------------------
2019-07-18T17:43:28.9522671Z stderr:
---
2019-07-18T17:43:29.0843688Z -Iter([], [])
2019-07-18T17:43:29.0846820Z -
2019-07-18T17:43:29.0853155Z 
2019-07-18T17:43:29.0853266Z The actual stdout differed from the expected stdout.
2019-07-18T17:43:29.0853312Z Actual stdout saved to /tmp/compiletestLiAEmM/vecdeque.stdout
2019-07-18T17:43:29.0859929Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-18T17:43:29.0860364Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-18T17:43:29.0860583Z      |
2019-07-18T17:43:29.0860675Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-18T17:43:29.0886655Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:29.0886697Z +
2019-07-18T17:43:29.0886737Z 
2019-07-18T17:43:29.0886777Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:29.0886829Z Actual stderr saved to /tmp/compiletestLiAEmM/vecdeque.stderr
2019-07-18T17:43:29.0886888Z To update references, run this command from build directory:
2019-07-18T17:43:29.0887137Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'vecdeque.rs'
2019-07-18T17:43:29.0887207Z error: 2 errors occurred comparing output.
2019-07-18T17:43:29.0887263Z status: exit code: 1
2019-07-18T17:43:29.0887263Z status: exit code: 1
2019-07-18T17:43:29.0887819Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vecdeque.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/vecdeque.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/vecdeque.stage-id.aux" "-A" "unused"
2019-07-18T17:43:29.0888337Z ------------------------------------------
2019-07-18T17:43:29.0888369Z 
2019-07-18T17:43:29.0889129Z ------------------------------------------
2019-07-18T17:43:29.0889201Z stderr:
---
2019-07-18T17:43:29.2281945Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:29.2281995Z +
2019-07-18T17:43:29.2282024Z 
2019-07-18T17:43:29.2282081Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:29.2282154Z Actual stderr saved to /tmp/compiletestLiAEmM/vecs.stderr
2019-07-18T17:43:29.2282206Z To update references, run this command from build directory:
2019-07-18T17:43:29.2282622Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'vecs.rs'
2019-07-18T17:43:29.2282702Z error: 1 errors occurred comparing output.
2019-07-18T17:43:29.2282737Z status: exit code: 1
2019-07-18T17:43:29.2282737Z status: exit code: 1
2019-07-18T17:43:29.2283380Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vecs.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/vecs.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/vecs.stage-id.aux" "-A" "unused"
2019-07-18T17:43:29.2283634Z ------------------------------------------
2019-07-18T17:43:29.2283680Z 
2019-07-18T17:43:29.2283860Z ------------------------------------------
2019-07-18T17:43:29.2283897Z stderr:
---
2019-07-18T17:43:29.2476032Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:29.2476080Z +
2019-07-18T17:43:29.2476105Z 
2019-07-18T17:43:29.2476166Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:29.2476213Z Actual stderr saved to /tmp/compiletestLiAEmM/volatile.stderr
2019-07-18T17:43:29.2476268Z To update references, run this command from build directory:
2019-07-18T17:43:29.2476565Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'volatile.rs'
2019-07-18T17:43:29.2476639Z error: 1 errors occurred comparing output.
2019-07-18T17:43:29.2476696Z status: exit code: 1
2019-07-18T17:43:29.2476696Z status: exit code: 1
2019-07-18T17:43:29.2477283Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/volatile.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/volatile.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/volatile.stage-id.aux" "-A" "unused"
2019-07-18T17:43:29.2477721Z ------------------------------------------
2019-07-18T17:43:29.2477755Z 
2019-07-18T17:43:29.2477981Z ------------------------------------------
2019-07-18T17:43:29.2478041Z stderr:
---
2019-07-18T17:43:29.3840837Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:29.3840905Z +
2019-07-18T17:43:29.3840933Z 
2019-07-18T17:43:29.3840981Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:29.3841293Z Actual stderr saved to /tmp/compiletestLiAEmM/without-validation.stderr
2019-07-18T17:43:29.3841350Z To update references, run this command from build directory:
2019-07-18T17:43:29.3841642Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'without-validation.rs'
2019-07-18T17:43:29.3841743Z error: 1 errors occurred comparing output.
2019-07-18T17:43:29.3841790Z status: exit code: 1
2019-07-18T17:43:29.3841790Z status: exit code: 1
2019-07-18T17:43:29.3842743Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/without-validation.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/without-validation.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletestLiAEmM/without-validation.stage-id.aux" "-A" "unused"
2019-07-18T17:43:29.3843689Z ------------------------------------------
2019-07-18T17:43:29.3843753Z 
2019-07-18T17:43:29.3844011Z ------------------------------------------
2019-07-18T17:43:29.3844058Z stderr:
---
2019-07-18T17:43:29.4481034Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:29.4481111Z +
2019-07-18T17:43:29.4481140Z 
2019-07-18T17:43:29.4481186Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:29.4481442Z Actual stderr saved to /tmp/compiletestLiAEmM/write-bytes.stderr
2019-07-18T17:43:29.4481515Z To update references, run this command from build directory:
2019-07-18T17:43:29.4481781Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'write-bytes.rs'
2019-07-18T17:43:29.4481968Z error: 1 errors occurred comparing output.
2019-07-18T17:43:29.4482014Z status: exit code: 1
2019-07-18T17:43:29.4482014Z status: exit code: 1
2019-07-18T17:43:29.4482829Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/write-bytes.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/write-bytes.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/write-bytes.stage-id.aux" "-A" "unused"
2019-07-18T17:43:29.4483107Z ------------------------------------------
2019-07-18T17:43:29.4483136Z 
2019-07-18T17:43:29.4483340Z ------------------------------------------
2019-07-18T17:43:29.4483380Z stderr:
---
2019-07-18T17:43:29.5514411Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:29.5514456Z +
2019-07-18T17:43:29.5514479Z 
2019-07-18T17:43:29.5514519Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:29.5514787Z Actual stderr saved to /tmp/compiletestLiAEmM/zero-sized-binary-heap-push.stderr
2019-07-18T17:43:29.5514836Z To update references, run this command from build directory:
2019-07-18T17:43:29.5515086Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'zero-sized-binary-heap-push.rs'
2019-07-18T17:43:29.5515271Z error: 1 errors occurred comparing output.
2019-07-18T17:43:29.5515311Z status: exit code: 1
2019-07-18T17:43:29.5515311Z status: exit code: 1
2019-07-18T17:43:29.5515959Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zero-sized-binary-heap-push.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/zero-sized-binary-heap-push.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/zero-sized-binary-heap-push.stage-id.aux" "-A" "unused"
2019-07-18T17:43:29.5516249Z ------------------------------------------
2019-07-18T17:43:29.5516299Z 
2019-07-18T17:43:29.5516507Z ------------------------------------------
2019-07-18T17:43:29.5516547Z stderr:
---
2019-07-18T17:43:29.6157994Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:29.6158036Z +
2019-07-18T17:43:29.6163405Z 
2019-07-18T17:43:29.6163489Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:29.6166834Z Actual stderr saved to /tmp/compiletestLiAEmM/zst.stderr
2019-07-18T17:43:29.6170335Z To update references, run this command from build directory:
2019-07-18T17:43:29.6174957Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'zst.rs'
2019-07-18T17:43:29.6180058Z error: 1 errors occurred comparing output.
2019-07-18T17:43:29.6180121Z status: exit code: 1
2019-07-18T17:43:29.6180121Z status: exit code: 1
2019-07-18T17:43:29.6184972Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/zst.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/zst.stage-id.aux" "-A" "unused"
2019-07-18T17:43:29.6185286Z ------------------------------------------
2019-07-18T17:43:29.6185335Z 
2019-07-18T17:43:29.6185525Z ------------------------------------------
2019-07-18T17:43:29.6185572Z stderr:
---
2019-07-18T17:43:29.6824971Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:29.6825012Z +
2019-07-18T17:43:29.6825034Z 
2019-07-18T17:43:29.6825079Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:29.6825120Z Actual stderr saved to /tmp/compiletestLiAEmM/zst_box.stderr
2019-07-18T17:43:29.6825235Z To update references, run this command from build directory:
2019-07-18T17:43:29.6825488Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'zst_box.rs'
2019-07-18T17:43:29.6825556Z error: 1 errors occurred comparing output.
2019-07-18T17:43:29.6825593Z status: exit code: 1
2019-07-18T17:43:29.6825593Z status: exit code: 1
2019-07-18T17:43:29.6826120Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst_box.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/zst_box.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/zst_box.stage-id.aux" "-A" "unused"
2019-07-18T17:43:29.6826387Z ------------------------------------------
2019-07-18T17:43:29.6826423Z 
2019-07-18T17:43:29.6826619Z ------------------------------------------
2019-07-18T17:43:29.6826663Z stderr:
---
2019-07-18T17:43:29.7446377Z +For more information about this error, try `rustc --explain E0080`.
2019-07-18T17:43:29.7446416Z +
2019-07-18T17:43:29.7446454Z 
2019-07-18T17:43:29.7446492Z The actual stderr differed from the expected stderr.
2019-07-18T17:43:29.7446534Z Actual stderr saved to /tmp/compiletestLiAEmM/zst_variant_drop.stderr
2019-07-18T17:43:29.7446600Z To update references, run this command from build directory:
2019-07-18T17:43:29.7446828Z tests/run-pass/update-references.sh '/tmp/compiletestLiAEmM' 'zst_variant_drop.rs'
2019-07-18T17:43:29.7446894Z error: 1 errors occurred comparing output.
2019-07-18T17:43:29.7446947Z status: exit code: 1
2019-07-18T17:43:29.7446947Z status: exit code: 1
2019-07-18T17:43:29.7447488Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst_variant_drop.rs" "-L" "/tmp/compiletestLiAEmM" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestLiAEmM/zst_variant_drop.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestLiAEmM/zst_variant_drop.stage-id.aux" "-A" "unused"
2019-07-18T17:43:29.7447765Z ------------------------------------------
2019-07-18T17:43:29.7447794Z 
2019-07-18T17:43:29.7447977Z ------------------------------------------
2019-07-18T17:43:29.7448032Z stderr:
---
2019-07-18T17:43:29.7857440Z Cloning into 'rust-toolstate'...
2019-07-18T17:43:30.5204588Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-18T17:43:30.5441945Z [master d757389] (linux CI update)
2019-07-18T17:43:30.5442055Z  1 file changed, 1 insertion(+)
2019-07-18T17:43:31.2635801Z remote: Invalid username or password.
2019-07-18T17:43:31.2636960Z fatal: Authentication failed for 'https://github.com/rust-lang-nursery/rust-toolstate.git/'
2019-07-18T17:43:31.5986526Z  * branch            master     -> FETCH_HEAD
2019-07-18T17:43:31.5986526Z  * branch            master     -> FETCH_HEAD
2019-07-18T17:43:31.6172922Z HEAD is now at 601d596 Merge pull request #13 from atouchet/links
2019-07-18T17:43:31.6293057Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-18T17:43:31.6464398Z [master 8d4ee6f] (linux CI update)
2019-07-18T17:43:31.6464464Z  1 file changed, 1 insertion(+)
2019-07-18T17:43:31.9791628Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-18T17:43:32.3360019Z  * branch            master     -> FETCH_HEAD
2019-07-18T17:43:32.3360019Z  * branch            master     -> FETCH_HEAD
2019-07-18T17:43:32.3502731Z HEAD is now at 601d596 Merge pull request #13 from atouchet/links
2019-07-18T17:43:32.3618961Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-18T17:43:32.3803151Z [master 55dd91f] (linux CI update)
2019-07-18T17:43:32.3803223Z  1 file changed, 1 insertion(+)
2019-07-18T17:43:32.6895480Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-18T17:43:34.0221978Z  * branch            master     -> FETCH_HEAD
2019-07-18T17:43:34.0221978Z  * branch            master     -> FETCH_HEAD
2019-07-18T17:43:34.0372608Z HEAD is now at 601d596 Merge pull request #13 from atouchet/links
2019-07-18T17:43:34.0478844Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-18T17:43:34.0653889Z [master 69cc905] (linux CI update)
2019-07-18T17:43:34.0654492Z  1 file changed, 1 insertion(+)
2019-07-18T17:43:34.3745720Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-18T17:43:34.7286707Z  * branch            master     -> FETCH_HEAD
2019-07-18T17:43:34.7286707Z  * branch            master     -> FETCH_HEAD
2019-07-18T17:43:34.7437374Z HEAD is now at 601d596 Merge pull request #13 from atouchet/links
2019-07-18T17:43:34.7553136Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-18T17:43:34.7732285Z [master 69cc905] (linux CI update)
2019-07-18T17:43:34.7732378Z  1 file changed, 1 insertion(+)
2019-07-18T17:43:35.0867854Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-18T17:43:37.4350364Z  * branch            master     -> FETCH_HEAD
2019-07-18T17:43:37.4350364Z  * branch            master     -> FETCH_HEAD
2019-07-18T17:43:37.4489321Z HEAD is now at 601d596 Merge pull request #13 from atouchet/links
2019-07-18T17:43:38.0902365Z ##[error]Bash exited with code '1'.
2019-07-18T17:43:38.0936723Z ##[section]Starting: Checkout
2019-07-18T17:43:38.0938607Z ==============================================================================
2019-07-18T17:43:38.0938649Z Task         : Get sources
2019-07-18T17:43:38.0938704Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
