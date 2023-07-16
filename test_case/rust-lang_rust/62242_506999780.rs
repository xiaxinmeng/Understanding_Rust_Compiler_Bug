plain
[01:49:04] test [ui] run-pass/issue-27901.rs ... ok
[01:49:04] thread '[ui] run-pass/issue-30530.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
[01:49:04] note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:49:04] normalized stderr:
[01:49:04] error: unused boxed `std::ops::Fn` trait object that must be used
[01:49:04]    |
[01:49:04]    |
[01:49:04] 24 |     take(Handler::Default, Box::new(main));
[01:49:04]    |
[01:49:04]    = note: `-D unused-must-use` implied by `-D unused`
[01:49:04]    = note: `-D unused-must-use` implied by `-D unused`
[01:49:04]    = note: closures are lazy and do nothing unless called
[01:49:04] error: aborting due to previous error
[01:49:04] 
[01:49:04] 
[01:49:04] 
[01:49:04] 
[01:49:04] expected stderr:
[01:49:04] 
[01:49:04] 
[01:49:04] diff of stderr:
[01:49:04] 
[01:49:04] +error: unused boxed `std::ops::Fn` trait object that must be used
[01:49:04] +   |
[01:49:04] +   |
[01:49:04] +24 |     take(Handler::Default, Box::new(main));
[01:49:04] +   |
[01:49:04] +   = note: `-D unused-must-use` implied by `-D unused`
[01:49:04] +   = note: `-D unused-must-use` implied by `-D unused`
[01:49:04] +   = note: closures are lazy and do nothing unless called
[01:49:04] +error: aborting due to previous error
[01:49:04] +
[01:49:04] +
[01:49:04] 
[01:49:04] 
[01:49:04] The actual stderr differed from the expected stderr.
[01:49:04] Actual stderr saved to /tmp/compiletestK1j1uS/issue-30530.stderr
[01:49:04] To update references, run this command from build directory:
[01:49:04] tests/run-pass/update-references.sh '/tmp/compiletestK1j1uS' 'issue-30530.rs'
[01:49:04] error: 1 errors occurred comparing output.
[01:49:04] status: exit code: 1
[01:49:04] status: exit code: 1
[01:49:04] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/issue-30530.rs" "-L" "/tmp/compiletestK1j1uS" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestK1j1uS/issue-30530.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestK1j1uS/issue-30530.stage-id.aux" "-A" "unused"
[01:49:04] ------------------------------------------
[01:49:04] 
[01:49:04] ------------------------------------------
[01:49:04] stderr:
[01:49:04] stderr:
[01:49:04] ------------------------------------------
[01:49:04] {"message":"unused boxed `std::ops::Fn` trait object that must be used","code":{"code":"unused_must_use","explanation":null},"level":"error","spans":[{"file_name":"tests/run-pass/issue-30530.rs","byte_start":864,"byte_end":903,"line_start":24,"line_end":24,"column_start":5,"column_end":44,"is_primary":true,"text":[{"text":"    take(Handler::Default, Box::new(main));","highlight_start":5,"highlight_end":44}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"`-D unused-must-use` implied by `-D unused`","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"closures are lazy and do nothing unless called","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: unused boxed `std::ops::Fn` trait object that must be used\n  --> tests/run-pass/issue-30530.rs:24:5\n   |\n24 |     take(Handler::Default, Box::new(main));\n   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = note: `-D unused-must-use` implied by `-D unused`\n   = note: closures are lazy and do nothing unless called\n\n"}
[01:49:04] 
[01:49:04] ------------------------------------------
[01:49:04] 
[01:49:04] test [ui] run-pass/issue-30530.rs ... FAILED
---
travis_time:end:0214cc40:start=1561859074872804316,finish=1561859074891495552,duration=18691236
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:008e146c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
