
   Compiling crossbeam-utils v0.4.0
     Running `rustc --crate-name crossbeam_utils C:\Users\Kyle\.cargo\registry\src\github.com-1ecc6299db9ec823\crossbeam-utils-0.4.0\src\lib.rs
 --crate-type lib --emit=dep-info,link -C debuginfo=2 --cfg "feature=\"default\"" --cfg "feature=\"use_std\"" -C metadata=bef58e98434bc762 -C e
xtra-filename=-bef58e98434bc762 --out-dir C:\Users\Kyle\Desktop\test\target\debug\deps -L dependency=C:\Users\Kyle\Desktop\test\target\debug\de
ps --cap-lints allow`
error[E0658]: the struct `#[repr(align(u16))]` attribute is experimental (see issue #33626)
  --> C:\Users\Kyle\.cargo\registry\src\github.com-1ecc6299db9ec823\crossbeam-utils-0.4.0\src\cache_padded.rs:19:1
   |
19 | #[repr(align(64))]
   | ^^^^^^^^^^^^^^^^^^
   |
   = help: add #![feature(repr_align)] to the crate attributes to enable

error[E0658]: non-string literals in attributes, or string literals in top-level positions, are experimental (see issue #34981)
  --> C:\Users\Kyle\.cargo\registry\src\github.com-1ecc6299db9ec823\crossbeam-utils-0.4.0\src\cache_padded.rs:19:1
   |
19 | #[repr(align(64))]
   | ^^^^^^^^^^^^^^^^^^
   |
   = help: add #![feature(attr_literals)] to the crate attributes to enable

error: aborting due to 2 previous errors

error: Could not compile `crossbeam-utils`.

Caused by:
  process didn't exit successfully: `rustc --crate-name crossbeam_utils C:\Users\Kyle\.cargo\registry\src\github.com-1ecc6299db9ec823\crossbeam
-utils-0.4.0\src\lib.rs --crate-type lib --emit=dep-info,link -C debuginfo=2 --cfg feature="default" --cfg feature="use_std" -C metadata=bef58e
98434bc762 -C extra-filename=-bef58e98434bc762 --out-dir C:\Users\Kyle\Desktop\test\target\debug\deps -L dependency=C:\Users\Kyle\Desktop\test\
target\debug\deps --cap-lints allow` (exit code: 101)
