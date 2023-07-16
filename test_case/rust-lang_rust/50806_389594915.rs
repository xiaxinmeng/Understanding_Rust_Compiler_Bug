plain
[00:49:31]    Compiling ansi_term v0.11.0
[00:49:31]    Compiling difference v2.0.0
[00:49:32]    Compiling pretty_assertions v0.5.1
[00:49:32]    Compiling bootstrap v0.0.0 (file:///checkout/src/bootstrap)
[00:49:37] error[E0063]: missing field `bless` in initializer of `flags::Subcommand`
[00:49:37]     --> bootstrap/builder.rs:1450:22
[00:49:37]      |
[00:49:37] 1450 |         config.cmd = Subcommand::Test {
[00:49:37]      |                      ^^^^^^^^^^^^^^^^ missing `bless`
[00:49:37] error: aborting due to previous error
[00:49:37] 
[00:49:37] For more information about this error, try `rustc --explain E0063`.
[00:49:37] For more information about this error, try `rustc --explain E0063`.
[00:49:37] error: Could not compile `bootstrap`.
[00:49:37] To learn more, run the command again with --verbose.
[00:49:37] 
[00:49:37] 
[00:49:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--"
[00:49:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--"
[00:49:37] expected success, got: exit code: 101
[00:49:37] 
[00:49:37] 
[00:49:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[00:49:37] Build completed unsuccessfully in 0:00:21
[00:49:37] make: *** [check] Error 1
[00:49:37] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:045cdfe4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
