plain
[00:20:37]    Compiling cc v1.0.22
[00:20:37]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:20:37]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:20:37]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]    --> libcore/num/dec2flt/rawfp.rs:203:43
[00:20:43]     |
[00:20:43] 203 |     fn from_bits(v: Self::Bits) -> Self { Self::from_bits(v) }
[00:20:43]     |
[00:20:43]     |
[00:20:43]     = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]    --> libcore/num/dec2flt/rawfp.rs:250:43
[00:20:43]     |
[00:20:43] 250 |     fn from_bits(v: Self::Bits) -> Self { Self::from_bits(v) }
[00:20:43]     |
[00:20:43]     |
[00:20:43]     = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43] 228  |                 !Self::min_value()
[00:20:43]      |                  ^^^^^^^^^^^^^^^^^
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2009 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48" }
[00:20:43]      |
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 651  |                 if rhs == 0 || (self == Self::min_value() && rhs == -1) {
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2009 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48" }
[00:20:43]      |
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 677  |                 if rhs == 0 || (self == Self::min_value() && rhs == -1) {
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2009 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48" }
[00:20:43]      |
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 704  |                 if rhs == 0 || (self == Self::min_value() && rhs == -1) {
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2009 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48" }
[00:20:43]      |
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 731  |                 if rhs == 0 || (self == Self::min_value() && rhs == -1) {
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2009 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48" }
[00:20:43]      |
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 888  |                     None if rhs >= 0 => Self::max_value(),
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2009 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48" }
[00:20:43]      |
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43] 889  |                     None => Self::min_value(),
[00:20:43]      |                             ^^^^^^^^^^^^^^^^^
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2009 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48" }
[00:20:43]      |
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 913  |                     None if rhs >= 0 => Self::min_value(),
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2009 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48" }
[00:20:43]      |
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43] 914  |                     None => Self::max_value(),
[00:20:43]      |                             ^^^^^^^^^^^^^^^^^
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2009 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48" }
[00:20:43]      |
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43] 940  |                         Self::max_value()
[00:20:43]      |                         ^^^^^^^^^^^^^^^^^
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2009 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48" }
[00:20:43]      |
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43] 942  |                         Self::min_value()
[00:20:43]      |                         ^^^^^^^^^^^^^^^^^
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2009 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48" }
[00:20:43]      |
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 970  |                     None if self < 0 && exp % 2 == 1 => Self::min_value(),
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2009 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48" }
[00:20:43]      |
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43] 971  |                     None => Self::max_value(),
[00:20:43]      |                             ^^^^^^^^^^^^^^^^^
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2009 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48" }
[00:20:43]      |
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 1412 |                 if self == Self::min_value() && rhs == -1 {
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2009 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48" }
[00:20:43]      |
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 1445 |                 if self == Self::min_value() && rhs == -1 {
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2009 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48" }
[00:20:43]      |
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 1477 |                 if self == Self::min_value() && rhs == -1 {
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2009 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48" }
[00:20:43]      rs are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 1540 |                 if self == Self::min_value() {
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2009 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48" }
[00:20:43]      |
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43] 1541 |                     (Self::min_value(), true)
[00:20:43]      |                      ^^^^^^^^^^^^^^^^^
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2009 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48" }
[00:20:43]      |
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 1961 |             Self::from_be(Self::from_ne_bytes(bytes))
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2009 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48" }
[00:20:43]      |
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 1961 |             Self::from_be(Self::from_ne_bytes(bytes))
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2009 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48" }
[00:20:43]      |
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 1978 |             Self::from_le(Self::from_ne_bytes(bytes))
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2009 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48" }
[00:20:43]      |
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 1978 |             Self::from_le(Self::from_ne_bytes(bytes))
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2009 |     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48" }
[00:20:43]      |     ---------------------------------------------------------------------3]     --> libcore/num/mod.rs:651:41
[00:20:43]      |
[00:20:43] 651  |                   if rhs == 0 || (self == Self::min_value() && rhs == -1) {
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2014 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
[00:20:43] 2015 | |         "0x2c48" }
[00:20:43]      | |__________________- in this macro invocation
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 677  |                   if rhs == 0 || (self == Self::min_value() && rhs == -1) {
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2014 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
[00:20:43] 2015 | |         "0x2c48" }
[00:20:43]      | |__________________- in this macro invocation
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 704  |                   if rhs == 0 || (self == Self::min_value() && rhs == -1) {
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2014 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
[00:20:43] 2015 | |         "0x2c48" }
[00:20:43]      | |__________________- in this macro invocation
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 731  |                   if rhs == 0 || (self == Self::min_value() && rhs == -1) {
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2014 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
[00:20:43] 2015 | |         "0x2c48" }
[00:20:43]      | |__________________- in this macro invocation
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 888  |                       None if rhs >= 0 => Self::max_value(),
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2014 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
[00:20:43] 2015 | |         "0x2c48" }
[00:20:43]      | |__________________- in this macro invocation
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43] 889  |                       None => Self::min_value(),
[00:20:43]      |                               ^^^^^^^^^^^^^^^^^
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2014 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
[00:20:43] 2015 | |         "0x2c48" }
[00:20:43]      | |__________________- in this macro invocation
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 913  |                       None if rhs >= 0 => Self::min_value(),
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2014 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
[00:20:43] 2015 | |         "0x2c48" }
[00:20:43]      | |__________________- in this macro invocation
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43] 914  |                       None => Self::max_value(),
[00:20:43]      |                               ^^^^^^^^^^^^^^^^^
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2014 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
[00:20:43] 2015 | |         "0x2c48" }
[00:20:43]      | |__________________- in this macro invocation
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43] 940  |                           Self::max_value()
[00:20:43]      |                           ^^^^^^^^^^^^^^^^^
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2014 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
[00:20:43] 2015 | |         "0x2c48" }
[00:20:43]      | |__________________- in this macro invocation
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43] 942  |                           Self::min_value()
[00:20:43]      |                           ^^^^^^^^^^^^^^^^^
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2014 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
[00:20:43] 2015 | |         "0x2c48" }
[00:20:43]      | |__________________- in this macro invocation
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 970  |                       None if self < 0 && exp % 2 == 1 => Self::min_value(),
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2014 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
[00:20:43] 2015 | |         "0x2c48" }
[00:20:43]      | |__________________- in this macro invocation
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43] 971  |                       None => Self::max_value(),
[00:20:43]      |                               ^^^^^^^^^^^^^^^^^
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2014 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
[00:20:43] 2015 | |         "0x2c48" }
[00:20:43]      | |__________________- in this macro invocation
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 1412 |                   if self == Self::min_value() && rhs == -1 {
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2014 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
[00:20:43] 2015 | |         "0x2c48" }
[00:20:43]      | |__________________- in this macro invocation
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 1445 |                   if self == Self::min_value() && rhs == -1 {
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2014 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
[00:20:43] 2015 | |         "0x2c48" }
[00:20:43]      | |__________________- in this macro invocation
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 1477 |                   if self == Self::min_value() && rhs == -1 {
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2014 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
[00:20:43] 2015 | |         "0x2c48" }
[00:20:43]      | |__________________- in this macro invocation
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 1510 |                   if self == Self::min_value() && rhs == -1 {
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2014 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
[00:20:43] 2015 | |error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43] 1541 |                       (Self::min_value(), true)
[00:20:43]      |                        ^^^^^^^^^^^^^^^^^
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2014 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
[00:20:43] 2015 | |         "0x2c48" }
[00:20:43]      | |__________________- in this macro invocation
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 1961 |               Self::from_be(Self::from_ne_bytes(bytes))
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2014 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
[00:20:43] 2015 | |         "0x2c48" }
[00:20:43]      | |__________________- in this macro invocation
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 1961 |               Self::from_be(Self::from_ne_bytes(bytes))
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2014 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
[00:20:43] 2015 | |         "0x2c48" }
[00:20:43]      | |__________________- in this macro invocation
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 1978 |               Self::from_le(Self::from_ne_bytes(bytes))
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2014 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
[00:20:43] 2015 | |         "0x2c48" }
[00:20:43]      | |__________________- in this macro invocation
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
[00:20:43] error[E0658]: tuple struct Self constructors are unstable (see issue #51994)
[00:20:43]      |
[00:20:43]      |
[00:20:43] 1978 |               Self::from_le(Self::from_ne_bytes(bytes))
[00:20:43] ...
[00:20:43] ...
[00:20:43] 2014 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
[00:20:43] 2015 | |         "0x2c48" }
[00:20:43]      | |__________________- in this macro invocation
[00:20:43]      |
[00:20:43]      = help: add #![feature(tuple_struct_self_ctor)] to the crate attributes to enable
[00:20:43] 
---
151200 ./src/tools/clang
149112 ./src/llvm-emscripten/test
148720 ./obj/build/bootstrap/debug/incremental
134288 ./obj/build/bootstrap/debug/incremental/bootstrap-11nz4fw202v9g
134284 ./obj/build/bootstrap/debug/incremental/bootstrap-11nz4fw202v9g/s-f49kqguj4l-2qa813-3b1qbyx5cod2e
103868 ./src/tools/lldb
98952 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
93756 ./src/tools/clang/test
90752 ./obj/build/x86_64-un6_64-unknown-linux-gnu/stage0-bootstrap-tools
---
travis_time:end:0ebb1440:start=1535435282394489426,finish=1535435282403207646,duration=8718220
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0bd604f6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06addfe0
travis_time:start:06addfe0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic
