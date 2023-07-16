
test client_did_change_configuration_duplicated_and_unknown_settings ... FAILED
test client_init_duplicated_and_unknown_settings ... FAILED
test client_invalid_member_toml_manifest ... ok
test client_invalid_toml_manifest ... ok
test client_dependency_typo_and_fix ... FAILED
thread panicked while panicking. aborting.
[2021-04-21T21:38:59Z ERROR rls::server] Can't read message
thread '[2021-04-21T21:38:59Z ERROR rls::server] Can't read message
main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun pass '--test client'

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/client-c22ae4f720a9c514` (signal: 4, SIGILL: illegal instruction)
