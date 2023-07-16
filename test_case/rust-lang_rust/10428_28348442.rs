
compile_and_link: x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/librustpkg.so
/home/jason/rust/src/librustpkg/path_util.rs:476:12: 476:23 error: invoking non-Rust fn in fn without #[fixed_stack_segment], #[deny(cstack)] on by default
/home/jason/rust/src/librustpkg/path_util.rs:476             libc::chmod(src_buf, S_IRUSR as libc::mode_t) == 0
                                                             ^~~~~~~~~~~
error: aborting due to previous error
