
test.rs:7:19: 7:24 error: the trait `core::ops::Add<&i32>` is not implemented for the type `<core::slice::Iter<'_, i32> as core::iter::Iterator>::Item` [E0277]
test.rs:7     assert_eq!(it.sum(), 15);
