
{ "type": "suite", "event": "started", "test_count": 2 }
{ "type": "test", "event": "started", "name": "it_does_not_work" }
{ "type": "test", "event": "started", "name": "it_works" }
{ "type": "test", "name": "it_works", "event": "ok" }
{ "type": "test", "name": "it_does_not_work", "event": "failed", "stdout": "thread 'it_does_not_work' panicked at 'assertion failed: `(left == right)`\n  left: `4`,\n right: `5`', src/lib.rs:10:3\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n" }
{ "type": "suite", "event": "failed", "passed": 1, "failed": 1, "ignored": 0, "measured": 0, "filtered_out": 0, "exec_time": 0.000431658 }
