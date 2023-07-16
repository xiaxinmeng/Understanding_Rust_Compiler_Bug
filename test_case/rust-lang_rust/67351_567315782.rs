plain
2019-12-19T03:29:07.5599510Z tests/run-pass/update-references.sh '/tmp/compiletestQ3430k' 'async-fn.rs'
2019-12-19T03:29:07.5601900Z 
2019-12-19T03:29:07.5601999Z error: 1 errors occurred comparing output.
2019-12-19T03:29:07.5607572Z status: exit code: 1
2019-12-19T03:29:07.5609170Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/async-fn.rs" "-L" "/tmp/compiletestQ3430k" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestQ3430k/async-fn.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestQ3430k/async-fn.stage-id.aux" "-A" "unused"
2019-12-19T03:29:07.5609923Z ------------------------------------------
2019-12-19T03:29:07.5609974Z 
2019-12-19T03:29:07.5610224Z ------------------------------------------
2019-12-19T03:29:07.5610287Z stderr:
---
2019-12-19T03:29:12.9126445Z tests/run-pass/update-references.sh '/tmp/compiletestQ3430k' 'generator.rs'
2019-12-19T03:29:12.9126514Z 
2019-12-19T03:29:12.9126591Z error: 1 errors occurred comparing output.
2019-12-19T03:29:12.9126651Z status: exit code: 1
2019-12-19T03:29:12.9127442Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/generator.rs" "-L" "/tmp/compiletestQ3430k" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestQ3430k/generator.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestQ3430k/generator.stage-id.aux" "-A" "unused"
2019-12-19T03:29:12.9127870Z ------------------------------------------
2019-12-19T03:29:12.9127930Z 
2019-12-19T03:29:12.9128152Z ------------------------------------------
2019-12-19T03:29:12.9128233Z stderr:
---
2019-12-19T03:29:17.7225301Z tests/run-pass/update-references.sh '/tmp/compiletestQ3430k' 'loop-break-value.rs'
2019-12-19T03:29:17.7225354Z 
2019-12-19T03:29:17.7225425Z error: 1 errors occurred comparing output.
2019-12-19T03:29:17.7225485Z status: exit code: 1
2019-12-19T03:29:17.7226219Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/loop-break-value.rs" "-L" "/tmp/compiletestQ3430k" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestQ3430k/loop-break-value.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestQ3430k/loop-break-value.stage-id.aux" "-A" "unused"
2019-12-19T03:29:17.7226642Z ------------------------------------------
2019-12-19T03:29:17.7226707Z 
2019-12-19T03:29:17.7226917Z ------------------------------------------
2019-12-19T03:29:17.7226996Z stderr:
---
2019-12-19T03:29:19.3841630Z tests/run-pass/update-references.sh '/tmp/compiletestQ3430k' 'panic/catch_panic.rs'
2019-12-19T03:29:19.3841677Z 
2019-12-19T03:29:19.3841741Z error: 1 errors occurred comparing output.
2019-12-19T03:29:19.3841794Z status: exit code: 1
2019-12-19T03:29:19.3842447Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/panic/catch_panic.rs" "-L" "/tmp/compiletestQ3430k" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestQ3430k/panic/catch_panic.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestQ3430k/panic/catch_panic.stage-id.aux" "-A" "unused"
2019-12-19T03:29:19.3843015Z ------------------------------------------
2019-12-19T03:29:19.3843055Z 
2019-12-19T03:29:19.3843273Z ------------------------------------------
2019-12-19T03:29:19.3862447Z stderr:
---
2019-12-19T03:29:27.2452542Z Verifying status of miri...
2019-12-19T03:29:27.2452742Z Verifying status of embedded-book...
2019-12-19T03:29:27.2452921Z Verifying status of rustc-guide...
2019-12-19T03:29:27.2494753Z Cloning into 'rust-toolstate'...
2019-12-19T03:29:27.8564174Z thread 'main' panicked at 'not yet implemented', src/bootstrap/toolstate.rs:387:13
2019-12-19T03:29:27.8604799Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test check-tools
2019-12-19T03:29:27.8604938Z Build completed unsuccessfully in 0:00:01
2019-12-19T03:29:27.8649063Z == clock drift check ==
2019-12-19T03:29:27.8668032Z   local time: Thu Dec 19 03:29:27 UTC 2019
2019-12-19T03:29:27.8668032Z   local time: Thu Dec 19 03:29:27 UTC 2019
2019-12-19T03:29:28.1357525Z   network time: Thu, 19 Dec 2019 03:29:28 GMT
2019-12-19T03:29:28.1361125Z == end clock drift check ==
2019-12-19T03:29:28.9972856Z 
2019-12-19T03:29:29.0063170Z ##[error]Bash exited with code '1'.
2019-12-19T03:29:29.0101872Z ##[section]Starting: Checkout
2019-12-19T03:29:29.0103792Z ==============================================================================
2019-12-19T03:29:29.0103862Z Task         : Get sources
2019-12-19T03:29:29.0103942Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
