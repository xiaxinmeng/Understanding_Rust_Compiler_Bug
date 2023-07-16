plain
............................F......
failures:

---- interval::tests::insert_collapses stdout ----
thread 'interval::tests::insert_collapses' panicked at 'wrong intervals after insert 9831..=9837 to IntervalSet { map: [(9831, 9837)], domain: 3000, _data: PhantomData }', compiler/rustc_index/src/interval.rs:136:9


failures:
    interval::tests::insert_collapses
    interval::tests::insert_collapses

test result: FAILED. 34 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s

error: test failed, to rerun pass '-p rustc_index --lib'
