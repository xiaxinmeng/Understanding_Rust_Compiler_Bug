plain

running 354 tests
ii.............i....i..ii.................iii........ii.i.......i.................ii.... 88/354
.............i............i..i.................i...iii........i..i......iii.ii........i. 176/354
.ii....i..F...............i...i...i.....ii..i..ii................ii..................... 264/354
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..
failures:


---- [codegen] src/test/codegen/issue-75659.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75659/issue-75659.ll" "/checkout/src/test/codegen/issue-75659.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/issue-75659.rs:12:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: memchr
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75659/issue-75659.ll:10:21: note: found here
; call core::slice::memchr::memchr_naive
                    ^~~~~~
/checkout/src/test/codegen/issue-75659.rs:20:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: memchr
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75659/issue-75659.ll:21:21: note: found here
; call core::slice::memchr::memchr_naive
                    ^~~~~~
/checkout/src/test/codegen/issue-75659.rs:28:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: memchr
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75659/issue-75659.ll:32:21: note: found here
; call core::slice::memchr::memchr_naive
                    ^~~~~~
/checkout/src/test/codegen/issue-75659.rs:36:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: memchr
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75659/issue-75659.ll:43:21: note: found here
; call core::slice::memchr::memchr_naive
                    ^~~~~~
/checkout/src/test/codegen/issue-75659.rs:44:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: memchr
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75659/issue-75659.ll:54:21: note: found here
; call core::slice::memchr::memchr_naive
                    ^~~~~~
/checkout/src/test/codegen/issue-75659.rs:52:16: error: CHECK-NOT: excluded string found in input
 // CHECK-NOT: memchr
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75659/issue-75659.ll:65:21: note: found here
; call core::slice::memchr::memchr_naive

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75659/issue-75659.ll
Check file: /checkout/src/test/codegen/issue-75659.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
        .
        .
        .
        .
        5:  
        6: ; Function Attrs: nonlazybind uwtable 
        7: define noundef zeroext i1 @foo1(i8 %0, [1 x i8]* noalias noundef readonly align 1 dereferenceable(1) %data) unnamed_addr #0 { 
        8: start: 
        9:  %_3.0 = bitcast [1 x i8]* %data to [0 x i8]* 
       10: ; call core::slice::memchr::memchr_naive 
not:12                         !~~~~~                error: no match expected
       11:  %1 = tail call { i64, i64 } @_ZN4core5slice6memchr12memchr_naive17h4fa07a27a2dde398E(i8 %0, [0 x i8]* noalias noundef nonnull readonly align 1 %_3.0, i64 1), !noalias !2 
       12:  %oldret.i.i.i = extractvalue { i64, i64 } %1, 0 
       13:  %2 = icmp eq i64 %oldret.i.i.i, 1 
       14:  ret i1 %2 
       15: } 
       16:  
       17: ; Function Attrs: nonlazybind uwtable 
       18: define noundef zeroext i1 @foo2(i8 %0, [2 x i8]* noalias noundef readonly align 1 dereferenceable(2) %data) unnamed_addr #0 { 
       19: start: 
       20:  %_3.0 = bitcast [2 x i8]* %data to [0 x i8]* 
       21: ; call core::slice::memchr::memchr_naive 
not:20                         !~~~~~                error: no match expected
       22:  %1 = tail call { i64, i64 } @_ZN4core5slice6memchr12memchr_naive17h4fa07a27a2dde398E(i8 %0, [0 x i8]* noalias noundef nonnull readonly align 1 %_3.0, i64 2), !noalias !7 
       23:  %oldret.i.i.i = extractvalue { i64, i64 } %1, 0 
       24:  %2 = icmp eq i64 %oldret.i.i.i, 1 
       25:  ret i1 %2 
       26: } 
       27:  
       28: ; Function Attrs: nonlazybind uwtable 
       29: define noundef zeroext i1 @foo3(i8 %0, [3 x i8]* noalias noundef readonly align 1 dereferenceable(3) %data) unnamed_addr #0 { 
       30: start: 
       31:  %_3.0 = bitcast [3 x i8]* %data to [0 x i8]* 
       32: ; call core::slice::memchr::memchr_naive 
not:28                         !~~~~~                error: no match expected
       33:  %1 = tail call { i64, i64 } @_ZN4core5slice6memchr12memchr_naive17h4fa07a27a2dde398E(i8 %0, [0 x i8]* noalias noundef nonnull readonly align 1 %_3.0, i64 3), !noalias !12 
       34:  %oldret.i.i.i = extractvalue { i64, i64 } %1, 0 
       35:  %2 = icmp eq i64 %oldret.i.i.i, 1 
       36:  ret i1 %2 
       37: } 
       38:  
       39: ; Function Attrs: nonlazybind uwtable 
       40: define noundef zeroext i1 @foo4(i8 %0, [4 x i8]* noalias noundef readonly align 1 dereferenceable(4) %data) unnamed_addr #0 { 
       41: start: 
       42:  %_3.0 = bitcast [4 x i8]* %data to [0 x i8]* 
       43: ; call core::slice::memchr::memchr_naive 
not:36                         !~~~~~                error: no match expected
       44:  %1 = tail call { i64, i64 } @_ZN4core5slice6memchr12memchr_naive17h4fa07a27a2dde398E(i8 %0, [0 x i8]* noalias noundef nonnull readonly align 1 %_3.0, i64 4), !noalias !17 
       45:  %oldret.i.i.i = extractvalue { i64, i64 } %1, 0 
       46:  %2 = icmp eq i64 %oldret.i.i.i, 1 
       47:  ret i1 %2 
       48: } 
       49:  
       50: ; Function Attrs: nonlazybind uwtable 
       51: define noundef zeroext i1 @foo8(i8 %0, [8 x i8]* noalias noundef readonly align 1 dereferenceable(8) %data) unnamed_addr #0 { 
       52: start: 
       53:  %_3.0 = bitcast [8 x i8]* %data to [0 x i8]* 
       54: ; call core::slice::memchr::memchr_naive 
not:44                         !~~~~~                error: no match expected
       55:  %1 = tail call { i64, i64 } @_ZN4core5slice6memchr12memchr_naive17h4fa07a27a2dde398E(i8 %0, [0 x i8]* noalias noundef nonnull readonly align 1 %_3.0, i64 8), !noalias !22 
       56:  %oldret.i.i.i = extractvalue { i64, i64 } %1, 0 
       57:  %2 = icmp eq i64 %oldret.i.i.i, 1 
       58:  ret i1 %2 
       59: } 
       60:  
       61: ; Function Attrs: nonlazybind uwtable 
       62: define noundef zeroext i1 @foo8_i8(i8 %0, [8 x i8]* noalias noundef readonly align 1 dereferenceable(8) %data) unnamed_addr #0 { 
       63: start: 
       64:  %_4.0 = bitcast [8 x i8]* %data to [0 x i8]* 
       65: ; call core::slice::memchr::memchr_naive 
not:52                         !~~~~~                error: no match expected
       66:  %1 = tail call { i64, i64 } @_ZN4core5slice6memchr12memchr_naive17h4fa07a27a2dde398E(i8 %0, [0 x i8]* noalias noundef nonnull readonly align 1 %_4.0, i64 8), !noalias !27 
       67:  %oldret.i.i.i = extractvalue { i64, i64 } %1, 0 
       68:  %2 = icmp ne i64 %oldret.i.i.i, 1 
       69:  ret i1 %2 
       70: } 
        .
        .
>>>>>>
------------------------------------------
