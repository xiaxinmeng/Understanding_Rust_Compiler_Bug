plain
travis_time:end:1b90ceb8:start=1541873425787093325,finish=1541873478314124831,duration=52527031506
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:50:19] ......................................................................................i............. 3600/5003
[00:50:20] .................................................................................................... 3700/5003
[00:50:22] .........................................i.......................................................... 3800/5003
[00:50:24] .................................................................................................... 3900/5003
[00:50:27] ...................................F................................................................ 4000/5003
[00:50:34] .................................................................................................... 4200/5003
[00:50:39] i................................................................................................... 4300/5003
[00:50:42] .................................................................................................... 4400/5003
[00:50:45] .................................................................................................... 4500/5003
[00:50:45] .................................................................................................... 4500/5003
[00:50:49] .........................................................................i.......................... 4600/5003
[00:50:52] .................................................................................................... 4700/5003
[00:50:56] .................................................................................................... 4800/5003
[00:50:58] .................................................................................................... 4900/5003
n-precedence.rs:24:11
[00:51:01] +   --> $DIR/range-inclusive-pattern-precedence.rs:24:9
[00:51:01] 15    |
[00:51:01] 16 LL |         &0...9 => {}
[00:51:01] -    |           ^^^ help: use `..=` for an inclusive range
[00:51:01] +    |         ^^^^^^ help: use `..=` for an inclusive range
[00:51:01] 19 note: lint level defined here
[00:51:01] 20   --> $DIR/range-inclusive-pattern-precedence.rs:19:9
[00:51:01] 
[00:51:01] 
[00:51:01] 
[00:51:01] The actual stderr differed from the expected stderr.
[00:51:01] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/range-inclusive-pattern-precedence/range-inclusive-pattern-precedence.stderr
[00:51:01] To update references, rerun the tests and pass the `--bless` flag
[00:51:01] To only update this specific test, also pass `--test-args range/range-inclusive-pattern-precedence.rs`
[00:51:01] error: 1 errors occurred comparing output.
[00:51:01] status: exit code: 1
[00:51:01] status: exit code: 1
[00:51:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/range/range-inclusive-pattern-precedence.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/range-inclusive-pattern-precedence/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/range-inclusive-pattern-precedence/auxiliary" "-A" "unused"
[00:51:01] stdout:
[00:51:01] ----------------------------------------te_end":1394,"line_start":38,"line_end":38,"column_start":13,"column_end":20,"is_primary":true,"text":[{"text":"        box 10..=15 => {}","highlight_start":13,"highlight_end":20}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"add parentheses to clarify the precedence","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/range/range-inclusive-pattern-precedence.rs","byte_start":1387,"byte_end":1394,"line_start":38,"line_end":38,"column_start":13,"column_end":20,"is_primary":true,"text":[{"text":"        box 10..=15 => {}","highlight_start":13,"highlight_end":20}],"label":null,"suggested_replacement":"(10 ..=15)","suggestion_applicability":"MaybeIncorrect","expansion":null}],"children":[],"rendered":null}],"rendered":"error: the range pattern here has ambiguous interpretation\n  --> /checkout/src/test/ui/range/range-inclusive-pattern-precedence.rs:38:13\n   |\nLL |         box 10..=15 => {}\n   |             ^^^^^^^ help: add parentheses to clarify the precedence: `(10 ..=15)`\n\n"}
[00:51:01] {"message":"`...` range patterns are deprecated","code":{"code":"ellipsis_inclusive_range_patterns","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/range/range-inclusive-pattern-precedence.rs","byte_start":900,"byte_end":906,"line_start":24,"line_end":24,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        &0...9 => {}","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"lint level defined here","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/ui/range/range-inclusive-pattern-precedence.rs","byte_start":822,"byte_end":855,"line_start":19,"line_end":19,"column_start":9,"column_end":42,"is_primary":true,"text":[{"text":"#![warn(ellipsis_inclusive_range_patterns)]","highlight_start":9,"highlight_end":42}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"use `..=` for an inclusive range","code":null,"level":"help","spans":[{"file_name":"/checkout/src/test/ui/range/range-inclusive-pattern-precedence.rs","byte_start":900,"byte_end":906,"line_start":24,"line_end":24,"column_start":9,"column_end":15,"is_primary":true,"text":[{"text":"        &0...9 => {}","highlight_start":9,"highlight_end":15}],"label":null,"suggested_replacement":"&(0..=9)","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"warning: `...` range patterns are deprecated\n  --> /checkout/src/test/ui/range/range-inclusive-pattern-precedence.rs:24:9\n   |\nLL |         &0...9 => {}\n   |         ^^^^^^ help: use `..=` for an inclusive range\n   |\nnote: lint level defined here\n  --> /checkout/src/test/ui/range/range-inclusive-pattern-precedence.rs:19:9\n   |\nLL | #![warn(ellipsis_inclusive_range_patterns)]\n   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:51:01] {"message":"`...` range patterns are deprecated","code":{"code":"ellipsis_inclusive_range_patterns","explanation":null},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/range/range-inclusive-pattern-precede:01] Makefile:58: recipe for target 'check' failed
[00:51:01] make: *** [check] Error 1
