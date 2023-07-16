plain
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 379 tests
i.......i............i....i..ii.................iii.........iii........i................ 88/379
.ii.................i............i.Fi..................i...iii........i..i......iii.ii.. 176/379
..............ii.ii.i.i............i..i....i.......iii......i...i.....................ii 352/379
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
iiiiii.i...................
failures:
failures:

---- [codegen] src/test/codegen/function-arguments-noopt.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll" "/checkout/src/test/codegen/function-arguments-noopt.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/function-arguments-noopt.rs:39:11: error: CHECK: expected string not found in input
// CHECK: void @struct_({{%S\*|ptr}} sret(%S){{( %0)?}}, {{%S\*|ptr}} %x)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:33:44: note: scanning from here
 %0 = call align 4 i32* %f(i32* align 4 %x)
                                           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:41:1: note: possible intended match here
define void @struct_(%S* sret(%S) align 4 %0, %S* align 4 %x) unnamed_addr #0 {
/checkout/src/test/codegen/function-arguments-noopt.rs:48:12: error: CHECK: expected string not found in input
/checkout/src/test/codegen/function-arguments-noopt.rs:48:12: error: CHECK: expected string not found in input
 // CHECK: call void %f({{%S\*|ptr}} sret(%S){{( %0)?}}, {{%S\*|ptr}} %{{.+}})
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:50:25: note: scanning from here
define void @struct_call(%S* sret(%S) align 4 %0, %S* align 4 %x, void (%S*, %S*)* %f) unnamed_addr #0 {
                        ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:56:2: note: possible intended match here
 call void %f(%S* sret(%S) align 4 %0, %S* align 4 %_4)

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll
Check file: /checkout/src/test/codegen/function-arguments-noopt.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
           28: } 
           29:  
           30: ; Function Attrs: nonlazybind uwtable 
           31: define align 4 i32* @borrow_call(i32* align 4 %x, i32* (i32*)* %f) unnamed_addr #0 { 
           32: start: 
           33:  %0 = call align 4 i32* %f(i32* align 4 %x) 
check:39'0                                                X error: no match found
           34:  br label %bb1 
check:39'0     ~~~~~~~~~~~~~~~
           35:  
check:39'0     ~
           36: bb1: ; preds = %start 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~
           37:  ret i32* %0 
check:39'0     ~~~~~~~~~~~~~
           38: } 
check:39'0     ~~
           39:  
check:39'0     ~
           40: ; Function Attrs: nonlazybind uwtable 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           41: define void @struct_(%S* sret(%S) align 4 %0, %S* align 4 %x) unnamed_addr #0 { 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:39'1     ?                                                                                possible intended match
           42: start: 
check:39'0     ~~~~~~~
           43:  %1 = bitcast %S* %0 to i8* 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           44:  %2 = bitcast %S* %x to i8* 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           45:  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %1, i8* align 4 %2, i64 32, i1 false) 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           46:  ret void 
check:39'0     ~~~~~~~~~~
           47: } 
check:39'0     ~~
           48:  
check:39'0     ~
           49: ; Function Attrs: nonlazybind uwtable 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           50: define void @struct_call(%S* sret(%S) align 4 %0, %S* align 4 %x, void (%S*, %S*)* %f) unnamed_addr #0 { 
check:39'0     ~~~~~~~~~~~~~~~~~~~~~~~~
check:48'0                             X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           51: start: 
check:48'0     ~~~~~~~
           52:  %_4 = alloca %S, align 4 
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
           53:  %1 = bitcast %S* %_4 to i8* 
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           54:  %2 = bitcast %S* %x to i8* 
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           55:  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %1, i8* align 4 %2, i64 32, i1 false) 
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           56:  call void %f(%S* sret(%S) align 4 %0, %S* align 4 %_4) 
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:48'1      ?                                                       possible intended match
           57:  br label %bb1 
check:48'0     ~~~~~~~~~~~~~~~
           58:  
check:48'0     ~
           59: bb1: ; preds = %start 
check:48'0     ~~~~~~~~~~~~~~~~~~~~~~
           60:  ret void 
check:48'0     ~~~~~~~~~~
           61: } 
check:48'0     ~~
            .
            .
>>>>>>
------------------------------------------
