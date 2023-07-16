
error: internal compiler error: src/librustc/traits/trans/mod.rs:75: Encountered error 

`OutputTypeParameterMismatch(
    Binder(<[closure@src/lib.rs:34:40: 34:61]
            as std::ops::FnMut<(
                <std::vec::IntoIter<i32> as StreamingIterator<'_>>::Item,
            )>>),
    Binder(<[closure@src/lib.rs:34:40: 34:61]
            as std::ops::FnMut<(i32,)>>),
    Sorts(ExpectedFound {
        expected: i32,
        found: <std::vec::IntoIter<i32> as StreamingIterator<'_>>::Item
    })
)`

selecting `Binder(<[closure@src/lib.rs:34:40: 34:61] as std::ops::FnMut<(i32,)>>)` during trans
