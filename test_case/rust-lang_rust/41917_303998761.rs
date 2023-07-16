
[00:45:50]    Compiling std v0.0.0 (file:///C:/projects/rust/src/libstd)
[00:45:51] The unwind destination does not have an exception handling instruction!
[00:45:51]   %10 = invoke i64 @"_ZN53_$LT$$u5b$T$u5d$$u20$as$u20$core..slice..SliceExt$GT$3len17h7d1f8aa2bdfa7863E"(%"num::flt2dec::Part"* noalias nonnull readonly %7, i64 %8)
[00:45:51]           to label %bb3 unwind label %bb2
[00:45:51] Instruction does not dominate all uses!
[00:45:51]   %cleanuppad = cleanuppad within none []
[00:45:51]   cleanupret from %cleanuppad unwind to caller
[00:45:51] The unwind destination does not have an exception handling instruction!
[00:45:51]   invoke void @_ZN4core9panicking5panic17h84035043e1652bf0E({ %str_slice, [0 x i8], %str_slice, [0 x i8], i32, [4 x i8] }* noalias readonly dereferenceable(40) bitcast
 ({ %str_slice, %str_slice, i32, [4 x i8] }* @_ZN4core3num7flt2dec15to_shortest_str14_MSG_FILE_LINE17h65ae36d95f0d6071E to { %str_slice, [0 x i8], %str_slice, [0 x i8], i32, [4 x
 i8] }*))
[00:45:51]           to label %unreachable unwind label %bb2
[00:45:51] The unwind destination does not have an exception handling instruction!
[00:45:51]   %13 = invoke i64 @"_ZN53_$LT$$u5b$T$u5d$$u20$as$u20$core..slice..SliceExt$GT$3len17hd3b275f0fa02ddc7E"(i8* noalias nonnull readonly %5, i64 %6)
[00:45:51]           to label %bb6 unwind label %bb2
[00:45:51] The unwind destination does not have an exception handling instruction!
[00:45:51]   invoke void @_ZN4core9panicking5panic17h84035043e1652bf0E({ %str_slice, [0 x i8], %str_slice, [0 x i8], i32, [4 x i8] }* noalias readonly dereferenceable(40) bitcast
 ({ %str_slice, %str_slice, i32, [4 x i8] }* @_ZN4core3num7flt2dec15to_shortest_str14_MSG_FILE_LINE17h043281eb634e5d92E to { %str_slice, [0 x i8], %str_slice, [0 x i8], i32, [4 x
 i8] }*))
[00:45:51]           to label %unreachable unwind label %bb2
