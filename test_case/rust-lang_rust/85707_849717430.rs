
error[E0107]: this associated function takes 0 generic arguments but 1 generic argument was supplied
   --> src/object.rs:102:24
    |
102 |         let obj2 = obj.try_into::<Ext>()?;
    |                        ^^^^^^^^------- help: remove these generics
    |                        |
    |                        expected 0 generic arguments
    |
note: associated function defined here, with 0 generic parameters
   --> rust/library/core/src/convert/mod.rs:395:8
    |
395 |     fn try_into(self) -> Result<T, Self::Error>;
    |        ^^^^^^^^
