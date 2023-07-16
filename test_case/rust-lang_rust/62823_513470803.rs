plain
2019-07-20T11:45:02.0031738Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-20T11:45:02.0235849Z ##[command]git config gc.auto 0
2019-07-20T11:45:02.0320062Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-20T11:45:02.0375441Z ##[command]git config --get-all http.proxy
2019-07-20T11:45:02.0525689Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62823/merge:refs/remotes/pull/62823/merge
---
2019-07-20T11:45:38.0236330Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-20T11:45:38.0236377Z 
2019-07-20T11:45:38.0236646Z   git checkout -b <new-branch-name>
2019-07-20T11:45:38.0236686Z 
2019-07-20T11:45:38.0236742Z HEAD is now at 3d17044db Merge 24c86eb2e887d614d55a92c21a79745ee6d9d068 into f69b07144a151f46aaee1b6230ba4160e9394562
2019-07-20T11:45:38.0404164Z ##[section]Finishing: Checkout
2019-07-20T11:45:38.0413877Z ##[section]Starting: Decide whether to run this job
2019-07-20T11:45:38.0418184Z Task         : Bash
2019-07-20T11:45:38.0418237Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-20T11:45:38.0418306Z Version      : 3.151.2
2019-07-20T11:45:38.0418353Z Author       : Microsoft Corporation
2019-07-20T11:45:38.0418353Z Author       : Microsoft Corporation
2019-07-20T11:45:38.0418410Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-20T11:45:38.0418485Z ==============================================================================
2019-07-20T11:45:38.2000788Z Generating script.
2019-07-20T11:45:38.2037700Z ========================== Starting Command Output ===========================
2019-07-20T11:45:38.2069768Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/04c0e9ca-1d46-4998-a51c-219ee5c14e67.sh
2019-07-20T11:45:38.2464906Z Executing the job since submodules are updated
2019-07-20T11:45:38.2601074Z ##[section]Finishing: Decide whether to run this job
2019-07-20T11:45:38.2611944Z ==============================================================================
2019-07-20T11:45:38.2612063Z Task         : Bash
2019-07-20T11:45:38.2612116Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-20T11:45:38.2612169Z Version      : 3.151.2
---
2019-07-20T14:10:41.8016445Z -args
2019-07-20T14:10:41.8020864Z -
2019-07-20T14:10:41.8027103Z 
2019-07-20T14:10:41.8027218Z The actual stdout differed from the expected stdout.
2019-07-20T14:10:41.8066506Z Actual stdout saved to /tmp/compiletest8i5dKZ/args.stdout
2019-07-20T14:10:41.8120414Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-20T14:10:41.8120915Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-20T14:10:41.8120971Z      |
2019-07-20T14:10:41.8121029Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-20T14:10:41.8124964Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:41.8125007Z +
2019-07-20T14:10:41.8125029Z 
2019-07-20T14:10:41.8125079Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:41.8125122Z Actual stderr saved to /tmp/compiletest8i5dKZ/args.stderr
2019-07-20T14:10:41.8125164Z To update references, run this command from build directory:
2019-07-20T14:10:41.8125396Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'args.rs'
2019-07-20T14:10:41.8125465Z error: 2 errors occurred comparing output.
2019-07-20T14:10:41.8125518Z status: exit code: 1
2019-07-20T14:10:41.8125518Z status: exit code: 1
2019-07-20T14:10:41.8126206Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/args.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/args.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/args.stage-id.aux" "-A" "unused"
2019-07-20T14:10:41.8126516Z ------------------------------------------
2019-07-20T14:10:41.8126548Z 
2019-07-20T14:10:41.8126754Z ------------------------------------------
2019-07-20T14:10:41.8126808Z stderr:
---
2019-07-20T14:10:41.8930847Z -[0, 42, 13, 71]
2019-07-20T14:10:41.8931058Z -
2019-07-20T14:10:41.8931089Z 
2019-07-20T14:10:41.8931134Z The actual stdout differed from the expected stdout.
2019-07-20T14:10:41.8931243Z Actual stdout saved to /tmp/compiletest8i5dKZ/arrays.stdout
2019-07-20T14:10:41.8931377Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-20T14:10:41.8931644Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-20T14:10:41.8931712Z      |
2019-07-20T14:10:41.8931762Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-20T14:10:41.8935521Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:41.8935570Z +
2019-07-20T14:10:41.8935597Z 
2019-07-20T14:10:41.8935658Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:41.8935709Z Actual stderr saved to /tmp/compiletest8i5dKZ/arrays.stderr
2019-07-20T14:10:41.8935759Z To update references, run this command from build directory:
2019-07-20T14:10:41.8936031Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'arrays.rs'
2019-07-20T14:10:41.8936126Z error: 2 errors occurred comparing output.
2019-07-20T14:10:41.8936170Z status: exit code: 1
2019-07-20T14:10:41.8936170Z status: exit code: 1
2019-07-20T14:10:41.8936889Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/arrays.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/arrays.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/arrays.stage-id.aux" "-A" "unused"
2019-07-20T14:10:41.8937268Z ------------------------------------------
2019-07-20T14:10:41.8937303Z 
2019-07-20T14:10:41.8937544Z ------------------------------------------
2019-07-20T14:10:41.8937589Z stderr:
---
2019-07-20T14:10:41.9771857Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:41.9771928Z +
2019-07-20T14:10:41.9771957Z 
2019-07-20T14:10:41.9772005Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:41.9772304Z Actual stderr saved to /tmp/compiletest8i5dKZ/associated-const.stderr
2019-07-20T14:10:41.9772376Z To update references, run this command from build directory:
2019-07-20T14:10:41.9772667Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'associated-const.rs'
2019-07-20T14:10:41.9772765Z error: 1 errors occurred comparing output.
2019-07-20T14:10:41.9772812Z status: exit code: 1
2019-07-20T14:10:41.9772812Z status: exit code: 1
2019-07-20T14:10:41.9773623Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/associated-const.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/associated-const.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/associated-const.stage-id.aux" "-A" "unused"
2019-07-20T14:10:41.9774230Z ------------------------------------------
2019-07-20T14:10:41.9774261Z 
2019-07-20T14:10:41.9774472Z ------------------------------------------
2019-07-20T14:10:41.9774511Z stderr:
---
2019-07-20T14:10:42.0476221Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:42.0476283Z +
2019-07-20T14:10:42.0476317Z 
2019-07-20T14:10:42.0476372Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:42.0476456Z Actual stderr saved to /tmp/compiletest8i5dKZ/assume_bug.stderr
2019-07-20T14:10:42.0476518Z To update references, run this command from build directory:
2019-07-20T14:10:42.0476873Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'assume_bug.rs'
2019-07-20T14:10:42.0476995Z error: 1 errors occurred comparing output.
2019-07-20T14:10:42.0477050Z status: exit code: 1
2019-07-20T14:10:42.0477050Z status: exit code: 1
2019-07-20T14:10:42.0477965Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/assume_bug.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/assume_bug.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/assume_bug.stage-id.aux" "-A" "unused"
2019-07-20T14:10:42.0478521Z ------------------------------------------
2019-07-20T14:10:42.0478570Z 
2019-07-20T14:10:42.0478878Z ------------------------------------------
2019-07-20T14:10:42.0478936Z stderr:
---
2019-07-20T14:10:42.1734869Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:42.1734932Z +
2019-07-20T14:10:42.1734958Z 
2019-07-20T14:10:42.1735001Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:42.1735253Z Actual stderr saved to /tmp/compiletest8i5dKZ/async-fn.stderr
2019-07-20T14:10:42.1735303Z To update references, run this command from build directory:
2019-07-20T14:10:42.1735539Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'async-fn.rs'
2019-07-20T14:10:42.1735627Z error: 1 errors occurred comparing output.
2019-07-20T14:10:42.1735668Z status: exit code: 1
2019-07-20T14:10:42.1735668Z status: exit code: 1
2019-07-20T14:10:42.1736390Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/async-fn.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/async-fn.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/async-fn.stage-id.aux" "-A" "unused"
2019-07-20T14:10:42.1736748Z ------------------------------------------
2019-07-20T14:10:42.1736780Z 
2019-07-20T14:10:42.1737000Z ------------------------------------------
2019-07-20T14:10:42.1737041Z stderr:
---
2019-07-20T14:10:42.2209331Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:42.2209473Z +
2019-07-20T14:10:42.2209500Z 
2019-07-20T14:10:42.2209542Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:42.2209802Z Actual stderr saved to /tmp/compiletest8i5dKZ/atomic-access-bool.stderr
2019-07-20T14:10:42.2209881Z To update references, run this command from build directory:
2019-07-20T14:10:42.2210642Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'atomic-access-bool.rs'
2019-07-20T14:10:42.2210734Z error: 1 errors occurred comparing output.
2019-07-20T14:10:42.2210798Z status: exit code: 1
2019-07-20T14:10:42.2210798Z status: exit code: 1
2019-07-20T14:10:42.2211611Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/atomic-access-bool.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/atomic-access-bool.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/atomic-access-bool.stage-id.aux" "-A" "unused"
2019-07-20T14:10:42.2211987Z ------------------------------------------
2019-07-20T14:10:42.2212021Z 
2019-07-20T14:10:42.2212256Z ------------------------------------------
2019-07-20T14:10:42.2212302Z stderr:
---
2019-07-20T14:10:42.3494761Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:42.3494812Z +
2019-07-20T14:10:42.3494842Z 
2019-07-20T14:10:42.3494908Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:42.3494961Z Actual stderr saved to /tmp/compiletest8i5dKZ/bad_substs.stderr
2019-07-20T14:10:42.3495013Z To update references, run this command from build directory:
2019-07-20T14:10:42.3495327Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'bad_substs.rs'
2019-07-20T14:10:42.3495567Z error: 1 errors occurred comparing output.
2019-07-20T14:10:42.3495624Z status: exit code: 1
2019-07-20T14:10:42.3495624Z status: exit code: 1
2019-07-20T14:10:42.3496330Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/bad_substs.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/bad_substs.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/bad_substs.stage-id.aux" "-A" "unused"
2019-07-20T14:10:42.3496680Z ------------------------------------------
2019-07-20T14:10:42.3496716Z 
2019-07-20T14:10:42.3496956Z ------------------------------------------
2019-07-20T14:10:42.3497018Z stderr:
---
2019-07-20T14:10:42.3974557Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:42.3974625Z +
2019-07-20T14:10:42.3974660Z 
2019-07-20T14:10:42.3974716Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:42.3975100Z Actual stderr saved to /tmp/compiletest8i5dKZ/atomic-compare_exchange.stderr
2019-07-20T14:10:42.3975171Z To update references, run this command from build directory:
2019-07-20T14:10:42.3975697Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'atomic-compare_exchange.rs'
2019-07-20T14:10:42.3975853Z error: 1 errors occurred comparing output.
2019-07-20T14:10:42.3975908Z status: exit code: 1
2019-07-20T14:10:42.3975908Z status: exit code: 1
2019-07-20T14:10:42.3976775Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/atomic-compare_exchange.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/atomic-compare_exchange.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/atomic-compare_exchange.stage-id.aux" "-A" "unused"
2019-07-20T14:10:42.3977182Z ------------------------------------------
2019-07-20T14:10:42.3977248Z 
2019-07-20T14:10:42.3977548Z ------------------------------------------
2019-07-20T14:10:42.3977607Z stderr:
---
2019-07-20T14:10:42.5947104Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:42.5948060Z +
2019-07-20T14:10:42.5948342Z 
2019-07-20T14:10:42.5948529Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:42.5948715Z Actual stderr saved to /tmp/compiletest8i5dKZ/bools.stderr
2019-07-20T14:10:42.5949170Z To update references, run this command from build directory:
2019-07-20T14:10:42.5950159Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'bools.rs'
2019-07-20T14:10:42.5950972Z error: 1 errors occurred comparing output.
2019-07-20T14:10:42.5951138Z status: exit code: 1
2019-07-20T14:10:42.5951138Z status: exit code: 1
2019-07-20T14:10:42.5991750Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/bools.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/bools.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/bools.stage-id.aux" "-A" "unused"
2019-07-20T14:10:42.5992681Z ------------------------------------------
2019-07-20T14:10:42.5992891Z 
2019-07-20T14:10:42.5993357Z ------------------------------------------
2019-07-20T14:10:42.5993604Z stderr:
---
2019-07-20T14:10:42.6005042Z test [ui] run-pass/bools.rs ... FAILED
2019-07-20T14:10:42.6005680Z thread '[ui] run-pass/bools.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-20T14:10:42.6005890Z 
2019-07-20T14:10:42.6006102Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:42.6006284Z Actual stderr saved to /tmp/compiletest8i5dKZ/binops.stderr
2019-07-20T14:10:42.6006633Z To update references, run this command from build directory:
2019-07-20T14:10:42.6007207Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'binops.rs'
2019-07-20T14:10:42.6007613Z error: 1 errors occurred comparing output.
2019-07-20T14:10:42.6007792Z status: exit code: 1
2019-07-20T14:10:42.6007792Z status: exit code: 1
2019-07-20T14:10:42.6008677Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/binops.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/binops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/binops.stage-id.aux" "-A" "unused"
2019-07-20T14:10:42.6009460Z ------------------------------------------
2019-07-20T14:10:42.6009667Z 
2019-07-20T14:10:42.6010686Z ------------------------------------------
2019-07-20T14:10:42.6011001Z stderr:
---
2019-07-20T14:10:42.7452338Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:42.7452393Z +
2019-07-20T14:10:42.7452425Z 
2019-07-20T14:10:42.7452477Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:42.7452691Z Actual stderr saved to /tmp/compiletest8i5dKZ/box_box_trait.stderr
2019-07-20T14:10:42.7452758Z To update references, run this command from build directory:
2019-07-20T14:10:42.7453117Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'box_box_trait.rs'
2019-07-20T14:10:42.7453228Z error: 1 errors occurred comparing output.
2019-07-20T14:10:42.7453280Z status: exit code: 1
2019-07-20T14:10:42.7453280Z status: exit code: 1
2019-07-20T14:10:42.7454032Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/box_box_trait.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/box_box_trait.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/box_box_trait.stage-id.aux" "-A" "unused"
2019-07-20T14:10:42.7454526Z ------------------------------------------
2019-07-20T14:10:42.7454598Z 
2019-07-20T14:10:42.7454872Z ------------------------------------------
2019-07-20T14:10:42.7454924Z stderr:
---
2019-07-20T14:10:42.7556184Z -foo #1 = Foo(1337)
2019-07-20T14:10:42.7556546Z -
2019-07-20T14:10:42.7556727Z 
2019-07-20T14:10:42.7556816Z The actual stdout differed from the expected stdout.
2019-07-20T14:10:42.7557113Z Actual stdout saved to /tmp/compiletest8i5dKZ/box-pair-to-vec.stdout
2019-07-20T14:10:42.7557741Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-20T14:10:42.7558110Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-20T14:10:42.7558183Z      |
2019-07-20T14:10:42.7558376Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-20T14:10:42.7564002Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:42.7564319Z +
2019-07-20T14:10:42.7564429Z 
2019-07-20T14:10:42.7564521Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:42.7564870Z Actual stderr saved to /tmp/compiletest8i5dKZ/box-pair-to-vec.stderr
2019-07-20T14:10:42.7565070Z To update references, run this command from build directory:
2019-07-20T14:10:42.7565439Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'box-pair-to-vec.rs'
2019-07-20T14:10:42.7565520Z error: 2 errors occurred comparing output.
2019-07-20T14:10:42.7565564Z status: exit code: 1
2019-07-20T14:10:42.7565564Z status: exit code: 1
2019-07-20T14:10:42.7566446Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/box-pair-to-vec.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/box-pair-to-vec.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/box-pair-to-vec.stage-id.aux" "-A" "unused"
2019-07-20T14:10:42.7566787Z ------------------------------------------
2019-07-20T14:10:42.7566821Z 
2019-07-20T14:10:42.7567036Z ------------------------------------------
2019-07-20T14:10:42.7567099Z stderr:
---
2019-07-20T14:10:42.9212676Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:42.9212726Z +
2019-07-20T14:10:42.9212772Z 
2019-07-20T14:10:42.9212963Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:42.9213014Z Actual stderr saved to /tmp/compiletest8i5dKZ/c_enums.stderr
2019-07-20T14:10:42.9213064Z To update references, run this command from build directory:
2019-07-20T14:10:42.9213388Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'c_enums.rs'
2019-07-20T14:10:42.9213466Z error: 1 errors occurred comparing output.
2019-07-20T14:10:42.9213631Z status: exit code: 1
2019-07-20T14:10:42.9213631Z status: exit code: 1
2019-07-20T14:10:42.9214216Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/c_enums.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/c_enums.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/c_enums.stage-id.aux" "-A" "unused"
2019-07-20T14:10:42.9214638Z ------------------------------------------
2019-07-20T14:10:42.9214668Z 
2019-07-20T14:10:42.9214861Z ------------------------------------------
2019-07-20T14:10:42.9214918Z stderr:
---
2019-07-20T14:10:42.9692653Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:42.9692800Z +
2019-07-20T14:10:42.9692828Z 
2019-07-20T14:10:42.9692883Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:42.9692954Z Actual stderr saved to /tmp/compiletest8i5dKZ/btreemap.stderr
2019-07-20T14:10:42.9693005Z To update references, run this command from build directory:
2019-07-20T14:10:42.9693313Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'btreemap.rs'
2019-07-20T14:10:42.9693412Z error: 1 errors occurred comparing output.
2019-07-20T14:10:42.9693458Z status: exit code: 1
2019-07-20T14:10:42.9693458Z status: exit code: 1
2019-07-20T14:10:42.9694118Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/btreemap.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/btreemap.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/btreemap.stage-id.aux" "-A" "unused"
2019-07-20T14:10:42.9694528Z ------------------------------------------
2019-07-20T14:10:42.9694576Z 
2019-07-20T14:10:42.9694782Z ------------------------------------------
2019-07-20T14:10:42.9694822Z stderr:
---
2019-07-20T14:10:43.0977426Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:43.0977483Z +
2019-07-20T14:10:43.0977515Z 
2019-07-20T14:10:43.0977586Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:43.0977647Z Actual stderr saved to /tmp/compiletest8i5dKZ/call_drop_on_array_elements.stderr
2019-07-20T14:10:43.0977708Z To update references, run this command from build directory:
2019-07-20T14:10:43.0978061Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'call_drop_on_array_elements.rs'
2019-07-20T14:10:43.0978155Z error: 1 errors occurred comparing output.
2019-07-20T14:10:43.0978225Z status: exit code: 1
2019-07-20T14:10:43.0978225Z status: exit code: 1
2019-07-20T14:10:43.0979029Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_array_elements.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/call_drop_on_array_elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/call_drop_on_array_elements.stage-id.aux" "-A" "unused"
2019-07-20T14:10:43.0979420Z ------------------------------------------
2019-07-20T14:10:43.0979461Z 
2019-07-20T14:10:43.0979728Z ------------------------------------------
2019-07-20T14:10:43.0979799Z stderr:
---
2019-07-20T14:10:43.1459772Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:43.1459827Z +
2019-07-20T14:10:43.1459876Z 
2019-07-20T14:10:43.1459928Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:43.1459991Z Actual stderr saved to /tmp/compiletest8i5dKZ/call_drop_on_fat_ptr_array_elements.stderr
2019-07-20T14:10:43.1460050Z To update references, run this command from build directory:
2019-07-20T14:10:43.1460450Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'call_drop_on_fat_ptr_array_elements.rs'
2019-07-20T14:10:43.1460544Z error: 1 errors occurred comparing output.
2019-07-20T14:10:43.1460613Z status: exit code: 1
2019-07-20T14:10:43.1460613Z status: exit code: 1
2019-07-20T14:10:43.1461865Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_fat_ptr_array_elements.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/call_drop_on_fat_ptr_array_elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/call_drop_on_fat_ptr_array_elements.stage-id.aux" "-A" "unused"
2019-07-20T14:10:43.1462252Z ------------------------------------------
2019-07-20T14:10:43.1462291Z 
2019-07-20T14:10:43.1462553Z ------------------------------------------
2019-07-20T14:10:43.1462604Z stderr:
---
2019-07-20T14:10:43.2694443Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:43.2694673Z +
2019-07-20T14:10:43.2694824Z 
2019-07-20T14:10:43.2695016Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:43.2695197Z Actual stderr saved to /tmp/compiletest8i5dKZ/call_drop_on_zst_array_elements.stderr
2019-07-20T14:10:43.2695374Z To update references, run this command from build directory:
2019-07-20T14:10:43.2695902Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'call_drop_on_zst_array_elements.rs'
2019-07-20T14:10:43.2696328Z error: 1 errors occurred comparing output.
2019-07-20T14:10:43.2696508Z status: exit code: 1
2019-07-20T14:10:43.2696508Z status: exit code: 1
2019-07-20T14:10:43.2697615Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_on_zst_array_elements.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/call_drop_on_zst_array_elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/call_drop_on_zst_array_elements.stage-id.aux" "-A" "unused"
2019-07-20T14:10:43.2698360Z ------------------------------------------
2019-07-20T14:10:43.2698549Z 
2019-07-20T14:10:43.2699006Z ------------------------------------------
2019-07-20T14:10:43.2699210Z stderr:
---
2019-07-20T14:10:43.3217812Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:43.3218097Z +
2019-07-20T14:10:43.3218271Z 
2019-07-20T14:10:43.3218494Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:43.3218701Z Actual stderr saved to /tmp/compiletest8i5dKZ/call_drop_through_owned_slice.stderr
2019-07-20T14:10:43.3218910Z To update references, run this command from build directory:
2019-07-20T14:10:43.3219526Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'call_drop_through_owned_slice.rs'
2019-07-20T14:10:43.3220001Z error: 1 errors occurred comparing output.
2019-07-20T14:10:43.3220221Z status: exit code: 1
2019-07-20T14:10:43.3220221Z status: exit code: 1
2019-07-20T14:10:43.3221914Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_owned_slice.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/call_drop_through_owned_slice.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/call_drop_through_owned_slice.stage-id.aux" "-A" "unused"
2019-07-20T14:10:43.3222983Z ------------------------------------------
2019-07-20T14:10:43.3223181Z 
2019-07-20T14:10:43.3223666Z ------------------------------------------
2019-07-20T14:10:43.3223873Z stderr:
---
2019-07-20T14:10:43.4297311Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:43.4297365Z +
2019-07-20T14:10:43.4297396Z 
2019-07-20T14:10:43.4297448Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:43.4297524Z Actual stderr saved to /tmp/compiletest8i5dKZ/call_drop_through_trait_object.stderr
2019-07-20T14:10:43.4297582Z To update references, run this command from build directory:
2019-07-20T14:10:43.4297895Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'call_drop_through_trait_object.rs'
2019-07-20T14:10:43.4298001Z error: 1 errors occurred comparing output.
2019-07-20T14:10:43.4298051Z status: exit code: 1
2019-07-20T14:10:43.4298051Z status: exit code: 1
2019-07-20T14:10:43.4298847Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_trait_object.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/call_drop_through_trait_object.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/call_drop_through_trait_object.stage-id.aux" "-A" "unused"
2019-07-20T14:10:43.4299225Z ------------------------------------------
2019-07-20T14:10:43.4299262Z 
2019-07-20T14:10:43.4299508Z ------------------------------------------
2019-07-20T14:10:43.4299558Z stderr:
---
2019-07-20T14:10:43.4573322Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:43.4573399Z +
2019-07-20T14:10:43.4573430Z 
2019-07-20T14:10:43.4573479Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:43.4573552Z Actual stderr saved to /tmp/compiletest8i5dKZ/call_drop_through_trait_object_rc.stderr
2019-07-20T14:10:43.4573610Z To update references, run this command from build directory:
2019-07-20T14:10:43.4574047Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'call_drop_through_trait_object_rc.rs'
2019-07-20T14:10:43.4574279Z error: 1 errors occurred comparing output.
2019-07-20T14:10:43.4574326Z status: exit code: 1
2019-07-20T14:10:43.4574326Z status: exit code: 1
2019-07-20T14:10:43.4575019Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/call_drop_through_trait_object_rc.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/call_drop_through_trait_object_rc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/call_drop_through_trait_object_rc.stage-id.aux" "-A" "unused"
2019-07-20T14:10:43.4575369Z ------------------------------------------
2019-07-20T14:10:43.4575426Z 
2019-07-20T14:10:43.4575655Z ------------------------------------------
2019-07-20T14:10:43.4575702Z stderr:
---
2019-07-20T14:10:43.6232010Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:43.6232317Z +
2019-07-20T14:10:43.6232473Z 
2019-07-20T14:10:43.6232656Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:43.6232854Z Actual stderr saved to /tmp/compiletest8i5dKZ/calloc.stderr
2019-07-20T14:10:43.6233031Z To update references, run this command from build directory:
2019-07-20T14:10:43.6233510Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'calloc.rs'
2019-07-20T14:10:43.6233945Z error: 1 errors occurred comparing output.
2019-07-20T14:10:43.6234131Z status: exit code: 1
2019-07-20T14:10:43.6234131Z status: exit code: 1
2019-07-20T14:10:43.6235063Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/calloc.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/calloc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/calloc.stage-id.aux" "-A" "unused"
2019-07-20T14:10:43.6235699Z ------------------------------------------
2019-07-20T14:10:43.6235875Z 
2019-07-20T14:10:43.6236267Z ------------------------------------------
2019-07-20T14:10:43.6236466Z stderr:
---
2019-07-20T14:10:43.6283787Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:43.6284060Z +
2019-07-20T14:10:43.6284218Z 
2019-07-20T14:10:43.6284547Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:43.6284853Z Actual stderr saved to /tmp/compiletest8i5dKZ/calls.stderr
2019-07-20T14:10:43.6285031Z To update references, run this command from build directory:
2019-07-20T14:10:43.6285483Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'calls.rs'
2019-07-20T14:10:43.6285842Z error: 1 errors occurred comparing output.
2019-07-20T14:10:43.6286013Z status: exit code: 1
2019-07-20T14:10:43.6286013Z status: exit code: 1
2019-07-20T14:10:43.6286746Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/calls.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/calls.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/calls.stage-id.aux" "-A" "unused"
2019-07-20T14:10:43.6287384Z ------------------------------------------
2019-07-20T14:10:43.6287578Z 
2019-07-20T14:10:43.6287950Z ------------------------------------------
2019-07-20T14:10:43.6288143Z stderr:
---
2019-07-20T14:10:43.7711796Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:43.7711865Z +
2019-07-20T14:10:43.7711895Z 
2019-07-20T14:10:43.7711969Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:43.7712021Z Actual stderr saved to /tmp/compiletest8i5dKZ/cast_fn_ptr.stderr
2019-07-20T14:10:43.7712072Z To update references, run this command from build directory:
2019-07-20T14:10:43.7712395Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'cast_fn_ptr.rs'
2019-07-20T14:10:43.7712483Z error: 1 errors occurred comparing output.
2019-07-20T14:10:43.7712529Z status: exit code: 1
2019-07-20T14:10:43.7712529Z status: exit code: 1
2019-07-20T14:10:43.7713245Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast_fn_ptr.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/cast_fn_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/cast_fn_ptr.stage-id.aux" "-A" "unused"
2019-07-20T14:10:43.7713604Z ------------------------------------------
2019-07-20T14:10:43.7713643Z 
2019-07-20T14:10:43.7713884Z ------------------------------------------
2019-07-20T14:10:43.7713956Z stderr:
---
2019-07-20T14:10:43.7930920Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:43.7930975Z +
2019-07-20T14:10:43.7931006Z 
2019-07-20T14:10:43.7931076Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:43.7931875Z Actual stderr saved to /tmp/compiletest8i5dKZ/cast-rfc0401-vtable-kinds.stderr
2019-07-20T14:10:43.7931944Z To update references, run this command from build directory:
2019-07-20T14:10:43.7932274Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'cast-rfc0401-vtable-kinds.rs'
2019-07-20T14:10:43.7932386Z error: 1 errors occurred comparing output.
2019-07-20T14:10:43.7932453Z status: exit code: 1
2019-07-20T14:10:43.7932453Z status: exit code: 1
2019-07-20T14:10:43.7933216Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast-rfc0401-vtable-kinds.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/cast-rfc0401-vtable-kinds.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/cast-rfc0401-vtable-kinds.stage-id.aux" "-A" "unused"
2019-07-20T14:10:43.7933578Z ------------------------------------------
2019-07-20T14:10:43.7933616Z 
2019-07-20T14:10:43.7933858Z ------------------------------------------
2019-07-20T14:10:43.7933930Z stderr:
---
2019-07-20T14:10:43.9220052Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:43.9220111Z +
2019-07-20T14:10:43.9220144Z 
2019-07-20T14:10:43.9220197Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:43.9220275Z Actual stderr saved to /tmp/compiletest8i5dKZ/cast_fn_ptr_unsafe.stderr
2019-07-20T14:10:43.9220334Z To update references, run this command from build directory:
2019-07-20T14:10:43.9220689Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'cast_fn_ptr_unsafe.rs'
2019-07-20T14:10:43.9220801Z error: 1 errors occurred comparing output.
2019-07-20T14:10:43.9221311Z status: exit code: 1
2019-07-20T14:10:43.9221311Z status: exit code: 1
2019-07-20T14:10:43.9223079Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/cast_fn_ptr_unsafe.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/cast_fn_ptr_unsafe.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/cast_fn_ptr_unsafe.stage-id.aux" "-A" "unused"
2019-07-20T14:10:43.9224383Z ------------------------------------------
2019-07-20T14:10:43.9224534Z 
2019-07-20T14:10:43.9224796Z ------------------------------------------
2019-07-20T14:10:43.9224863Z stderr:
---
2019-07-20T14:10:43.9241680Z -1
2019-07-20T14:10:43.9241879Z -
2019-07-20T14:10:43.9241910Z 
2019-07-20T14:10:43.9241979Z The actual stdout differed from the expected stdout.
2019-07-20T14:10:43.9242037Z Actual stdout saved to /tmp/compiletest8i5dKZ/catch.stdout
2019-07-20T14:10:43.9242170Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-20T14:10:43.9242450Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-20T14:10:43.9242504Z      |
2019-07-20T14:10:43.9243251Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-20T14:10:43.9313837Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:43.9313911Z +
2019-07-20T14:10:43.9313941Z 
2019-07-20T14:10:43.9313993Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:43.9314051Z Actual stderr saved to /tmp/compiletest8i5dKZ/catch.stderr
2019-07-20T14:10:43.9314126Z To update references, run this command from build directory:
2019-07-20T14:10:43.9314456Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'catch.rs'
2019-07-20T14:10:43.9314565Z error: 2 errors occurred comparing output.
2019-07-20T14:10:43.9314616Z status: exit code: 1
2019-07-20T14:10:43.9314616Z status: exit code: 1
2019-07-20T14:10:43.9315550Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/catch.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/catch.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/catch.stage-id.aux" "-A" "unused"
2019-07-20T14:10:43.9316015Z ------------------------------------------
2019-07-20T14:10:43.9316056Z 
2019-07-20T14:10:43.9316346Z ------------------------------------------
2019-07-20T14:10:43.9316398Z stderr:
---
2019-07-20T14:10:44.0656596Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:44.0656638Z +
2019-07-20T14:10:44.0656663Z 
2019-07-20T14:10:44.0656721Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:44.0656948Z Actual stderr saved to /tmp/compiletest8i5dKZ/closure-drop.stderr
2019-07-20T14:10:44.0656995Z To update references, run this command from build directory:
2019-07-20T14:10:44.0657240Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'closure-drop.rs'
2019-07-20T14:10:44.0657312Z error: 1 errors occurred comparing output.
2019-07-20T14:10:44.0657380Z status: exit code: 1
2019-07-20T14:10:44.0657380Z status: exit code: 1
2019-07-20T14:10:44.0658097Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closure-drop.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/closure-drop.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/closure-drop.stage-id.aux" "-A" "unused"
2019-07-20T14:10:44.0658446Z ------------------------------------------
2019-07-20T14:10:44.0658478Z 
2019-07-20T14:10:44.0658689Z ------------------------------------------
2019-07-20T14:10:44.0658747Z stderr:
---
2019-07-20T14:10:44.0928221Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:44.0928292Z +
2019-07-20T14:10:44.0928328Z 
2019-07-20T14:10:44.0928384Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:44.0928465Z Actual stderr saved to /tmp/compiletest8i5dKZ/char.stderr
2019-07-20T14:10:44.0928578Z To update references, run this command from build directory:
2019-07-20T14:10:44.0928938Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'char.rs'
2019-07-20T14:10:44.0929078Z error: 1 errors occurred comparing output.
2019-07-20T14:10:44.0929283Z status: exit code: 1
2019-07-20T14:10:44.0929283Z status: exit code: 1
2019-07-20T14:10:44.0930108Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/char.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/char.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/char.stage-id.aux" "-A" "unused"
2019-07-20T14:10:44.0930533Z ------------------------------------------
2019-07-20T14:10:44.0930598Z 
2019-07-20T14:10:44.0930902Z ------------------------------------------
2019-07-20T14:10:44.0930961Z stderr:
---
2019-07-20T14:10:44.2358174Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:44.2358222Z +
2019-07-20T14:10:44.2358249Z 
2019-07-20T14:10:44.2358295Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:44.2358579Z Actual stderr saved to /tmp/compiletest8i5dKZ/closure-field-ty.stderr
2019-07-20T14:10:44.2358730Z To update references, run this command from build directory:
2019-07-20T14:10:44.2359046Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'closure-field-ty.rs'
2019-07-20T14:10:44.2359148Z error: 1 errors occurred comparing output.
2019-07-20T14:10:44.2359194Z status: exit code: 1
2019-07-20T14:10:44.2359194Z status: exit code: 1
2019-07-20T14:10:44.2359876Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closure-field-ty.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/closure-field-ty.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/closure-field-ty.stage-id.aux" "-A" "unused"
2019-07-20T14:10:44.2360213Z ------------------------------------------
2019-07-20T14:10:44.2360248Z 
2019-07-20T14:10:44.2360597Z ------------------------------------------
2019-07-20T14:10:44.2360655Z stderr:
---
2019-07-20T14:10:44.3348497Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:44.3349283Z +
2019-07-20T14:10:44.3351812Z 
2019-07-20T14:10:44.3353021Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:44.3355940Z Actual stderr saved to /tmp/compiletest8i5dKZ/closures.stderr
2019-07-20T14:10:44.3357005Z To update references, run this command from build directory:
2019-07-20T14:10:44.3358576Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'closures.rs'
2019-07-20T14:10:44.3391681Z error: 1 errors occurred comparing output.
2019-07-20T14:10:44.3391996Z status: exit code: 1
2019-07-20T14:10:44.3391996Z status: exit code: 1
2019-07-20T14:10:44.3392990Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/closures.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/closures.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/closures.stage-id.aux" "-A" "unused"
2019-07-20T14:10:44.3393659Z ------------------------------------------
2019-07-20T14:10:44.3393830Z 
2019-07-20T14:10:44.3394202Z ------------------------------------------
2019-07-20T14:10:44.3394487Z stderr:
---
2019-07-20T14:10:44.3794166Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:44.3794212Z +
2019-07-20T14:10:44.3794238Z 
2019-07-20T14:10:44.3794403Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:44.3794909Z Actual stderr saved to /tmp/compiletest8i5dKZ/const-vec-of-fns.stderr
2019-07-20T14:10:44.3794959Z To update references, run this command from build directory:
2019-07-20T14:10:44.3795218Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'const-vec-of-fns.rs'
2019-07-20T14:10:44.3795294Z error: 1 errors occurred comparing output.
2019-07-20T14:10:44.3795338Z status: exit code: 1
2019-07-20T14:10:44.3795338Z status: exit code: 1
2019-07-20T14:10:44.3795993Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/const-vec-of-fns.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/const-vec-of-fns.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/const-vec-of-fns.stage-id.aux" "-A" "unused"
2019-07-20T14:10:44.3796438Z ------------------------------------------
2019-07-20T14:10:44.3796473Z 
2019-07-20T14:10:44.3796700Z ------------------------------------------
2019-07-20T14:10:44.3796758Z stderr:
---
2019-07-20T14:10:44.5032034Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:44.5032081Z +
2019-07-20T14:10:44.5032107Z 
2019-07-20T14:10:44.5032152Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:44.5032363Z Actual stderr saved to /tmp/compiletest8i5dKZ/constants.stderr
2019-07-20T14:10:44.5032423Z To update references, run this command from build directory:
2019-07-20T14:10:44.5032714Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'constants.rs'
2019-07-20T14:10:44.5032809Z error: 1 errors occurred comparing output.
2019-07-20T14:10:44.5032853Z status: exit code: 1
2019-07-20T14:10:44.5032853Z status: exit code: 1
2019-07-20T14:10:44.5033481Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/constants.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/constants.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/constants.stage-id.aux" "-A" "unused"
2019-07-20T14:10:44.5033795Z ------------------------------------------
2019-07-20T14:10:44.5033941Z 
2019-07-20T14:10:44.5034199Z ------------------------------------------
2019-07-20T14:10:44.5034245Z stderr:
---
2019-07-20T14:10:44.6357737Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:44.6357777Z +
2019-07-20T14:10:44.6357816Z 
2019-07-20T14:10:44.6357942Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:44.6357992Z Actual stderr saved to /tmp/compiletest8i5dKZ/drop_empty_slice.stderr
2019-07-20T14:10:44.6358050Z To update references, run this command from build directory:
2019-07-20T14:10:44.6358309Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'drop_empty_slice.rs'
2019-07-20T14:10:44.6358386Z error: 1 errors occurred comparing output.
2019-07-20T14:10:44.6358444Z status: exit code: 1
2019-07-20T14:10:44.6358444Z status: exit code: 1
2019-07-20T14:10:44.6359037Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/drop_empty_slice.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/drop_empty_slice.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/drop_empty_slice.stage-id.aux" "-A" "unused"
2019-07-20T14:10:44.6359456Z ------------------------------------------
2019-07-20T14:10:44.6359488Z 
2019-07-20T14:10:44.6359717Z ------------------------------------------
2019-07-20T14:10:44.6359759Z stderr:
---
2019-07-20T14:10:44.6853858Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:44.6854035Z +
2019-07-20T14:10:44.6854081Z 
2019-07-20T14:10:44.6854136Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:44.6854551Z Actual stderr saved to /tmp/compiletest8i5dKZ/deriving-associated-types.stderr
2019-07-20T14:10:44.6854624Z To update references, run this command from build directory:
2019-07-20T14:10:44.6855026Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'deriving-associated-types.rs'
2019-07-20T14:10:44.6855152Z error: 1 errors occurred comparing output.
2019-07-20T14:10:44.6855207Z status: exit code: 1
2019-07-20T14:10:44.6855207Z status: exit code: 1
2019-07-20T14:10:44.6856053Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/deriving-associated-types.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/deriving-associated-types.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/deriving-associated-types.stage-id.aux" "-A" "unused"
2019-07-20T14:10:44.6856613Z ------------------------------------------
2019-07-20T14:10:44.6856661Z 
2019-07-20T14:10:44.6856961Z ------------------------------------------
2019-07-20T14:10:44.6857021Z stderr:
---
2019-07-20T14:10:44.8135198Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:44.8135263Z +
2019-07-20T14:10:44.8135296Z 
2019-07-20T14:10:44.8135352Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:44.8135716Z Actual stderr saved to /tmp/compiletest8i5dKZ/dst-irrefutable-bind.stderr
2019-07-20T14:10:44.8135786Z To update references, run this command from build directory:
2019-07-20T14:10:44.8136133Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'dst-irrefutable-bind.rs'
2019-07-20T14:10:44.8136253Z error: 1 errors occurred comparing output.
2019-07-20T14:10:44.8136307Z status: exit code: 1
2019-07-20T14:10:44.8136307Z status: exit code: 1
2019-07-20T14:10:44.8137111Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-irrefutable-bind.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/dst-irrefutable-bind.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/dst-irrefutable-bind.stage-id.aux" "-A" "unused"
2019-07-20T14:10:44.8137672Z ------------------------------------------
2019-07-20T14:10:44.8137741Z 
2019-07-20T14:10:44.8138040Z ------------------------------------------
2019-07-20T14:10:44.8138097Z stderr:
---
2019-07-20T14:10:44.8796590Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:44.8796674Z +
2019-07-20T14:10:44.8796710Z 
2019-07-20T14:10:44.8796766Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:44.8797105Z Actual stderr saved to /tmp/compiletest8i5dKZ/dst-field-align.stderr
2019-07-20T14:10:44.8797196Z To update references, run this command from build directory:
2019-07-20T14:10:44.8797546Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'dst-field-align.rs'
2019-07-20T14:10:44.8797647Z error: 1 errors occurred comparing output.
2019-07-20T14:10:44.8797723Z status: exit code: 1
2019-07-20T14:10:44.8797723Z status: exit code: 1
2019-07-20T14:10:44.8798516Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-field-align.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/dst-field-align.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/dst-field-align.stage-id.aux" "-A" "unused"
2019-07-20T14:10:44.8799096Z ------------------------------------------
2019-07-20T14:10:44.8799141Z 
2019-07-20T14:10:44.8799457Z ------------------------------------------
2019-07-20T14:10:44.8799517Z stderr:
---
2019-07-20T14:10:45.0980493Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:45.0980552Z +
2019-07-20T14:10:45.0980576Z 
2019-07-20T14:10:45.0980617Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:45.0980869Z Actual stderr saved to /tmp/compiletest8i5dKZ/dst-struct-sole.stderr
2019-07-20T14:10:45.0980917Z To update references, run this command from build directory:
2019-07-20T14:10:45.0981266Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'dst-struct-sole.rs'
2019-07-20T14:10:45.0981361Z error: 1 errors occurred comparing output.
2019-07-20T14:10:45.0981858Z status: exit code: 1
2019-07-20T14:10:45.0981858Z status: exit code: 1
2019-07-20T14:10:45.0982592Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-struct-sole.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/dst-struct-sole.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/dst-struct-sole.stage-id.aux" "-A" "unused"
2019-07-20T14:10:45.0982923Z ------------------------------------------
2019-07-20T14:10:45.0982976Z 
2019-07-20T14:10:45.0983195Z ------------------------------------------
2019-07-20T14:10:45.0983240Z stderr:
---
2019-07-20T14:10:45.0995969Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:45.0996030Z +
2019-07-20T14:10:45.0996055Z 
2019-07-20T14:10:45.0996097Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:45.0996567Z Actual stderr saved to /tmp/compiletest8i5dKZ/dst-raw.stderr
2019-07-20T14:10:45.0996636Z To update references, run this command from build directory:
2019-07-20T14:10:45.0996998Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'dst-raw.rs'
2019-07-20T14:10:45.0997184Z error: 1 errors occurred comparing output.
2019-07-20T14:10:45.0997230Z status: exit code: 1
2019-07-20T14:10:45.0997230Z status: exit code: 1
2019-07-20T14:10:45.0997873Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-raw.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/dst-raw.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/dst-raw.stage-id.aux" "-A" "unused"
2019-07-20T14:10:45.0998193Z ------------------------------------------
2019-07-20T14:10:45.0998226Z 
2019-07-20T14:10:45.0998458Z ------------------------------------------
2019-07-20T14:10:45.0998502Z stderr:
---
2019-07-20T14:10:45.2384263Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:45.2384326Z +
2019-07-20T14:10:45.2384388Z 
2019-07-20T14:10:45.2384446Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:45.2384811Z Actual stderr saved to /tmp/compiletest8i5dKZ/enum-nullable-const-null-with-fields.stderr
2019-07-20T14:10:45.2384904Z To update references, run this command from build directory:
2019-07-20T14:10:45.2385444Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'enum-nullable-const-null-with-fields.rs'
2019-07-20T14:10:45.2385551Z error: 1 errors occurred comparing output.
2019-07-20T14:10:45.2385632Z status: exit code: 1
2019-07-20T14:10:45.2385632Z status: exit code: 1
2019-07-20T14:10:45.2386482Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/enum-nullable-const-null-with-fields.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/enum-nullable-const-null-with-fields.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/enum-nullable-const-null-with-fields.stage-id.aux" "-A" "unused"
2019-07-20T14:10:45.2386912Z ------------------------------------------
2019-07-20T14:10:45.2386958Z 
2019-07-20T14:10:45.2387272Z ------------------------------------------
2019-07-20T14:10:45.2387354Z stderr:
---
2019-07-20T14:10:45.3817303Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:45.3817364Z +
2019-07-20T14:10:45.3817398Z 
2019-07-20T14:10:45.3817477Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:45.3817960Z Actual stderr saved to /tmp/compiletest8i5dKZ/dst-struct.stderr
2019-07-20T14:10:45.3818029Z To update references, run this command from build directory:
2019-07-20T14:10:45.3818390Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'dst-struct.rs'
2019-07-20T14:10:45.3818491Z error: 1 errors occurred comparing output.
2019-07-20T14:10:45.3818545Z status: exit code: 1
2019-07-20T14:10:45.3818545Z status: exit code: 1
2019-07-20T14:10:45.3819324Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/dst-struct.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/dst-struct.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/dst-struct.stage-id.aux" "-A" "unused"
2019-07-20T14:10:45.3819757Z ------------------------------------------
2019-07-20T14:10:45.3819813Z 
2019-07-20T14:10:45.3820109Z ------------------------------------------
2019-07-20T14:10:45.3820189Z stderr:
---
2019-07-20T14:10:45.4135150Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:45.4135232Z +
2019-07-20T14:10:45.4135371Z 
2019-07-20T14:10:45.4135437Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:45.4135498Z Actual stderr saved to /tmp/compiletest8i5dKZ/enums.stderr
2019-07-20T14:10:45.4135578Z To update references, run this command from build directory:
2019-07-20T14:10:45.4135952Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'enums.rs'
2019-07-20T14:10:45.4136074Z error: 1 errors occurred comparing output.
2019-07-20T14:10:45.4136129Z status: exit code: 1
2019-07-20T14:10:45.4136129Z status: exit code: 1
2019-07-20T14:10:45.4136877Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/enums.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/enums.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/enums.stage-id.aux" "-A" "unused"
2019-07-20T14:10:45.4137319Z ------------------------------------------
2019-07-20T14:10:45.4137365Z 
2019-07-20T14:10:45.4137684Z ------------------------------------------
2019-07-20T14:10:45.4137745Z stderr:
---
2019-07-20T14:10:45.5385430Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:45.5385828Z +
2019-07-20T14:10:45.5385988Z 
2019-07-20T14:10:45.5386164Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:45.5386355Z Actual stderr saved to /tmp/compiletest8i5dKZ/exit.stderr
2019-07-20T14:10:45.5386531Z To update references, run this command from build directory:
2019-07-20T14:10:45.5387032Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'exit.rs'
2019-07-20T14:10:45.5387418Z error: 1 errors occurred comparing output.
2019-07-20T14:10:45.5387584Z status: exit code: 1
2019-07-20T14:10:45.5387584Z status: exit code: 1
2019-07-20T14:10:45.5388596Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/exit.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/exit.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/exit.stage-id.aux" "-A" "unused"
2019-07-20T14:10:45.5389329Z ------------------------------------------
2019-07-20T14:10:45.5389539Z 
2019-07-20T14:10:45.5390251Z ------------------------------------------
2019-07-20T14:10:45.5390616Z stderr:
---
2019-07-20T14:10:45.5722237Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:45.5722292Z +
2019-07-20T14:10:45.5722341Z 
2019-07-20T14:10:45.5722391Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:45.5722442Z Actual stderr saved to /tmp/compiletest8i5dKZ/env.stderr
2019-07-20T14:10:45.5722511Z To update references, run this command from build directory:
2019-07-20T14:10:45.5722791Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'env.rs'
2019-07-20T14:10:45.5722875Z error: 1 errors occurred comparing output.
2019-07-20T14:10:45.5722943Z status: exit code: 1
2019-07-20T14:10:45.5722943Z status: exit code: 1
2019-07-20T14:10:45.5723594Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/env.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/env.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/env.stage-id.aux" "-A" "unused"
2019-07-20T14:10:45.5723958Z ------------------------------------------
2019-07-20T14:10:45.5723997Z 
2019-07-20T14:10:45.5724239Z ------------------------------------------
2019-07-20T14:10:45.5724311Z stderr:
---
2019-07-20T14:10:45.6839536Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:45.6839981Z +
2019-07-20T14:10:45.6840180Z 
2019-07-20T14:10:45.6840413Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:45.6840635Z Actual stderr saved to /tmp/compiletest8i5dKZ/extern_types.stderr
2019-07-20T14:10:45.6840848Z To update references, run this command from build directory:
2019-07-20T14:10:45.6841413Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'extern_types.rs'
2019-07-20T14:10:45.6842925Z error: 1 errors occurred comparing output.
2019-07-20T14:10:45.6843765Z status: exit code: 1
2019-07-20T14:10:45.6843765Z status: exit code: 1
2019-07-20T14:10:45.6844771Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/extern_types.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/extern_types.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/extern_types.stage-id.aux" "-A" "unused"
2019-07-20T14:10:45.6845664Z ------------------------------------------
2019-07-20T14:10:45.6845822Z 
2019-07-20T14:10:45.6846183Z ------------------------------------------
2019-07-20T14:10:45.6846354Z stderr:
---
2019-07-20T14:10:45.7533557Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:45.7533750Z +
2019-07-20T14:10:45.7533873Z 
2019-07-20T14:10:45.7534037Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:45.7534188Z Actual stderr saved to /tmp/compiletest8i5dKZ/float_fast_math.stderr
2019-07-20T14:10:45.7534337Z To update references, run this command from build directory:
2019-07-20T14:10:45.7534762Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'float_fast_math.rs'
2019-07-20T14:10:45.7535173Z error: 1 errors occurred comparing output.
2019-07-20T14:10:45.7535314Z status: exit code: 1
2019-07-20T14:10:45.7535314Z status: exit code: 1
2019-07-20T14:10:45.7536016Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/float_fast_math.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/float_fast_math.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/float_fast_math.stage-id.aux" "-A" "unused"
2019-07-20T14:10:45.7536566Z ------------------------------------------
2019-07-20T14:10:45.7536716Z 
2019-07-20T14:10:45.7537073Z ------------------------------------------
2019-07-20T14:10:45.7537241Z stderr:
---
2019-07-20T14:10:45.9210173Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:45.9210217Z +
2019-07-20T14:10:45.9210244Z 
2019-07-20T14:10:45.9210289Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:45.9210351Z Actual stderr saved to /tmp/compiletest8i5dKZ/floats.stderr
2019-07-20T14:10:45.9210400Z To update references, run this command from build directory:
2019-07-20T14:10:45.9210659Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'floats.rs'
2019-07-20T14:10:45.9210751Z error: 1 errors occurred comparing output.
2019-07-20T14:10:45.9210794Z status: exit code: 1
2019-07-20T14:10:45.9210794Z status: exit code: 1
2019-07-20T14:10:45.9211425Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/floats.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/floats.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/floats.stage-id.aux" "-A" "unused"
2019-07-20T14:10:45.9211743Z ------------------------------------------
2019-07-20T14:10:45.9211777Z 
2019-07-20T14:10:45.9212532Z ------------------------------------------
2019-07-20T14:10:45.9212589Z stderr:
---
2019-07-20T14:10:45.9225613Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:45.9225659Z +
2019-07-20T14:10:45.9225684Z 
2019-07-20T14:10:45.9225727Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:45.9225987Z Actual stderr saved to /tmp/compiletest8i5dKZ/foreign-fn-linkname.stderr
2019-07-20T14:10:45.9226038Z To update references, run this command from build directory:
2019-07-20T14:10:45.9226289Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'foreign-fn-linkname.rs'
2019-07-20T14:10:45.9226380Z error: 1 errors occurred comparing output.
2019-07-20T14:10:45.9226422Z status: exit code: 1
2019-07-20T14:10:45.9226422Z status: exit code: 1
2019-07-20T14:10:45.9227075Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/foreign-fn-linkname.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/foreign-fn-linkname.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/foreign-fn-linkname.stage-id.aux" "-A" "unused"
2019-07-20T14:10:45.9227374Z ------------------------------------------
2019-07-20T14:10:45.9227425Z 
2019-07-20T14:10:45.9227636Z ------------------------------------------
2019-07-20T14:10:45.9227678Z stderr:
---
2019-07-20T14:10:46.0572761Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:46.0572817Z +
2019-07-20T14:10:46.0572846Z 
2019-07-20T14:10:46.0572947Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:46.0572999Z Actual stderr saved to /tmp/compiletest8i5dKZ/from_utf8.stderr
2019-07-20T14:10:46.0573695Z thread '[ui] run-pass/from_utf8.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-20T14:10:46.0573695Z thread '[ui] run-pass/from_utf8.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-20T14:10:46.0574238Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'from_utf8.rs'
2019-07-20T14:10:46.0574367Z error: 1 errors occurred comparing output.
2019-07-20T14:10:46.0574412Z status: exit code: 1
2019-07-20T14:10:46.0574412Z status: exit code: 1
2019-07-20T14:10:46.0575079Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/from_utf8.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/from_utf8.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/from_utf8.stage-id.aux" "-A" "unused"
2019-07-20T14:10:46.0575810Z ------------------------------------------
2019-07-20T14:10:46.0575856Z 
2019-07-20T14:10:46.0576146Z ------------------------------------------
2019-07-20T14:10:46.0576350Z stderr:
---
2019-07-20T14:10:46.0819967Z -hello00000
2019-07-20T14:10:46.0820150Z -
2019-07-20T14:10:46.0820394Z 
2019-07-20T14:10:46.0820454Z The actual stdout differed from the expected stdout.
2019-07-20T14:10:46.0820527Z Actual stdout saved to /tmp/compiletest8i5dKZ/format.stdout
2019-07-20T14:10:46.0820659Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-20T14:10:46.0820958Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-20T14:10:46.0821010Z      |
2019-07-20T14:10:46.0821071Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-20T14:10:46.0825281Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:46.0825329Z +
2019-07-20T14:10:46.0825465Z 
2019-07-20T14:10:46.0825508Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:46.0825695Z Actual stderr saved to /tmp/compiletest8i5dKZ/format.stderr
2019-07-20T14:10:46.0825741Z To update references, run this command from build directory:
2019-07-20T14:10:46.0825980Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'format.rs'
2019-07-20T14:10:46.0826069Z error: 2 errors occurred comparing output.
2019-07-20T14:10:46.0826110Z status: exit code: 1
2019-07-20T14:10:46.0826110Z status: exit code: 1
2019-07-20T14:10:46.0826713Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/format.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/format.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/format.stage-id.aux" "-A" "unused"
2019-07-20T14:10:46.0827003Z ------------------------------------------
2019-07-20T14:10:46.0827051Z 
2019-07-20T14:10:46.0827255Z ------------------------------------------
2019-07-20T14:10:46.0827296Z stderr:
---
2019-07-20T14:10:46.2851362Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:46.2851445Z +
2019-07-20T14:10:46.2851480Z 
2019-07-20T14:10:46.2851535Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:46.2851597Z Actual stderr saved to /tmp/compiletest8i5dKZ/function_pointers.stderr
2019-07-20T14:10:46.2851679Z To update references, run this command from build directory:
2019-07-20T14:10:46.2852957Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'function_pointers.rs'
2019-07-20T14:10:46.2853073Z error: 1 errors occurred comparing output.
2019-07-20T14:10:46.2853153Z status: exit code: 1
2019-07-20T14:10:46.2853153Z status: exit code: 1
2019-07-20T14:10:46.2853990Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/function_pointers.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/function_pointers.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/function_pointers.stage-id.aux" "-A" "unused"
2019-07-20T14:10:46.2854415Z ------------------------------------------
2019-07-20T14:10:46.2854460Z 
2019-07-20T14:10:46.2854753Z ------------------------------------------
2019-07-20T14:10:46.2854811Z stderr:
---
2019-07-20T14:10:46.2968166Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:46.2968220Z +
2019-07-20T14:10:46.2968250Z 
2019-07-20T14:10:46.2968315Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:46.2968368Z Actual stderr saved to /tmp/compiletest8i5dKZ/generator.stderr
2019-07-20T14:10:46.2968420Z To update references, run this command from build directory:
2019-07-20T14:10:46.2968735Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'generator.rs'
2019-07-20T14:10:46.2968822Z error: 1 errors occurred comparing output.
2019-07-20T14:10:46.2968868Z status: exit code: 1
2019-07-20T14:10:46.2968868Z status: exit code: 1
2019-07-20T14:10:46.2969552Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/generator.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/generator.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/generator.stage-id.aux" "-A" "unused"
2019-07-20T14:10:46.2969920Z ------------------------------------------
2019-07-20T14:10:46.2969958Z 
2019-07-20T14:10:46.2970214Z ------------------------------------------
2019-07-20T14:10:46.2970262Z stderr:
---
2019-07-20T14:10:46.4419961Z -Hello, world!
2019-07-20T14:10:46.4420194Z -
2019-07-20T14:10:46.4425155Z 
2019-07-20T14:10:46.4425257Z The actual stdout differed from the expected stdout.
2019-07-20T14:10:46.4429958Z Actual stdout saved to /tmp/compiletest8i5dKZ/hello.stdout
2019-07-20T14:10:46.4435993Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-20T14:10:46.4436457Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-20T14:10:46.4436541Z      |
2019-07-20T14:10:46.4436588Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-20T14:10:46.4456095Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:46.4456157Z +
2019-07-20T14:10:46.4456216Z 
2019-07-20T14:10:46.4456261Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:46.4456310Z Actual stderr saved to /tmp/compiletest8i5dKZ/hello.stderr
2019-07-20T14:10:46.4456375Z To update references, run this command from build directory:
2019-07-20T14:10:46.4456640Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'hello.rs'
2019-07-20T14:10:46.4456720Z error: 2 errors occurred comparing output.
2019-07-20T14:10:46.4456783Z status: exit code: 1
2019-07-20T14:10:46.4456783Z status: exit code: 1
2019-07-20T14:10:46.4457385Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/hello.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/hello.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/hello.stage-id.aux" "-A" "unused"
2019-07-20T14:10:46.4457868Z ------------------------------------------
2019-07-20T14:10:46.4457905Z 
2019-07-20T14:10:46.4458152Z ------------------------------------------
2019-07-20T14:10:46.4458200Z stderr:
---
2019-07-20T14:10:46.4609407Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:46.4609459Z +
2019-07-20T14:10:46.4609484Z 
2019-07-20T14:10:46.4609527Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:46.4609593Z Actual stderr saved to /tmp/compiletest8i5dKZ/heap.stderr
2019-07-20T14:10:46.4609640Z To update references, run this command from build directory:
2019-07-20T14:10:46.4610003Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'heap.rs'
2019-07-20T14:10:46.4610092Z error: 1 errors occurred comparing output.
2019-07-20T14:10:46.4610133Z status: exit code: 1
2019-07-20T14:10:46.4610133Z status: exit code: 1
2019-07-20T14:10:46.4610800Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/heap.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/heap.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/heap.stage-id.aux" "-A" "unused"
2019-07-20T14:10:46.4611120Z ------------------------------------------
2019-07-20T14:10:46.4611168Z 
2019-07-20T14:10:46.4611371Z ------------------------------------------
2019-07-20T14:10:46.4611411Z stderr:
---
2019-07-20T14:10:47.0929517Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:47.0929584Z +
2019-07-20T14:10:47.0929614Z 
2019-07-20T14:10:47.0929659Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:47.0929927Z Actual stderr saved to /tmp/compiletest8i5dKZ/integer-ops.stderr
2019-07-20T14:10:47.0929998Z To update references, run this command from build directory:
2019-07-20T14:10:47.0930267Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'integer-ops.rs'
2019-07-20T14:10:47.0930363Z error: 1 errors occurred comparing output.
2019-07-20T14:10:47.0930407Z status: exit code: 1
2019-07-20T14:10:47.0930407Z status: exit code: 1
2019-07-20T14:10:47.0931241Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/integer-ops.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/integer-ops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/integer-ops.stage-id.aux" "-A" "unused"
2019-07-20T14:10:47.0931789Z ------------------------------------------
2019-07-20T14:10:47.0931823Z 
2019-07-20T14:10:47.0932527Z ------------------------------------------
2019-07-20T14:10:47.0932584Z stderr:
---
2019-07-20T14:10:47.6181938Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:47.6181990Z +
2019-07-20T14:10:47.6182020Z 
2019-07-20T14:10:47.6182705Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:47.6183101Z Actual stderr saved to /tmp/compiletest8i5dKZ/intrinsics-math.stderr
2019-07-20T14:10:47.6183164Z To update references, run this command from build directory:
2019-07-20T14:10:47.6183450Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'intrinsics-math.rs'
2019-07-20T14:10:47.6183565Z error: 1 errors occurred comparing output.
2019-07-20T14:10:47.6183612Z status: exit code: 1
2019-07-20T14:10:47.6183612Z status: exit code: 1
2019-07-20T14:10:47.6184440Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics-math.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/intrinsics-math.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/intrinsics-math.stage-id.aux" "-A" "unused"
2019-07-20T14:10:47.6184866Z ------------------------------------------
2019-07-20T14:10:47.6184907Z 
2019-07-20T14:10:47.6185154Z ------------------------------------------
2019-07-20T14:10:47.6185204Z stderr:
---
2019-07-20T14:10:47.8012933Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:47.8012996Z +
2019-07-20T14:10:47.8013023Z 
2019-07-20T14:10:47.8013069Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:47.8013135Z Actual stderr saved to /tmp/compiletest8i5dKZ/intrinsics.stderr
2019-07-20T14:10:47.8013186Z To update references, run this command from build directory:
2019-07-20T14:10:47.8013523Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'intrinsics.rs'
2019-07-20T14:10:47.8013618Z error: 1 errors occurred comparing output.
2019-07-20T14:10:47.8013662Z status: exit code: 1
2019-07-20T14:10:47.8013662Z status: exit code: 1
2019-07-20T14:10:47.8014456Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/intrinsics.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/intrinsics.stage-id.aux" "-A" "unused"
2019-07-20T14:10:47.8014843Z ------------------------------------------
2019-07-20T14:10:47.8014893Z 
2019-07-20T14:10:47.8015111Z ------------------------------------------
2019-07-20T14:10:47.8015155Z stderr:
---
2019-07-20T14:10:48.0174804Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:48.0174871Z +
2019-07-20T14:10:48.0174907Z 
2019-07-20T14:10:48.0174988Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:48.0175050Z Actual stderr saved to /tmp/compiletest8i5dKZ/ints.stderr
2019-07-20T14:10:48.0175111Z To update references, run this command from build directory:
2019-07-20T14:10:48.0175479Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'ints.rs'
2019-07-20T14:10:48.0175582Z error: 1 errors occurred comparing output.
2019-07-20T14:10:48.0175637Z status: exit code: 1
2019-07-20T14:10:48.0175637Z status: exit code: 1
2019-07-20T14:10:48.0176701Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ints.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/ints.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/ints.stage-id.aux" "-A" "unused"
2019-07-20T14:10:48.0177207Z ------------------------------------------
2019-07-20T14:10:48.0177254Z 
2019-07-20T14:10:48.0177549Z ------------------------------------------
2019-07-20T14:10:48.0177607Z stderr:
---
2019-07-20T14:10:48.1616316Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:48.1616360Z +
2019-07-20T14:10:48.1616401Z 
2019-07-20T14:10:48.1616444Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:48.1616673Z Actual stderr saved to /tmp/compiletest8i5dKZ/issue-15063.stderr
2019-07-20T14:10:48.1616722Z To update references, run this command from build directory:
2019-07-20T14:10:48.1616974Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'issue-15063.rs'
2019-07-20T14:10:48.1617197Z error: 1 errors occurred comparing output.
2019-07-20T14:10:48.1617268Z status: exit code: 1
2019-07-20T14:10:48.1617268Z status: exit code: 1
2019-07-20T14:10:48.1617898Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15063.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/issue-15063.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/issue-15063.stage-id.aux" "-A" "unused"
2019-07-20T14:10:48.1618194Z ------------------------------------------
2019-07-20T14:10:48.1618224Z 
2019-07-20T14:10:48.1618424Z ------------------------------------------
2019-07-20T14:10:48.1618481Z stderr:
---
2019-07-20T14:10:48.3552216Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:48.3552520Z +
2019-07-20T14:10:48.3553374Z 
2019-07-20T14:10:48.3553624Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:48.3554189Z Actual stderr saved to /tmp/compiletest8i5dKZ/issue-15080.stderr
2019-07-20T14:10:48.3554523Z To update references, run this command from build directory:
2019-07-20T14:10:48.3555034Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'issue-15080.rs'
2019-07-20T14:10:48.3555564Z error: 1 errors occurred comparing output.
2019-07-20T14:10:48.3555807Z status: exit code: 1
2019-07-20T14:10:48.3555807Z status: exit code: 1
2019-07-20T14:10:48.3557019Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15080.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/issue-15080.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/issue-15080.stage-id.aux" "-A" "unused"
2019-07-20T14:10:48.3557826Z ------------------------------------------
2019-07-20T14:10:48.3558077Z 
2019-07-20T14:10:48.3558501Z ------------------------------------------
2019-07-20T14:10:48.3558773Z stderr:
---
2019-07-20T14:10:48.5479191Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:48.5479277Z +
2019-07-20T14:10:48.5479315Z 
2019-07-20T14:10:48.5479372Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:48.5479735Z Actual stderr saved to /tmp/compiletest8i5dKZ/issue-15523-big.stderr
2019-07-20T14:10:48.5479807Z To update references, run this command from build directory:
2019-07-20T14:10:48.5480157Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'issue-15523-big.rs'
2019-07-20T14:10:48.5480483Z error: 1 errors occurred comparing output.
2019-07-20T14:10:48.5480560Z status: exit code: 1
2019-07-20T14:10:48.5480560Z status: exit code: 1
2019-07-20T14:10:48.5481406Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-15523-big.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/issue-15523-big.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/issue-15523-big.stage-id.aux" "-A" "unused"
2019-07-20T14:10:48.5481836Z ------------------------------------------
2019-07-20T14:10:48.5481884Z 
2019-07-20T14:10:48.5482206Z ------------------------------------------
2019-07-20T14:10:48.5482267Z stderr:
---
2019-07-20T14:10:48.7096530Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:48.7096579Z +
2019-07-20T14:10:48.7096606Z 
2019-07-20T14:10:48.7096652Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:48.7097146Z Actual stderr saved to /tmp/compiletest8i5dKZ/issue-17877.stderr
2019-07-20T14:10:48.7097212Z To update references, run this command from build directory:
2019-07-20T14:10:48.7097509Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'issue-17877.rs'
2019-07-20T14:10:48.7097653Z error: 1 errors occurred comparing output.
2019-07-20T14:10:48.7097698Z status: exit code: 1
2019-07-20T14:10:48.7097698Z status: exit code: 1
2019-07-20T14:10:48.7098499Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-17877.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/issue-17877.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/issue-17877.stage-id.aux" "-A" "unused"
2019-07-20T14:10:48.7098796Z ------------------------------------------
2019-07-20T14:10:48.7098982Z 
2019-07-20T14:10:48.7099359Z ------------------------------------------
2019-07-20T14:10:48.7099508Z stderr:
---
2019-07-20T14:10:48.8695594Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:48.8695699Z +
2019-07-20T14:10:48.8695727Z 
2019-07-20T14:10:48.8695772Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:48.8696079Z Actual stderr saved to /tmp/compiletest8i5dKZ/issue-20575.stderr
2019-07-20T14:10:48.8696381Z To update references, run this command from build directory:
2019-07-20T14:10:48.8696685Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'issue-20575.rs'
2019-07-20T14:10:48.8696824Z error: 1 errors occurred comparing output.
2019-07-20T14:10:48.8696867Z status: exit code: 1
2019-07-20T14:10:48.8696867Z status: exit code: 1
2019-07-20T14:10:48.8751302Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-20575.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/issue-20575.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/issue-20575.stage-id.aux" "-A" "unused"
2019-07-20T14:10:48.8751876Z ------------------------------------------
2019-07-20T14:10:48.8751915Z 
2019-07-20T14:10:48.8752252Z ------------------------------------------
2019-07-20T14:10:48.8752472Z stderr:
---
2019-07-20T14:10:49.0611140Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:49.0611195Z +
2019-07-20T14:10:49.0611227Z 
2019-07-20T14:10:49.0611280Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:49.0611777Z Actual stderr saved to /tmp/compiletest8i5dKZ/issue-23261.stderr
2019-07-20T14:10:49.0611852Z To update references, run this command from build directory:
2019-07-20T14:10:49.0612208Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'issue-23261.rs'
2019-07-20T14:10:49.0612322Z error: 1 errors occurred comparing output.
2019-07-20T14:10:49.0612374Z status: exit code: 1
2019-07-20T14:10:49.0612374Z status: exit code: 1
2019-07-20T14:10:49.0613565Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-23261.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/issue-23261.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/issue-23261.stage-id.aux" "-A" "unused"
2019-07-20T14:10:49.0613948Z ------------------------------------------
2019-07-20T14:10:49.0614133Z 
2019-07-20T14:10:49.0614431Z ------------------------------------------
2019-07-20T14:10:49.0614482Z stderr:
---
2019-07-20T14:10:49.2795926Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:49.2797504Z +
2019-07-20T14:10:49.2797800Z 
2019-07-20T14:10:49.2798024Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:49.2798719Z Actual stderr saved to /tmp/compiletest8i5dKZ/issue-26709.stderr
2019-07-20T14:10:49.2799028Z To update references, run this command from build directory:
2019-07-20T14:10:49.2799593Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'issue-26709.rs'
2019-07-20T14:10:49.2800054Z error: 1 errors occurred comparing output.
2019-07-20T14:10:49.2800252Z status: exit code: 1
2019-07-20T14:10:49.2800252Z status: exit code: 1
2019-07-20T14:10:49.2801244Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-26709.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/issue-26709.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/issue-26709.stage-id.aux" "-A" "unused"
2019-07-20T14:10:49.2802311Z ------------------------------------------
2019-07-20T14:10:49.2802576Z 
2019-07-20T14:10:49.2803636Z ------------------------------------------
2019-07-20T14:10:49.2803935Z stderr:
---
2019-07-20T14:10:49.4616796Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:49.4616872Z +
2019-07-20T14:10:49.4616904Z 
2019-07-20T14:10:49.4616957Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:49.4617265Z Actual stderr saved to /tmp/compiletest8i5dKZ/issue-27901.stderr
2019-07-20T14:10:49.4617345Z To update references, run this command from build directory:
2019-07-20T14:10:49.4617662Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'issue-27901.rs'
2019-07-20T14:10:49.4617772Z error: 1 errors occurred comparing output.
2019-07-20T14:10:49.4617823Z status: exit code: 1
2019-07-20T14:10:49.4617823Z status: exit code: 1
2019-07-20T14:10:49.4618580Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-27901.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/issue-27901.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/issue-27901.stage-id.aux" "-A" "unused"
2019-07-20T14:10:49.4619111Z ------------------------------------------
2019-07-20T14:10:49.4619152Z 
2019-07-20T14:10:49.4619442Z ------------------------------------------
2019-07-20T14:10:49.4619494Z stderr:
---
2019-07-20T14:10:49.5562681Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:49.5562735Z +
2019-07-20T14:10:49.5562767Z 
2019-07-20T14:10:49.5562818Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:49.5563543Z Actual stderr saved to /tmp/compiletest8i5dKZ/intrinsics-integer.stderr
2019-07-20T14:10:49.5563617Z To update references, run this command from build directory:
2019-07-20T14:10:49.5563988Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'intrinsics-integer.rs'
2019-07-20T14:10:49.5564100Z error: 1 errors occurred comparing output.
2019-07-20T14:10:49.5564151Z status: exit code: 1
2019-07-20T14:10:49.5564151Z status: exit code: 1
2019-07-20T14:10:49.5564933Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/intrinsics-integer.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/intrinsics-integer.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/intrinsics-integer.stage-id.aux" "-A" "unused"
2019-07-20T14:10:49.5565462Z ------------------------------------------
2019-07-20T14:10:49.5565500Z 
2019-07-20T14:10:49.5565750Z ------------------------------------------
2019-07-20T14:10:49.5565799Z stderr:
---
2019-07-20T14:10:49.6656029Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:49.6656082Z +
2019-07-20T14:10:49.6656112Z 
2019-07-20T14:10:49.6656181Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:49.6656456Z Actual stderr saved to /tmp/compiletest8i5dKZ/issue-29746.stderr
2019-07-20T14:10:49.6656644Z To update references, run this command from build directory:
2019-07-20T14:10:49.6656933Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'issue-29746.rs'
2019-07-20T14:10:49.6657013Z error: 1 errors occurred comparing output.
2019-07-20T14:10:49.6657074Z status: exit code: 1
2019-07-20T14:10:49.6657074Z status: exit code: 1
2019-07-20T14:10:49.6657730Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-29746.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/issue-29746.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/issue-29746.stage-id.aux" "-A" "unused"
2019-07-20T14:10:49.6658206Z ------------------------------------------
2019-07-20T14:10:49.6658244Z 
2019-07-20T14:10:49.6658473Z ------------------------------------------
2019-07-20T14:10:49.6658539Z stderr:
---
2019-07-20T14:10:49.6854470Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:49.6854891Z +
2019-07-20T14:10:49.6854926Z 
2019-07-20T14:10:49.6854972Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:49.6855344Z Actual stderr saved to /tmp/compiletest8i5dKZ/issue-30530.stderr
2019-07-20T14:10:49.6855404Z To update references, run this command from build directory:
2019-07-20T14:10:49.6855691Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'issue-30530.rs'
2019-07-20T14:10:49.6855795Z error: 1 errors occurred comparing output.
2019-07-20T14:10:49.6855963Z status: exit code: 1
2019-07-20T14:10:49.6855963Z status: exit code: 1
2019-07-20T14:10:49.6856801Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-30530.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/issue-30530.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/issue-30530.stage-id.aux" "-A" "unused"
2019-07-20T14:10:49.6857282Z ------------------------------------------
2019-07-20T14:10:49.6857335Z 
2019-07-20T14:10:49.6857568Z ------------------------------------------
2019-07-20T14:10:49.6857614Z stderr:
---
2019-07-20T14:10:49.8133535Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:49.8133685Z +
2019-07-20T14:10:49.8133721Z 
2019-07-20T14:10:49.8133771Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:49.8134151Z Actual stderr saved to /tmp/compiletest8i5dKZ/issue-33387.stderr
2019-07-20T14:10:49.8134213Z To update references, run this command from build directory:
2019-07-20T14:10:49.8134505Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'issue-33387.rs'
2019-07-20T14:10:49.8134612Z error: 1 errors occurred comparing output.
2019-07-20T14:10:49.8134661Z status: exit code: 1
2019-07-20T14:10:49.8134661Z status: exit code: 1
2019-07-20T14:10:49.8135437Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-33387.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/issue-33387.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/issue-33387.stage-id.aux" "-A" "unused"
2019-07-20T14:10:49.8135967Z ------------------------------------------
2019-07-20T14:10:49.8136005Z 
2019-07-20T14:10:49.8136254Z ------------------------------------------
2019-07-20T14:10:49.8136303Z stderr:
---
2019-07-20T14:10:49.8208431Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:49.8208501Z +
2019-07-20T14:10:49.8208533Z 
2019-07-20T14:10:49.8208584Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:49.8208900Z Actual stderr saved to /tmp/compiletest8i5dKZ/issue-31267-additional.stderr
2019-07-20T14:10:49.8208980Z To update references, run this command from build directory:
2019-07-20T14:10:49.8209305Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'issue-31267-additional.rs'
2019-07-20T14:10:49.8209412Z error: 1 errors occurred comparing output.
2019-07-20T14:10:49.8209551Z status: exit code: 1
2019-07-20T14:10:49.8209551Z status: exit code: 1
2019-07-20T14:10:49.8210388Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-31267-additional.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/issue-31267-additional.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/issue-31267-additional.stage-id.aux" "-A" "unused"
2019-07-20T14:10:49.8210782Z ------------------------------------------
2019-07-20T14:10:49.8210821Z 
2019-07-20T14:10:49.8211108Z ------------------------------------------
2019-07-20T14:10:49.8211160Z stderr:
---
2019-07-20T14:10:49.9558317Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:49.9558368Z +
2019-07-20T14:10:49.9558414Z 
2019-07-20T14:10:49.9558461Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:49.9558741Z Actual stderr saved to /tmp/compiletest8i5dKZ/issue-34571.stderr
2019-07-20T14:10:49.9558797Z To update references, run this command from build directory:
2019-07-20T14:10:49.9559133Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'issue-34571.rs'
2019-07-20T14:10:49.9559319Z error: 1 errors occurred comparing output.
2019-07-20T14:10:49.9559383Z status: exit code: 1
2019-07-20T14:10:49.9559383Z status: exit code: 1
2019-07-20T14:10:49.9560071Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-34571.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/issue-34571.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/issue-34571.stage-id.aux" "-A" "unused"
2019-07-20T14:10:49.9560426Z ------------------------------------------
2019-07-20T14:10:49.9560463Z 
2019-07-20T14:10:49.9560710Z ------------------------------------------
2019-07-20T14:10:49.9560779Z stderr:
---
2019-07-20T14:10:49.9731489Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:49.9731542Z +
2019-07-20T14:10:49.9731570Z 
2019-07-20T14:10:49.9731616Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:49.9732046Z Actual stderr saved to /tmp/compiletest8i5dKZ/issue-35815.stderr
2019-07-20T14:10:49.9732105Z To update references, run this command from build directory:
2019-07-20T14:10:49.9732524Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'issue-35815.rs'
2019-07-20T14:10:49.9732637Z error: 1 errors occurred comparing output.
2019-07-20T14:10:49.9732682Z status: exit code: 1
2019-07-20T14:10:49.9732682Z status: exit code: 1
2019-07-20T14:10:49.9733799Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-35815.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/issue-35815.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/issue-35815.stage-id.aux" "-A" "unused"
2019-07-20T14:10:49.9734235Z ------------------------------------------
2019-07-20T14:10:49.9734295Z 
2019-07-20T14:10:49.9734547Z ------------------------------------------
2019-07-20T14:10:49.9734597Z stderr:
---
2019-07-20T14:10:50.1243095Z -S { s: 5 }
2019-07-20T14:10:50.1243296Z -
2019-07-20T14:10:50.1243347Z 
2019-07-20T14:10:50.1243674Z The actual stdout differed from the expected stdout.
2019-07-20T14:10:50.1244063Z Actual stdout saved to /tmp/compiletest8i5dKZ/issue-3794.stdout
2019-07-20T14:10:50.1244218Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-20T14:10:50.1244831Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-20T14:10:50.1244916Z      |
2019-07-20T14:10:50.1244967Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-20T14:10:50.1249162Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:50.1249215Z +
2019-07-20T14:10:50.1249246Z 
2019-07-20T14:10:50.1249314Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:50.1249617Z Actual stderr saved to /tmp/compiletest8i5dKZ/issue-3794.stderr
2019-07-20T14:10:50.1249681Z To update references, run this command from build directory:
2019-07-20T14:10:50.1250010Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'issue-3794.rs'
2019-07-20T14:10:50.1250100Z error: 2 errors occurred comparing output.
2019-07-20T14:10:50.1250167Z status: exit code: 1
2019-07-20T14:10:50.1250167Z status: exit code: 1
2019-07-20T14:10:50.1250921Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-3794.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/issue-3794.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/issue-3794.stage-id.aux" "-A" "unused"
2019-07-20T14:10:50.1251303Z ------------------------------------------
2019-07-20T14:10:50.1251341Z 
2019-07-20T14:10:50.1251609Z ------------------------------------------
2019-07-20T14:10:50.1251679Z stderr:
---
2019-07-20T14:10:50.1461562Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:50.1461618Z +
2019-07-20T14:10:50.1461649Z 
2019-07-20T14:10:50.1461701Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:50.1462046Z Actual stderr saved to /tmp/compiletest8i5dKZ/issue-36278-prefix-nesting.stderr
2019-07-20T14:10:50.1462109Z To update references, run this command from build directory:
2019-07-20T14:10:50.1462439Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'issue-36278-prefix-nesting.rs'
2019-07-20T14:10:50.1462568Z error: 1 errors occurred comparing output.
2019-07-20T14:10:50.1462620Z status: exit code: 1
2019-07-20T14:10:50.1462620Z status: exit code: 1
2019-07-20T14:10:50.1463878Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-36278-prefix-nesting.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/issue-36278-prefix-nesting.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/issue-36278-prefix-nesting.stage-id.aux" "-A" "unused"
2019-07-20T14:10:50.1464611Z ------------------------------------------
2019-07-20T14:10:50.1464657Z 
2019-07-20T14:10:50.1464914Z ------------------------------------------
2019-07-20T14:10:50.1464963Z stderr:
---
2019-07-20T14:10:50.2572497Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:50.2572546Z +
2019-07-20T14:10:50.2572589Z 
2019-07-20T14:10:50.2572636Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:50.2572905Z Actual stderr saved to /tmp/compiletest8i5dKZ/issue-53728.stderr
2019-07-20T14:10:50.2572974Z To update references, run this command from build directory:
2019-07-20T14:10:50.2573271Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'issue-53728.rs'
2019-07-20T14:10:50.2573732Z error: 1 errors occurred comparing output.
2019-07-20T14:10:50.2573806Z status: exit code: 1
2019-07-20T14:10:50.2573806Z status: exit code: 1
2019-07-20T14:10:50.2574522Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-53728.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/issue-53728.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/issue-53728.stage-id.aux" "-A" "unused"
2019-07-20T14:10:50.2574847Z ------------------------------------------
2019-07-20T14:10:50.2574881Z 
2019-07-20T14:10:50.2575114Z ------------------------------------------
2019-07-20T14:10:50.2575173Z stderr:
---
2019-07-20T14:10:50.3054476Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:50.3054559Z +
2019-07-20T14:10:50.3054588Z 
2019-07-20T14:10:50.3054643Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:50.3054925Z Actual stderr saved to /tmp/compiletest8i5dKZ/issue-5917.stderr
2019-07-20T14:10:50.3054998Z To update references, run this command from build directory:
2019-07-20T14:10:50.3055275Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'issue-5917.rs'
2019-07-20T14:10:50.3055374Z error: 1 errors occurred comparing output.
2019-07-20T14:10:50.3055419Z status: exit code: 1
2019-07-20T14:10:50.3055419Z status: exit code: 1
2019-07-20T14:10:50.3056068Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-5917.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/issue-5917.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/issue-5917.stage-id.aux" "-A" "unused"
2019-07-20T14:10:50.3056434Z ------------------------------------------
2019-07-20T14:10:50.3056468Z 
2019-07-20T14:10:50.3056728Z ------------------------------------------
2019-07-20T14:10:50.3056774Z stderr:
---
2019-07-20T14:10:50.3974863Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:50.3974940Z +
2019-07-20T14:10:50.3974971Z 
2019-07-20T14:10:50.3975044Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:50.3975349Z Actual stderr saved to /tmp/compiletest8i5dKZ/issue-miri-184.stderr
2019-07-20T14:10:50.3975410Z To update references, run this command from build directory:
2019-07-20T14:10:50.3975734Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'issue-miri-184.rs'
2019-07-20T14:10:50.3975825Z error: 1 errors occurred comparing output.
2019-07-20T14:10:50.3975873Z status: exit code: 1
2019-07-20T14:10:50.3975873Z status: exit code: 1
2019-07-20T14:10:50.3976586Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-miri-184.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/issue-miri-184.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/issue-miri-184.stage-id.aux" "-A" "unused"
2019-07-20T14:10:50.3977031Z ------------------------------------------
2019-07-20T14:10:50.3977077Z 
2019-07-20T14:10:50.3977352Z ------------------------------------------
2019-07-20T14:10:50.3977425Z stderr:
---
2019-07-20T14:10:50.5334209Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:50.5334323Z +
2019-07-20T14:10:50.5334372Z 
2019-07-20T14:10:50.5334428Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:50.5334480Z Actual stderr saved to /tmp/compiletest8i5dKZ/iter.stderr
2019-07-20T14:10:50.5334588Z To update references, run this command from build directory:
2019-07-20T14:10:50.5334888Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'iter.rs'
2019-07-20T14:10:50.5334979Z error: 1 errors occurred comparing output.
2019-07-20T14:10:50.5335102Z status: exit code: 1
2019-07-20T14:10:50.5335102Z status: exit code: 1
2019-07-20T14:10:50.5335751Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/iter.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/iter.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/iter.stage-id.aux" "-A" "unused"
2019-07-20T14:10:50.5336248Z ------------------------------------------
2019-07-20T14:10:50.5336289Z 
2019-07-20T14:10:50.5336592Z ------------------------------------------
2019-07-20T14:10:50.5336645Z stderr:
---
2019-07-20T14:10:50.5722566Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:50.5722806Z +
2019-07-20T14:10:50.5722968Z 
2019-07-20T14:10:50.5723734Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:50.5724301Z Actual stderr saved to /tmp/compiletest8i5dKZ/last-use-in-cap-clause.stderr
2019-07-20T14:10:50.5724569Z To update references, run this command from build directory:
2019-07-20T14:10:50.5725044Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'last-use-in-cap-clause.rs'
2019-07-20T14:10:50.5725461Z error: 1 errors occurred comparing output.
2019-07-20T14:10:50.5725635Z status: exit code: 1
2019-07-20T14:10:50.5725635Z status: exit code: 1
2019-07-20T14:10:50.5726640Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/last-use-in-cap-clause.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/last-use-in-cap-clause.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/last-use-in-cap-clause.stage-id.aux" "-A" "unused"
2019-07-20T14:10:50.5727610Z ------------------------------------------
2019-07-20T14:10:50.5727819Z 
2019-07-20T14:10:50.5728218Z ------------------------------------------
2019-07-20T14:10:50.5728417Z stderr:
---
2019-07-20T14:10:50.7938343Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:50.7938397Z +
2019-07-20T14:10:50.7938427Z 
2019-07-20T14:10:50.7938497Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:50.7938776Z Actual stderr saved to /tmp/compiletest8i5dKZ/linked-list.stderr
2019-07-20T14:10:50.7938835Z To update references, run this command from build directory:
2019-07-20T14:10:50.7939144Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'linked-list.rs'
2019-07-20T14:10:50.7939231Z error: 1 errors occurred comparing output.
2019-07-20T14:10:50.7939300Z status: exit code: 1
2019-07-20T14:10:50.7939300Z status: exit code: 1
2019-07-20T14:10:50.7940024Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/linked-list.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/linked-list.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/linked-list.stage-id.aux" "-A" "unused"
2019-07-20T14:10:50.7940387Z ------------------------------------------
2019-07-20T14:10:50.7940424Z 
2019-07-20T14:10:50.7940667Z ------------------------------------------
2019-07-20T14:10:50.7940734Z stderr:
---
2019-07-20T14:10:50.8172971Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:50.8173046Z +
2019-07-20T14:10:50.8173078Z 
2019-07-20T14:10:50.8173130Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:50.8173432Z Actual stderr saved to /tmp/compiletest8i5dKZ/loop-break-value.stderr
2019-07-20T14:10:50.8173510Z To update references, run this command from build directory:
2019-07-20T14:10:50.8174132Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'loop-break-value.rs'
2019-07-20T14:10:50.8174258Z error: 1 errors occurred comparing output.
2019-07-20T14:10:50.8174309Z status: exit code: 1
2019-07-20T14:10:50.8174309Z status: exit code: 1
2019-07-20T14:10:50.8175110Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/loop-break-value.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/loop-break-value.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/loop-break-value.stage-id.aux" "-A" "unused"
2019-07-20T14:10:50.8175483Z ------------------------------------------
2019-07-20T14:10:50.8175520Z 
2019-07-20T14:10:50.8175785Z ------------------------------------------
2019-07-20T14:10:50.8175835Z stderr:
---
2019-07-20T14:10:50.9398304Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:50.9398357Z +
2019-07-20T14:10:50.9398388Z 
2019-07-20T14:10:50.9398456Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:50.9398514Z Actual stderr saved to /tmp/compiletest8i5dKZ/main_fn.stderr
2019-07-20T14:10:50.9398571Z To update references, run this command from build directory:
2019-07-20T14:10:50.9398902Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'main_fn.rs'
2019-07-20T14:10:50.9398994Z error: 1 errors occurred comparing output.
2019-07-20T14:10:50.9399046Z status: exit code: 1
2019-07-20T14:10:50.9399046Z status: exit code: 1
2019-07-20T14:10:50.9399795Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/main_fn.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/main_fn.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/main_fn.stage-id.aux" "-A" "unused"
2019-07-20T14:10:50.9400177Z ------------------------------------------
2019-07-20T14:10:50.9400216Z 
2019-07-20T14:10:50.9400481Z ------------------------------------------
2019-07-20T14:10:50.9400548Z stderr:
---
2019-07-20T14:10:50.9931534Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:50.9931598Z +
2019-07-20T14:10:50.9931625Z 
2019-07-20T14:10:50.9931671Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:50.9931723Z Actual stderr saved to /tmp/compiletest8i5dKZ/loops.stderr
2019-07-20T14:10:50.9931791Z To update references, run this command from build directory:
2019-07-20T14:10:50.9932065Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'loops.rs'
2019-07-20T14:10:50.9932160Z error: 1 errors occurred comparing output.
2019-07-20T14:10:50.9932205Z status: exit code: 1
2019-07-20T14:10:50.9932205Z status: exit code: 1
2019-07-20T14:10:50.9932862Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/loops.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/loops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/loops.stage-id.aux" "-A" "unused"
2019-07-20T14:10:50.9933203Z ------------------------------------------
2019-07-20T14:10:50.9933237Z 
2019-07-20T14:10:50.9933897Z ------------------------------------------
2019-07-20T14:10:50.9933956Z stderr:
---
2019-07-20T14:10:51.0953135Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:51.0953201Z +
2019-07-20T14:10:51.0953228Z 
2019-07-20T14:10:51.0953274Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:51.0953340Z Actual stderr saved to /tmp/compiletest8i5dKZ/many_shr_bor.stderr
2019-07-20T14:10:51.0953921Z To update references, run this command from build directory:
2019-07-20T14:10:51.0954275Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'many_shr_bor.rs'
2019-07-20T14:10:51.0954388Z error: 1 errors occurred comparing output.
2019-07-20T14:10:51.0954436Z status: exit code: 1
2019-07-20T14:10:51.0954436Z status: exit code: 1
2019-07-20T14:10:51.0955103Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/many_shr_bor.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/many_shr_bor.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/many_shr_bor.stage-id.aux" "-A" "unused"
2019-07-20T14:10:51.0955446Z ------------------------------------------
2019-07-20T14:10:51.0955501Z 
2019-07-20T14:10:51.0955742Z ------------------------------------------
2019-07-20T14:10:51.0955788Z stderr:
---
2019-07-20T14:10:51.1154521Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:51.1154576Z +
2019-07-20T14:10:51.1154603Z 
2019-07-20T14:10:51.1154646Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:51.1154717Z Actual stderr saved to /tmp/compiletest8i5dKZ/match_slice.stderr
2019-07-20T14:10:51.1154768Z To update references, run this command from build directory:
2019-07-20T14:10:51.1155032Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'match_slice.rs'
2019-07-20T14:10:51.1155148Z error: 1 errors occurred comparing output.
2019-07-20T14:10:51.1155193Z status: exit code: 1
2019-07-20T14:10:51.1155193Z status: exit code: 1
2019-07-20T14:10:51.1155849Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/match_slice.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/match_slice.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/match_slice.stage-id.aux" "-A" "unused"
2019-07-20T14:10:51.1156170Z ------------------------------------------
2019-07-20T14:10:51.1156204Z 
2019-07-20T14:10:51.1156422Z ------------------------------------------
2019-07-20T14:10:51.1156465Z stderr:
---
2019-07-20T14:10:51.3385406Z +
2019-07-20T14:10:51.3385826Z thread '[ui] run-pass/memchr.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-20T14:10:51.3385878Z 
2019-07-20T14:10:51.3385952Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:51.3386013Z Actual stderr saved to /tmp/compiletest8i5dKZ/memchr.stderr
2019-07-20T14:10:51.3386092Z To update references, run this command from build directory:
2019-07-20T14:10:51.3386414Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'memchr.rs'
2019-07-20T14:10:51.3386531Z error: 1 errors occurred comparing output.
2019-07-20T14:10:51.3386583Z status: exit code: 1
2019-07-20T14:10:51.3386583Z status: exit code: 1
2019-07-20T14:10:51.3387353Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/memchr.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/memchr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/memchr.stage-id.aux" "-A" "unused"
2019-07-20T14:10:51.3387746Z ------------------------------------------
2019-07-20T14:10:51.3387790Z 
2019-07-20T14:10:51.3391240Z ------------------------------------------
2019-07-20T14:10:51.3391478Z stderr:
---
2019-07-20T14:10:51.3416770Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:51.3416835Z +
2019-07-20T14:10:51.3416870Z 
2019-07-20T14:10:51.3416925Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:51.3417006Z Actual stderr saved to /tmp/compiletest8i5dKZ/mir_coercions.stderr
2019-07-20T14:10:51.3417068Z To update references, run this command from build directory:
2019-07-20T14:10:51.3417451Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'mir_coercions.rs'
2019-07-20T14:10:51.3417576Z error: 1 errors occurred comparing output.
2019-07-20T14:10:51.3417630Z status: exit code: 1
2019-07-20T14:10:51.3417630Z status: exit code: 1
2019-07-20T14:10:51.3418428Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mir_coercions.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/mir_coercions.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/mir_coercions.stage-id.aux" "-A" "unused"
2019-07-20T14:10:51.3418849Z ------------------------------------------
2019-07-20T14:10:51.3418896Z 
2019-07-20T14:10:51.3419194Z ------------------------------------------
2019-07-20T14:10:51.3419254Z stderr:
---
2019-07-20T14:10:51.4894700Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:51.4894781Z +
2019-07-20T14:10:51.4894809Z 
2019-07-20T14:10:51.4894855Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:51.4895124Z Actual stderr saved to /tmp/compiletest8i5dKZ/miri-issue-133.stderr
2019-07-20T14:10:51.4895216Z To update references, run this command from build directory:
2019-07-20T14:10:51.4895484Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'miri-issue-133.rs'
2019-07-20T14:10:51.4895579Z error: 1 errors occurred comparing output.
2019-07-20T14:10:51.4895624Z status: exit code: 1
2019-07-20T14:10:51.4895624Z status: exit code: 1
2019-07-20T14:10:51.4896283Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/miri-issue-133.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/miri-issue-133.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/miri-issue-133.stage-id.aux" "-A" "unused"
2019-07-20T14:10:51.4896610Z ------------------------------------------
2019-07-20T14:10:51.4896643Z 
2019-07-20T14:10:51.4896893Z ------------------------------------------
2019-07-20T14:10:51.4897057Z stderr:
---
2019-07-20T14:10:51.5024758Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:51.5024805Z +
2019-07-20T14:10:51.5024831Z 
2019-07-20T14:10:51.5024892Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:51.5024962Z Actual stderr saved to /tmp/compiletest8i5dKZ/mir_fat_ptr.stderr
2019-07-20T14:10:51.5025012Z To update references, run this command from build directory:
2019-07-20T14:10:51.5025295Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'mir_fat_ptr.rs'
2019-07-20T14:10:51.5025372Z error: 1 errors occurred comparing output.
2019-07-20T14:10:51.5025417Z status: exit code: 1
2019-07-20T14:10:51.5025417Z status: exit code: 1
2019-07-20T14:10:51.5026072Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mir_fat_ptr.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/mir_fat_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/mir_fat_ptr.stage-id.aux" "-A" "unused"
2019-07-20T14:10:51.5026391Z ------------------------------------------
2019-07-20T14:10:51.5026549Z 
2019-07-20T14:10:51.5026811Z ------------------------------------------
2019-07-20T14:10:51.5026874Z stderr:
---
2019-07-20T14:10:51.6473563Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:51.6473610Z +
2019-07-20T14:10:51.6473653Z 
2019-07-20T14:10:51.6473737Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:51.6474834Z Actual stderr saved to /tmp/compiletest8i5dKZ/move-arg-2-unique.stderr
2019-07-20T14:10:51.6474905Z To update references, run this command from build directory:
2019-07-20T14:10:51.6475259Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'move-arg-2-unique.rs'
2019-07-20T14:10:51.6475341Z error: 1 errors occurred comparing output.
2019-07-20T14:10:51.6475385Z status: exit code: 1
2019-07-20T14:10:51.6475385Z status: exit code: 1
2019-07-20T14:10:51.6476075Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-arg-2-unique.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/move-arg-2-unique.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/move-arg-2-unique.stage-id.aux" "-A" "unused"
2019-07-20T14:10:51.6476634Z ------------------------------------------
2019-07-20T14:10:51.6476669Z 
2019-07-20T14:10:51.6476891Z ------------------------------------------
2019-07-20T14:10:51.6476952Z stderr:
---
2019-07-20T14:10:51.6809597Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:51.6809644Z +
2019-07-20T14:10:51.6809670Z 
2019-07-20T14:10:51.6809713Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:51.6809982Z Actual stderr saved to /tmp/compiletest8i5dKZ/move-arg-3-unique.stderr
2019-07-20T14:10:51.6810032Z To update references, run this command from build directory:
2019-07-20T14:10:51.6810277Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'move-arg-3-unique.rs'
2019-07-20T14:10:51.6810369Z error: 1 errors occurred comparing output.
2019-07-20T14:10:51.6810411Z status: exit code: 1
2019-07-20T14:10:51.6810411Z status: exit code: 1
2019-07-20T14:10:51.6811186Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-arg-3-unique.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/move-arg-3-unique.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/move-arg-3-unique.stage-id.aux" "-A" "unused"
2019-07-20T14:10:51.6811549Z ------------------------------------------
2019-07-20T14:10:51.6811582Z 
2019-07-20T14:10:51.6811786Z ------------------------------------------
2019-07-20T14:10:51.6811827Z stderr:
---
2019-07-20T14:10:51.7767998Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:51.7768080Z +
2019-07-20T14:10:51.7768115Z 
2019-07-20T14:10:51.7768171Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:51.7768509Z Actual stderr saved to /tmp/compiletest8i5dKZ/move-undef-primval.stderr
2019-07-20T14:10:51.7768598Z To update references, run this command from build directory:
2019-07-20T14:10:51.7768949Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'move-undef-primval.rs'
2019-07-20T14:10:51.7769072Z error: 1 errors occurred comparing output.
2019-07-20T14:10:51.7769128Z status: exit code: 1
2019-07-20T14:10:51.7769128Z status: exit code: 1
2019-07-20T14:10:51.7770028Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/move-undef-primval.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/move-undef-primval.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/move-undef-primval.stage-id.aux" "-A" "unused"
2019-07-20T14:10:51.7770515Z ------------------------------------------
2019-07-20T14:10:51.7770561Z 
2019-07-20T14:10:51.7770883Z ------------------------------------------
2019-07-20T14:10:51.7770943Z stderr:
---
2019-07-20T14:10:51.8921417Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:51.8921462Z +
2019-07-20T14:10:51.8921488Z 
2019-07-20T14:10:51.8921548Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:51.8921595Z Actual stderr saved to /tmp/compiletest8i5dKZ/mpsc.stderr
2019-07-20T14:10:51.8921641Z To update references, run this command from build directory:
2019-07-20T14:10:51.8921916Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'mpsc.rs'
2019-07-20T14:10:51.8921991Z error: 1 errors occurred comparing output.
2019-07-20T14:10:51.8922051Z status: exit code: 1
2019-07-20T14:10:51.8922051Z status: exit code: 1
2019-07-20T14:10:51.8922739Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/mpsc.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/mpsc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/mpsc.stage-id.aux" "-A" "unused"
2019-07-20T14:10:51.8923103Z ------------------------------------------
2019-07-20T14:10:51.8923136Z 
2019-07-20T14:10:51.8923363Z ------------------------------------------
2019-07-20T14:10:51.8923424Z stderr:
---
2019-07-20T14:10:51.9360580Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:51.9360625Z +
2019-07-20T14:10:51.9360651Z 
2019-07-20T14:10:51.9360695Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:51.9360762Z Actual stderr saved to /tmp/compiletest8i5dKZ/multi_arg_closure.stderr
2019-07-20T14:10:51.9360811Z To update references, run this command from build directory:
2019-07-20T14:10:51.9361094Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'multi_arg_closure.rs'
2019-07-20T14:10:51.9361188Z error: 1 errors occurred comparing output.
2019-07-20T14:10:51.9361230Z status: exit code: 1
2019-07-20T14:10:51.9361230Z status: exit code: 1
2019-07-20T14:10:51.9362000Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/multi_arg_closure.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/multi_arg_closure.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/multi_arg_closure.stage-id.aux" "-A" "unused"
2019-07-20T14:10:51.9362365Z ------------------------------------------
2019-07-20T14:10:51.9362417Z 
2019-07-20T14:10:51.9362652Z ------------------------------------------
2019-07-20T14:10:51.9362694Z stderr:
---
2019-07-20T14:10:52.0378898Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:52.0379200Z +
2019-07-20T14:10:52.0379440Z 
2019-07-20T14:10:52.0379774Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:52.0380000Z Actual stderr saved to /tmp/compiletest8i5dKZ/negative_discriminant.stderr
2019-07-20T14:10:52.0380242Z To update references, run this command from build directory:
2019-07-20T14:10:52.0380853Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'negative_discriminant.rs'
2019-07-20T14:10:52.0430068Z error: 1 errors occurred comparing output.
2019-07-20T14:10:52.0430593Z status: exit code: 1
2019-07-20T14:10:52.0430593Z status: exit code: 1
2019-07-20T14:10:52.0431681Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/negative_discriminant.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/negative_discriminant.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/negative_discriminant.stage-id.aux" "-A" "unused"
2019-07-20T14:10:52.0433035Z ------------------------------------------
2019-07-20T14:10:52.0433284Z 
2019-07-20T14:10:52.0433736Z ------------------------------------------
2019-07-20T14:10:52.0433907Z stderr:
---
2019-07-20T14:10:52.1282583Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:52.1282831Z +
2019-07-20T14:10:52.1282992Z 
2019-07-20T14:10:52.1283218Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:52.1283420Z Actual stderr saved to /tmp/compiletest8i5dKZ/non_capture_closure_to_fn_ptr.stderr
2019-07-20T14:10:52.1283642Z To update references, run this command from build directory:
2019-07-20T14:10:52.1284824Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'non_capture_closure_to_fn_ptr.rs'
2019-07-20T14:10:52.1285385Z error: 1 errors occurred comparing output.
2019-07-20T14:10:52.1285581Z status: exit code: 1
2019-07-20T14:10:52.1285581Z status: exit code: 1
2019-07-20T14:10:52.1286650Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/non_capture_closure_to_fn_ptr.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/non_capture_closure_to_fn_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/non_capture_closure_to_fn_ptr.stage-id.aux" "-A" "unused"
2019-07-20T14:10:52.1287461Z ------------------------------------------
2019-07-20T14:10:52.1287683Z 
2019-07-20T14:10:52.1288208Z ------------------------------------------
2019-07-20T14:10:52.1288621Z stderr:
---
2019-07-20T14:10:52.2052274Z +
2019-07-20T14:10:52.2052821Z thread '[ui] run-pass/observed_local_mut.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-20T14:10:52.2052924Z 
2019-07-20T14:10:52.2052984Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:52.2053047Z Actual stderr saved to /tmp/compiletest8i5dKZ/observed_local_mut.stderr
2019-07-20T14:10:52.2053129Z To update references, run this command from build directory:
2019-07-20T14:10:52.2053518Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'observed_local_mut.rs'
2019-07-20T14:10:52.2053620Z error: 1 errors occurred comparing output.
2019-07-20T14:10:52.2053698Z status: exit code: 1
2019-07-20T14:10:52.2053698Z status: exit code: 1
2019-07-20T14:10:52.2055135Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/observed_local_mut.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/observed_local_mut.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletest8i5dKZ/observed_local_mut.stage-id.aux" "-A" "unused"
2019-07-20T14:10:52.2055749Z ------------------------------------------
2019-07-20T14:10:52.2055795Z 
2019-07-20T14:10:52.2056097Z ------------------------------------------
2019-07-20T14:10:52.2056157Z stderr:
---
2019-07-20T14:10:52.2832329Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:52.2832390Z +
2019-07-20T14:10:52.2832420Z 
2019-07-20T14:10:52.2832621Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:52.2832704Z Actual stderr saved to /tmp/compiletest8i5dKZ/option_box_transmute_ptr.stderr
2019-07-20T14:10:52.2832755Z To update references, run this command from build directory:
2019-07-20T14:10:52.2833106Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'option_box_transmute_ptr.rs'
2019-07-20T14:10:52.2833206Z error: 1 errors occurred comparing output.
2019-07-20T14:10:52.2833251Z status: exit code: 1
2019-07-20T14:10:52.2833251Z status: exit code: 1
2019-07-20T14:10:52.2834326Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/option_box_transmute_ptr.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/option_box_transmute_ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/option_box_transmute_ptr.stage-id.aux" "-A" "unused"
2019-07-20T14:10:52.2834880Z ------------------------------------------
2019-07-20T14:10:52.2834936Z 
2019-07-20T14:10:52.2835166Z ------------------------------------------
2019-07-20T14:10:52.2835211Z stderr:
---
2019-07-20T14:10:52.3853080Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:52.3853142Z +
2019-07-20T14:10:52.3853169Z 
2019-07-20T14:10:52.3853214Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:52.3853262Z Actual stderr saved to /tmp/compiletest8i5dKZ/option_eq.stderr
2019-07-20T14:10:52.3853327Z To update references, run this command from build directory:
2019-07-20T14:10:52.3853599Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'option_eq.rs'
2019-07-20T14:10:52.3853692Z error: 1 errors occurred comparing output.
2019-07-20T14:10:52.3853736Z status: exit code: 1
2019-07-20T14:10:52.3853736Z status: exit code: 1
2019-07-20T14:10:52.3854877Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/option_eq.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/option_eq.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/option_eq.stage-id.aux" "-A" "unused"
2019-07-20T14:10:52.3855486Z ------------------------------------------
2019-07-20T14:10:52.3855521Z 
2019-07-20T14:10:52.3855766Z ------------------------------------------
2019-07-20T14:10:52.3855811Z stderr:
---
2019-07-20T14:10:52.4573623Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:52.4573673Z +
2019-07-20T14:10:52.4573701Z 
2019-07-20T14:10:52.4573767Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:52.4576002Z Actual stderr saved to /tmp/compiletest8i5dKZ/overloaded-calls-simple.stderr
2019-07-20T14:10:52.4576076Z To update references, run this command from build directory:
2019-07-20T14:10:52.4576390Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'overloaded-calls-simple.rs'
2019-07-20T14:10:52.4576473Z error: 1 errors occurred comparing output.
2019-07-20T14:10:52.4576518Z status: exit code: 1
2019-07-20T14:10:52.4576518Z status: exit code: 1
2019-07-20T14:10:52.4577246Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/overloaded-calls-simple.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/overloaded-calls-simple.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/overloaded-calls-simple.stage-id.aux" "-A" "unused"
2019-07-20T14:10:52.4577787Z ------------------------------------------
2019-07-20T14:10:52.4577822Z 
2019-07-20T14:10:52.4578043Z ------------------------------------------
2019-07-20T14:10:52.4578105Z stderr:
---
2019-07-20T14:10:52.5443487Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:52.5443567Z +
2019-07-20T14:10:52.5443604Z 
2019-07-20T14:10:52.5443659Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:52.5443721Z Actual stderr saved to /tmp/compiletest8i5dKZ/packed_static.stderr
2019-07-20T14:10:52.5443801Z To update references, run this command from build directory:
2019-07-20T14:10:52.5444582Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'packed_static.rs'
2019-07-20T14:10:52.5444886Z error: 1 errors occurred comparing output.
2019-07-20T14:10:52.5444965Z status: exit code: 1
2019-07-20T14:10:52.5444965Z status: exit code: 1
2019-07-20T14:10:52.5445838Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/packed_static.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/packed_static.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/packed_static.stage-id.aux" "-A" "unused"
2019-07-20T14:10:52.5446245Z ------------------------------------------
2019-07-20T14:10:52.5446288Z 
2019-07-20T14:10:52.5446580Z ------------------------------------------
2019-07-20T14:10:52.5446637Z stderr:
---
2019-07-20T14:10:52.6652376Z +
2019-07-20T14:10:52.6652700Z thread '[ui] run-pass/packed_struct.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-20T14:10:52.6652737Z 
2019-07-20T14:10:52.6652796Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:52.6652842Z Actual stderr saved to /tmp/compiletest8i5dKZ/packed_struct.stderr
2019-07-20T14:10:52.6652889Z To update references, run this command from build directory:
2019-07-20T14:10:52.6653266Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'packed_struct.rs'
2019-07-20T14:10:52.6653338Z error: 1 errors occurred comparing output.
2019-07-20T14:10:52.6653395Z status: exit code: 1
2019-07-20T14:10:52.6653395Z status: exit code: 1
2019-07-20T14:10:52.6653982Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/packed_struct.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/packed_struct.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/packed_struct.stage-id.aux" "-A" "unused"
2019-07-20T14:10:52.6654753Z ------------------------------------------
2019-07-20T14:10:52.6654806Z 
2019-07-20T14:10:52.6655096Z ------------------------------------------
2019-07-20T14:10:52.6655164Z stderr:
---
2019-07-20T14:10:52.7616418Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:52.7616488Z +
2019-07-20T14:10:52.7616517Z 
2019-07-20T14:10:52.7616565Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:52.7616633Z Actual stderr saved to /tmp/compiletest8i5dKZ/pointers.stderr
2019-07-20T14:10:52.7616685Z To update references, run this command from build directory:
2019-07-20T14:10:52.7616980Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'pointers.rs'
2019-07-20T14:10:52.7617206Z error: 1 errors occurred comparing output.
2019-07-20T14:10:52.7617252Z status: exit code: 1
2019-07-20T14:10:52.7617252Z status: exit code: 1
2019-07-20T14:10:52.7618033Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/pointers.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/pointers.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/pointers.stage-id.aux" "-A" "unused"
2019-07-20T14:10:52.7618480Z ------------------------------------------
2019-07-20T14:10:52.7618529Z 
2019-07-20T14:10:52.7618733Z ------------------------------------------
2019-07-20T14:10:52.7618776Z stderr:
---
2019-07-20T14:10:52.8611559Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:52.8611608Z +
2019-07-20T14:10:52.8611636Z 
2019-07-20T14:10:52.8611699Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:52.8611750Z Actual stderr saved to /tmp/compiletest8i5dKZ/products.stderr
2019-07-20T14:10:52.8611801Z To update references, run this command from build directory:
2019-07-20T14:10:52.8612105Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'products.rs'
2019-07-20T14:10:52.8616356Z error: 1 errors occurred comparing output.
2019-07-20T14:10:52.8616429Z status: exit code: 1
2019-07-20T14:10:52.8616429Z status: exit code: 1
2019-07-20T14:10:52.8617188Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/products.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/products.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/products.stage-id.aux" "-A" "unused"
2019-07-20T14:10:52.8617516Z ------------------------------------------
2019-07-20T14:10:52.8617550Z 
2019-07-20T14:10:52.8617770Z ------------------------------------------
2019-07-20T14:10:52.8617831Z stderr:
---
2019-07-20T14:10:52.9179242Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:52.9179289Z +
2019-07-20T14:10:52.9179316Z 
2019-07-20T14:10:52.9179376Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:52.9179426Z Actual stderr saved to /tmp/compiletest8i5dKZ/ptr_arith_offset.stderr
2019-07-20T14:10:52.9179474Z To update references, run this command from build directory:
2019-07-20T14:10:52.9179905Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'ptr_arith_offset.rs'
2019-07-20T14:10:52.9179984Z error: 1 errors occurred comparing output.
2019-07-20T14:10:52.9180027Z status: exit code: 1
2019-07-20T14:10:52.9180027Z status: exit code: 1
2019-07-20T14:10:52.9180685Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_arith_offset.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/ptr_arith_offset.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/ptr_arith_offset.stage-id.aux" "-A" "unused"
2019-07-20T14:10:52.9181008Z ------------------------------------------
2019-07-20T14:10:52.9181041Z 
2019-07-20T14:10:52.9181263Z ------------------------------------------
2019-07-20T14:10:52.9181330Z stderr:
---
2019-07-20T14:10:53.0255871Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:53.0255959Z +
2019-07-20T14:10:53.0255992Z 
2019-07-20T14:10:53.0256046Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:53.0256117Z Actual stderr saved to /tmp/compiletest8i5dKZ/ptr_arith_offset_overflow.stderr
2019-07-20T14:10:53.0256311Z To update references, run this command from build directory:
2019-07-20T14:10:53.0256685Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'ptr_arith_offset_overflow.rs'
2019-07-20T14:10:53.0256806Z error: 1 errors occurred comparing output.
2019-07-20T14:10:53.0256859Z status: exit code: 1
2019-07-20T14:10:53.0256859Z status: exit code: 1
2019-07-20T14:10:53.0257667Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_arith_offset_overflow.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/ptr_arith_offset_overflow.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/ptr_arith_offset_overflow.stage-id.aux" "-A" "unused"
2019-07-20T14:10:53.0258103Z ------------------------------------------
2019-07-20T14:10:53.0258166Z 
2019-07-20T14:10:53.0258448Z ------------------------------------------
2019-07-20T14:10:53.0258505Z stderr:
---
2019-07-20T14:10:53.1044594Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:53.1045089Z +
2019-07-20T14:10:53.1045132Z 
2019-07-20T14:10:53.1045183Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:53.1045254Z Actual stderr saved to /tmp/compiletest8i5dKZ/ptr_int_casts.stderr
2019-07-20T14:10:53.1045312Z To update references, run this command from build directory:
2019-07-20T14:10:53.1045720Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'ptr_int_casts.rs'
2019-07-20T14:10:53.1046169Z thread '[ui] run-pass/ptr_int_casts.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-20T14:10:53.1046236Z error: 1 errors occurred comparing output.
2019-07-20T14:10:53.1046287Z status: exit code: 1
2019-07-20T14:10:53.1046287Z status: exit code: 1
2019-07-20T14:10:53.1047049Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_int_casts.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/ptr_int_casts.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/ptr_int_casts.stage-id.aux" "-A" "unused"
2019-07-20T14:10:53.1047415Z ------------------------------------------
2019-07-20T14:10:53.1047452Z 
2019-07-20T14:10:53.1047696Z ------------------------------------------
2019-07-20T14:10:53.1047762Z stderr:
---
2019-07-20T14:10:53.2256162Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:53.2256397Z +
2019-07-20T14:10:53.2256752Z 
2019-07-20T14:10:53.2256943Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:53.2257119Z Actual stderr saved to /tmp/compiletest8i5dKZ/ptr_int_ops.stderr
2019-07-20T14:10:53.2257275Z To update references, run this command from build directory:
2019-07-20T14:10:53.2259844Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'ptr_int_ops.rs'
2019-07-20T14:10:53.2261178Z error: 1 errors occurred comparing output.
2019-07-20T14:10:53.2261382Z status: exit code: 1
2019-07-20T14:10:53.2261382Z status: exit code: 1
2019-07-20T14:10:53.2263383Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_int_ops.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/ptr_int_ops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/ptr_int_ops.stage-id.aux" "-A" "unused"
2019-07-20T14:10:53.2267757Z ------------------------------------------
2019-07-20T14:10:53.2270018Z 
2019-07-20T14:10:53.2275941Z ------------------------------------------
2019-07-20T14:10:53.2310959Z stderr:
---
2019-07-20T14:10:53.2707783Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:53.2707860Z +
2019-07-20T14:10:53.2707891Z 
2019-07-20T14:10:53.2707944Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:53.2708090Z Actual stderr saved to /tmp/compiletest8i5dKZ/ptr_offset.stderr
2019-07-20T14:10:53.2708148Z To update references, run this command from build directory:
2019-07-20T14:10:53.2708475Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'ptr_offset.rs'
2019-07-20T14:10:53.2708585Z error: 1 errors occurred comparing output.
2019-07-20T14:10:53.2708637Z status: exit code: 1
2019-07-20T14:10:53.2708637Z status: exit code: 1
2019-07-20T14:10:53.2709388Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ptr_offset.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/ptr_offset.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/ptr_offset.stage-id.aux" "-A" "unused"
2019-07-20T14:10:53.2710148Z ------------------------------------------
2019-07-20T14:10:53.2710219Z 
2019-07-20T14:10:53.2710592Z ------------------------------------------
2019-07-20T14:10:53.2710643Z stderr:
---
2019-07-20T14:10:53.4335325Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:53.4335632Z +
2019-07-20T14:10:53.4335859Z 
2019-07-20T14:10:53.4336094Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:53.4336325Z Actual stderr saved to /tmp/compiletest8i5dKZ/raw.stderr
2019-07-20T14:10:53.4336572Z To update references, run this command from build directory:
2019-07-20T14:10:53.4337095Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'raw.rs'
2019-07-20T14:10:53.4337620Z error: 1 errors occurred comparing output.
2019-07-20T14:10:53.4337848Z status: exit code: 1
2019-07-20T14:10:53.4337848Z status: exit code: 1
2019-07-20T14:10:53.4338727Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/raw.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/raw.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/raw.stage-id.aux" "-A" "unused"
2019-07-20T14:10:53.4339574Z ------------------------------------------
2019-07-20T14:10:53.4339864Z 
2019-07-20T14:10:53.4340343Z ------------------------------------------
2019-07-20T14:10:53.4340670Z stderr:
---
2019-07-20T14:10:53.5614036Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:53.5614106Z +
2019-07-20T14:10:53.5614137Z 
2019-07-20T14:10:53.5614190Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:53.5614247Z Actual stderr saved to /tmp/compiletest8i5dKZ/rc.stderr
2019-07-20T14:10:53.5614382Z To update references, run this command from build directory:
2019-07-20T14:10:53.5615234Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'rc.rs'
2019-07-20T14:10:53.5615341Z error: 1 errors occurred comparing output.
2019-07-20T14:10:53.5615412Z status: exit code: 1
2019-07-20T14:10:53.5615412Z status: exit code: 1
2019-07-20T14:10:53.5616170Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rc.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/rc.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/rc.stage-id.aux" "-A" "unused"
2019-07-20T14:10:53.5616556Z ------------------------------------------
2019-07-20T14:10:53.5616596Z 
2019-07-20T14:10:53.5616887Z ------------------------------------------
2019-07-20T14:10:53.5616940Z stderr:
---
2019-07-20T14:10:53.5939072Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:53.5939150Z +
2019-07-20T14:10:53.5939178Z 
2019-07-20T14:10:53.5939224Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:53.5939290Z Actual stderr saved to /tmp/compiletest8i5dKZ/recursive_static.stderr
2019-07-20T14:10:53.5939341Z To update references, run this command from build directory:
2019-07-20T14:10:53.5939630Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'recursive_static.rs'
2019-07-20T14:10:53.5939724Z error: 1 errors occurred comparing output.
2019-07-20T14:10:53.5939768Z status: exit code: 1
2019-07-20T14:10:53.5939768Z status: exit code: 1
2019-07-20T14:10:53.5940442Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/recursive_static.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/recursive_static.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/recursive_static.stage-id.aux" "-A" "unused"
2019-07-20T14:10:53.5940854Z ------------------------------------------
2019-07-20T14:10:53.5940904Z 
2019-07-20T14:10:53.5941134Z ------------------------------------------
2019-07-20T14:10:53.5941178Z stderr:
---
2019-07-20T14:10:53.6956087Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:53.6956137Z +
2019-07-20T14:10:53.6956165Z 
2019-07-20T14:10:53.6956229Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:53.6956504Z Actual stderr saved to /tmp/compiletest8i5dKZ/ref-invalid-ptr.stderr
2019-07-20T14:10:53.6956560Z To update references, run this command from build directory:
2019-07-20T14:10:53.6956840Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'ref-invalid-ptr.rs'
2019-07-20T14:10:53.6956941Z error: 1 errors occurred comparing output.
2019-07-20T14:10:53.6956986Z status: exit code: 1
2019-07-20T14:10:53.6956986Z status: exit code: 1
2019-07-20T14:10:53.6957717Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/ref-invalid-ptr.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/ref-invalid-ptr.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletest8i5dKZ/ref-invalid-ptr.stage-id.aux" "-A" "unused"
2019-07-20T14:10:53.6958150Z ------------------------------------------
2019-07-20T14:10:53.6958182Z 
2019-07-20T14:10:53.6958387Z ------------------------------------------
2019-07-20T14:10:53.6958429Z stderr:
---
2019-07-20T14:10:53.7444139Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:53.7444189Z +
2019-07-20T14:10:53.7444237Z 
2019-07-20T14:10:53.7444282Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:53.7444332Z Actual stderr saved to /tmp/compiletest8i5dKZ/refcell.stderr
2019-07-20T14:10:53.7444379Z To update references, run this command from build directory:
2019-07-20T14:10:53.7445196Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'refcell.rs'
2019-07-20T14:10:53.7445312Z error: 1 errors occurred comparing output.
2019-07-20T14:10:53.7445380Z status: exit code: 1
2019-07-20T14:10:53.7445380Z status: exit code: 1
2019-07-20T14:10:53.7446068Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/refcell.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/refcell.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/refcell.stage-id.aux" "-A" "unused"
2019-07-20T14:10:53.7446409Z ------------------------------------------
2019-07-20T14:10:53.7446442Z 
2019-07-20T14:10:53.7446675Z ------------------------------------------
2019-07-20T14:10:53.7446737Z stderr:
---
2019-07-20T14:10:53.8371104Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:53.8371162Z +
2019-07-20T14:10:53.8371187Z 
2019-07-20T14:10:53.8371228Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:53.8371480Z Actual stderr saved to /tmp/compiletest8i5dKZ/regions-lifetime-nonfree-late-bound.stderr
2019-07-20T14:10:53.8371546Z To update references, run this command from build directory:
2019-07-20T14:10:53.8371814Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'regions-lifetime-nonfree-late-bound.rs'
2019-07-20T14:10:53.8371908Z error: 1 errors occurred comparing output.
2019-07-20T14:10:53.8371948Z status: exit code: 1
2019-07-20T14:10:53.8371948Z status: exit code: 1
2019-07-20T14:10:53.8372582Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/regions-lifetime-nonfree-late-bound.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/regions-lifetime-nonfree-late-bound.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/regions-lifetime-nonfree-late-bound.stage-id.aux" "-A" "unused"
2019-07-20T14:10:53.8372884Z ------------------------------------------
2019-07-20T14:10:53.8372915Z 
2019-07-20T14:10:53.8373139Z ------------------------------------------
2019-07-20T14:10:53.8373188Z stderr:
---
2019-07-20T14:10:53.9035765Z +
2019-07-20T14:10:53.9035798Z 
2019-07-20T14:10:53.9036233Z thread '[ui] run-pass/regions-mock-trans.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-20T14:10:53.9036339Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:53.9036655Z Actual stderr saved to /tmp/compiletest8i5dKZ/regions-mock-trans.stderr
2019-07-20T14:10:53.9036723Z To update references, run this command from build directory:
2019-07-20T14:10:53.9037067Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'regions-mock-trans.rs'
2019-07-20T14:10:53.9037164Z error: 1 errors occurred comparing output.
2019-07-20T14:10:53.9037216Z status: exit code: 1
2019-07-20T14:10:53.9037216Z status: exit code: 1
2019-07-20T14:10:53.9038012Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/regions-mock-trans.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/regions-mock-trans.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/regions-mock-trans.stage-id.aux" "-A" "unused"
2019-07-20T14:10:53.9038496Z ------------------------------------------
2019-07-20T14:10:53.9038539Z 
2019-07-20T14:10:53.9038812Z ------------------------------------------
2019-07-20T14:10:53.9038890Z stderr:
---
2019-07-20T14:10:54.0257763Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:54.0257853Z +
2019-07-20T14:10:54.0257887Z 
2019-07-20T14:10:54.0257944Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:54.0258027Z Actual stderr saved to /tmp/compiletest8i5dKZ/rfc1623.stderr
2019-07-20T14:10:54.0258090Z To update references, run this command from build directory:
2019-07-20T14:10:54.0258440Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'rfc1623.rs'
2019-07-20T14:10:54.0258565Z error: 1 errors occurred comparing output.
2019-07-20T14:10:54.0258619Z status: exit code: 1
2019-07-20T14:10:54.0258619Z status: exit code: 1
2019-07-20T14:10:54.0259397Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rfc1623.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/rfc1623.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/rfc1623.stage-id.aux" "-A" "unused"
2019-07-20T14:10:54.0259831Z ------------------------------------------
2019-07-20T14:10:54.0259877Z 
2019-07-20T14:10:54.0260203Z ------------------------------------------
2019-07-20T14:10:54.0260262Z stderr:
---
2019-07-20T14:10:54.1122223Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:54.1122283Z +
2019-07-20T14:10:54.1122310Z 
2019-07-20T14:10:54.1122354Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:54.1122625Z Actual stderr saved to /tmp/compiletest8i5dKZ/rust-lang-org.stderr
2019-07-20T14:10:54.1122676Z To update references, run this command from build directory:
2019-07-20T14:10:54.1122935Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'rust-lang-org.rs'
2019-07-20T14:10:54.1123028Z error: 1 errors occurred comparing output.
2019-07-20T14:10:54.1123071Z status: exit code: 1
2019-07-20T14:10:54.1123071Z status: exit code: 1
2019-07-20T14:10:54.1123845Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/rust-lang-org.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/rust-lang-org.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/rust-lang-org.stage-id.aux" "-A" "unused"
2019-07-20T14:10:54.1124167Z ------------------------------------------
2019-07-20T14:10:54.1124201Z 
2019-07-20T14:10:54.1124414Z ------------------------------------------
2019-07-20T14:10:54.1124458Z stderr:
---
2019-07-20T14:10:54.2077877Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:54.2077948Z +
2019-07-20T14:10:54.2077978Z 
2019-07-20T14:10:54.2078029Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:54.2078320Z Actual stderr saved to /tmp/compiletest8i5dKZ/send-is-not-static-par-for.stderr
2019-07-20T14:10:54.2078457Z To update references, run this command from build directory:
2019-07-20T14:10:54.2078757Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'send-is-not-static-par-for.rs'
2019-07-20T14:10:54.2078862Z error: 1 errors occurred comparing output.
2019-07-20T14:10:54.2078913Z status: exit code: 1
2019-07-20T14:10:54.2078913Z status: exit code: 1
2019-07-20T14:10:54.2079681Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/send-is-not-static-par-for.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/send-is-not-static-par-for.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/send-is-not-static-par-for.stage-id.aux" "-A" "unused"
2019-07-20T14:10:54.2080091Z ------------------------------------------
2019-07-20T14:10:54.2080128Z 
2019-07-20T14:10:54.2080388Z ------------------------------------------
2019-07-20T14:10:54.2080437Z stderr:
---
2019-07-20T14:10:54.2631548Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:54.2631673Z +
2019-07-20T14:10:54.2631708Z 
2019-07-20T14:10:54.2631762Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:54.2632115Z Actual stderr saved to /tmp/compiletest8i5dKZ/sendable-class.stderr
2019-07-20T14:10:54.2632177Z To update references, run this command from build directory:
2019-07-20T14:10:54.2632495Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'sendable-class.rs'
2019-07-20T14:10:54.2632611Z error: 1 errors occurred comparing output.
2019-07-20T14:10:54.2632664Z status: exit code: 1
2019-07-20T14:10:54.2632664Z status: exit code: 1
2019-07-20T14:10:54.2633447Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sendable-class.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/sendable-class.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/sendable-class.stage-id.aux" "-A" "unused"
2019-07-20T14:10:54.2633865Z ------------------------------------------
2019-07-20T14:10:54.2633925Z 
2019-07-20T14:10:54.2634211Z ------------------------------------------
2019-07-20T14:10:54.2634263Z stderr:
---
2019-07-20T14:10:54.4138783Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:54.4138838Z +
2019-07-20T14:10:54.4138869Z 
2019-07-20T14:10:54.4138939Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:54.4139289Z Actual stderr saved to /tmp/compiletest8i5dKZ/simd-intrinsic-generic-elements.stderr
2019-07-20T14:10:54.4139351Z To update references, run this command from build directory:
2019-07-20T14:10:54.4139689Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'simd-intrinsic-generic-elements.rs'
2019-07-20T14:10:54.4139781Z error: 1 errors occurred comparing output.
2019-07-20T14:10:54.4139861Z status: exit code: 1
2019-07-20T14:10:54.4139861Z status: exit code: 1
2019-07-20T14:10:54.4140653Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/simd-intrinsic-generic-elements.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/simd-intrinsic-generic-elements.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/simd-intrinsic-generic-elements.stage-id.aux" "-A" "unused"
2019-07-20T14:10:54.4141035Z ------------------------------------------
2019-07-20T14:10:54.4141073Z 
2019-07-20T14:10:54.4141329Z ------------------------------------------
2019-07-20T14:10:54.4141397Z stderr:
---
2019-07-20T14:10:54.5641632Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:54.5641691Z +
2019-07-20T14:10:54.5641723Z 
2019-07-20T14:10:54.5657303Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:54.5657387Z Actual stderr saved to /tmp/compiletest8i5dKZ/small_enum_size_bug.stderr
2019-07-20T14:10:54.5657444Z To update references, run this command from build directory:
2019-07-20T14:10:54.5658021Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'small_enum_size_bug.rs'
2019-07-20T14:10:54.5658120Z error: 1 errors occurred comparing output.
2019-07-20T14:10:54.5658189Z status: exit code: 1
2019-07-20T14:10:54.5658189Z status: exit code: 1
2019-07-20T14:10:54.5658954Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/small_enum_size_bug.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/small_enum_size_bug.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/small_enum_size_bug.stage-id.aux" "-A" "unused"
2019-07-20T14:10:54.5659332Z ------------------------------------------
2019-07-20T14:10:54.5659370Z 
2019-07-20T14:10:54.5659620Z ------------------------------------------
2019-07-20T14:10:54.5659688Z stderr:
---
2019-07-20T14:10:54.6210240Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:54.6210295Z +
2019-07-20T14:10:54.6210326Z 
2019-07-20T14:10:54.6210378Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:54.6210456Z Actual stderr saved to /tmp/compiletest8i5dKZ/slices.stderr
2019-07-20T14:10:54.6210532Z To update references, run this command from build directory:
2019-07-20T14:10:54.6210856Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'slices.rs'
2019-07-20T14:10:54.6210964Z error: 1 errors occurred comparing output.
2019-07-20T14:10:54.6211016Z status: exit code: 1
2019-07-20T14:10:54.6211016Z status: exit code: 1
2019-07-20T14:10:54.6211754Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/slices.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/slices.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/slices.stage-id.aux" "-A" "unused"
2019-07-20T14:10:54.6212139Z ------------------------------------------
2019-07-20T14:10:54.6212178Z 
2019-07-20T14:10:54.6212451Z ------------------------------------------
2019-07-20T14:10:54.6212606Z stderr:
---
2019-07-20T14:10:54.7382217Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:54.7382296Z +
2019-07-20T14:10:54.7382330Z 
2019-07-20T14:10:54.7382384Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:54.7382465Z Actual stderr saved to /tmp/compiletest8i5dKZ/specialization.stderr
2019-07-20T14:10:54.7382541Z To update references, run this command from build directory:
2019-07-20T14:10:54.7382896Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'specialization.rs'
2019-07-20T14:10:54.7383010Z error: 1 errors occurred comparing output.
2019-07-20T14:10:54.7383064Z status: exit code: 1
2019-07-20T14:10:54.7383064Z status: exit code: 1
2019-07-20T14:10:54.7383818Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/specialization.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/specialization.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/specialization.stage-id.aux" "-A" "unused"
2019-07-20T14:10:54.7384214Z ------------------------------------------
2019-07-20T14:10:54.7384266Z 
2019-07-20T14:10:54.7384682Z ------------------------------------------
2019-07-20T14:10:54.7384745Z stderr:
---
2019-07-20T14:10:54.7937227Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:54.7937282Z +
2019-07-20T14:10:54.7937313Z 
2019-07-20T14:10:54.7937405Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:54.7937738Z Actual stderr saved to /tmp/compiletest8i5dKZ/stacked-borrows/2phase.stderr
2019-07-20T14:10:54.7937802Z To update references, run this command from build directory:
2019-07-20T14:10:54.7938155Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'stacked-borrows/2phase.rs'
2019-07-20T14:10:54.7938246Z error: 1 errors occurred comparing output.
2019-07-20T14:10:54.7938308Z status: exit code: 1
2019-07-20T14:10:54.7938308Z status: exit code: 1
2019-07-20T14:10:54.7939082Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/2phase.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/stacked-borrows/2phase.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/stacked-borrows/2phase.stage-id.aux" "-A" "unused"
2019-07-20T14:10:54.7952285Z ------------------------------------------
2019-07-20T14:10:54.7952330Z 
2019-07-20T14:10:54.7952655Z ------------------------------------------
2019-07-20T14:10:54.7952708Z stderr:
---
2019-07-20T14:10:54.9534579Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:54.9534748Z +
2019-07-20T14:10:54.9534790Z 
2019-07-20T14:10:54.9534833Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:54.9535541Z Actual stderr saved to /tmp/compiletest8i5dKZ/stacked-borrows/interior_mutability.stderr
2019-07-20T14:10:54.9535628Z To update references, run this command from build directory:
2019-07-20T14:10:54.9535973Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'stacked-borrows/interior_mutability.rs'
2019-07-20T14:10:54.9536054Z error: 1 errors occurred comparing output.
2019-07-20T14:10:54.9536115Z status: exit code: 1
2019-07-20T14:10:54.9536115Z status: exit code: 1
2019-07-20T14:10:54.9537017Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/interior_mutability.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/stacked-borrows/interior_mutability.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/stacked-borrows/interior_mutability.stage-id.aux" "-A" "unused"
2019-07-20T14:10:54.9537415Z ------------------------------------------
2019-07-20T14:10:54.9537449Z 
2019-07-20T14:10:54.9537689Z ------------------------------------------
2019-07-20T14:10:54.9537734Z stderr:
---
2019-07-20T14:10:55.0611921Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:55.0611976Z +
2019-07-20T14:10:55.0612006Z 
2019-07-20T14:10:55.0612073Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:55.0612403Z Actual stderr saved to /tmp/compiletest8i5dKZ/stacked-borrows/stacked-borrows.stderr
2019-07-20T14:10:55.0612465Z To update references, run this command from build directory:
2019-07-20T14:10:55.0612815Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'stacked-borrows/stacked-borrows.rs'
2019-07-20T14:10:55.0612906Z error: 1 errors occurred comparing output.
2019-07-20T14:10:55.0612958Z status: exit code: 1
2019-07-20T14:10:55.0612958Z status: exit code: 1
2019-07-20T14:10:55.0613860Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/stacked-borrows/stacked-borrows.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/stacked-borrows/stacked-borrows.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/stacked-borrows/stacked-borrows.stage-id.aux" "-A" "unused"
2019-07-20T14:10:55.0614374Z ------------------------------------------
2019-07-20T14:10:55.0614414Z 
2019-07-20T14:10:55.0614685Z ------------------------------------------
2019-07-20T14:10:55.0614754Z stderr:
---
2019-07-20T14:10:55.1170644Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:55.1170921Z +
2019-07-20T14:10:55.1172537Z 
2019-07-20T14:10:55.1172620Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:55.1172917Z Actual stderr saved to /tmp/compiletest8i5dKZ/static_memory_modification.stderr
2019-07-20T14:10:55.1174159Z To update references, run this command from build directory:
2019-07-20T14:10:55.1174762Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'static_memory_modification.rs'
2019-07-20T14:10:55.1175152Z error: 1 errors occurred comparing output.
2019-07-20T14:10:55.1230142Z status: exit code: 1
2019-07-20T14:10:55.1230142Z status: exit code: 1
2019-07-20T14:10:55.1231173Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/static_memory_modification.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/static_memory_modification.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/static_memory_modification.stage-id.aux" "-A" "unused"
2019-07-20T14:10:55.1231842Z ------------------------------------------
2019-07-20T14:10:55.1231903Z 
2019-07-20T14:10:55.1232128Z ------------------------------------------
2019-07-20T14:10:55.1232341Z stderr:
---
2019-07-20T14:10:55.2336350Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:55.2336623Z +
2019-07-20T14:10:55.2336702Z 
2019-07-20T14:10:55.2336785Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:55.2336892Z Actual stderr saved to /tmp/compiletest8i5dKZ/static_mut.stderr
2019-07-20T14:10:55.2337140Z To update references, run this command from build directory:
2019-07-20T14:10:55.2337802Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'static_mut.rs'
2019-07-20T14:10:55.2338181Z error: 1 errors occurred comparing output.
2019-07-20T14:10:55.2338294Z status: exit code: 1
2019-07-20T14:10:55.2338294Z status: exit code: 1
2019-07-20T14:10:55.2339206Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/static_mut.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/static_mut.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/static_mut.stage-id.aux" "-A" "unused"
2019-07-20T14:10:55.2339892Z ------------------------------------------
2019-07-20T14:10:55.2340157Z 
2019-07-20T14:10:55.2340557Z ------------------------------------------
2019-07-20T14:10:55.2340984Z stderr:
---
2019-07-20T14:10:55.3156754Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:55.3157066Z +
2019-07-20T14:10:55.3157151Z 
2019-07-20T14:10:55.3157242Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:55.3157339Z Actual stderr saved to /tmp/compiletest8i5dKZ/strings.stderr
2019-07-20T14:10:55.3157645Z To update references, run this command from build directory:
2019-07-20T14:10:55.3158308Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'strings.rs'
2019-07-20T14:10:55.3158720Z error: 1 errors occurred comparing output.
2019-07-20T14:10:55.3158776Z status: exit code: 1
2019-07-20T14:10:55.3158776Z status: exit code: 1
2019-07-20T14:10:55.3159624Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/strings.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/strings.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/strings.stage-id.aux" "-A" "unused"
2019-07-20T14:10:55.3160304Z ------------------------------------------
2019-07-20T14:10:55.3160559Z 
2019-07-20T14:10:55.3161350Z ------------------------------------------
2019-07-20T14:10:55.3161667Z stderr:
---
2019-07-20T14:10:55.4492615Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:55.4492675Z +
2019-07-20T14:10:55.4492709Z 
2019-07-20T14:10:55.4492786Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:55.4492962Z Actual stderr saved to /tmp/compiletest8i5dKZ/subslice_array.stderr
2019-07-20T14:10:55.4493038Z To update references, run this command from build directory:
2019-07-20T14:10:55.4493445Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'subslice_array.rs'
2019-07-20T14:10:55.4493546Z error: 1 errors occurred comparing output.
2019-07-20T14:10:55.4493599Z status: exit code: 1
2019-07-20T14:10:55.4493599Z status: exit code: 1
2019-07-20T14:10:55.4494384Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/subslice_array.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/subslice_array.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/subslice_array.stage-id.aux" "-A" "unused"
2019-07-20T14:10:55.4494933Z ------------------------------------------
2019-07-20T14:10:55.4494992Z 
2019-07-20T14:10:55.4495295Z ------------------------------------------
2019-07-20T14:10:55.4495378Z stderr:
---
2019-07-20T14:10:55.5485263Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:55.5487042Z +
2019-07-20T14:10:55.5488171Z 
2019-07-20T14:10:55.5489443Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:55.5490545Z Actual stderr saved to /tmp/compiletest8i5dKZ/sums.stderr
2019-07-20T14:10:55.5491780Z To update references, run this command from build directory:
2019-07-20T14:10:55.5493400Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'sums.rs'
2019-07-20T14:10:55.5496154Z error: 1 errors occurred comparing output.
2019-07-20T14:10:55.5497371Z status: exit code: 1
2019-07-20T14:10:55.5497371Z status: exit code: 1
2019-07-20T14:10:55.5499419Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sums.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/sums.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/sums.stage-id.aux" "-A" "unused"
2019-07-20T14:10:55.5502165Z ------------------------------------------
2019-07-20T14:10:55.5503414Z 
2019-07-20T14:10:55.5504796Z ------------------------------------------
2019-07-20T14:10:55.5506494Z stderr:
---
2019-07-20T14:10:55.6091006Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:55.6091063Z +
2019-07-20T14:10:55.6091179Z 
2019-07-20T14:10:55.6091230Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:55.6091295Z Actual stderr saved to /tmp/compiletest8i5dKZ/sync.stderr
2019-07-20T14:10:55.6091345Z To update references, run this command from build directory:
2019-07-20T14:10:55.6091655Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'sync.rs'
2019-07-20T14:10:55.6091749Z error: 1 errors occurred comparing output.
2019-07-20T14:10:55.6091794Z status: exit code: 1
2019-07-20T14:10:55.6091794Z status: exit code: 1
2019-07-20T14:10:55.6092546Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/sync.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/sync.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/sync.stage-id.aux" "-A" "unused"
2019-07-20T14:10:55.6092997Z ------------------------------------------
2019-07-20T14:10:55.6093047Z 
2019-07-20T14:10:55.6093274Z ------------------------------------------
2019-07-20T14:10:55.6093319Z stderr:
---
2019-07-20T14:10:55.6888904Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:55.6889447Z +
2019-07-20T14:10:55.6892123Z 
2019-07-20T14:10:55.6904361Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:55.6910213Z Actual stderr saved to /tmp/compiletest8i5dKZ/tag-align-dyn-u64.stderr
2019-07-20T14:10:55.6910532Z To update references, run this command from build directory:
2019-07-20T14:10:55.6911102Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'tag-align-dyn-u64.rs'
2019-07-20T14:10:55.6911747Z error: 1 errors occurred comparing output.
2019-07-20T14:10:55.6911912Z status: exit code: 1
2019-07-20T14:10:55.6911912Z status: exit code: 1
2019-07-20T14:10:55.6912797Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tag-align-dyn-u64.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/tag-align-dyn-u64.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/tag-align-dyn-u64.stage-id.aux" "-A" "unused"
2019-07-20T14:10:55.6916614Z ------------------------------------------
2019-07-20T14:10:55.6919324Z 
2019-07-20T14:10:55.6919916Z ------------------------------------------
2019-07-20T14:10:55.6920223Z stderr:
---
2019-07-20T14:10:55.8011998Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:55.8012064Z +
2019-07-20T14:10:55.8012092Z 
2019-07-20T14:10:55.8012139Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:55.8012507Z Actual stderr saved to /tmp/compiletest8i5dKZ/thread-local.stderr
2019-07-20T14:10:55.8012575Z To update references, run this command from build directory:
2019-07-20T14:10:55.8012829Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'thread-local.rs'
2019-07-20T14:10:55.8012926Z error: 1 errors occurred comparing output.
2019-07-20T14:10:55.8012971Z status: exit code: 1
2019-07-20T14:10:55.8012971Z status: exit code: 1
2019-07-20T14:10:55.8013738Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/thread-local.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/thread-local.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/thread-local.stage-id.aux" "-A" "unused"
2019-07-20T14:10:55.8014161Z ------------------------------------------
2019-07-20T14:10:55.8014193Z 
2019-07-20T14:10:55.8014415Z ------------------------------------------
2019-07-20T14:10:55.8014457Z stderr:
---
2019-07-20T14:10:55.8210458Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:55.8210503Z +
2019-07-20T14:10:55.8226987Z 
2019-07-20T14:10:55.8227097Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:55.8227631Z Actual stderr saved to /tmp/compiletest8i5dKZ/too-large-primval-write-problem.stderr
2019-07-20T14:10:55.8227691Z To update references, run this command from build directory:
2019-07-20T14:10:55.8227987Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'too-large-primval-write-problem.rs'
2019-07-20T14:10:55.8228093Z error: 1 errors occurred comparing output.
2019-07-20T14:10:55.8228138Z status: exit code: 1
2019-07-20T14:10:55.8228138Z status: exit code: 1
2019-07-20T14:10:55.8228979Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/too-large-primval-write-problem.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/too-large-primval-write-problem.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/too-large-primval-write-problem.stage-id.aux" "-A" "unused"
2019-07-20T14:10:55.8229485Z ------------------------------------------
2019-07-20T14:10:55.8229517Z 
2019-07-20T14:10:55.8229879Z ------------------------------------------
2019-07-20T14:10:55.8229924Z stderr:
---
2019-07-20T14:10:55.9653659Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:55.9653709Z +
2019-07-20T14:10:55.9653737Z 
2019-07-20T14:10:55.9653799Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:55.9653850Z Actual stderr saved to /tmp/compiletest8i5dKZ/transmute_fat.stderr
2019-07-20T14:10:55.9653899Z To update references, run this command from build directory:
2019-07-20T14:10:55.9654209Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'transmute_fat.rs'
2019-07-20T14:10:55.9654288Z error: 1 errors occurred comparing output.
2019-07-20T14:10:55.9654446Z status: exit code: 1
2019-07-20T14:10:55.9654446Z status: exit code: 1
2019-07-20T14:10:55.9655293Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/transmute_fat.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/transmute_fat.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletest8i5dKZ/transmute_fat.stage-id.aux" "-A" "unused"
2019-07-20T14:10:55.9655610Z ------------------------------------------
2019-07-20T14:10:55.9655643Z 
2019-07-20T14:10:55.9656047Z ------------------------------------------
2019-07-20T14:10:55.9656111Z stderr:
---
2019-07-20T14:10:56.0237382Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:56.0237435Z +
2019-07-20T14:10:56.0237465Z 
2019-07-20T14:10:56.0237512Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:56.0237581Z Actual stderr saved to /tmp/compiletest8i5dKZ/traits.stderr
2019-07-20T14:10:56.0237634Z To update references, run this command from build directory:
2019-07-20T14:10:56.0237932Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'traits.rs'
2019-07-20T14:10:56.0238163Z error: 1 errors occurred comparing output.
2019-07-20T14:10:56.0238210Z status: exit code: 1
2019-07-20T14:10:56.0238210Z status: exit code: 1
2019-07-20T14:10:56.0239034Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/traits.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/traits.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/traits.stage-id.aux" "-A" "unused"
2019-07-20T14:10:56.0239338Z ------------------------------------------
2019-07-20T14:10:56.0239387Z 
2019-07-20T14:10:56.0239596Z ------------------------------------------
2019-07-20T14:10:56.0239637Z stderr:
---
2019-07-20T14:10:56.1132856Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:56.1133801Z +
2019-07-20T14:10:56.1133838Z 
2019-07-20T14:10:56.1133883Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:56.1133951Z Actual stderr saved to /tmp/compiletest8i5dKZ/trivial.stderr
2019-07-20T14:10:56.1134000Z To update references, run this command from build directory:
2019-07-20T14:10:56.1134374Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'trivial.rs'
2019-07-20T14:10:56.1134605Z error: 1 errors occurred comparing output.
2019-07-20T14:10:56.1134649Z status: exit code: 1
2019-07-20T14:10:56.1134649Z status: exit code: 1
2019-07-20T14:10:56.1135330Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/trivial.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/trivial.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/trivial.stage-id.aux" "-A" "unused"
2019-07-20T14:10:56.1136098Z ------------------------------------------
2019-07-20T14:10:56.1136142Z 
2019-07-20T14:10:56.1136378Z ------------------------------------------
2019-07-20T14:10:56.1136423Z stderr:
---
2019-07-20T14:10:56.1689273Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:56.1689322Z +
2019-07-20T14:10:56.1689350Z 
2019-07-20T14:10:56.1689397Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:56.1689695Z Actual stderr saved to /tmp/compiletest8i5dKZ/try-operator-custom.stderr
2019-07-20T14:10:56.1689749Z To update references, run this command from build directory:
2019-07-20T14:10:56.1690166Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'try-operator-custom.rs'
2019-07-20T14:10:56.1690305Z error: 1 errors occurred comparing output.
2019-07-20T14:10:56.1690351Z status: exit code: 1
2019-07-20T14:10:56.1690351Z status: exit code: 1
2019-07-20T14:10:56.1691058Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/try-operator-custom.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/try-operator-custom.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/try-operator-custom.stage-id.aux" "-A" "unused"
2019-07-20T14:10:56.1691390Z ------------------------------------------
2019-07-20T14:10:56.1691450Z 
2019-07-20T14:10:56.1691696Z ------------------------------------------
2019-07-20T14:10:56.1691751Z stderr:
---
2019-07-20T14:10:56.2894980Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:56.2895048Z +
2019-07-20T14:10:56.2895077Z 
2019-07-20T14:10:56.2895124Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:56.2895307Z Actual stderr saved to /tmp/compiletest8i5dKZ/tuple_like_enum_variant_constructor.stderr
2019-07-20T14:10:56.2895359Z To update references, run this command from build directory:
2019-07-20T14:10:56.2895712Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'tuple_like_enum_variant_constructor.rs'
2019-07-20T14:10:56.2896307Z error: 1 errors occurred comparing output.
2019-07-20T14:10:56.2896357Z status: exit code: 1
2019-07-20T14:10:56.2896357Z status: exit code: 1
2019-07-20T14:10:56.2897133Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/tuple_like_enum_variant_constructor.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/tuple_like_enum_variant_constructor.stage-id.aux" "-A" "unused"
2019-07-20T14:10:56.2897503Z ------------------------------------------
2019-07-20T14:10:56.2897554Z 
2019-07-20T14:10:56.2897786Z ------------------------------------------
2019-07-20T14:10:56.2897830Z stderr:
---
2019-07-20T14:10:56.3534332Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:56.3534401Z +
2019-07-20T14:10:56.3534430Z 
2019-07-20T14:10:56.3534477Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:56.3534534Z Actual stderr saved to /tmp/compiletest8i5dKZ/tuple_like_enum_variant_constructor_pointer_opt.stderr
2019-07-20T14:10:56.3534603Z To update references, run this command from build directory:
2019-07-20T14:10:56.3535054Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'tuple_like_enum_variant_constructor_pointer_opt.rs'
2019-07-20T14:10:56.3535269Z error: 1 errors occurred comparing output.
2019-07-20T14:10:56.3535314Z status: exit code: 1
2019-07-20T14:10:56.3535314Z status: exit code: 1
2019-07-20T14:10:56.3536552Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor_pointer_opt.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/tuple_like_enum_variant_constructor_pointer_opt.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/tuple_like_enum_variant_constructor_pointer_opt.stage-id.aux" "-A" "unused"
2019-07-20T14:10:56.3536962Z ------------------------------------------
2019-07-20T14:10:56.3536997Z 
2019-07-20T14:10:56.3537250Z ------------------------------------------
2019-07-20T14:10:56.3537295Z stderr:
---
2019-07-20T14:10:56.5012296Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:56.5012341Z +
2019-07-20T14:10:56.5012366Z 
2019-07-20T14:10:56.5012424Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:56.5012476Z Actual stderr saved to /tmp/compiletest8i5dKZ/tuple_like_enum_variant_constructor_struct_pointer_opt.stderr
2019-07-20T14:10:56.5012525Z To update references, run this command from build directory:
2019-07-20T14:10:56.5012861Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'tuple_like_enum_variant_constructor_struct_pointer_opt.rs'
2019-07-20T14:10:56.5012937Z error: 1 errors occurred comparing output.
2019-07-20T14:10:56.5012992Z status: exit code: 1
2019-07-20T14:10:56.5012992Z status: exit code: 1
2019-07-20T14:10:56.5013696Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_enum_variant_constructor_struct_pointer_opt.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/tuple_like_enum_variant_constructor_struct_pointer_opt.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/tuple_like_enum_variant_constructor_struct_pointer_opt.stage-id.aux" "-A" "unused"
2019-07-20T14:10:56.5014014Z ------------------------------------------
2019-07-20T14:10:56.5014045Z 
2019-07-20T14:10:56.5014327Z ------------------------------------------
2019-07-20T14:10:56.5014389Z stderr:
---
2019-07-20T14:10:56.5154180Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:56.5154234Z +
2019-07-20T14:10:56.5154258Z 
2019-07-20T14:10:56.5154323Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:56.5154372Z Actual stderr saved to /tmp/compiletest8i5dKZ/tuple_like_struct_constructor.stderr
2019-07-20T14:10:56.5154419Z To update references, run this command from build directory:
2019-07-20T14:10:56.5154714Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'tuple_like_struct_constructor.rs'
2019-07-20T14:10:56.5154787Z error: 1 errors occurred comparing output.
2019-07-20T14:10:56.5154828Z status: exit code: 1
2019-07-20T14:10:56.5154828Z status: exit code: 1
2019-07-20T14:10:56.5155468Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/tuple_like_struct_constructor.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/tuple_like_struct_constructor.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/tuple_like_struct_constructor.stage-id.aux" "-A" "unused"
2019-07-20T14:10:56.5156385Z ------------------------------------------
2019-07-20T14:10:56.5156431Z 
2019-07-20T14:10:56.5156668Z ------------------------------------------
2019-07-20T14:10:56.5156735Z stderr:
---
2019-07-20T14:10:56.6885612Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:56.6885689Z +
2019-07-20T14:10:56.6885718Z 
2019-07-20T14:10:56.6885770Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:56.6886594Z Actual stderr saved to /tmp/compiletest8i5dKZ/union-overwrite.stderr
2019-07-20T14:10:56.6886675Z To update references, run this command from build directory:
2019-07-20T14:10:56.6886953Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'union-overwrite.rs'
2019-07-20T14:10:56.6887051Z error: 1 errors occurred comparing output.
2019-07-20T14:10:56.6887097Z status: exit code: 1
2019-07-20T14:10:56.6887097Z status: exit code: 1
2019-07-20T14:10:56.6887760Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/union-overwrite.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/union-overwrite.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/union-overwrite.stage-id.aux" "-A" "unused"
2019-07-20T14:10:56.6888100Z ------------------------------------------
2019-07-20T14:10:56.6888134Z 
2019-07-20T14:10:56.6888370Z ------------------------------------------
2019-07-20T14:10:56.6888414Z stderr:
---
2019-07-20T14:10:56.8575533Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:56.8575582Z +
2019-07-20T14:10:56.8575609Z 
2019-07-20T14:10:56.8575674Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:56.8575722Z Actual stderr saved to /tmp/compiletest8i5dKZ/union.stderr
2019-07-20T14:10:56.8575769Z To update references, run this command from build directory:
2019-07-20T14:10:56.8576497Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'union.rs'
2019-07-20T14:10:56.8576586Z error: 1 errors occurred comparing output.
2019-07-20T14:10:56.8576694Z status: exit code: 1
2019-07-20T14:10:56.8576694Z status: exit code: 1
2019-07-20T14:10:56.8577382Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/union.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/union.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/union.stage-id.aux" "-A" "unused"
2019-07-20T14:10:56.8577720Z ------------------------------------------
2019-07-20T14:10:56.8577752Z 
2019-07-20T14:10:56.8577976Z ------------------------------------------
2019-07-20T14:10:56.8578036Z stderr:
---
2019-07-20T14:10:56.9212649Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:56.9212697Z +
2019-07-20T14:10:56.9212722Z 
2019-07-20T14:10:56.9212764Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:56.9212826Z Actual stderr saved to /tmp/compiletest8i5dKZ/u128.stderr
2019-07-20T14:10:56.9212871Z To update references, run this command from build directory:
2019-07-20T14:10:56.9213140Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'u128.rs'
2019-07-20T14:10:56.9213227Z error: 1 errors occurred comparing output.
2019-07-20T14:10:56.9213267Z status: exit code: 1
2019-07-20T14:10:56.9213267Z status: exit code: 1
2019-07-20T14:10:56.9213857Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/u128.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/u128.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/u128.stage-id.aux" "-A" "unused"
2019-07-20T14:10:56.9214163Z ------------------------------------------
2019-07-20T14:10:56.9214210Z 
2019-07-20T14:10:56.9214426Z ------------------------------------------
2019-07-20T14:10:56.9214466Z stderr:
---
2019-07-20T14:10:57.0136812Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:57.0136861Z +
2019-07-20T14:10:57.0136889Z 
2019-07-20T14:10:57.0136934Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:57.0137001Z Actual stderr saved to /tmp/compiletest8i5dKZ/unops.stderr
2019-07-20T14:10:57.0137049Z To update references, run this command from build directory:
2019-07-20T14:10:57.0137315Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'unops.rs'
2019-07-20T14:10:57.0137405Z error: 1 errors occurred comparing output.
2019-07-20T14:10:57.0137449Z status: exit code: 1
2019-07-20T14:10:57.0137449Z status: exit code: 1
2019-07-20T14:10:57.0138098Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/unops.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/unops.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/unops.stage-id.aux" "-A" "unused"
2019-07-20T14:10:57.0138414Z ------------------------------------------
2019-07-20T14:10:57.0138463Z 
2019-07-20T14:10:57.0138680Z ------------------------------------------
2019-07-20T14:10:57.0138725Z stderr:
---
2019-07-20T14:10:57.1004665Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:57.1004715Z +
2019-07-20T14:10:57.1004742Z 
2019-07-20T14:10:57.1004805Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:57.1005091Z Actual stderr saved to /tmp/compiletest8i5dKZ/unsized-tuple-impls.stderr
2019-07-20T14:10:57.1005146Z To update references, run this command from build directory:
2019-07-20T14:10:57.1005568Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'unsized-tuple-impls.rs'
2019-07-20T14:10:57.1005767Z error: 1 errors occurred comparing output.
2019-07-20T14:10:57.1005807Z status: exit code: 1
2019-07-20T14:10:57.1005807Z status: exit code: 1
2019-07-20T14:10:57.1007280Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/unsized-tuple-impls.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/unsized-tuple-impls.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/unsized-tuple-impls.stage-id.aux" "-A" "unused"
2019-07-20T14:10:57.1007665Z ------------------------------------------
2019-07-20T14:10:57.1007699Z 
2019-07-20T14:10:57.1007919Z ------------------------------------------
2019-07-20T14:10:57.1007979Z stderr:
---
2019-07-20T14:10:57.1653524Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:57.1653576Z +
2019-07-20T14:10:57.1653603Z 
2019-07-20T14:10:57.1653666Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:57.1653716Z Actual stderr saved to /tmp/compiletest8i5dKZ/validation_lifetime_resolution.stderr
2019-07-20T14:10:57.1653763Z To update references, run this command from build directory:
2019-07-20T14:10:57.1654100Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'validation_lifetime_resolution.rs'
2019-07-20T14:10:57.1654174Z error: 1 errors occurred comparing output.
2019-07-20T14:10:57.1654215Z status: exit code: 1
2019-07-20T14:10:57.1654215Z status: exit code: 1
2019-07-20T14:10:57.1654883Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/validation_lifetime_resolution.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/validation_lifetime_resolution.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/validation_lifetime_resolution.stage-id.aux" "-A" "unused"
2019-07-20T14:10:57.1655204Z ------------------------------------------
2019-07-20T14:10:57.1655236Z 
2019-07-20T14:10:57.1655444Z ------------------------------------------
2019-07-20T14:10:57.1655501Z stderr:
---
2019-07-20T14:10:57.2893405Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:57.2894250Z +
2019-07-20T14:10:57.2894298Z 
2019-07-20T14:10:57.2894341Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:57.2894670Z Actual stderr saved to /tmp/compiletest8i5dKZ/vec-matching-fold.stderr
2019-07-20T14:10:57.2894737Z To update references, run this command from build directory:
2019-07-20T14:10:57.2894994Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'vec-matching-fold.rs'
2019-07-20T14:10:57.2895085Z error: 1 errors occurred comparing output.
2019-07-20T14:10:57.2895257Z status: exit code: 1
2019-07-20T14:10:57.2895257Z status: exit code: 1
2019-07-20T14:10:57.2895876Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vec-matching-fold.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/vec-matching-fold.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/vec-matching-fold.stage-id.aux" "-A" "unused"
2019-07-20T14:10:57.2896978Z ------------------------------------------
2019-07-20T14:10:57.2897020Z 
2019-07-20T14:10:57.2897268Z ------------------------------------------
2019-07-20T14:10:57.2897314Z stderr:
---
2019-07-20T14:10:57.3691796Z -Iter([], [])
2019-07-20T14:10:57.3695402Z -
2019-07-20T14:10:57.3700126Z 
2019-07-20T14:10:57.3700218Z The actual stdout differed from the expected stdout.
2019-07-20T14:10:57.3704233Z Actual stdout saved to /tmp/compiletest8i5dKZ/vecdeque.stdout
2019-07-20T14:10:57.3709841Z error[E0080]: Miri evaluation error: attempted to do invalid arithmetic on pointers that would leak base addresses, e.g., comparing pointers into different allocations
2019-07-20T14:10:57.3710243Z     --> /checkout/src/libcore/intrinsics.rs:1340:19
2019-07-20T14:10:57.3710317Z      |
2019-07-20T14:10:57.3710362Z 1340 |     let diff = if src_usize > dst_usize {
---
2019-07-20T14:10:57.3728532Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:57.3728581Z +
2019-07-20T14:10:57.3728625Z 
2019-07-20T14:10:57.3728671Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:57.3728723Z Actual stderr saved to /tmp/compiletest8i5dKZ/vecdeque.stderr
2019-07-20T14:10:57.3728791Z To update references, run this command from build directory:
2019-07-20T14:10:57.3729078Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'vecdeque.rs'
2019-07-20T14:10:57.3729159Z error: 2 errors occurred comparing output.
2019-07-20T14:10:57.3729345Z status: exit code: 1
2019-07-20T14:10:57.3729345Z status: exit code: 1
2019-07-20T14:10:57.3729994Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vecdeque.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/vecdeque.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/vecdeque.stage-id.aux" "-A" "unused"
2019-07-20T14:10:57.3730327Z ------------------------------------------
2019-07-20T14:10:57.3730356Z 
2019-07-20T14:10:57.3730549Z ------------------------------------------
2019-07-20T14:10:57.3730607Z stderr:
---
2019-07-20T14:10:57.5415155Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:57.5415203Z +
2019-07-20T14:10:57.5415230Z 
2019-07-20T14:10:57.5415297Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:57.5415347Z Actual stderr saved to /tmp/compiletest8i5dKZ/volatile.stderr
2019-07-20T14:10:57.5415396Z To update references, run this command from build directory:
2019-07-20T14:10:57.5415710Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'volatile.rs'
2019-07-20T14:10:57.5415787Z error: 1 errors occurred comparing output.
2019-07-20T14:10:57.5415831Z status: exit code: 1
2019-07-20T14:10:57.5415831Z status: exit code: 1
2019-07-20T14:10:57.5417098Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/volatile.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/volatile.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/volatile.stage-id.aux" "-A" "unused"
2019-07-20T14:10:57.5417544Z ------------------------------------------
2019-07-20T14:10:57.5417580Z 
2019-07-20T14:10:57.5417813Z ------------------------------------------
2019-07-20T14:10:57.5417876Z stderr:
---
2019-07-20T14:10:57.5799430Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:57.5799495Z +
2019-07-20T14:10:57.5799523Z 
2019-07-20T14:10:57.5799569Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:57.5799635Z Actual stderr saved to /tmp/compiletest8i5dKZ/vecs.stderr
2019-07-20T14:10:57.5799685Z To update references, run this command from build directory:
2019-07-20T14:10:57.5799962Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'vecs.rs'
2019-07-20T14:10:57.5800058Z error: 1 errors occurred comparing output.
2019-07-20T14:10:57.5800201Z status: exit code: 1
2019-07-20T14:10:57.5800201Z status: exit code: 1
2019-07-20T14:10:57.5800861Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/vecs.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/vecs.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/vecs.stage-id.aux" "-A" "unused"
2019-07-20T14:10:57.5801201Z ------------------------------------------
2019-07-20T14:10:57.5801251Z 
2019-07-20T14:10:57.5801488Z ------------------------------------------
2019-07-20T14:10:57.5801534Z stderr:
---
2019-07-20T14:10:57.7135781Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:57.7135828Z +
2019-07-20T14:10:57.7135873Z 
2019-07-20T14:10:57.7135917Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:57.7136698Z Actual stderr saved to /tmp/compiletest8i5dKZ/without-validation.stderr
2019-07-20T14:10:57.7136784Z To update references, run this command from build directory:
2019-07-20T14:10:57.7137104Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'without-validation.rs'
2019-07-20T14:10:57.7137353Z error: 1 errors occurred comparing output.
2019-07-20T14:10:57.7137427Z status: exit code: 1
2019-07-20T14:10:57.7137427Z status: exit code: 1
2019-07-20T14:10:57.7138142Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/without-validation.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/without-validation.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-Zmiri-disable-validation" "-L" "/tmp/compiletest8i5dKZ/without-validation.stage-id.aux" "-A" "unused"
2019-07-20T14:10:57.7138476Z ------------------------------------------
2019-07-20T14:10:57.7138510Z 
2019-07-20T14:10:57.7138744Z ------------------------------------------
2019-07-20T14:10:57.7138789Z stderr:
---
2019-07-20T14:10:57.7779724Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:57.7779789Z +
2019-07-20T14:10:57.7779817Z 
2019-07-20T14:10:57.7779864Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:57.7780261Z Actual stderr saved to /tmp/compiletest8i5dKZ/write-bytes.stderr
2019-07-20T14:10:57.7780324Z To update references, run this command from build directory:
2019-07-20T14:10:57.7780630Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'write-bytes.rs'
2019-07-20T14:10:57.7780728Z error: 1 errors occurred comparing output.
2019-07-20T14:10:57.7780773Z status: exit code: 1
2019-07-20T14:10:57.7780773Z status: exit code: 1
2019-07-20T14:10:57.7781422Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/write-bytes.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/write-bytes.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/write-bytes.stage-id.aux" "-A" "unused"
2019-07-20T14:10:57.7781877Z ------------------------------------------
2019-07-20T14:10:57.7781938Z 
2019-07-20T14:10:57.7782192Z ------------------------------------------
2019-07-20T14:10:57.7782238Z stderr:
---
2019-07-20T14:10:58.5272734Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:58.5272946Z +
2019-07-20T14:10:58.5273003Z 
2019-07-20T14:10:58.5273047Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:58.5273395Z Actual stderr saved to /tmp/compiletest8i5dKZ/zero-sized-binary-heap-push.stderr
2019-07-20T14:10:58.5273789Z To update references, run this command from build directory:
2019-07-20T14:10:58.5274286Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'zero-sized-binary-heap-push.rs'
2019-07-20T14:10:58.5274559Z error: 1 errors occurred comparing output.
2019-07-20T14:10:58.5274601Z status: exit code: 1
2019-07-20T14:10:58.5274601Z status: exit code: 1
2019-07-20T14:10:58.5275575Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zero-sized-binary-heap-push.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/zero-sized-binary-heap-push.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/zero-sized-binary-heap-push.stage-id.aux" "-A" "unused"
2019-07-20T14:10:58.5276282Z ------------------------------------------
2019-07-20T14:10:58.5276791Z 
2019-07-20T14:10:58.5278672Z ------------------------------------------
2019-07-20T14:10:58.5278759Z stderr:
---
2019-07-20T14:10:58.5295150Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:58.5295217Z +
2019-07-20T14:10:58.5295242Z 
2019-07-20T14:10:58.5295383Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:58.5295436Z Actual stderr saved to /tmp/compiletest8i5dKZ/zst.stderr
2019-07-20T14:10:58.5295498Z To update references, run this command from build directory:
2019-07-20T14:10:58.5295765Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'zst.rs'
2019-07-20T14:10:58.5296021Z error: 1 errors occurred comparing output.
2019-07-20T14:10:58.5296093Z status: exit code: 1
2019-07-20T14:10:58.5296093Z status: exit code: 1
2019-07-20T14:10:58.5297270Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/zst.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/zst.stage-id.aux" "-A" "unused"
2019-07-20T14:10:58.5298042Z ------------------------------------------
2019-07-20T14:10:58.5298243Z 
2019-07-20T14:10:58.5298552Z ------------------------------------------
2019-07-20T14:10:58.5298598Z stderr:
---
2019-07-20T14:10:58.5314741Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:58.5314949Z +
2019-07-20T14:10:58.5315110Z 
2019-07-20T14:10:58.5315398Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:58.5315495Z Actual stderr saved to /tmp/compiletest8i5dKZ/zst_box.stderr
2019-07-20T14:10:58.5315565Z To update references, run this command from build directory:
2019-07-20T14:10:58.5315878Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'zst_box.rs'
2019-07-20T14:10:58.5316146Z error: 1 errors occurred comparing output.
2019-07-20T14:10:58.5316188Z status: exit code: 1
2019-07-20T14:10:58.5316188Z status: exit code: 1
2019-07-20T14:10:58.5317714Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst_box.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/zst_box.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/zst_box.stage-id.aux" "-A" "unused"
2019-07-20T14:10:58.5318518Z ------------------------------------------
2019-07-20T14:10:58.5318690Z 
2019-07-20T14:10:58.5319005Z ------------------------------------------
2019-07-20T14:10:58.5319073Z stderr:
---
2019-07-20T14:10:58.5335743Z +For more information about this error, try `rustc --explain E0080`.
2019-07-20T14:10:58.5335787Z +
2019-07-20T14:10:58.5335947Z 
2019-07-20T14:10:58.5336044Z The actual stderr differed from the expected stderr.
2019-07-20T14:10:58.5336093Z Actual stderr saved to /tmp/compiletest8i5dKZ/zst_variant_drop.stderr
2019-07-20T14:10:58.5336732Z To update references, run this command from build directory:
2019-07-20T14:10:58.5337156Z tests/run-pass/update-references.sh '/tmp/compiletest8i5dKZ' 'zst_variant_drop.rs'
2019-07-20T14:10:58.5337422Z error: 1 errors occurred comparing output.
2019-07-20T14:10:58.5337488Z status: exit code: 1
2019-07-20T14:10:58.5337488Z status: exit code: 1
2019-07-20T14:10:58.5338507Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/zst_variant_drop.rs" "-L" "/tmp/compiletest8i5dKZ" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest8i5dKZ/zst_variant_drop.stage-id" "-Zmir-opt-level=3" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletest8i5dKZ/zst_variant_drop.stage-id.aux" "-A" "unused"
2019-07-20T14:10:58.5339099Z ------------------------------------------
2019-07-20T14:10:58.5339267Z 
2019-07-20T14:10:58.5339612Z ------------------------------------------
2019-07-20T14:10:58.5339661Z stderr:
---
2019-07-20T14:10:58.5485320Z Verifying status of clippy-driver...
2019-07-20T14:10:58.5485497Z Verifying status of miri...
2019-07-20T14:10:58.5485960Z This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
2019-07-20T14:10:58.5486151Z 
2019-07-20T14:10:58.5494045Z ⚠️ We detected that this PR updated 'miri', but its tests failed.
2019-07-20T14:10:58.5494378Z 
2019-07-20T14:10:58.5494988Z If you do intend to update 'miri', please check the error messages above and
2019-07-20T14:10:58.5495045Z commit another update.
2019-07-20T14:10:58.5495074Z 
2019-07-20T14:10:58.5495559Z If you do NOT intend to update 'miri', please ensure you did not accidentally
2019-07-20T14:10:58.5495822Z change the submodule at 'src/tools/miri'. You may ask your reviewer for the
2019-07-20T14:10:58.5495870Z proper steps.
2019-07-20T14:10:58.7297451Z ##[error]Bash exited with code '3'.
2019-07-20T14:10:58.7333955Z ##[section]Starting: Checkout
2019-07-20T14:10:58.7335715Z ==============================================================================
2019-07-20T14:10:58.7335922Z Task         : Get sources
2019-07-20T14:10:58.7335967Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
