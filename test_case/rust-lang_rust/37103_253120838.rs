
error[E0206]: the trait `Copy` may not be implemented for this type
   --> src/libcore/marker.rs:271:17
    |
271 | copy_impl! { i8 u8 i16 u16 i32 u32 i64 u64 isize usize f32 f64 bool char }
    | ----------------^^--------------------------------------------------------
    | |               |
    | |               type is not a structure or enumeration
    | in this macro invocation
