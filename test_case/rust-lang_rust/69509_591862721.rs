
/home/r/src/rust/rustc.3/src/test/codegen/repeat-trusted-len.rs:16:11: error: CHECK: expected string not found in input
// CHECK: call void @llvm.memset.p0i8.[[USIZE]](i8* {{(nonnull )?}}align 1{{.*}} %{{[0-9]+}}, i8 42, [[USIZE]] 100000, i1 false)
          ^
/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len/repeat-trusted-len.ll:17:33: note: scanning from here
define void @repeat_take_collect(%"alloc::vec::Vec<u8>"* noalias nocapture sret dereferenceable(12)) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
                                ^
/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len/repeat-trusted-len.ll:17:33: note: with "USIZE" equal to "i32"
define void @repeat_take_collect(%"alloc::vec::Vec<u8>"* noalias nocapture sret dereferenceable(12)) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
                                ^
/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len/repeat-trusted-len.ll:17:33: note: with "USIZE" equal to "i32"
define void @repeat_take_collect(%"alloc::vec::Vec<u8>"* noalias nocapture sret dereferenceable(12)) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
                                ^
/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/test/codegen/repeat-trusted-len/repeat-trusted-len.ll:30:2: note: possible intended match here
 tail call void @llvm.memset.p0i8.i64(i8* nonnull align 1 %1, i8 42, i64 100000, i1 false) #4, !noalias !11
 ^
