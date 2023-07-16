plain
failures:

---- [codegen] codegen/consts.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll" "/checkout/obj/build/tmp/distcheck/src/test/codegen/consts.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/obj/build/tmp/distcheck/src/test/codegen/consts.rs:13:11: error: CHECK: expected string not found in input
// CHECK: @alloc9 = {{.*}}, align 2
          ^
/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:10:100: note: scanning from here
@STATIC = constant <{ [4 x i8], [4 x i8] }> <{ [4 x i8] zeroinitializer, [4 x i8] undef }>, align 4
                                                                                                   ^
/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:11:101: note: possible intended match here
@alloc12 = private unnamed_addr constant <{ [3 x i8], [1 x i8] }> <{ [3 x i8] zeroinitializer, [1 x i8] undef }>, align 2
                                                                                                    ^
/checkout/obj/build/tmp/distcheck/src/test/codegen/consts.rs:46:148: error: undefined variable: LOW_HIGH
 // CHECK: memcpy.p0i8.p0i8.i{{(32|64)}}(i8* align 2 %1, i8* align 2 getelementptr inbounds (<{ [4 x i8], [4 x i8] }>, <{ [4 x i8], [4 x i8] }>* [[LOW_HIGH]], i32 0, i32 0, i32 0), i{{(32|64)}} 8, i1 false)
                                                                                                                                                   ^
/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:41:18: note: possible intended match here
 call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 2 %1, i8* align 2 getelementptr inbounds (<{ [4 x i8], [4 x i8] }>, <{ [4 x i8], [4 x i8] }>* @alloc22, i32 0, i32 0, i32 0), i64 8, i1 false)
                 ^
/checkout/obj/build/tmp/distcheck/src/test/codegen/consts.rs:54:148: error: undefined variable: LOW_HIGH
 // CHECK: memcpy.p0i8.p0i8.i{{(32|64)}}(i8* align 4 %1, i8* align 4 getelementptr inbounds (<{ [4 x i8], [4 x i8] }>, <{ [4 x i8], [4 x i8] }>* [[LOW_HIGH]], i32 0, i32 0, i32 0), i{{(32|64)}} 8, i1 false)
                                                                                                                                                   ^
/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll:52:18: note: possible intended match here
 call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %1, i8* align 4 getelementptr inbounds (<{ [4 x i8], [4 x i8] }>, <{ [4 x i8], [4 x i8] }>* @alloc22, i32 0, i32 0, i32 0), i64 8, i1 false)

Input file: /checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/test/codegen/consts/consts.ll
Check file: /checkout/obj/build/tmp/distcheck/src/test/codegen/consts.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
            5:  
            6: %"E<i16, i32>" = type { i16, [3 x i16] } 
            7: %"E<i8, i16>" = type { i16, [1 x i16] } 
            8: %"E<i16, [i16; 3]>" = type { i16, [3 x i16] } 
            9:  
           10: @STATIC = constant <{ [4 x i8], [4 x i8] }> <{ [4 x i8] zeroinitializer, [4 x i8] undef }>, align 4 
check:13'0                                                                                                        X error: no match found
           11: @alloc12 = private unnamed_addr constant <{ [3 x i8], [1 x i8] }> <{ [3 x i8] zeroinitializer, [1 x i8] undef }>, align 2 
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:13'1                                                                                                         ?                      possible intended match
           12: @alloc22 = private unnamed_addr constant <{ [4 x i8], [4 x i8] }> <{ [4 x i8] zeroinitializer, [4 x i8] undef }>, align 4 
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           13:  
check:13'0     ~
           14: ; Function Attrs: nonlazybind uwtable 
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           15: define i64 @static_enum_const() unnamed_addr #0 { 
check:13'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
            .
            .
           32:  %3 = load i32, i32* %2, align 2 
           33:  ret i32 %3 
           34: } 
           35:  
           36: ; Function Attrs: nonlazybind uwtable 
           37: define i64 @low_align_const() unnamed_addr #0 { 
check:46'0                                X~~~~~~~~~~~~~~~~~~~~ error: match failed for invalid pattern
check:46'1                                                      undefined variable: LOW_HIGH
           38: start: 
check:46'0     ~~~~~~~
           39:  %0 = alloca %"E<i16, [i16; 3]>", align 2 
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           40:  %1 = bitcast %"E<i16, [i16; 3]>"* %0 to i8* 
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           41:  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 2 %1, i8* align 2 getelementptr inbounds (<{ [4 x i8], [4 x i8] }>, <{ [4 x i8], [4 x i8] }>* @alloc22, i32 0, i32 0, i32 0), i64 8, i1 false) 
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:46'2                      ?                                                                                                                                                                              possible intended match
           42:  %2 = bitcast %"E<i16, [i16; 3]>"* %0 to i64* 
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           43:  %3 = load i64, i64* %2, align 2 
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           44:  ret i64 %3 
check:46'0     ~~~~~~~~~~~~
           45: } 
check:46'0     ~~
           46:  
check:46'0     ~
           47: ; Function Attrs: nonlazybind uwtable 
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           48: define i64 @high_align_const() unnamed_addr #0 { 
check:46'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:54'0                                 X~~~~~~~~~~~~~~~~~~~~ error: match failed for invalid pattern
check:54'1                                                       undefined variable: LOW_HIGH
           49: start: 
check:54'0     ~~~~~~~
           50:  %0 = alloca %"E<i16, i32>", align 4 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           51:  %1 = bitcast %"E<i16, i32>"* %0 to i8* 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           52:  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %1, i8* align 4 getelementptr inbounds (<{ [4 x i8], [4 x i8] }>, <{ [4 x i8], [4 x i8] }>* @alloc22, i32 0, i32 0, i32 0), i64 8, i1 false) 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:54'2                      ?                                                                                                                                                                              possible intended match
           53:  %2 = bitcast %"E<i16, i32>"* %0 to i64* 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           54:  %3 = load i64, i64* %2, align 4 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           55:  ret i64 %3 
check:54'0     ~~~~~~~~~~~~
           56: } 
check:54'0     ~~
           57:  
check:54'0     ~
            .
            .
>>>>>>
------------------------------------------
---

test result: FAILED. 287 passed; 1 failed; 40 ignored; 0 measured; 0 filtered out; finished in 3.66s

Build completed unsuccessfully in 0:14:32
make: *** [check] Error 1
Makefile:42: recipe for target 'check' failed
