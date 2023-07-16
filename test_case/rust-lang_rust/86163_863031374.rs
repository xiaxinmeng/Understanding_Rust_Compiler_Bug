
glaubitz@gcc202:~$ rustc --crate-type dylib franta.rs
warning: attribute should be applied to a function or static
 --> franta.rs:2:1
  |
2 |   #[no_mangle]
  |   ^^^^^^^^^^^^
3 |   #[repr(C)]
4 | / pub struct Franta {
5 | |         a:f32,
6 | |         b:f32,
7 | |         c:f32,
8 | |         d:f32,
9 | | }
  | |_- not a function or static
  |
  = note: `#[warn(unused_attributes)]` on by default
  = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

warning: 1 warning emitted

glaubitz@gcc202:~$ gcc ./testfranta.c ./libfranta.so 
glaubitz@gcc202:~$ ./a.out 
./a.out: error while loading shared libraries: libfranta.so: cannot open shared object file: No such file or directory
glaubitz@gcc202:~$ LD_LIBRARY_PATH=. ./a.out 
XXX start Franta { a: 2.0, b: 2.2, c: 0.0, d: 0.0 } 0
glaubitz@gcc202:~$
