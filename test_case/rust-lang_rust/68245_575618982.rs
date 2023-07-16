rust
_2 = &(promoted[0]: (&str, u32, u32)); // bb0[2]: scope 0 at <::std::macros::panic macros>:4:14: 5:75
_1 = _2;                         // bb0[3]: scope 0 at <::std::macros::panic macros>:4:14: 5:75
const std::rt::begin_panic::<&str>(const "assertion failed: false", move _1);
