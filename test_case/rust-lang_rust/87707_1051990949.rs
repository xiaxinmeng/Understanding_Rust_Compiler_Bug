
failures:

---- empty_table stdout ----
# This panic causes the poisoning
thread 'empty_table' panicked at 'bad parameter or other API misuse', tests/util.rs:23:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- null_rows stdout ----
# Source of poisoning now unknown
thread 'null_rows' panicked at 'Once instance has previously been poisoned', library/std/src/sync/once.rs:400:21

---- data_types stdout ----
thread 'data_types' panicked at 'Once instance has previously been poisoned', library/std/src/sync/once.rs:400:21

---- simple_count_error stdout ----
thread 'simple_count_error' panicked at 'Once instance has previously been poisoned', library/std/src/sync/once.rs:400:21

# ....
