
[TIMING] Std { target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } } -- 71.597
fatal: Invalid gitfile format: .git
thread 'main' panicked at 'command did not execute successfully: "git" "rev-parse" "HEAD"
expected success, got: exit code: 128', src/build_helper/lib.rs:139:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host mips-unknown-linux-gnu --target mips-unknown-linux-gnu
