patch
--- bitvec.extend.slow.decompiled.c     2020-11-23 23:19:02.971052349 +0000
+++ bitvec.extend.fast.decompiled.c     2020-11-23 23:10:45.442938517 +0000
@@ -99,11 +99,13 @@
   do {
     bVar2 = *param_2;
     if (bVar2 == 2) {
-                    /* WARNING: Subroutine does not return */
       core::panicking::panic
                 (
                  "called `Option::unwrap()` on a `None`valuelibrary/std/src/panicking.rslibrary/std/src/env.rsfailed to get environmentvariable ``: already mutablyborrowedlibrary/std/src/sys_common/thread_info.rsassertion failed:c.borrow().is_none()Rust panics must be rethrownRust cannot catch foreignexceptionslibrary/std/src/sys/unix/rwlock.rs<unnamed>note: run with`RUST_BACKTRACE=1` environment variable to display a backtrace\n\' panicked at\'\', rwlock maximum reader count exceededrwlock read lock would result indeadlockthread panicked while panicking. aborting.\nthread panicked whileprocessing panic.aborting.\n.debug_library/std/src/../../backtrace/src/symbolize/gimli/elf.rs/home/adema/code/rust/library/alloc/src/vec.rslibrary/std/src/path.rsinternal error:entered unreachable code"
                  ,0x2b,&PTR_UINT_0014f3c8);
+      do {
+        invalidInstructionException();
+      } while( true );
     }
     pointer::BitPtr<T>::into_bitslice(*param_1,param_1[1]);
     pointer::BitPtr<T>::into_bitslice(*param_1);

