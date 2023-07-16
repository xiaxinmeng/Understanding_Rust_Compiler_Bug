plain
        unix_wait_status(
            132,
        ),
    ),
    stdout: "",
    stderr: "thread 'main' panicked at 'Box<dyn Any>', /checkout/src/test/ui/runtime/rt-explody-panic-payloads.rs:24:17\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\nthread 'main' panicked at 'Box<dyn Any>', /checkout/src/test/ui/runtime/rt-explody-panic-payloads.rs:16:9\n",
------------------------------------------
--- stderr -------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
thread 'main' panicked at 'assertion failed: stderr.map(|v|\n            {\n                v.ends_with(\"fatal runtime error: drop of the panic payload panicked\\n\")\n            }).unwrap_or(false)', /checkout/src/test/ui/runtime/rt-explody-panic-payloads.rs:28:5
------------------------------------------



