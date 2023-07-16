
C:\projects\rust\build\x86_64-pc-windows-gnu\test\codegen\slice-position-bounds-check\slice-position-bounds-check.ll:13:233: error: CHECK-NOT: string occurred!
define zeroext i1 @position_no_bounds_check([0 x i32]* noalias nonnull %y.0, i64 %y.1, i32* noalias nocapture readonly dereferenceable(4) %x, i32* noalias nocapture readonly dereferenceable(4) %z) unnamed_addr #0 personality i32 (%"panic_unwind::windows::EXCEPTION_RECORD"*, i8*, %"panic_unwind::windows::CONTEXT"*, %"panic_unwind::windows::DISPATCHER_CONTEXT"*)* @rust_eh_personality {
                                                                                                                                                                                                                                        ^
C:\projects\rust\src/test\codegen\slice-position-bounds-check.rs:28:16: note: CHECK-NOT: pattern specified here
 // CHECK-NOT: panic
               ^
