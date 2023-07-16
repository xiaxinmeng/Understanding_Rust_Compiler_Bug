
test client_use_statement_completion_doesnt_suggest_arguments ... ok
test client_workspace_symbol ... ok
test client_workspace_symbol_duplicates ... ok
thread panicked while panicking. aborting.
[2020-04-02T01:54:47Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: test failed, to rerun pass '--test client'

Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/client-e6c2853afc83d084` (signal: 4, SIGILL: illegal instruction)


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rls/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--"
expected success, got: exit code: 101


