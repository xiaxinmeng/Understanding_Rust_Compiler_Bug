plain
running 1595 tests
........................................................................................   88/1595
........................................................................................  176/1595
........................................................................................  264/1595
......................................................................F.FF..F...........  352/1595
........................................................................................  528/1595
........................................................................................  616/1595
.............................................................ii.........................  704/1595
........................................................................................  792/1595
---
...........

failures:

---- iter::adapters::map_windows::output_n1 stdout ----
thread 'iter::adapters::map_windows::output_n1' panicked at '`Fuse<I>` must be fused since returning a `None`', /checkout/library/core/src/iter/adapters/map_windows.rs:89:21
error: test failed, to rerun pass `-p core --test coretests`
---- iter::adapters::map_windows::output_n2 stdout ----
---- iter::adapters::map_windows::output_n2 stdout ----
thread 'iter::adapters::map_windows::output_n2' panicked at '`Fuse<I>` must be fused since returning a `None`', /checkout/library/core/src/iter/adapters/map_windows.rs:89:21

---- iter::adapters::map_windows::test_laziness stdout ----
thread 'iter::adapters::map_windows::test_laziness' panicked at '`Fuse<I>` must be fused since returning a `None`', /checkout/library/core/src/iter/adapters/map_windows.rs:89:21
---- iter::adapters::map_windows::test_zero_sized_type stdout ----
---- iter::adapters::map_windows::test_zero_sized_type stdout ----
thread 'iter::adapters::map_windows::test_zero_sized_type' panicked at '`Fuse<I>` must be fused since returning a `None`', /checkout/library/core/src/iter/adapters/map_windows.rs:89:21

failures:
    iter::adapters::map_windows::output_n1
    iter::adapters::map_windows::output_n2
