
error: non-defining opaque type use in defining scope
  --> src/main.rs:64:6
   |
64 | ) -> Lens<'t, CombinedOptGetter<'t, Get, SomeGetter<'t, B>, A, Option<B>, B>, A, B>
   |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: used non-generic type `impl OptGetter<'t, Option<B>, B>` for generic parameter
  --> src/main.rs:20:34
   |
20 | type CombinedOptGetter<'t, Get1, Get2, A, B, C> = impl OptGetter<'t, A, C>;
   |                                  ^^^^

error: non-defining opaque type use in defining scope
  --> src/main.rs:64:6
   |
64 | ) -> Lens<'t, CombinedOptGetter<'t, Get, SomeGetter<'t, B>, A, Option<B>, B>, A, B>
   |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: used non-generic type `Option<B>` for generic parameter
  --> src/main.rs:20:43
   |
20 | type CombinedOptGetter<'t, Get1, Get2, A, B, C> = impl OptGetter<'t, A, C>;
   |                                           ^

error: aborting due to 2 previous errors
