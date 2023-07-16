rust
error: implementation has missing stability attribute
   --> library/core/src/array/mod.rs:201:1
    |
201 | / impl<T, I, const N: usize> Index<I> for [T; N]
202 | | where
203 | |     I: SliceIndex<[T]>,
204 | | {
...   |
212 | |     }
213 | | }
    | |_^

error: implementation has missing stability attribute
   --> library/core/src/array/mod.rs:215:1
    |
215 | / impl<T, I, const N: usize> IndexMut<I> for [T; N]
