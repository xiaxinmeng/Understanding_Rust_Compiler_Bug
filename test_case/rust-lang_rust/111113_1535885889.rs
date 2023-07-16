plain
failures:

---- [codegen] tests/codegen/align-offset.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-offset/align-offset.ll" "/checkout/tests/codegen/align-offset.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
/checkout/tests/codegen/align-offset.rs:74:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/align-offset.rs:74:12: error: CHECK: expected string not found in input
 // CHECK: %[[R:.+]] = select i1 %[[ORIGINAL_ALIGNED]], [[USIZE]] %[[OFFSET]], i64 -1
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-offset/align-offset.ll:66:41: note: scanning from here
 %2 = lshr exact i32 %byte_offset.i.i, 2
                                        ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-offset/align-offset.ll:66:41: note: with "ORIGINAL_ALIGNED" equal to "1"
 %2 = lshr exact i32 %byte_offset.i.i, 2
                                        ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-offset/align-offset.ll:66:41: note: with "USIZE" equal to "i32"
 %2 = lshr exact i32 %byte_offset.i.i, 2
                                        ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-offset/align-offset.ll:66:41: note: with "OFFSET" equal to "2"
 %2 = lshr exact i32 %byte_offset.i.i, 2
                                        ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-offset/align-offset.ll:67:5: note: possible intended match here
 %.0.i.i = select i1 %1, i32 %2, i32 -1


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-offset/align-offset.ll
Check file: /checkout/tests/codegen/align-offset.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'align_offset.d4a577fa05bd2dac-cgu.0' 
            2: source_filename = "align_offset.d4a577fa05bd2dac-cgu.0" 
            3: target datalayout = "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-f64:32:64-f80:32-n8:16:32-S128" 
            4: target triple = "i586-unknown-linux-gnu" 
            5:  
            6: ; Function Attrs: mustprogress nofree nosync nounwind nonlazybind willreturn memory(inaccessiblemem: readwrite) uwtable 
            7: define noundef zeroext i1 @align8(ptr noundef %p) unnamed_addr #0 { 
            8: start: 
            9:  %addr.i.i = ptrtoint ptr %p to i32 
           10:  %_11.i.i = add i32 %addr.i.i, 7 
           11:  %aligned_address.i.i = and i32 %_11.i.i, -8 
           12:  %byte_offset.i.i = sub i32 %aligned_address.i.i, %addr.i.i 
           13:  %_15.i.i = icmp ult i32 %byte_offset.i.i, 8 
           14:  tail call void @llvm.assume(i1 %_15.i.i) 
           15:  ret i1 true 
           16: } 
           17:  
           18: ; Function Attrs: mustprogress nofree nosync nounwind nonlazybind willreturn memory(inaccessiblemem: readwrite) uwtable 
           19: define noundef zeroext i1 @align_to4(ptr noalias noundef nonnull readonly align 1 %x.0, i32 noundef %x.1) unnamed_addr #0 personality ptr @rust_eh_personality { 
           20: start: 
           21:  %addr.i.i = ptrtoint ptr %x.0 to i32 
           22:  %_11.i.i = add i32 %addr.i.i, 3 
           23:  %aligned_address.i.i = and i32 %_11.i.i, -4 
           24:  %byte_offset.i.i = sub i32 %aligned_address.i.i, %addr.i.i 
           25:  %_15.i.i = icmp ult i32 %byte_offset.i.i, 4 
           26:  tail call void @llvm.assume(i1 %_15.i.i) 
           27:  ret i1 true 
           28: } 
           29:  
           30: ; Function Attrs: mustprogress nofree nosync nounwind nonlazybind willreturn memory(inaccessiblemem: readwrite) uwtable 
           31: define noundef i32 @align_offset_byte_ptr(ptr noundef %ptr) unnamed_addr #0 { 
           32: start: 
           33:  %addr.i.i = ptrtoint ptr %ptr to i32 
           34:  %_11.i.i = add i32 %addr.i.i, 31 
           35:  %aligned_address.i.i = and i32 %_11.i.i, -32 
           36:  %byte_offset.i.i = sub i32 %aligned_address.i.i, %addr.i.i 
           37:  %_15.i.i = icmp ult i32 %byte_offset.i.i, 32 
           38:  tail call void @llvm.assume(i1 %_15.i.i) 
           39:  ret i32 %byte_offset.i.i 
           40: } 
           41:  
           42: ; Function Attrs: mustprogress nofree nosync nounwind nonlazybind willreturn memory(inaccessiblemem: readwrite) uwtable 
           43: define noundef i32 @align_offset_word_slice(ptr noalias noundef nonnull readonly align 4 %slice.0, i32 noundef %slice.1) unnamed_addr #0 { 
           44: start: 
           45:  %addr.i.i = ptrtoint ptr %slice.0 to i32 
           46:  %_11.i.i = add i32 %addr.i.i, 31 
           47:  %aligned_address.i.i = and i32 %_11.i.i, -32 
           48:  %byte_offset.i.i = sub i32 %aligned_address.i.i, %addr.i.i 
           49:  %_15.i.i = icmp ult i32 %byte_offset.i.i, 32 
           50:  tail call void @llvm.assume(i1 %_15.i.i) 
           51:  %0 = lshr exact i32 %byte_offset.i.i, 2 
           52:  ret i32 %0 
           53: } 
           54:  
           55: ; Function Attrs: mustprogress nofree nosync nounwind nonlazybind willreturn memory(inaccessiblemem: readwrite) uwtable 
           56: define noundef i32 @align_offset_word_ptr(ptr noundef %ptr) unnamed_addr #0 { 
           57: start: 
           58:  %addr.i.i = ptrtoint ptr %ptr to i32 
           59:  %_11.i.i = add i32 %addr.i.i, 31 
           60:  %aligned_address.i.i = and i32 %_11.i.i, -32 
           61:  %byte_offset.i.i = sub i32 %aligned_address.i.i, %addr.i.i 
           62:  %_15.i.i = icmp ult i32 %byte_offset.i.i, 32 
           63:  tail call void @llvm.assume(i1 %_15.i.i) 
           64:  %0 = and i32 %addr.i.i, 3 
           65:  %1 = icmp eq i32 %0, 0 
           66:  %2 = lshr exact i32 %byte_offset.i.i, 2 
check:74'0                                             X error: no match found
check:74'1                                               with "ORIGINAL_ALIGNED" equal to "1"
check:74'2                                               with "USIZE" equal to "i32"
check:74'3                                               with "OFFSET" equal to "2"
           67:  %.0.i.i = select i1 %1, i32 %2, i32 -1 
check:74'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:74'4         ?                                    possible intended match
           68:  ret i32 %.0.i.i 
check:74'0     ~~~~~~~~~~~~~~~~~
           69: } 
check:74'0     ~~
           70:  
check:74'0     ~
           71: ; Function Attrs: nonlazybind uwtable 
check:74'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           72: declare noundef i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #1 
check:74'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           73:  
check:74'0     ~
           74: ; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) 
check:74'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           75: declare void @llvm.assume(i1 noundef) #2 
check:74'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           76:  
check:74'0     ~
           77: attributes #0 = { mustprogress nofree nosync nounwind nonlazybind willreturn memory(inaccessiblemem: readwrite) uwtable "probe-stack"="inline-asm" "target-cpu"="pentium" } 
check:74'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           78: attributes #1 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="pentium" } 
check:74'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           79: attributes #2 = { mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) } 
check:74'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           80:  
check:74'0     ~
           81: !llvm.module.flags = !{!0, !1} 
check:74'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           82:  
check:74'0     ~
           83: !0 = !{i32 8, !"PIC Level", i32 2} 
check:74'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           84: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:74'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
------------------------------------------



