
error[E0405]: cannot find trait `Copy` in this scope
  --> C:\projects\rust\src/rtstartup\rsbegin.rs:61:22
   |
61 |                   impl Copy for $t {}
   |                        ^^^^ not found in this scope
...
66 | /     impl_copy! {
67 | |         usize u8 u16 u32 u64 u128
68 | |         isize i8 i16 i32 i64 i128
69 | |         f32 f64
70 | |         bool char
71 | |     }
   | |_____- in this macro invocation
help: possible candidate is found in another module, you can import it into scope
   |
48 |     use Copy;
   |
error: stability attributes may not be used outside of the standard library
  --> C:\projects\rust\src/rtstartup\rsbegin.rs:60:17
   |
60 |                   #[stable(feature = "rust1", since = "1.0.0")]
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
66 | /     impl_copy! {
67 | |         usize u8 u16 u32 u64 u128
68 | |         isize i8 i16 i32 i64 i128
69 | |         f32 f64
70 | |         bool char
71 | |     }
   | |_____- in this macro invocation
error: aborting due to 2 previous errors
