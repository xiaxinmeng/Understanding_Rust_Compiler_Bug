
failures: 

---- string::test_try_reserve stdout ---- 
thread 'string::test_try_reserve' panicked at 'isize::MAX + 1 should trigger an overflow!', src/liballoc/../liballoc/tests/string.rs:594:17 

---- string::test_try_reserve_exact stdout ---- 
thread 'string::test_try_reserve_exact' panicked at 'isize::MAX + 1 should trigger an overflow!', src/liballoc/../liballoc/tests/string.rs:670:17 

---- vec::test_try_reserve stdout ---- 
thread 'vec::test_try_reserve' panicked at 'isize::MAX + 1 should trigger an overflow!', src/liballoc/../liballoc/tests/vec.rs:1176:17 

---- vec::test_try_reserve_exact stdout ---- 
thread 'vec::test_try_reserve_exact' panicked at 'isize::MAX + 1 should trigger an overflow!', src/liballoc/../liballoc/tests/vec.rs:1280:17 

---- vec_deque::test_try_reserve stdout ---- 
thread 'vec_deque::test_try_reserve' panicked at 'isize::MAX + 1 should trigger an overflow!', src/liballoc/../liballoc/tests/vec_deque.rs:1173:17 

---- vec_deque::test_try_reserve_exact stdout ---- 
thread 'vec_deque::test_try_reserve_exact' panicked at 'isize::MAX + 1 should trigger an overflow!', src/liballoc/../liballoc/tests/vec_deque.rs:1274:17 

failures: 
    string::test_try_reserve 
    string::test_try_reserve_exact 
    vec::test_try_reserve 
    vec::test_try_reserve_exact 
    vec_deque::test_try_reserve 
    vec_deque::test_try_reserve_exact 
