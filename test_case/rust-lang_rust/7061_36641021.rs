
7061.rs:6:42: 6:46 error: mismatched types: expected `@BarStruct` but found `&'a mut BarStruct` (expected @-ptr but found &-ptr)
7061.rs:6     fn foo(&'a mut self) -> @BarStruct { self }
                                                   ^~~~
