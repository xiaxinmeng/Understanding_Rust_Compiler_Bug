rust
# MIRI_LIB_SRC=~/src/rust/library ./run-test.sh alloc --all-targets
A libstd for Miri is now available in `/home/jess/.cache/miri/HOST`.
error: Undefined Behavior: trying to reborrow <4385> for Unique permission at alloc6094[0x0], but that tag does not exist in the borrow stack for this location
    --> /home/jess/src/rust/library/alloc/src/vec/mod.rs:2649:24
     |
2649 |             IntoIter { buf, phantom: PhantomData, ptr: begin, end }
     |                        ^^^
     |                        |
     |                        trying to reborrow <4385> for Unique permission at alloc6094[0x0], but that tag does not exist in the borrow stack for this location
     |                        this error occurs as part of a reborrow at alloc6094[0x0..0x18]
     |
     = help: this indicates a potential bug in the program: it performed an invalid operation, but the rules it violated are still experimental
     = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
             
     = note: inside `<std::vec::Vec<std::ffi::OsString> as core::iter::IntoIterator>::into_iter` at /home/jess/src/rust/library/alloc/src/vec/mod.rs:2649:24
     = note: inside `std::sys::unix::args::imp::args` at /home/jess/src/rust/library/std/src/sys/unix/args.rs:128:22
     = note: inside `std::sys::unix::args::args` at /home/jess/src/rust/library/std/src/sys/unix/args.rs:19:5
     = note: inside `std::env::args_os` at /home/jess/src/rust/library/std/src/env.rs:802:21
     = note: inside `std::env::args` at /home/jess/src/rust/library/std/src/env.rs:767:19
     = note: inside `test::test_main_static` at /home/jess/src/rust/library/test/src/lib.rs:131:16
     = note: inside `main`
