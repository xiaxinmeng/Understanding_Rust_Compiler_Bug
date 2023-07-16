console
$ rustc --crate-type=lib -Ccodegen-units=1 a.rs 
$ nm --demangle liba.rlib 

a.a.5de1f1ba-cgu.0.rcgu.o:
00000000 T a::f
00000000 T a::g
00000000 T core::array::<impl core::ops::index::Index<I> for [T; N]>::index
00000000 T core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
         U core::panicking::panic_bounds_check
00000000 T <usize as core::slice::index::SliceIndex<[T]>>::index

lib.rmeta:
nm: lib.rmeta: no symbols
$ rustc -Zverbose --crate-type=lib -Ccodegen-units=1 a.rs 
$ nm --demangle liba.rlib 

a.a.5de1f1ba-cgu.0.rcgu.o:
00000000 T a::f
00000000 T a::g
00000000 T core::array::<impl core::ops::index::Index<I> for [T; Const { ty. usize, val. Param(N/#2) }]>::index
00000000 T core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
         U core::panicking::panic_bounds_check
00000000 T <usize as core::slice::index::SliceIndex<[T]>>::index
