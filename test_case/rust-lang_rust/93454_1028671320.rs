json
{ "type": "suite", "event": "started", "test_count": 1 }
{ "type": "test", "event": "started", "name": "tests::it_works" }
{ "type": "test", "name": "tests::it_works", "event": "ok" }
{ "type": "suite", "event": "ok", "passed": 1, "failed": 0, "allowed_fail": 0, "ignored": 0, "measured": 0, "filtered_out": 0, "exec_time": 0.000127229 }
{ "type": "suite", "event": "started", "test_count": 1 }
{ "type": "test", "event": "started", "name": "tests::it_works" }
{ "type": "test", "name": "tests::it_works", "event": "ok" }
{ "type": "suite", "event": "ok", "passed": 1, "failed": 0, "allowed_fail": 0, "ignored": 0, "measured": 0, "filtered_out": 0, "exec_time": 0.000227738 }
{ "type": "suite", "event": "started", "test_count": 2 }
{ "type": "test", "event": "started", "name": "tests::it_fails" }
{ "type": "test", "event": "started", "name": "tests::it_works" }
{ "type": "test", "name": "tests::it_works", "event": "ok" }
{ "type": "test", "name": "tests::it_fails", "event": "failed", "stdout": "thread 'tests::it_fails' panicked at 'assertion failed: `(left == right)`\n  left: `4`,\n right: `5`', tests/test_something.rs:12:9\nnote: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n" }
{ "type": "suite", "event": "failed", "passed": 1, "failed": 1, "allowed_fail": 0, "ignored": 0, "measured": 0, "filtered_out": 0, "exec_time": 0.000585028 }
