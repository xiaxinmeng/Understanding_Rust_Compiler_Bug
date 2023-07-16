plain
running 8 tests
FFFFFFF.
failures:

---- coverage::tests::test_find_loop_backedges_none stdout ----
thread 'coverage::tests::test_find_loop_backedges_none' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9
---- coverage::tests::test_covgraph_switchint_then_loop_else_return stdout ----
---- coverage::tests::test_covgraph_switchint_then_loop_else_return stdout ----
thread 'coverage::tests::test_covgraph_switchint_then_loop_else_return' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9
---- coverage::tests::test_covgraph_goto_switchint stdout ----
---- coverage::tests::test_covgraph_goto_switchint stdout ----
thread 'coverage::tests::test_covgraph_goto_switchint' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9

---- coverage::tests::test_covgraph_switchint_loop_then_inner_loop_else_break stdout ----
---- coverage::tests::test_covgraph_switchint_loop_then_inner_loop_else_break stdout ----
thread 'coverage::tests::test_covgraph_switchint_loop_then_inner_loop_else_break' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9
---- coverage::tests::test_find_loop_backedges_one stdout ----
---- coverage::tests::test_find_loop_backedges_one stdout ----
thread 'coverage::tests::test_find_loop_backedges_one' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9

error: test failed, to rerun pass '-p rustc_mir_transform --lib'
---- coverage::tests::test_find_loop_backedges_two stdout ----
thread 'coverage::tests::test_find_loop_backedges_two' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9
---- coverage::tests::test_traverse_coverage_with_loops stdout ----
---- coverage::tests::test_traverse_coverage_with_loops stdout ----
thread 'coverage::tests::test_traverse_coverage_with_loops' panicked at 'cannot access a scoped thread local variable without calling `set` first', /cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:168:9

failures:
    coverage::tests::test_covgraph_goto_switchint
    coverage::tests::test_covgraph_switchint_loop_then_inner_loop_else_break
