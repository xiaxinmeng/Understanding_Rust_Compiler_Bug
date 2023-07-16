plain
   |
46 | use libc::dirfd;
   |     ^^^^^^^^^^^
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `libc::fstatat64`
  --> /checkout/library/std/src/sys/unix/fs.rs:48:5
   |
48 | use libc::fstatat64;
