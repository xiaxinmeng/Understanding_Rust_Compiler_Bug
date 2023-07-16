
error[E0520]: `Error` specializes an item from a parent `impl`, but that item is not marked `default`
    --> src\libcore\num\mod.rs:2522:13
     |
2522 |               type Error = Infallible;
     |               ^^^^^^^^^^^^^^^^^^^^^^^^ cannot specialize default item `Error`
...
2671 |       rev!(try_from_unbounded, usize, u8, u16, u32);
     |       ---------------------------------------------- in this macro invocation
     | 
    ::: src\libcore\convert.rs
     |
440  | / impl<T, U> TryFrom<U> for T where T: From<U> {
441  | |     type Error = Infallible;
442  | |
443  | |     fn try_from(value: U) -> Result<Self, Self::Error> {
444  | |         Ok(T::from(value))
445  | |     }
446  | | }
     | |_- parent `impl` is here
     |
     = note: to specialize, `Error` in the parent `impl` must be marked `default`
