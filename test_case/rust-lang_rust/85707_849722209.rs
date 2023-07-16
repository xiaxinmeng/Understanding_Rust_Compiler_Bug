
error[E0061]: this function takes 0 arguments but 2 arguments were supplied
   --> ffishim/src/field.rs:40:44
    |
40  |             crate::types::switch(&self.ty).try_into(&self.ty, receiver)
    |                                            ^^^^^^^^ --------  -------- supplied 2 arguments
    |                                            |
    |                                            expected 0 arguments
    |
note: associated function defined here
   --> rust/library/core/src/convert/mod.rs:395:8
    |
395 |     fn try_into(self) -> Result<T, Self::Error>;
    |        ^^^^^^^^
