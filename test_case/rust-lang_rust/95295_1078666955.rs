plain
.................................................................................................... 100/672
.................................................................................................... 200/672
.................................................................................................... 300/672
.................................................................................................... 400/672
.............................................................F......F............................... 500/672
.......................................................................F....F....................... 600/672
...........................................................F.....F......

---- string::test_try_reserve stdout ----
---- string::test_try_reserve stdout ----
thread 'string::test_try_reserve' panicked at 'assertion failed: `(left matches right)`
  left: `Err(CapacityOverflow)`,
 right: `Err(AllocError { .. })`: isize::MAX + 1 should trigger an OOM!', library/alloc/tests/string.rs:731:13
---- string::test_try_reserve_exact stdout ----
---- string::test_try_reserve_exact stdout ----
thread 'string::test_try_reserve_exact' panicked at 'assertion failed: `(left matches right)`
  left: `Err(CapacityOverflow)`,
 right: `Err(AllocError { .. })`: isize::MAX + 1 should trigger an OOM!', library/alloc/tests/string.rs:815:13
---- vec::test_try_reserve_exact stdout ----
error: test failed, to rerun pass '-p alloc --test collectionstests'
error: test failed, to rerun pass '-p alloc --test collectionstests'
thread 'vec::test_try_reserve_exact' panicked at 'assertion failed: `(left matches right)`
  left: `Err(CapacityOverflow)`,
 right: `Err(AllocError { .. })`: isize::MAX + 1 should trigger an OOM!', library/alloc/tests/vec.rs:1644:13
---- vec::test_try_reserve stdout ----
---- vec::test_try_reserve stdout ----
thread 'vec::test_try_reserve' panicked at 'assertion failed: `(left matches right)`
  left: `Err(CapacityOverflow)`,
 right: `Err(AllocError { .. })`: isize::MAX + 1 should trigger an OOM!', library/alloc/tests/vec.rs:1527:13
---- vec_deque::test_try_reserve stdout ----
---- vec_deque::test_try_reserve stdout ----
thread 'vec_deque::test_try_reserve' panicked at 'assertion failed: `(left matches right)`
  left: `Err(CapacityOverflow)`,
 right: `Err(AllocError { .. })`: isize::MAX + 1 should trigger an OOM!', library/alloc/tests/vec_deque.rs:1227:13
---- vec_deque::test_try_reserve_exact stdout ----
---- vec_deque::test_try_reserve_exact stdout ----
thread 'vec_deque::test_try_reserve_exact' panicked at 'assertion failed: `(left matches right)`
  left: `Err(CapacityOverflow)`,
 right: `Err(AllocError { .. })`: isize::MAX + 1 should trigger an OOM!', library/alloc/tests/vec_deque.rs:1344:13

failures:
    string::test_try_reserve
    string::test_try_reserve_exact
