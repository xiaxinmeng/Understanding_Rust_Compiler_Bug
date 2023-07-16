plain
 finished in 14.910 seconds
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 399 tests
i......i.............i....i..ii...F.............iii........iii.i........i............F.. 88/399
....ii.................i............i..i.........F.....i....i.F.iiii...F...i..i.....i.ii 176/399
iiFiii.........i.iii....i..i......................iF..ii...iF....ii..i.ii....i.....F.... 264/399
FF...ii.F.........F.........F..i.i.ii.i.i.............i.i....i....i...iii......i...ii... 352/399
................F.iiiiiiii.i...................

---- [codegen] tests/codegen/adjustments.rs stdout ----


error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/adjustments/adjustments.ll" "/checkout/tests/codegen/adjustments.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/adjustments.rs:16:11: error: CHECK: expected string not found in input
// CHECK: %0 = insertvalue { {{\[0 x i8\]\*|ptr}}, [[USIZE]] } undef, {{\[0 x i8\]\*|ptr}} %x.0, 0
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/adjustments/adjustments.ll:13:50: note: scanning from here
define { [0 x i8]*, i64 } @no_op_slice_adjustment([0 x i8]* align 1 %0, i64 %1) unnamed_addr #0 {
                                                 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/adjustments/adjustments.ll:13:50: note: with "USIZE" equal to "i64"
define { [0 x i8]*, i64 } @no_op_slice_adjustment([0 x i8]* align 1 %0, i64 %1) unnamed_addr #0 {
                                                 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/adjustments/adjustments.ll:24:2: note: possible intended match here
 %8 = insertvalue { [0 x i8]*, i64 } undef, [0 x i8]* %5, 0

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/adjustments/adjustments.ll
Check file: /checkout/tests/codegen/adjustments.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'adjustments.7749b467-cgu.0' 
            2: source_filename = "adjustments.7749b467-cgu.0" 
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
            4: target triple = "x86_64-unknown-linux-gnu" 
            5:  
            6: ; Function Attrs: nonlazybind uwtable 
            7: define void @helper(i64 %_1) unnamed_addr #0 { 
            9:  ret void 
           10: } 
           11:  
           11:  
           12: ; Function Attrs: nonlazybind uwtable 
           13: define { [0 x i8]*, i64 } @no_op_slice_adjustment([0 x i8]* align 1 %0, i64 %1) unnamed_addr #0 { 
check:16'0                                                      X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
check:16'1                                                                                                        with "USIZE" equal to "i64"
           14: start: 
check:16'0     ~~~~~~~
           15:  %x = alloca { [0 x i8]*, i64 }, align 8 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           16:  %2 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %x, i32 0, i32 0 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           17:  store [0 x i8]* %0, [0 x i8]** %2, align 8 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           18:  %3 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %x, i32 0, i32 1 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           19:  store i64 %1, i64* %3, align 8 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           20:  %4 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %x, i32 0, i32 0 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           21:  %5 = load [0 x i8]*, [0 x i8]** %4, align 8, !nonnull !2, !align !3, !noundef !2 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           22:  %6 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %x, i32 0, i32 1 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           23:  %7 = load i64, i64* %6, align 8, !noundef !2 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           24:  %8 = insertvalue { [0 x i8]*, i64 } undef, [0 x i8]* %5, 0 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:16'2      ?                                                           possible intended match
           25:  %9 = insertvalue { [0 x i8]*, i64 } %8, i64 %7, 1 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           26:  ret { [0 x i8]*, i64 } %9 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
           27: } 
check:16'0     ~~
           28:  
check:16'0     ~
           29: ; Function Attrs: nonlazybind uwtable 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           30: define { [0 x i8]*, i64 } @no_op_slice_adjustment2([0 x i8]* align 1 %0, i64 %1) unnamed_addr #0 { 
check:16'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           31: start: 
           32:  %x = alloca { [0 x i8]*, i64 }, align 8 
           33:  %2 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %x, i32 0, i32 0 
           34:  store [0 x i8]* %0, [0 x i8]** %2, align 8 
           35:  %3 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %x, i32 0, i32 1 
           36:  store i64 %1, i64* %3, align 8 
           37:  %4 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %x, i32 0, i32 0 
           38:  %_2.0 = load [0 x i8]*, [0 x i8]** %4, align 8, !nonnull !2, !align !3, !noundef !2 
           39:  %5 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %x, i32 0, i32 1 
           40:  %_2.1 = load i64, i64* %5, align 8, !noundef !2 
           41:  %6 = call { [0 x i8]*, i64 } @no_op_slice_adjustment([0 x i8]* align 1 %_2.0, i64 %_2.1) 
           42:  %7 = extractvalue { [0 x i8]*, i64 } %6, 0 
           43:  %8 = extractvalue { [0 x i8]*, i64 } %6, 1 
           44:  %9 = insertvalue { [0 x i8]*, i64 } undef, [0 x i8]* %7, 0 
           45:  %10 = insertvalue { [0 x i8]*, i64 } %9, i64 %8, 1 
           46:  ret { [0 x i8]*, i64 } %10 
           47: } 
           48:  
           49: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
           50:  
           51: !llvm.module.flags = !{!0, !1} 
           52:  
           53: !0 = !{i32 7, !"PIC Level", i32 2} 
           54: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
           55: !2 = !{} 
           56: !3 = !{i64 1} 
------------------------------------------


---- [codegen] tests/codegen/c-variadic.rs stdout ----
---- [codegen] tests/codegen/c-variadic.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/c-variadic/c-variadic.ll" "/checkout/tests/codegen/c-variadic.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/c-variadic.rs:31:12: error: CHECK: expected string not found in input
 // CHECK: call void ({{.*}}, ...) @foreign_c_variadic_1({{.*}} %ap)
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/c-variadic/c-variadic.ll:435:76: note: scanning from here
 call void (i32, ...) @foreign_c_variadic_0(i32 0, i32 42, i32 1024, i32 0) #13
                                                                           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/c-variadic/c-variadic.ll:446:2: note: possible intended match here
 call void (i64*, ...) @foreign_c_variadic_1(i64* align 8 %_3) #13

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/c-variadic/c-variadic.ll
Check file: /checkout/tests/codegen/c-variadic.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
          335:  %_28 = and i64 %_29, %_33.0 
          336:  %9 = icmp eq i64 %_28, 0 
          337:  ret i1 %9 
          338:  
          339: panic: ; preds = %bb3 
          340: ; call core::panicking::panic 
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
          341:  call void @_ZN4core9panicking5panic17h76cd3873110cd701E([0 x i8]* align 1 bitcast ([33 x i8]* @str.1 to [0 x i8]*), i64 33, %"core::panic::location::Location<'_>"* align 8 bitcast (<{ i8*, [16 x i8] }>* @alloc36 to %"core::panic::location::Location<'_>"*)) #11 
          342:  unreachable 
          343: } 
          344:  
          345: ; core::iter::range::<impl core::iter::traits::iterator::Iterator for core::ops::range::Range<A>>::next 
          346: ; Function Attrs: inlinehint nonlazybind uwtable 
          347: define { i32, i32 } @"_ZN4core4iter5range101_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..Range$LT$A$GT$$GT$4next17hf71b11b304702967E"({ i32, i32 }* align 4 %self) unnamed_addr #0 { 
          348: start: 
          349: ; call <core::ops::range::Range<T> as core::iter::range::RangeIteratorImpl>::spec_next 
          350:  %0 = call { i32, i32 } @"_ZN89_$LT$core..ops..range..Range$LT$T$GT$$u20$as$u20$core..iter..range..RangeIteratorImpl$GT$9spec_next17hd7de1dfc97793212E"({ i32, i32 }* align 4 %self) 
          351:  %1 = extractvalue { i32, i32 } %0, 0 
          352:  %2 = extractvalue { i32, i32 } %0, 1 
          353:  %3 = insertvalue { i32, i32 } undef, i32 %1, 0 
          354:  %4 = insertvalue { i32, i32 } %3, i32 %2, 1 
          355:  ret { i32, i32 } %4 
          356: } 
          358: ; core::clone::impls::<impl core::clone::Clone for i32>::clone 
          358: ; core::clone::impls::<impl core::clone::Clone for i32>::clone 
          359: ; Function Attrs: alwaysinline nonlazybind uwtable 
          360: define internal i32 @"_ZN4core5clone5impls52_$LT$impl$u20$core..clone..Clone$u20$for$u20$i32$GT$5clone17h80616a8bd8618268E"(i32* align 4 %0) unnamed_addr #2 { 
          361: start: 
          362:  %self = alloca i32*, align 8 
          363:  store i32* %0, i32** %self, align 8 
          364:  %1 = load i32*, i32** %self, align 8, !nonnull !2, !align !5, !noundef !2 
          365:  %2 = load i32, i32* %1, align 4, !noundef !2 
          366:  ret i32 %2 
          367: } 
          369: ; <I as core::iter::traits::collect::IntoIterator>::into_iter 
          369: ; <I as core::iter::traits::collect::IntoIterator>::into_iter 
          370: ; Function Attrs: inlinehint nonlazybind uwtable 
          371: define { i32, i32 } @"_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h863584537de96d09E"(i32 %0, i32 %1) unnamed_addr #0 { 
          372: start: 
          373:  %self = alloca { i32, i32 }, align 4 
          374:  %2 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %self, i32 0, i32 0 
          375:  store i32 %0, i32* %2, align 4 
          376:  %3 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %self, i32 0, i32 1 
          377:  store i32 %1, i32* %3, align 4 
          378:  %4 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %self, i32 0, i32 0 
          379:  %5 = load i32, i32* %4, align 4, !noundef !2 
          380:  %6 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %self, i32 0, i32 1 
          381:  %7 = load i32, i32* %6, align 4, !noundef !2 
          382:  %8 = insertvalue { i32, i32 } undef, i32 %5, 0 
          383:  %9 = insertvalue { i32, i32 } %8, i32 %7, 1 
          384:  ret { i32, i32 } %9 
          385: } 
          386:  
          387: ; <core::ops::range::Range<T> as core::iter::range::RangeIteratorImpl>::spec_next 
          388: ; Function Attrs: inlinehint nonlazybind uwtable 
          389: define { i32, i32 } @"_ZN89_$LT$core..ops..range..Range$LT$T$GT$$u20$as$u20$core..iter..range..RangeIteratorImpl$GT$9spec_next17hd7de1dfc97793212E"({ i32, i32 }* align 4 %self) unnamed_addr #0 { 
          390: start: 
          391:  %0 = alloca { i32, i32 }, align 4 
          392:  %_3 = bitcast { i32, i32 }* %self to i32* 
          393:  %_4 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %self, i32 0, i32 1 
          394: ; call core::cmp::impls::<impl core::cmp::PartialOrd for i32>::lt 
          395:  %_2 = call zeroext i1 @"_ZN4core3cmp5impls55_$LT$impl$u20$core..cmp..PartialOrd$u20$for$u20$i32$GT$2lt17hf602bf42528b9669E"(i32* align 4 %_3, i32* align 4 %_4) 
          396:  br i1 %_2, label %bb2, label %bb6 
          397:  
          398: bb6: ; preds = %start 
          399:  %1 = bitcast { i32, i32 }* %0 to i32* 
          400:  store i32 0, i32* %1, align 4 
          401:  br label %bb7 
          402:  
          403: bb2: ; preds = %start 
          404:  %_7 = bitcast { i32, i32 }* %self to i32* 
          405: ; call core::clone::impls::<impl core::clone::Clone for i32>::clone 
          406:  %_6 = call i32 @"_ZN4core5clone5impls52_$LT$impl$u20$core..clone..Clone$u20$for$u20$i32$GT$5clone17h80616a8bd8618268E"(i32* align 4 %_7) 
          407: ; call <i32 as core::iter::range::Step>::forward_unchecked 
          408:  %n = call i32 @"_ZN47_$LT$i32$u20$as$u20$core..iter..range..Step$GT$17forward_unchecked17h026dc418ac855e11E"(i32 %_6, i64 1) 
          409:  %_10 = bitcast { i32, i32 }* %self to i32* 
          410: ; call core::mem::replace 
          411:  %_8 = call i32 @_ZN4core3mem7replace17h84f248d3ab9dd638E(i32* align 4 %_10, i32 %n) 
          412:  %2 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %0, i32 0, i32 1 
          413:  store i32 %_8, i32* %2, align 4 
          414:  %3 = bitcast { i32, i32 }* %0 to i32* 
          415:  store i32 1, i32* %3, align 4 
          416:  br label %bb7 
          417:  
          418: bb7: ; preds = %bb6, %bb2 
          419:  %4 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %0, i32 0, i32 0 
          420:  %5 = load i32, i32* %4, align 4, !range !6, !noundef !2 
          421:  %6 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %0, i32 0, i32 1 
          422:  %7 = load i32, i32* %6, align 4 
          423:  %8 = insertvalue { i32, i32 } undef, i32 %5, 0 
          424:  %9 = insertvalue { i32, i32 } %8, i32 %7, 1 
          425:  ret { i32, i32 } %9 
          426: } 
          427:  
          428: ; c_variadic::use_foreign_c_variadic_0 
          429: ; Function Attrs: nounwind nonlazybind uwtable 
          430: define void @_ZN10c_variadic24use_foreign_c_variadic_017hb24c0db917d5e778E() unnamed_addr #3 { 
          431: start: 
          432:  call void (i32, ...) @foreign_c_variadic_0(i32 0) #13 
          433:  call void (i32, ...) @foreign_c_variadic_0(i32 0, i32 42) #13 
          434:  call void (i32, ...) @foreign_c_variadic_0(i32 0, i32 42, i32 1024) #13 
          435:  call void (i32, ...) @foreign_c_variadic_0(i32 0, i32 42, i32 1024, i32 0) #13 
check:31'0                                                                                X~~~~ error: no match found
          436:  ret void 
check:31'0     ~~~~~~~~~~
          437: } 
check:31'0     ~~
          438:  
check:31'0     ~
          439: ; c_variadic::use_foreign_c_variadic_1_0 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          440: ; Function Attrs: nounwind nonlazybind uwtable 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          441: define void @_ZN10c_variadic26use_foreign_c_variadic_1_017h9e86813e5453645aE(i64* align 8 %0) unnamed_addr #3 { 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          442: start: 
check:31'0     ~~~~~~~
          443:  %ap = alloca i64*, align 8 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          444:  store i64* %0, i64** %ap, align 8 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          445:  %_3 = load i64*, i64** %ap, align 8, !nonnull !2, !align !4, !noundef !2 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          446:  call void (i64*, ...) @foreign_c_variadic_1(i64* align 8 %_3) #13 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:31'1      ?                                                                  possible intended match
          447:  ret void 
check:31'0     ~~~~~~~~~~
          448: } 
check:31'0     ~~
          449:  
check:31'0     ~
          450: ; c_variadic::use_foreign_c_variadic_1_1 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          451: ; Function Attrs: nounwind nonlazybind uwtable 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          452: define void @_ZN10c_variadic26use_foreign_c_variadic_1_117h2f89775e336c438cE(i64* align 8 %0) unnamed_addr #3 { 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          453: start: 
check:31'0     ~~~~~~~
          454:  %ap = alloca i64*, align 8 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          455:  store i64* %0, i64** %ap, align 8 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          456:  %_3 = load i64*, i64** %ap, align 8, !nonnull !2, !align !4, !noundef !2 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          457:  call void (i64*, ...) @foreign_c_variadic_1(i64* align 8 %_3, i32 42) #13 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          458:  ret void 
check:31'0     ~~~~~~~~~~
          459: } 
check:31'0     ~~
          460:  
check:31'0     ~
          461: ; c_variadic::use_foreign_c_variadic_1_2 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          462: ; Function Attrs: nounwind nonlazybind uwtable 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          463: define void @_ZN10c_variadic26use_foreign_c_variadic_1_217h454f51de919c78adE(i64* align 8 %0) unnamed_addr #3 { 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          464: start: 
check:31'0     ~~~~~~~
          465:  %ap = alloca i64*, align 8 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          466:  store i64* %0, i64** %ap, align 8 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          467:  %_3 = load i64*, i64** %ap, align 8, !nonnull !2, !align !4, !noundef !2 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          468:  call void (i64*, ...) @foreign_c_variadic_1(i64* align 8 %_3, i32 2, i32 42) #13 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          469:  ret void 
check:31'0     ~~~~~~~~~~
          470: } 
check:31'0     ~~
          471:  
check:31'0     ~
          472: ; c_variadic::use_foreign_c_variadic_1_3 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          473: ; Function Attrs: nounwind nonlazybind uwtable 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          474: define void @_ZN10c_variadic26use_foreign_c_variadic_1_317h1cab16a5af4c9c7aE(i64* align 8 %0) unnamed_addr #3 { 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          475: start: 
check:31'0     ~~~~~~~
          476:  %ap = alloca i64*, align 8 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          477:  store i64* %0, i64** %ap, align 8 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          478:  %_3 = load i64*, i64** %ap, align 8, !nonnull !2, !align !4, !noundef !2 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          479:  call void (i64*, ...) @foreign_c_variadic_1(i64* align 8 %_3, i32 2, i32 42, i32 0) #13 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          480:  ret void 
check:31'0     ~~~~~~~~~~
          481: } 
check:31'0     ~~
          482:  
check:31'0     ~
          483: ; Function Attrs: nounwind nonlazybind uwtable 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          484: define i32 @c_variadic(i32 %n, ...) unnamed_addr #3 personality i32 (...)* @rust_eh_personality { 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          485: start: 
check:31'0     ~~~~~~~
          486:  %0 = alloca { i8*, i32 }, align 8 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          487:  %_7 = alloca { i32, i32 }, align 4 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          488:  %iter = alloca { i32, i32 }, align 4 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          489:  %_4 = alloca { i32, i32 }, align 4 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          490:  %sum = alloca i32, align 4 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          491:  %ap = alloca %"core::ffi::VaListImpl<'_>", align 8 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          492:  %1 = bitcast %"core::ffi::VaListImpl<'_>"* %ap to i8* 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          493:  call void @llvm.va_start(i8* %1) 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          494:  store i32 0, i32* %sum, align 4 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          495:  %2 = bitcast { i32, i32 }* %_4 to i32* 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          496:  store i32 0, i32* %2, align 4 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          497:  %3 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %_4, i32 0, i32 1 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          498:  store i32 %n, i32* %3, align 4 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          499:  %4 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %_4, i32 0, i32 0 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          500:  %5 = load i32, i32* %4, align 4, !noundef !2 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          501:  %6 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %_4, i32 0, i32 1 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          502:  %7 = load i32, i32* %6, align 4, !noundef !2 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          503: ; invoke <I as core::iter::traits::collect::IntoIterator>::into_iter 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          504:  %8 = invoke { i32, i32 } @"_ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h863584537de96d09E"(i32 %5, i32 %7) 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          505:  to label %bb1 unwind label %cleanup 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          506:  
check:31'0     ~
          507: bb10: ; preds = %cleanup 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
          508: ; call core::panicking::panic_cannot_unwind 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          509:  call void @_ZN4core9panicking19panic_cannot_unwind17he8d167102e31790dE() #14 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          510:  unreachable 
check:31'0     ~~~~~~~~~~~~~
          511:  
check:31'0     ~
          512: cleanup: ; preds = %bb6, %panic, %bb4, %bb2, %start 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          513:  %9 = landingpad { i8*, i32 } 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          514:  cleanup 
check:31'0     ~~~~~~~~~
          515:  %10 = extractvalue { i8*, i32 } %9, 0 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          516:  %11 = extractvalue { i8*, i32 } %9, 1 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          517:  %12 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 0 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          518:  store i8* %10, i8** %12, align 8 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          519:  %13 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %0, i32 0, i32 1 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          520:  store i32 %11, i32* %13, align 8 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          521:  br label %bb10 
check:31'0     ~~~~~~~~~~~~~~~~
          522:  
check:31'0     ~
          523: bb1: ; preds = %start 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~
          524:  %_3.0 = extractvalue { i32, i32 } %8, 0 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          525:  %_3.1 = extractvalue { i32, i32 } %8, 1 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          526:  %14 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %iter, i32 0, i32 0 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          527:  store i32 %_3.0, i32* %14, align 4 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          528:  %15 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %iter, i32 0, i32 1 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          529:  store i32 %_3.1, i32* %15, align 4 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          530:  br label %bb2 
check:31'0     ~~~~~~~~~~~~~~~
          531:  
check:31'0     ~
          532: bb2: ; preds = %bb8, %bb1 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~
          533: ; invoke core::iter::range::<impl core::iter::traits::iterator::Iterator for core::ops::range::Range<A>>::next 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          534:  %16 = invoke { i32, i32 } @"_ZN4core4iter5range101_$LT$impl$u20$core..iter..traits..iterator..Iterator$u20$for$u20$core..ops..range..Range$LT$A$GT$$GT$4next17hf71b11b304702967E"({ i32, i32 }* align 4 %iter) 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          535:  to label %bb3 unwind label %cleanup 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          536:  
check:31'0     ~
          537: bb3: ; preds = %bb2 
check:31'0     ~~~~~~~~~~~~~~~~~~~~
          538:  store { i32, i32 } %16, { i32, i32 }* %_7, align 4 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          539:  %17 = bitcast { i32, i32 }* %_7 to i32* 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          540:  %18 = load i32, i32* %17, align 4, !range !6, !noundef !2 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          541:  %_10 = zext i32 %18 to i64 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          542:  %19 = icmp eq i64 %_10, 0 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
          543:  br i1 %19, label %bb6, label %bb4 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          544:  
check:31'0     ~
          545: bb6: ; preds = %bb3 
check:31'0     ~~~~~~~~~~~~~~~~~~~~
          546: ; invoke core::ptr::drop_in_place<core::ffi::VaListImpl> 
check:31'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
------------------------------------------


---- [codegen] tests/codegen/function-arguments-noopt.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll" "/checkout/tests/codegen/function-arguments-noopt.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/function-arguments-noopt.rs:13:11: error: CHECK: expected string not found in input
// CHECK: zeroext i1 @boolean(i1 zeroext %x)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:1:1: note: scanning from here
; ModuleID = 'function_arguments_noopt.8beba419-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:9:8: note: possible intended match here
define zeroext i1 @boolean(i1 zeroext %0) unnamed_addr #0 {
/checkout/tests/codegen/function-arguments-noopt.rs:22:11: error: CHECK: expected string not found in input
/checkout/tests/codegen/function-arguments-noopt.rs:22:11: error: CHECK: expected string not found in input
// CHECK: call zeroext i1 %f(i1 zeroext %x)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments-noopt/function-arguments-noopt.ll:20:32: note: scanning from here
define zeroext i1 @boolean_call(i1 zeroext %x, i1 (i1)* %0) unnamed_addr #0 {
                               ^
