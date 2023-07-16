
[01:27:28] ---- build::wrong_message_format_option stdout ----
[01:27:28] 	running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo build --message-format XML`
[01:27:28] thread 'build::wrong_message_format_option' panicked at '
[01:27:28] Expected: execs
[01:27:28]     but: expected to find:
[01:27:28] error: 'XML' isn't a valid value for '--message-format <FMT>'
[01:27:28] <tab>[possible values: human, json]
[01:27:28] 
[01:27:28] 
[01:27:28] did not find in output:
[01:27:28] error: 'XML' isn't a valid value for '--message-format <FMT>'
[01:27:28] <tab>[values: human, json]
[01:27:28] 
[01:27:28] 
[01:27:28] USAGE:
[01:27:28]     cargo build --message-format <FMT>
[01:27:28] 
[01:27:28] For more information try --help
[01:27:28] ', tools/cargo/tests/testsuite/hamcrest.rs:13:9
[01:27:28] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:27:28] 
[01:27:28] ---- metadata::cargo_metadata_bad_version stdout ----
[01:27:28] 	running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo metadata --no-deps --format-version 2`
[01:27:28] thread 'metadata::cargo_metadata_bad_version' panicked at '
[01:27:28] Expected: execs
[01:27:28]     but: expected to find:
[01:27:28] error: '2' isn't a valid value for '--format-version <VERSION>'
[01:27:28] <tab>[possible values: 1]
[01:27:28] 
[01:27:28] 
[01:27:28] did not find in output:
[01:27:28] error: '2' isn't a valid value for '--format-version <VERSION>'
[01:27:28] <tab>[values: 1]
[01:27:28] 
[01:27:28] 
[01:27:28] USAGE:
[01:27:28]     cargo metadata --format-version <VERSION> --no-deps
[01:27:28] 
[01:27:28] For more information try --help
[01:27:28] ', tools/cargo/tests/testsuite/hamcrest.rs:13:9
[01:27:28] 
[01:27:28] ---- run::dashes_are_forwarded stdout ----
[01:27:28] 	running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo run -- a -- b`
[01:27:28] thread 'run::dashes_are_forwarded' panicked at '
[01:27:28] Expected: execs
[01:27:28]     but: exited with exit code: 101
[01:27:28] --- stdout
[01:27:28] 
[01:27:28] --- stderr
[01:27:28] warning: path `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t897/foo/src/main.rs` was erroneously implicitly accepted for binary `bar`,
[01:27:28] please set bin.path in Cargo.toml
[01:27:28]    Compiling foo v0.0.1 (file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/cit/t897/foo)
[01:27:28]     Finished dev [unoptimized + debuginfo] target(s) in 0.77 secs
[01:27:28]      Running `target/debug/bar a b`
[01:27:28] thread 'main' panicked at 'assertion failed: `(left == right)`
[01:27:28]   left: `"b"`,
[01:27:28]  right: `"--"`', src/main.rs:5:17
[01:27:28] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:27:28] ', tools/cargo/tests/testsuite/hamcrest.rs:13:9
[01:27:28] 
[01:27:28] 
[01:27:28] failures:
[01:27:28]     build::wrong_message_format_option
[01:27:28]     metadata::cargo_metadata_bad_version
[01:27:28]     run::dashes_are_forwarded
[01:27:28] 
[01:27:28] test result: FAILED. 1177 passed; 3 failed; 1 ignored; 0 measured; 0 filtered out
