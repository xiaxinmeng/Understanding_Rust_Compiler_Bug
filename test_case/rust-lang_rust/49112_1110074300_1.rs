
warning: unnecessary `unsafe` block
  [--> src/main.rs:7:17
](https://play.rust-lang.org/#)   |
7  |                 unsafe { ::core::ptr::write(x.as_mut_ptr(), $item) };
   |                 ^^^^^^
   |                 |
   |                 unnecessary `unsafe` block
   |                 because it's nested under this `unsafe` block
...
16 |         arr![arr![5; 3]; 2]
   |              ---------- in this macro invocation
   |
   = note: `#[warn(unused_unsafe)]` on by default
   = note: this warning originates in the macro `arr` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: unnecessary `unsafe` block
  [--> src/main.rs:9:35
](https://play.rust-lang.org/#)   |
7  |                 unsafe { ::core::ptr::write(x.as_mut_ptr(), $item) };
   |                 ------ because it's nested under this `unsafe` block
8  |             }
9  |             array_uninit.map(|mu| unsafe { mu.assume_init() })
   |                                   ^^^^^^ unnecessary `unsafe` block
...
16 |         arr![arr![5; 3]; 2]
   |              ---------- in this macro invocation
   |
   = note: this warning originates in the macro `arr` (in Nightly builds, run with -Z macro-backtrace for more info)
