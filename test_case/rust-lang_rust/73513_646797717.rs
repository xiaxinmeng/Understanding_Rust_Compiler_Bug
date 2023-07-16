
error: this arithmetic operation will overflow
  --> $DIR/const-err2.rs:19:13
   |
LL |     let a = -std::i8::MIN;
   |             ^^^^^^^^^^^^^ attempt to negate `-128` would be greater than `i8::MAX`
   |
   = note: `#[deny(arithmetic_overflow)]` on by default
