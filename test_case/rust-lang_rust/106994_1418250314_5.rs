rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

struct IncorrectTypeOfDefaultValueIsNOTcheckedBeforeICE<const U: usize = {1u8 + 0i32 as usize}>
where
    [(); U]:;
