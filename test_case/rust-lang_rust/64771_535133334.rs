rust
   Compiling libc v0.2.64 (https://github.com/gnzlbg/libc.git?branch=fixs#fe67d7d9)
error: no rules expected the token `(`
  --> /folk/bpang/.cargo/git/checkouts/libc-2d0b80a096c2fb1e/fe67d7d/src/unix/linux_like/linux/gnu/b64/x86_64/align.rs:3:27
   |
3  |     pub struct max_align_t([f64; 4]);
   |                           ^ no rules expected this token in macro call
   |
  ::: /folk/bpang/.cargo/git/checkouts/libc-2d0b80a096c2fb1e/fe67d7d/src/macros.rs:66:1
   |
66 | macro_rules! s {
   | -------------- when calling this macro

error: aborting due to previous error
