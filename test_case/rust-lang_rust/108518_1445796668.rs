plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 409 tests
i.....i..i............i.....i.ii.................iii........i.i.i........i.............. 88/409
.....ii.................i................i..Fi...............i.....i..iiii.....F.i..i... 176/409
...............ii........................i.i.ii.i.i...............i....i....i..iii...... 352/409
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..i..i......................iiiiiiii.i...................
failures:
failures:

---- [codegen] tests/codegen/function-arguments-noopt.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-14/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll" "/checkout/tests/codegen/function-arguments-noopt.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/function-arguments-noopt.rs:45:11: error: CHECK: expected string not found in input
// CHECK: void @struct_({{%S\*|ptr}} sret(%S){{( %0)?}}, {{%S\*|ptr}} %x)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:36:44: note: scanning from here
 %0 = call align 4 i32* %f(i32* align 4 %x)
                                           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:41:1: note: possible intended match here
define void @struct_(%S* sret(%S) align 4 %0, %S* align 4 %x) unnamed_addr #0 {
/checkout/tests/codegen/function-arguments-noopt.rs:54:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/function-arguments-noopt.rs:54:12: error: CHECK: expected string not found in input
 // CHECK: call void %f({{%S\*|ptr}} sret(%S){{( %0)?}}, {{%S\*|ptr}} %{{.+}})
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:50:25: note: scanning from here
define void @struct_call(%S* sret(%S) align 4 %0, %S* align 4 %x, void (%S*, %S*)* %f) unnamed_addr #0 {
                        ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:52:2: note: possible intended match here
 call void %f(%S* sret(%S) align 4 %0, %S* align 4 %x)

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll
Check file: /checkout/tests/codegen/function-arguments-noopt.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'function_arguments_noopt.8beba419-cgu.0' 
            2: source_filename = "function_arguments_noopt.8beba419-cgu.0" 
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
            4: target triple = "x86_64-unknown-linux-gnu" 
            5:  
            6: %S = type { [8 x i32] } 
            7:  
            8: ; Function Attrs: nonlazybind uwtable 
            9: define zeroext i1 @boolean(i1 zeroext %x) unnamed_addr #0 { 
           10: start: 
           11:  ret i1 %x 
           12: } 
           13:  
           14: ; Function Attrs: nonlazybind uwtable 
           15: define zeroext i1 @boolean_call(i1 zeroext %x, i1 (i1)* %f) unnamed_addr #0 { 
           16: start: 
           17:  %0 = call zeroext i1 %f(i1 zeroext %x) 
           18:  ret i1 %0 
           19: } 
           20:  
           21: ; Function Attrs: nonlazybind uwtable 
           22: define align 4 i32* @borrow(i32* align 4 %x) unnamed_addr #0 { 
           23: start: 
           24:  ret i32* %x 
           25: } 
           26:  
           27: ; Function Attrs: nonlazybind uwtable 
           28: define align 4 i32* @borrow_mut(i32* align 4 %x) unnamed_addr #0 { 
           29: start: 
           30:  ret i32* %x 
           31: } 
           32:  
           33: ; Function Attrs: nonlazybind uwtable 
           34: define align 4 i32* @borrow_call(i32* align 4 %x, i32* (i32*)* %f) unnamed_addr #0 { 
           35: start: 
           36:  %0 = call align 4 i32* %f(i32* align 4 %x) 
check:45'0                                                X error: no match found
           37:  ret i32* %0 
check:45'0     ~~~~~~~~~~~~~
           38: } 
check:45'0     ~~
           39:  
check:45'0     ~
           40: ; Function Attrs: nonlazybind uwtable 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           41: define void @struct_(%S* sret(%S) align 4 %0, %S* align 4 %x) unnamed_addr #0 { 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:45'1     ?                                                                                possible intended match
           42: start: 
check:45'0     ~~~~~~~
           43:  %1 = bitcast %S* %0 to i8* 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           44:  %2 = bitcast %S* %x to i8* 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           45:  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %1, i8* align 4 %2, i64 32, i1 false) 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           46:  ret void 
check:45'0     ~~~~~~~~~~
           47: } 
check:45'0     ~~
           48:  
check:45'0     ~
           49: ; Function Attrs: nonlazybind uwtable 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           50: define void @struct_call(%S* sret(%S) align 4 %0, %S* align 4 %x, void (%S*, %S*)* %f) unnamed_addr #0 { 
check:45'0     ~~~~~~~~~~~~~~~~~~~~~~~~
check:54'0                             X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
           51: start: 
check:54'0     ~~~~~~~
           52:  call void %f(%S* sret(%S) align 4 %0, %S* align 4 %x) 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:54'1      ?                                                      possible intended match
           53:  ret void 
check:54'0     ~~~~~~~~~~
           54: } 
check:54'0     ~~
           55:  
check:54'0     ~
           56: ; Function Attrs: nonlazybind uwtable 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           57: define { i8, i8 } @enum_(i1 zeroext %x.0, i8 %x.1) unnamed_addr #0 { 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           58: start: 
check:54'0     ~~~~~~~
           59:  %0 = zext i1 %x.0 to i8 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
           60:  %1 = insertvalue { i8, i8 } undef, i8 %0, 0 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           61:  %2 = insertvalue { i8, i8 } %1, i8 %x.1, 1 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           62:  ret { i8, i8 } %2 
check:54'0     ~~~~~~~~~~~~~~~~~~~
           63: } 
check:54'0     ~~
           64:  
check:54'0     ~
           65: ; Function Attrs: nonlazybind uwtable 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           66: define { i8, i8 } @enum_call(i1 zeroext %x.0, i8 %x.1, { i8, i8 } (i1, i8)* %f) unnamed_addr #0 { 
check:54'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           67: start: 
           68:  %0 = call { i8, i8 } %f(i1 zeroext %x.0, i8 %x.1) 
           69:  %1 = extractvalue { i8, i8 } %0, 0 
           70:  %2 = trunc i8 %1 to i1 
           71:  %3 = extractvalue { i8, i8 } %0, 1 
           72:  %4 = zext i1 %2 to i8 
           73:  %5 = insertvalue { i8, i8 } undef, i8 %4, 0 
           74:  %6 = insertvalue { i8, i8 } %5, i8 %3, 1 
           75:  ret { i8, i8 } %6 
           76: } 
           77:  
           78: ; Function Attrs: argmemonly nofree nounwind willreturn 
           79: declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #1 
           80:  
           81: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
           82: attributes #1 = { argmemonly nofree nounwind willreturn } 
           83:  
           84: !llvm.module.flags = !{!0, !1} 
           85:  
           86: !0 = !{i32 7, !"PIC Level", i32 2} 
           87: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
------------------------------------------


---- [codegen] tests/codegen/function-arguments.rs stdout ----
---- [codegen] tests/codegen/function-arguments.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-14/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll" "/checkout/tests/codegen/function-arguments.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/function-arguments.rs:145:11: error: CHECK: expected string not found in input
// CHECK: @indirect_struct({{%S\*|ptr}} noalias nocapture noundef readonly dereferenceable(32) %_1)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll:461:91: note: scanning from here
define void @notunpin_borrow(i32* noalias noundef readonly align 4 dereferenceable(4) %_1) unnamed_addr #0 {
                                                                                          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll:467:10: note: possible intended match here
define void @indirect_struct(%S* noalias nocapture noundef readonly align 4 dereferenceable(32) %_1) unnamed_addr #0 {

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll
Check file: /checkout/tests/codegen/function-arguments.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
             .
             .
             .
             .
           361:  ret i1 %x 
           362: } 
           363:  
           364: ; Function Attrs: nonlazybind uwtable 
           365: define i8 @maybeuninit_enum_bool(i8 %x) unnamed_addr #0 { 
           366: start: 
           367:  ret i8 %x 
           368: } 
           369:  
           370: ; Function Attrs: nonlazybind uwtable 
           371: define noundef i32 @char(i32 noundef %x) unnamed_addr #0 { 
           372: start: 
           373:  ret i32 %x 
           374: } 
           375:  
           376: ; Function Attrs: nonlazybind uwtable 
           377: define i32 @maybeuninit_char(i32 %x) unnamed_addr #0 { 
           378: start: 
           379:  ret i32 %x 
           380: } 
           381:  
           382: ; Function Attrs: nonlazybind uwtable 
           383: define noundef i64 @int(i64 noundef %x) unnamed_addr #0 { 
           384: start: 
           385:  ret i64 %x 
           386: } 
           387:  
           388: ; Function Attrs: nonlazybind uwtable 
           389: define noundef i64 @nonzero_int(i64 noundef %x) unnamed_addr #0 { 
           390: start: 
           391:  ret i64 %x 
           392: } 
           393:  
           394: ; Function Attrs: nonlazybind uwtable 
           395: define noundef i64 @option_nonzero_int(i64 noundef %x) unnamed_addr #0 { 
           396: start: 
           397:  ret i64 %x 
           398: } 
           399:  
           400: ; Function Attrs: nonlazybind uwtable 
           401: define void @readonly_borrow(i32* noalias noundef readonly align 4 dereferenceable(4) %_1) unnamed_addr #0 { 
           403:  ret void 
           404: } 
           405:  
           405:  
           406: ; Function Attrs: nonlazybind uwtable 
           407: define noundef align 4 dereferenceable(4) i32* @readonly_borrow_ret() unnamed_addr #0 { 
           408: start: 
           409:  br label %bb1 
           410:  
           411: bb1: ; preds = %bb1, %start 
           412:  br label %bb1 
           413: } 
           414:  
           415: ; Function Attrs: nonlazybind uwtable 
           416: define void @static_borrow(i32* noalias noundef readonly align 4 dereferenceable(4) %_1) unnamed_addr #0 { 
           418:  ret void 
           419: } 
           420:  
           420:  
           421: ; Function Attrs: nonlazybind uwtable 
           422: define void @named_borrow(i32* noalias noundef readonly align 4 dereferenceable(4) %_1) unnamed_addr #0 { 
           424:  ret void 
           425: } 
           426:  
           426:  
           427: ; Function Attrs: nonlazybind uwtable 
           428: define void @unsafe_borrow(i16* noundef nonnull align 2 %_1) unnamed_addr #0 { 
           430:  ret void 
           431: } 
           432:  
           432:  
           433: ; Function Attrs: nonlazybind uwtable 
           434: define void @mutable_unsafe_borrow(i16* noalias noundef align 2 dereferenceable(2) %_1) unnamed_addr #0 { 
           436:  ret void 
           437: } 
           438:  
           438:  
           439: ; Function Attrs: nonlazybind uwtable 
           440: define void @mutable_borrow(i32* noalias noundef align 4 dereferenceable(4) %_1) unnamed_addr #0 { 
           442:  ret void 
           443: } 
           444:  
           444:  
           445: ; Function Attrs: nonlazybind uwtable 
           446: define noundef align 4 dereferenceable(4) i32* @mutable_borrow_ret() unnamed_addr #0 { 
           447: start: 
           448:  br label %bb1 
           449:  
           450: bb1: ; preds = %bb1, %start 
           451:  br label %bb1 
           452: } 
           453:  
           454: ; Function Attrs: nonlazybind uwtable 
           455: define void @mutable_notunpin_borrow(i32* noundef nonnull align 4 %_1) unnamed_addr #0 { 
           457:  ret void 
           458: } 
           459:  
           459:  
           460: ; Function Attrs: nonlazybind uwtable 
           461: define void @notunpin_borrow(i32* noalias noundef readonly align 4 dereferenceable(4) %_1) unnamed_addr #0 { 
check:145'0                                                                                               X~~~~~~~~~~~~~~~~~~ error: no match found
           462: start: 
check:145'0     ~~~~~~~
           463:  ret void 
check:145'0     ~~~~~~~~~~
           464: } 
check:145'0     ~~
           465:  
check:145'0     ~
           466: ; Function Attrs: nonlazybind uwtable 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           467: define void @indirect_struct(%S* noalias nocapture noundef readonly align 4 dereferenceable(32) %_1) unnamed_addr #0 { 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:145'1              ?                                                                                                              possible intended match
           468: start: 
check:145'0     ~~~~~~~
           469:  ret void 
check:145'0     ~~~~~~~~~~
           470: } 
check:145'0     ~~
           471:  
check:145'0     ~
           472: ; Function Attrs: nonlazybind uwtable 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           473: define void @borrowed_struct(%S* noalias noundef readonly align 4 dereferenceable(32) %_1) unnamed_addr #0 { 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           474: start: 
check:145'0     ~~~~~~~
           475:  ret void 
check:145'0     ~~~~~~~~~~
           476: } 
check:145'0     ~~
           477:  
check:145'0     ~
           478: ; Function Attrs: nonlazybind uwtable 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           479: define void @option_borrow(i32* noalias noundef readonly align 4 dereferenceable_or_null(4) %x) unnamed_addr #0 { 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           480: start: 
check:145'0     ~~~~~~~
           481:  ret void 
check:145'0     ~~~~~~~~~~
           482: } 
check:145'0     ~~
           483:  
check:145'0     ~
           484: ; Function Attrs: nonlazybind uwtable 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           485: define void @option_borrow_mut(i32* noalias noundef align 4 dereferenceable_or_null(4) %x) unnamed_addr #0 { 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           486: start: 
check:145'0     ~~~~~~~
           487:  ret void 
check:145'0     ~~~~~~~~~~
           488: } 
check:145'0     ~~
           489:  
check:145'0     ~
           490: ; Function Attrs: nonlazybind uwtable 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           491: define void @raw_struct(%S* noundef %_1) unnamed_addr #0 { 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           492: start: 
check:145'0     ~~~~~~~
           493:  ret void 
check:145'0     ~~~~~~~~~~
           494: } 
check:145'0     ~~
           495:  
check:145'0     ~
           496: ; Function Attrs: nonlazybind uwtable 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           497: define void @raw_option_nonnull_struct(i32* noundef %_1) unnamed_addr #0 { 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           498: start: 
check:145'0     ~~~~~~~
           499:  ret void 
check:145'0     ~~~~~~~~~~
           500: } 
check:145'0     ~~
           501:  
check:145'0     ~
           502: ; Function Attrs: nonlazybind uwtable 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           503: define noundef nonnull align 4 i32* @_box(i32* noalias noundef nonnull align 4 %x) unnamed_addr #0 { 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           504: start: 
check:145'0     ~~~~~~~
           505:  ret i32* %x 
check:145'0     ~~~~~~~~~~~~~
           506: } 
check:145'0     ~~
           507:  
check:145'0     ~
           508: ; Function Attrs: nonlazybind uwtable 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           509: define noundef nonnull align 4 i32* @notunpin_box(i32* noundef nonnull align 4 %x) unnamed_addr #0 { 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           510: start: 
check:145'0     ~~~~~~~
           511:  ret i32* %x 
check:145'0     ~~~~~~~~~~~~~
           512: } 
check:145'0     ~~
           513:  
check:145'0     ~
           514: ; Function Attrs: nonlazybind uwtable 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           515: define void @struct_return(%S* noalias nocapture noundef sret(%S) align 4 dereferenceable(32) %0) unnamed_addr #0 { 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           516: start: 
check:145'0     ~~~~~~~
           517:  %_1 = alloca [8 x i32], align 4 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           518:  %1 = bitcast [8 x i32]* %_1 to i8* 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           519:  call void @llvm.lifetime.start.p0i8(i64 32, i8* %1) 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           520:  %2 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 0 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           521:  store i32 0, i32* %2, align 4 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           522:  %3 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 1 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           523:  store i32 0, i32* %3, align 4 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           524:  %4 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 2 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           525:  store i32 0, i32* %4, align 4 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           526:  %5 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 3 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           527:  store i32 0, i32* %5, align 4 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           528:  %6 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 4 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           529:  store i32 0, i32* %6, align 4 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           530:  %7 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 5 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           531:  store i32 0, i32* %7, align 4 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           532:  %8 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 6 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           533:  store i32 0, i32* %8, align 4 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           534:  %9 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 7 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           535:  store i32 0, i32* %9, align 4 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           536:  %10 = bitcast %S* %0 to [8 x i32]* 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           537:  %11 = bitcast [8 x i32]* %10 to i8* 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           538:  %12 = bitcast [8 x i32]* %_1 to i8* 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           539:  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %11, i8* align 4 %12, i64 32, i1 false) 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           540:  %13 = bitcast [8 x i32]* %_1 to i8* 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           541:  call void @llvm.lifetime.end.p0i8(i64 32, i8* %13) 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           542:  ret void 
check:145'0     ~~~~~~~~~~
           543: } 
check:145'0     ~~
           544:  
check:145'0     ~
           545: ; Function Attrs: nonlazybind uwtable 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           546: define void @helper(i64 noundef %_1) unnamed_addr #0 { 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           547: start: 
check:145'0     ~~~~~~~
           548:  ret void 
check:145'0     ~~~~~~~~~~
           549: } 
check:145'0     ~~
           550:  
check:145'0     ~
           551: ; Function Attrs: nonlazybind uwtable 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           552: define void @slice([0 x i8]* noalias noundef nonnull readonly align 1 %_1.0, i64 noundef %_1.1) unnamed_addr #0 { 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           553: start: 
check:145'0     ~~~~~~~
           554:  ret void 
check:145'0     ~~~~~~~~~~
           555: } 
check:145'0     ~~
           556:  
check:145'0     ~
           557: ; Function Attrs: nonlazybind uwtable 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           558: define void @mutable_slice([0 x i8]* noalias noundef nonnull align 1 %_1.0, i64 noundef %_1.1) unnamed_addr #0 { 
check:145'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           559: start: 
check:145'0     ~~~~~~~
           560:  ret void 
check:145'0     ~~~~~~~~~~
           561: } 
check:145'0     ~~
           562:  
