plain
2019-12-18T19:09:21.6488565Z  123 │ declared [`UniversalRegionRelations`].
2019-12-18T19:09:21.6488886Z      │          ^ Server responded with 404 Not Found
2019-12-18T19:09:21.6489160Z      │
2019-12-18T19:09:21.6489206Z 
2019-12-18T19:09:21.6489345Z error: The server responded with 503 Service Unavailable for "https://www.usenix.org/legacy/events/hotos03/tech/full_papers/candea/candea.pdf"
2019-12-18T19:09:21.6489741Z     ┌── appendix/bibliography.md:40:3 ───
2019-12-18T19:09:21.6490210Z     │
2019-12-18T19:09:21.6490210Z     │
2019-12-18T19:09:21.6490647Z  40 │ * [Crash-only software](https://www.usenix.org/legacy/events/hotos03/tech/full_papers/candea/candea.pdf)
2019-12-18T19:09:21.6491144Z     │   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Server responded with 503 Service Unavailable
2019-12-18T19:09:21.6491502Z 
2019-12-18T19:09:21.6491502Z 
2019-12-18T19:09:21.6491946Z error: The server responded with 503 Service Unavailable for "https://www.usenix.org/conference/hotpar12/parallel-closures-new-twist-old-idea"
2019-12-18T19:09:21.6492356Z     ┌── appendix/bibliography.md:50:3 ───
2019-12-18T19:09:21.6492629Z     │
2019-12-18T19:09:21.6492629Z     │
2019-12-18T19:09:21.6492946Z  50 │   * [Parallel closures: a new twist on an old
2019-12-18T19:09:21.6493224Z     │ ╭───^
2019-12-18T19:09:21.6493604Z  51 │ │   idea](https://www.usenix.org/conference/hotpar12/parallel-closures-new-twist-old-idea)
2019-12-18T19:09:21.6510691Z     │ ╰────────────────────────────────────────────────────────────────────────────────────────^ Server responded with 503 Service Unavailable
2019-12-18T19:09:21.6511127Z 
2019-12-18T19:09:21.6511840Z Error: One or more incorrect links
2019-12-18T19:09:21.6512009Z 
2019-12-18T19:09:21.6512051Z 
---
2019-12-18T20:01:59.7808817Z +For more information about this error, try `rustc --explain E0658`.
2019-12-18T20:01:59.7808926Z +
2019-12-18T20:01:59.7808968Z 
2019-12-18T20:01:59.7809100Z The actual stderr differed from the expected stderr.
2019-12-18T20:01:59.7809451Z Actual stderr saved to /tmp/compiletestqIwf9c/async-fn.stderr
2019-12-18T20:01:59.7809570Z To update references, run this command from build directory:
2019-12-18T20:01:59.7809902Z tests/run-pass/update-references.sh '/tmp/compiletestqIwf9c' 'async-fn.rs'
2019-12-18T20:01:59.7810074Z error: 1 errors occurred comparing output.
2019-12-18T20:01:59.7810171Z status: exit code: 1
2019-12-18T20:01:59.7810171Z status: exit code: 1
2019-12-18T20:01:59.7811077Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/async-fn.rs" "-L" "/tmp/compiletestqIwf9c" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestqIwf9c/async-fn.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestqIwf9c/async-fn.stage-id.aux" "-A" "unused"
2019-12-18T20:01:59.7811697Z ------------------------------------------
2019-12-18T20:01:59.7811760Z 
2019-12-18T20:01:59.7812056Z ------------------------------------------
2019-12-18T20:01:59.7812157Z stderr:
---
2019-12-18T20:02:05.8553238Z +For more information about this error, try `rustc --explain E0658`.
2019-12-18T20:02:05.8553335Z +
2019-12-18T20:02:05.8553662Z 
2019-12-18T20:02:05.8553768Z The actual stderr differed from the expected stderr.
2019-12-18T20:02:05.8554525Z Actual stderr saved to /tmp/compiletestqIwf9c/generator.stderr
2019-12-18T20:02:05.8554618Z To update references, run this command from build directory:
2019-12-18T20:02:05.8555389Z tests/run-pass/update-references.sh '/tmp/compiletestqIwf9c' 'generator.rs'
2019-12-18T20:02:05.8555588Z error: 1 errors occurred comparing output.
2019-12-18T20:02:05.8592833Z status: exit code: 1
2019-12-18T20:02:05.8592833Z status: exit code: 1
2019-12-18T20:02:05.8594421Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/generator.rs" "-L" "/tmp/compiletestqIwf9c" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestqIwf9c/generator.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestqIwf9c/generator.stage-id.aux" "-A" "unused"
2019-12-18T20:02:05.8596532Z ------------------------------------------
2019-12-18T20:02:05.8596729Z 
2019-12-18T20:02:05.8597047Z ------------------------------------------
2019-12-18T20:02:05.8597165Z stderr:
---
2019-12-18T20:02:11.4363219Z +For more information about this error, try `rustc --explain E0658`.
2019-12-18T20:02:11.4363314Z +
2019-12-18T20:02:11.4363351Z 
2019-12-18T20:02:11.4363447Z The actual stderr differed from the expected stderr.
2019-12-18T20:02:11.4363745Z Actual stderr saved to /tmp/compiletestqIwf9c/loop-break-value.stderr
2019-12-18T20:02:11.4363854Z To update references, run this command from build directory:
2019-12-18T20:02:11.4364158Z tests/run-pass/update-references.sh '/tmp/compiletestqIwf9c' 'loop-break-value.rs'
2019-12-18T20:02:11.4364311Z error: 1 errors occurred comparing output.
2019-12-18T20:02:11.4364397Z status: exit code: 1
2019-12-18T20:02:11.4364397Z status: exit code: 1
2019-12-18T20:02:11.4365243Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/loop-break-value.rs" "-L" "/tmp/compiletestqIwf9c" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestqIwf9c/loop-break-value.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestqIwf9c/loop-break-value.stage-id.aux" "-A" "unused"
2019-12-18T20:02:11.4365761Z ------------------------------------------
2019-12-18T20:02:11.4365829Z 
2019-12-18T20:02:11.4366074Z ------------------------------------------
2019-12-18T20:02:11.4366153Z stderr:
---
2019-12-18T20:02:13.4106327Z +For more information about this error, try `rustc --explain E0658`.
2019-12-18T20:02:13.4106411Z  
2019-12-18T20:02:13.4106458Z 
2019-12-18T20:02:13.4106545Z The actual stderr differed from the expected stderr.
2019-12-18T20:02:13.4106633Z Actual stderr saved to /tmp/compiletestqIwf9c/panic/catch_panic.stderr
2019-12-18T20:02:13.4106751Z To update references, run this command from build directory:
2019-12-18T20:02:13.4107078Z tests/run-pass/update-references.sh '/tmp/compiletestqIwf9c' 'panic/catch_panic.rs'
2019-12-18T20:02:13.4107234Z error: 1 errors occurred comparing output.
2019-12-18T20:02:13.4107308Z status: exit code: 1
2019-12-18T20:02:13.4107308Z status: exit code: 1
2019-12-18T20:02:13.4108182Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/panic/catch_panic.rs" "-L" "/tmp/compiletestqIwf9c" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestqIwf9c/panic/catch_panic.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletestqIwf9c/panic/catch_panic.stage-id.aux" "-A" "unused"
2019-12-18T20:02:13.4108721Z ------------------------------------------
2019-12-18T20:02:13.4108777Z 
2019-12-18T20:02:13.4109033Z ------------------------------------------
2019-12-18T20:02:13.4109127Z stderr:
---
2019-12-18T20:02:22.8084859Z Verifying status of miri...
2019-12-18T20:02:22.8085359Z Verifying status of embedded-book...
2019-12-18T20:02:22.8085864Z Verifying status of rustc-guide...
2019-12-18T20:02:22.8150102Z Cloning into 'rust-toolstate'...
2019-12-18T20:02:23.3538665Z thread 'main' panicked at 'linux/windows only', src/libcore/option.rs:1185:5
2019-12-18T20:02:23.3589161Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test check-tools
2019-12-18T20:02:23.3589715Z Build completed unsuccessfully in 0:00:01
2019-12-18T20:02:23.3703356Z == clock drift check ==
2019-12-18T20:02:23.3720337Z   local time: Wed Dec 18 20:02:23 UTC 2019
2019-12-18T20:02:23.3720337Z   local time: Wed Dec 18 20:02:23 UTC 2019
2019-12-18T20:02:23.6375820Z   network time: Wed, 18 Dec 2019 20:02:23 GMT
2019-12-18T20:02:23.6376940Z == end clock drift check ==
2019-12-18T20:02:24.5516566Z 
2019-12-18T20:02:24.5630629Z ##[error]Bash exited with code '1'.
2019-12-18T20:02:24.5680601Z ##[section]Starting: Checkout
2019-12-18T20:02:24.5682697Z ==============================================================================
2019-12-18T20:02:24.5682817Z Task         : Get sources
2019-12-18T20:02:24.5682909Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
