plain
   Compiling addr2line v0.14.0
error: expected pattern, found `,`
   --> library/std/src/sys/unix/mod.rs:161:52
    |
161 |         libc::ENETDOWN              => NetworkDown,,
    |                                                    ^ expected pattern
error: cannot find attribute `rustfmt` in this scope
   --> library/std/src/sys/unix/mod.rs:138:7
    |
    |
138 |     #[rustfmt(skip)]

error: cannot find attribute `rustfmt` in this scope
   --> library/std/src/io/error.rs:290:11
    |
    |
290 |         #[rustfmt(skip)]

error: aborting due to 3 previous errors

error: could not compile `std`
