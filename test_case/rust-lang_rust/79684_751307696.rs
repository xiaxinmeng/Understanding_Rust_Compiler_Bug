plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Checking rand v0.7.3
    Checking core v0.0.0 (/checkout/library/core)
    Checking std v0.0.0 (/checkout/library/std)
    Checking alloc v0.0.0 (/checkout/library/alloc)
error: any use of this value will cause an error
     |
     |
1868 |     unsafe { copy_nonoverlapping(src, dst, count) }
     |              |
     |              |
     |              "calling intrinsic `copy_nonoverlapping`" needs an rfc before being allowed inside constants
     |              inside `copy_nonoverlapping::<i32>` at /checkout/library/core/src/intrinsics.rs:1868:14
     |              inside `std::ptr::read::<i32>` at /checkout/library/core/src/ptr/mod.rs:699:9
     |              inside `const_ptr::read::FOO` at library/core/tests/const_ptr.rs:13:31
    ::: library/core/tests/const_ptr.rs:13:5
     |
     |
13   |     const FOO: i32 = unsafe { ptr::read(&42 as *const i32) };
     |
     |
     = note: `#[deny(const_err)]` on by default

error: any use of this value will cause an error
     |
     |
1868 |     unsafe { copy_nonoverlapping(src, dst, count) }
     |              |
     |              |
     |              "calling intrinsic `copy_nonoverlapping`" needs an rfc before being allowed inside constants
     |              inside `copy_nonoverlapping::<u8>` at /checkout/library/core/src/intrinsics.rs:1868:14
     |              inside `read_unaligned::<i32>` at /checkout/library/core/src/ptr/mod.rs:799:9
     |              inside `const_ptr::read::ALIGNED` at library/core/tests/const_ptr.rs:16:35
    ::: library/core/tests/const_ptr.rs:16:5
     |
     |
16   |     const ALIGNED: i32 = unsafe { ptr::read_unaligned(&42 as *const i32) };


error: any use of this value will cause an error
     |
     |
1868 |     unsafe { copy_nonoverlapping(src, dst, count) }
     |              |
     |              |
     |              "calling intrinsic `copy_nonoverlapping`" needs an rfc before being allowed inside constants
     |              inside `copy_nonoverlapping::<u8>` at /checkout/library/core/src/intrinsics.rs:1868:14
     |              inside `read_unaligned::<u16>` at /checkout/library/core/src/ptr/mod.rs:799:9
     |              inside `const_ptr::read::UNALIGNED` at library/core/tests/const_ptr.rs:21:37
    ::: library/core/tests/const_ptr.rs:21:5
     |
     |
21   |     const UNALIGNED: u16 = unsafe { ptr::read_unaligned(UNALIGNED_PTR) };


error: any use of this value will cause an error
     |
     |
1868 |     unsafe { copy_nonoverlapping(src, dst, count) }
     |              |
     |              |
     |              "calling intrinsic `copy_nonoverlapping`" needs an rfc before being allowed inside constants
     |              inside `copy_nonoverlapping::<i32>` at /checkout/library/core/src/intrinsics.rs:1868:14
     |              inside `std::ptr::read::<i32>` at /checkout/library/core/src/ptr/mod.rs:699:9
     |              inside `std::ptr::const_ptr::<impl *const i32>::read` at /checkout/library/core/src/ptr/const_ptr.rs:735:18
     |              inside `const_ptr::const_ptr_read::FOO` at library/core/tests/const_ptr.rs:27:31
    ::: library/core/tests/const_ptr.rs:27:5
     |
     |
27   |     const FOO: i32 = unsafe { (&42 as *const i32).read() };


error: any use of this value will cause an error
     |
     |
1868 |     unsafe { copy_nonoverlapping(src, dst, count) }
     |              |
     |              |
     |              "calling intrinsic `copy_nonoverlapping`" needs an rfc before being allowed inside constants
     |              inside `copy_nonoverlapping::<u8>` at /checkout/library/core/src/intrinsics.rs:1868:14
     |              inside `read_unaligned::<i32>` at /checkout/library/core/src/ptr/mod.rs:799:9
     |              inside `std::ptr::const_ptr::<impl *const i32>::read_unaligned` at /checkout/library/core/src/ptr/const_ptr.rs:774:18
     |              inside `const_ptr::const_ptr_read::ALIGNED` at library/core/tests/const_ptr.rs:30:35
    ::: library/core/tests/const_ptr.rs:30:5
     |
     |
30   |     const ALIGNED: i32 = unsafe { (&42 as *const i32).read_unaligned() };


error: any use of this value will cause an error
     |
     |
1868 |     unsafe { copy_nonoverlapping(src, dst, count) }
     |              |
     |              |
     |              "calling intrinsic `copy_nonoverlapping`" needs an rfc before being allowed inside constants
     |              inside `copy_nonoverlapping::<u8>` at /checkout/library/core/src/intrinsics.rs:1868:14
     |              inside `read_unaligned::<u16>` at /checkout/library/core/src/ptr/mod.rs:799:9
     |              inside `std::ptr::const_ptr::<impl *const u16>::read_unaligned` at /checkout/library/core/src/ptr/const_ptr.rs:774:18
     |              inside `const_ptr::const_ptr_read::UNALIGNED` at library/core/tests/const_ptr.rs:35:37
    ::: library/core/tests/const_ptr.rs:35:5
     |
     |
35   |     const UNALIGNED: u16 = unsafe { UNALIGNED_PTR.read_unaligned() };


error: any use of this value will cause an error
     |
     |
1868 |     unsafe { copy_nonoverlapping(src, dst, count) }
     |              |
     |              |
     |              "calling intrinsic `copy_nonoverlapping`" needs an rfc before being allowed inside constants
     |              inside `copy_nonoverlapping::<i32>` at /checkout/library/core/src/intrinsics.rs:1868:14
     |              inside `std::ptr::read::<i32>` at /checkout/library/core/src/ptr/mod.rs:699:9
     |              inside `std::ptr::mut_ptr::<impl *mut i32>::read` at /checkout/library/core/src/ptr/mut_ptr.rs:842:18
     |              inside `const_ptr::mut_ptr_read::FOO` at library/core/tests/const_ptr.rs:41:31
    ::: library/core/tests/const_ptr.rs:41:5
     |
     |
41   |     const FOO: i32 = unsafe { (&42 as *const i32 as *mut i32).read() };


error: any use of this value will cause an error
     |
     |
1868 |     unsafe { copy_nonoverlapping(src, dst, count) }
     |              |
     |              |
     |              "calling intrinsic `copy_nonoverlapping`" needs an rfc before being allowed inside constants
     |              inside `copy_nonoverlapping::<u8>` at /checkout/library/core/src/intrinsics.rs:1868:14
     |              inside `read_unaligned::<i32>` at /checkout/library/core/src/ptr/mod.rs:799:9
     |              inside `std::ptr::mut_ptr::<impl *mut i32>::read_unaligned` at /checkout/library/core/src/ptr/mut_ptr.rs:881:18
     |              inside `const_ptr::mut_ptr_read::ALIGNED` at library/core/tests/const_ptr.rs:44:35
    ::: library/core/tests/const_ptr.rs:44:5
     |
     |
44   |     const ALIGNED: i32 = unsafe { (&42 as *const i32 as *mut i32).read_unaligned() };


error: any use of this value will cause an error
     |
     |
1868 |     unsafe { copy_nonoverlapping(src, dst, count) }
     |              |
     |              |
     |              "calling intrinsic `copy_nonoverlapping`" needs an rfc before being allowed inside constants
     |              inside `copy_nonoverlapping::<u8>` at /checkout/library/core/src/intrinsics.rs:1868:14
     |              inside `read_unaligned::<u16>` at /checkout/library/core/src/ptr/mod.rs:799:9
     |              inside `std::ptr::mut_ptr::<impl *mut u16>::read_unaligned` at /checkout/library/core/src/ptr/mut_ptr.rs:881:18
     |              inside `const_ptr::mut_ptr_read::UNALIGNED` at library/core/tests/const_ptr.rs:49:37
    ::: library/core/tests/const_ptr.rs:49:5
     |
     |
49   |     const UNALIGNED: u16 = unsafe { UNALIGNED_PTR.read_unaligned() };


error: any use of this value will cause an error
    |
    |
584 |             intrinsics::assert_inhabited::<T>();
    |             |
    |             |
    |             "calling intrinsic `assert_inhabited`" needs an rfc before being allowed inside constants
    |             inside `std::mem::MaybeUninit::<u32>::assume_init_read` at /checkout/library/core/src/mem/maybe_uninit.rs:584:13
    |             inside `mem::uninit_const_assume_init_read::FOO` at library/core/tests/mem.rs:273:31
   ::: library/core/tests/mem.rs:273:5
    |
    |
273 |     const FOO: u32 = unsafe { MaybeUninit::new(42).assume_init_read() };

error: aborting due to 10 previous errors

error: could not compile `core`
error: could not compile `core`

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--all-targets" "-p" "test" "-p" "core" "-p" "term" "-p" "proc_macro" "-p" "std" "-p" "unwind" "-p" "alloc" "-p" "panic_unwind" "-p" "panic_abort" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu --all-targets
Build completed unsuccessfully in 0:00:47
