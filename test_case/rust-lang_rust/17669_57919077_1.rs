
into_iter.rs:27:13: 27:27 error: multiple applicable methods in scope [E0034]
into_iter.rs:27     let i = v.into_iter_();  // ~ Err1
                            ^~~~~~~~~~~~~~
into_iter.rs:8:5: 10:6 note: candidate #1 is `I.IntoIter<I>::into_iter_`
into_iter.rs:8     fn into_iter_(self) -> I {
into_iter.rs:9         self
into_iter.rs:10     }
into_iter.rs:14:5: 16:6 note: candidate #2 is `Vec<T>.IntoIter<MoveItems<T>>::into_iter_`
into_iter.rs:14     fn into_iter_(self) -> MoveItems<T> {
into_iter.rs:15         self.into_iter()
into_iter.rs:16     }
into_iter.rs:27:13: 27:27 error: the trait `core::iter::Iterator<<generic #185>>` is not implemented for the type `collections::vec::Vec<u8>`
into_iter.rs:27     let i = v.into_iter_();  // ~ Err1
                            ^~~~~~~~~~~~~~
error: aborting due to 2 previous errors
