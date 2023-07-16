
error[E0107]: missing generics for trait `TryFrom`
  --> src/lib.rs:61:30
   |
61 |         let result = <u32 as TryFrom>::try_from("hello");
   |                              ^^^^^^^ expected 1 generic argument
   |
note: trait defined here, with 1 generic parameter: `T`
  --> src/lib.rs:19:11
   |
19 | pub trait TryFrom<T>: Sized {
   |           ^^^^^^^ -
help: add missing generic argument
   |
61 |         let result = <u32 as TryFrom<T>>::try_from("hello");
   |                              ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
  --> src/char.rs:29:23
   |
29 |     assert_eq!(<u8 as TryFrom>::try_from('~'), Ok(0x7e));
   |                       ^^^^^^^ expected 1 generic argument
   |
note: trait defined here, with 1 generic parameter: `T`
  --> src/lib.rs:19:11
   |
19 | pub trait TryFrom<T>: Sized {
   |           ^^^^^^^ -
help: add missing generic argument
   |
29 |     assert_eq!(<u8 as TryFrom<T>>::try_from('~'), Ok(0x7e));
   |                       ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
  --> src/char.rs:30:23
   |
30 |     assert_eq!(<u8 as TryFrom>::try_from('\u{100}'), Err(TryFromIntError::Overflow));
   |                       ^^^^^^^ expected 1 generic argument
   |
note: trait defined here, with 1 generic parameter: `T`
  --> src/lib.rs:19:11
   |
19 | pub trait TryFrom<T>: Sized {
   |           ^^^^^^^ -
help: add missing generic argument
   |
30 |     assert_eq!(<u8 as TryFrom<T>>::try_from('\u{100}'), Err(TryFromIntError::Overflow));
   |                       ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
  --> src/char.rs:46:25
   |
46 |     assert_eq!(<char as TryFrom>::try_from(0x7e), Ok('~'));
   |                         ^^^^^^^ expected 1 generic argument
   |
note: trait defined here, with 1 generic parameter: `T`
  --> src/lib.rs:19:11
   |
19 | pub trait TryFrom<T>: Sized {
   |           ^^^^^^^ -
help: add missing generic argument
   |
46 |     assert_eq!(<char as TryFrom<T>>::try_from(0x7e), Ok('~'));
   |                         ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
  --> src/char.rs:47:25
   |
47 |     assert_eq!(<char as TryFrom>::try_from('~'), Ok('~'));
   |                         ^^^^^^^ expected 1 generic argument
   |
note: trait defined here, with 1 generic parameter: `T`
  --> src/lib.rs:19:11
   |
19 | pub trait TryFrom<T>: Sized {
   |           ^^^^^^^ -
help: add missing generic argument
   |
47 |     assert_eq!(<char as TryFrom<T>>::try_from('~'), Ok('~'));
   |                         ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
  --> src/char.rs:56:31
   |
56 |                 match <u32 as TryFrom>::try_from(n)? {
   |                               ^^^^^^^ expected 1 generic argument
...
68 | impl_int_to_char!(i8, i16, i32, i64, isize, u16, u32, u64, usize);
   | ------------------------------------------------------------------ in this macro invocation
   |
note: trait defined here, with 1 generic parameter: `T`
  --> src/lib.rs:19:11
   |
19 | pub trait TryFrom<T>: Sized {
   |           ^^^^^^^ -
   = note: this error originates in the macro `impl_int_to_char` (in Nightly builds, run with -Z macro-backtrace for more info)
help: add missing generic argument
   |
56 |                 match <u32 as TryFrom<T>>::try_from(n)? {
   |                               ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
  --> src/char.rs:72:25
   |
72 |     assert_eq!(<char as TryFrom>::try_from(-1), Err(TryFromIntToCharError::Underflow));
   |                         ^^^^^^^ expected 1 generic argument
   |
note: trait defined here, with 1 generic parameter: `T`
  --> src/lib.rs:19:11
   |
19 | pub trait TryFrom<T>: Sized {
   |           ^^^^^^^ -
help: add missing generic argument
   |
72 |     assert_eq!(<char as TryFrom<T>>::try_from(-1), Err(TryFromIntToCharError::Underflow));
   |                         ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
  --> src/char.rs:73:25
   |
73 |     assert_eq!(<char as TryFrom>::try_from(0x7eu32), Ok('~'));
   |                         ^^^^^^^ expected 1 generic argument
   |
note: trait defined here, with 1 generic parameter: `T`
  --> src/lib.rs:19:11
   |
19 | pub trait TryFrom<T>: Sized {
   |           ^^^^^^^ -
help: add missing generic argument
   |
73 |     assert_eq!(<char as TryFrom<T>>::try_from(0x7eu32), Ok('~'));
   |                         ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
  --> src/char.rs:74:25
   |
74 |     assert_eq!(<char as TryFrom>::try_from(0xd888), Err(TryFromIntToCharError::Reserved));
   |                         ^^^^^^^ expected 1 generic argument
   |
note: trait defined here, with 1 generic parameter: `T`
  --> src/lib.rs:19:11
   |
19 | pub trait TryFrom<T>: Sized {
   |           ^^^^^^^ -
help: add missing generic argument
   |
74 |     assert_eq!(<char as TryFrom<T>>::try_from(0xd888), Err(TryFromIntToCharError::Reserved));
   |                         ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
  --> src/char.rs:75:25
   |
75 |     assert_eq!(<char as TryFrom>::try_from(0x10ffff), Ok('\u{10ffff}'));
   |                         ^^^^^^^ expected 1 generic argument
   |
note: trait defined here, with 1 generic parameter: `T`
  --> src/lib.rs:19:11
   |
19 | pub trait TryFrom<T>: Sized {
   |           ^^^^^^^ -
help: add missing generic argument
   |
75 |     assert_eq!(<char as TryFrom<T>>::try_from(0x10ffff), Ok('\u{10ffff}'));
   |                         ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
  --> src/char.rs:77:18
   |
77 |         <char as TryFrom>::try_from(0x110000),
   |                  ^^^^^^^ expected 1 generic argument
   |
note: trait defined here, with 1 generic parameter: `T`
  --> src/lib.rs:19:11
   |
19 | pub trait TryFrom<T>: Sized {
   |           ^^^^^^^ -
help: add missing generic argument
   |
77 |         <char as TryFrom<T>>::try_from(0x110000),
   |                  ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
  --> src/int.rs:77:24
   |
77 |     assert_eq!(<u64 as TryFrom>::try_from(usize::MAX), Ok(usize::MAX as u64));
   |                        ^^^^^^^ expected 1 generic argument
   |
note: trait defined here, with 1 generic parameter: `T`
  --> src/lib.rs:19:11
   |
19 | pub trait TryFrom<T>: Sized {
   |           ^^^^^^^ -
help: add missing generic argument
   |
77 |     assert_eq!(<u64 as TryFrom<T>>::try_from(usize::MAX), Ok(usize::MAX as u64));
   |                        ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:106:23
    |
106 |     assert_eq!(<u8 as TryFrom>::try_from(0xffu16), Ok(0xffu8));
    |                       ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
106 |     assert_eq!(<u8 as TryFrom<T>>::try_from(0xffu16), Ok(0xffu8));
    |                       ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:107:23
    |
107 |     assert_eq!(<u8 as TryFrom>::try_from(0x100u16), Err(TryFromIntError::Overflow));
    |                       ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
107 |     assert_eq!(<u8 as TryFrom<T>>::try_from(0x100u16), Err(TryFromIntError::Overflow));
    |                       ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:110:28
    |
110 |         assert_eq!(<u32 as TryFrom>::try_from(usize::MAX), Ok(u32::MAX));
    |                            ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
110 |         assert_eq!(<u32 as TryFrom<T>>::try_from(usize::MAX), Ok(u32::MAX));
    |                            ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:111:30
    |
111 |         assert_eq!(<usize as TryFrom>::try_from(u64::MAX), Err(TryFromIntError::Overflow));
    |                              ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
111 |         assert_eq!(<usize as TryFrom<T>>::try_from(u64::MAX), Err(TryFromIntError::Overflow));
    |                              ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:113:28
    |
113 |         assert_eq!(<u32 as TryFrom>::try_from(usize::MAX), Err(TryFromIntError::Overflow));
    |                            ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
113 |         assert_eq!(<u32 as TryFrom<T>>::try_from(usize::MAX), Err(TryFromIntError::Overflow));
    |                            ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:114:30
    |
114 |         assert_eq!(<usize as TryFrom>::try_from(u64::MAX), Ok(usize::MAX));
    |                              ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
114 |         assert_eq!(<usize as TryFrom<T>>::try_from(u64::MAX), Ok(usize::MAX));
    |                              ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:147:23
    |
147 |     assert_eq!(<u8 as TryFrom>::try_from(0i16), Ok(0u8));
    |                       ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
147 |     assert_eq!(<u8 as TryFrom<T>>::try_from(0i16), Ok(0u8));
    |                       ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:148:23
    |
148 |     assert_eq!(<u8 as TryFrom>::try_from(-1i16), Err(TryFromIntError::Underflow));
    |                       ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
148 |     assert_eq!(<u8 as TryFrom<T>>::try_from(-1i16), Err(TryFromIntError::Underflow));
    |                       ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:149:23
    |
149 |     assert_eq!(<u8 as TryFrom>::try_from(256i16), Err(TryFromIntError::Overflow));
    |                       ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
149 |     assert_eq!(<u8 as TryFrom<T>>::try_from(256i16), Err(TryFromIntError::Overflow));
    |                       ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:151:25
    |
151 |     assert_eq!(<u128 as TryFrom>::try_from(i32::MAX), Ok(i32::MAX as u128));
    |                         ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
151 |     assert_eq!(<u128 as TryFrom<T>>::try_from(i32::MAX), Ok(i32::MAX as u128));
    |                         ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:152:25
    |
152 |     assert_eq!(<u128 as TryFrom>::try_from(-1i32), Err(TryFromIntError::Underflow));
    |                         ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
152 |     assert_eq!(<u128 as TryFrom<T>>::try_from(-1i32), Err(TryFromIntError::Underflow));
    |                         ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:154:28
    |
154 |         assert_eq!(<u32 as TryFrom>::try_from(isize::MAX), Ok(0x7fff_ffffu32));
    |                            ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
154 |         assert_eq!(<u32 as TryFrom<T>>::try_from(isize::MAX), Ok(0x7fff_ffffu32));
    |                            ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:155:30
    |
155 |         assert_eq!(<usize as TryFrom>::try_from(i64::MAX), Err(TryFromIntError::Overflow));
    |                              ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
155 |         assert_eq!(<usize as TryFrom<T>>::try_from(i64::MAX), Err(TryFromIntError::Overflow));
    |                              ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:157:28
    |
157 |         assert_eq!(<u32 as TryFrom>::try_from(isize::MAX), Err(TryFromIntError::Overflow));
    |                            ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
157 |         assert_eq!(<u32 as TryFrom<T>>::try_from(isize::MAX), Err(TryFromIntError::Overflow));
    |                            ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:158:27
    |
158 |         assert!(<usize as TryFrom>::try_from(i64::MAX).unwrap() > 0xffff_ffffusize);
    |                           ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
158 |         assert!(<usize as TryFrom<T>>::try_from(i64::MAX).unwrap() > 0xffff_ffffusize);
    |                           ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:188:23
    |
188 |     assert_eq!(<i8 as TryFrom>::try_from(0x7fu8), Ok(0x7fi8));
    |                       ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
188 |     assert_eq!(<i8 as TryFrom<T>>::try_from(0x7fu8), Ok(0x7fi8));
    |                       ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:189:23
    |
189 |     assert_eq!(<i8 as TryFrom>::try_from(0x80u8), Err(TryFromIntError::Overflow));
    |                       ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
189 |     assert_eq!(<i8 as TryFrom<T>>::try_from(0x80u8), Err(TryFromIntError::Overflow));
    |                       ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:191:24
    |
191 |     assert_eq!(<i64 as TryFrom>::try_from(i64::MAX as u128), Ok(i64::MAX));
    |                        ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
191 |     assert_eq!(<i64 as TryFrom<T>>::try_from(i64::MAX as u128), Ok(i64::MAX));
    |                        ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:192:24
    |
192 |     assert_eq!(<i64 as TryFrom>::try_from(u128::MAX), Err(TryFromIntError::Overflow));
    |                        ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
192 |     assert_eq!(<i64 as TryFrom<T>>::try_from(u128::MAX), Err(TryFromIntError::Overflow));
    |                        ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:194:28
    |
194 |         assert_eq!(<i64 as TryFrom>::try_from(usize::MAX), Ok(0xffff_ffffi64));
    |                            ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
194 |         assert_eq!(<i64 as TryFrom<T>>::try_from(usize::MAX), Ok(0xffff_ffffi64));
    |                            ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:196:23
    |
196 |             <isize as TryFrom>::try_from(0x8000_0000u64),
    |                       ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
196 |             <isize as TryFrom<T>>::try_from(0x8000_0000u64),
    |                       ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:200:28
    |
200 |         assert_eq!(<i64 as TryFrom>::try_from(usize::MAX), Err(TryFromIntError::Overflow));
    |                            ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
200 |         assert_eq!(<i64 as TryFrom<T>>::try_from(usize::MAX), Err(TryFromIntError::Overflow));
    |                            ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:201:27
    |
201 |         assert!(<isize as TryFrom>::try_from(0x8000_0000u64).unwrap() > 0x7fff_ffff);
    |                           ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
201 |         assert!(<isize as TryFrom<T>>::try_from(0x8000_0000u64).unwrap() > 0x7fff_ffff);
    |                           ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:234:23
    |
234 |     assert_eq!(<i8 as TryFrom>::try_from(127i16), Ok(127i8));
    |                       ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
234 |     assert_eq!(<i8 as TryFrom<T>>::try_from(127i16), Ok(127i8));
    |                       ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:235:23
    |
235 |     assert_eq!(<i8 as TryFrom>::try_from(128i16), Err(TryFromIntError::Overflow));
    |                       ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
235 |     assert_eq!(<i8 as TryFrom<T>>::try_from(128i16), Err(TryFromIntError::Overflow));
    |                       ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:236:23
    |
236 |     assert_eq!(<i8 as TryFrom>::try_from(-128i16), Ok(-128i8));
    |                       ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
236 |     assert_eq!(<i8 as TryFrom<T>>::try_from(-128i16), Ok(-128i8));
    |                       ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:237:23
    |
237 |     assert_eq!(<i8 as TryFrom>::try_from(-129i16), Err(TryFromIntError::Underflow));
    |                       ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
237 |     assert_eq!(<i8 as TryFrom<T>>::try_from(-129i16), Err(TryFromIntError::Underflow));
    |                       ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:239:24
    |
239 |     assert_eq!(<i64 as TryFrom>::try_from(i64::MAX as i128), Ok(i64::MAX));
    |                        ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
239 |     assert_eq!(<i64 as TryFrom<T>>::try_from(i64::MAX as i128), Ok(i64::MAX));
    |                        ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:240:24
    |
240 |     assert_eq!(<i64 as TryFrom>::try_from(i128::MAX), Err(TryFromIntError::Overflow));
    |                        ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
240 |     assert_eq!(<i64 as TryFrom<T>>::try_from(i128::MAX), Err(TryFromIntError::Overflow));
    |                        ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:243:28
    |
243 |         assert_eq!(<i32 as TryFrom>::try_from(isize::MAX), Ok(i32::MAX));
    |                            ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
243 |         assert_eq!(<i32 as TryFrom<T>>::try_from(isize::MAX), Ok(i32::MAX));
    |                            ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:244:30
    |
244 |         assert_eq!(<isize as TryFrom>::try_from(i64::MAX), Err(TryFromIntError::Overflow));
    |                              ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
244 |         assert_eq!(<isize as TryFrom<T>>::try_from(i64::MAX), Err(TryFromIntError::Overflow));
    |                              ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:246:28
    |
246 |         assert_eq!(<i32 as TryFrom>::try_from(isize::MAX), Err(TryFromIntError::Overflow));
    |                            ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
246 |         assert_eq!(<i32 as TryFrom<T>>::try_from(isize::MAX), Err(TryFromIntError::Overflow));
    |                            ^^^^^^^^^^

error[E0107]: missing generics for trait `TryFrom`
   --> src/int.rs:247:27
    |
247 |         assert!(<isize as TryFrom>::try_from(i64::MAX).unwrap() > 0x7fff_ffffisize);
    |                           ^^^^^^^ expected 1 generic argument
    |
note: trait defined here, with 1 generic parameter: `T`
   --> src/lib.rs:19:11
    |
19  | pub trait TryFrom<T>: Sized {
    |           ^^^^^^^ -
help: add missing generic argument
    |
247 |         assert!(<isize as TryFrom<T>>::try_from(i64::MAX).unwrap() > 0x7fff_ffffisize);
    |                           ^^^^^^^^^^

error: aborting due to 46 previous errors
