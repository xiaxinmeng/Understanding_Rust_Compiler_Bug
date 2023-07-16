
error: implementation of `Fake` is not general enough
  --> src/main.rs:34:5
   |
34 |     foo(Wrapper);
   |     ^^^
   |
   = note: Due to a where-clause on `foo`,
   = note: `fn(&u8) -> Wrapper<'_> {Wrapper<'_>}` must implement `Fake<'0, u8, Wrapper<'_>>`, for any lifetime `'0`
   = note: but `fn(&u8) -> Wrapper<'_> {Wrapper<'_>}` actually implements `Fake<'1, u8, Wrapper<'_>>`, for some specific lifetime `'1`
