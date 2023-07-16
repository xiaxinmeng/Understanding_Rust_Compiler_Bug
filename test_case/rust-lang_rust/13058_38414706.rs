 rust
test.rs:22:1: 32:2 note: consider using an explicit lifetime parameter as shown: fn check<'a, I: Iterator<uint>, T: Itble<'a, uint, I>>(cont: &'a T) -> bool
test.rs:22 fn check<'r, I: Iterator<uint>, T: Itble<'r, uint, I>>(cont: &T) -> bool
test.rs:23 {
test.rs:24     let cont_iter = cont.iter();
test.rs:25     let result = cont_iter.fold(Some(0u16), |state, val| {
test.rs:26         state.map_or(None, |mask| {
test.rs:27             let bit = 1 << val;
           ...
test.rs:24:21: 24:32 error: cannot infer an appropriate lifetime for autoref due to conflicting requirements
test.rs:24     let cont_iter = cont.iter();
                               ^~~~~~~~~~~
