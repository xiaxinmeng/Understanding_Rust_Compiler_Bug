plain
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 397 tests
i......i.............i.....i.ii.................i.ii.......iii.i........i............... 88/397
....ii.................i............i..i...............i....i....iii.......i..iF....i.i. 176/397
...ii........................i.i.ii.i...i..........i..i....i.....i.iii.......i....ii.... 352/397
................iiiiiiii.i...................
failures:
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [codegen] src/test/codegen/function-arguments.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll" "/checkout/src/test/codegen/function-arguments.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/function-arguments.rs:219:11: error: CHECK: expected string not found in input
// CHECK: @option_trait_borrow({{\{\}\*|ptr}} noundef align 1 %x.0, {{\[3 x i64\]\*|ptr}} %x.1)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll:567:132: note: scanning from here
define void @trait_borrow({}* noundef nonnull align 1 %_1.0, [3 x i64]* noalias noundef readonly align 8 dereferenceable(24) %_1.1) unnamed_addr #0 {
                                                                                                                                   ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll:573:13: note: possible intended match here
define void @option_trait_borrow(i8* noundef align 1 %x.0, i8* %x.1) unnamed_addr #0 {

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll
Check file: /checkout/src/test/codegen/function-arguments.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
             .
             .
             .
             .
           467: } 
           468:  
           469: ; Function Attrs: nonlazybind uwtable 
           470: define void @option_borrow(i32* noalias noundef readonly align 4 dereferenceable_or_null(4) %x) unnamed_addr #0 { 
           472:  ret void 
           473: } 
           474:  
           474:  
           475: ; Function Attrs: nonlazybind uwtable 
           476: define void @option_borrow_mut(i32* noalias noundef align 4 dereferenceable_or_null(4) %x) unnamed_addr #0 { 
           478:  ret void 
           479: } 
           480:  
           480:  
           481: ; Function Attrs: nonlazybind uwtable 
           482: define void @raw_struct(%S* noundef %_1) unnamed_addr #0 { 
           484:  ret void 
           485: } 
           486:  
           486:  
           487: ; Function Attrs: nonlazybind uwtable 
           488: define void @raw_option_nonnull_struct(i32* noundef %_1) unnamed_addr #0 { 
           490:  ret void 
           491: } 
           492:  
           492:  
           493: ; Function Attrs: nonlazybind uwtable 
           494: define noundef nonnull align 4 i32* @_box(i32* noalias noundef nonnull align 4 %x) unnamed_addr #0 { 
           495: start: 
           496:  ret i32* %x 
           497: } 
           498:  
           499: ; Function Attrs: nonlazybind uwtable 
           500: define void @struct_return(%S* noalias nocapture noundef sret(%S) dereferenceable(32) %0) unnamed_addr #0 { 
           501: start: 
           502:  %_1 = alloca [8 x i32], align 4 
           503:  %1 = bitcast [8 x i32]* %_1 to i8* 
           504:  call void @llvm.lifetime.start.p0i8(i64 32, i8* %1) 
           505:  %2 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 0 
           506:  store i32 0, i32* %2, align 4 
           507:  %3 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 1 
           508:  store i32 0, i32* %3, align 4 
           509:  %4 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 2 
           510:  store i32 0, i32* %4, align 4 
           511:  %5 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 3 
           512:  store i32 0, i32* %5, align 4 
           513:  %6 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 4 
           514:  store i32 0, i32* %6, align 4 
           515:  %7 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 5 
           516:  store i32 0, i32* %7, align 4 
           517:  %8 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 6 
           518:  store i32 0, i32* %8, align 4 
           519:  %9 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 7 
           520:  store i32 0, i32* %9, align 4 
           521:  %10 = bitcast %S* %0 to [8 x i32]* 
           522:  %11 = bitcast [8 x i32]* %10 to i8* 
           523:  %12 = bitcast [8 x i32]* %_1 to i8* 
           524:  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %11, i8* align 4 %12, i64 32, i1 false) 
           525:  %13 = bitcast [8 x i32]* %_1 to i8* 
           526:  call void @llvm.lifetime.end.p0i8(i64 32, i8* %13) 
           527:  ret void 
           528: } 
           529:  
           530: ; Function Attrs: nonlazybind uwtable 
           531: define void @helper(i64 noundef %_1) unnamed_addr #0 { 
           533:  ret void 
           534: } 
           535:  
           535:  
           536: ; Function Attrs: nonlazybind uwtable 
           537: define void @slice([0 x i8]* noalias noundef nonnull readonly align 1 %_1.0, i64 noundef %_1.1) unnamed_addr #0 { 
           539:  ret void 
           540: } 
           541:  
           541:  
           542: ; Function Attrs: nonlazybind uwtable 
           543: define void @mutable_slice([0 x i8]* noalias noundef nonnull align 1 %_1.0, i64 noundef %_1.1) unnamed_addr #0 { 
           545:  ret void 
           546: } 
           547:  
           547:  
           548: ; Function Attrs: nonlazybind uwtable 
           549: define void @unsafe_slice([0 x i16]* noundef nonnull align 2 %_1.0, i64 noundef %_1.1) unnamed_addr #0 { 
           551:  ret void 
           552: } 
           553:  
           553:  
           554: ; Function Attrs: nonlazybind uwtable 
           555: define void @raw_slice([0 x i8]* noundef %_1.0, i64 noundef %_1.1) unnamed_addr #0 { 
           557:  ret void 
           558: } 
           559:  
           559:  
           560: ; Function Attrs: nonlazybind uwtable 
           561: define void @str([0 x i8]* noalias noundef nonnull readonly align 1 %_1.0, i64 noundef %_1.1) unnamed_addr #0 { 
           563:  ret void 
           564: } 
           565:  
           565:  
           566: ; Function Attrs: nonlazybind uwtable 
           567: define void @trait_borrow({}* noundef nonnull align 1 %_1.0, [3 x i64]* noalias noundef readonly align 8 dereferenceable(24) %_1.1) unnamed_addr #0 { 
check:219'0                                                                                                                                        X~~~~~~~~~~~~~~~~~~ error: no match found
           568: start: 
check:219'0     ~~~~~~~
           569:  ret void 
check:219'0     ~~~~~~~~~~
           570: } 
check:219'0     ~~
           571:  
check:219'0     ~
           572: ; Function Attrs: nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           573: define void @option_trait_borrow(i8* noundef align 1 %x.0, i8* %x.1) unnamed_addr #0 { 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:219'1                 ?                                                                           possible intended match
           574: start: 
check:219'0     ~~~~~~~
           575:  ret void 
check:219'0     ~~~~~~~~~~
           576: } 
check:219'0     ~~
           577:  
check:219'0     ~
           578: ; Function Attrs: nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           579: define void @option_trait_borrow_mut(i8* noundef align 1 %x.0, i8* %x.1) unnamed_addr #0 { 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           580: start: 
check:219'0     ~~~~~~~
           581:  ret void 
check:219'0     ~~~~~~~~~~
           582: } 
check:219'0     ~~
           583:  
check:219'0     ~
           584: ; Function Attrs: nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           585: define void @trait_raw({}* noundef %_1.0, [3 x i64]* noalias noundef readonly align 8 dereferenceable(24) %_1.1) unnamed_addr #0 { 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           586: start: 
check:219'0     ~~~~~~~
           587:  ret void 
check:219'0     ~~~~~~~~~~
           588: } 
check:219'0     ~~
           589:  
check:219'0     ~
           590: ; Function Attrs: nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           591: define void @trait_box({}* noalias noundef nonnull align 1 %0, [3 x i64]* noalias noundef readonly align 8 dereferenceable(24) %1) unnamed_addr #0 { 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           592: start: 
check:219'0     ~~~~~~~
           593:  %_1 = alloca { {}*, [3 x i64]* }, align 8 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           594:  %2 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_1, i32 0, i32 0 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           595:  store {}* %0, {}** %2, align 8 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           596:  %3 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_1, i32 0, i32 1 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           597:  store [3 x i64]* %1, [3 x i64]** %3, align 8 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           598: ; call core::ptr::drop_in_place<alloc::boxed::Box<dyn core::ops::drop::Drop>> 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           599:  call void @"_ZN4core3ptr75drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$core..ops..drop..Drop$GT$$GT$17hacd19a8cef50d470E"({ {}*, [3 x i64]* }* noundef %_1) 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           600:  ret void 
check:219'0     ~~~~~~~~~~
           601: } 
check:219'0     ~~
           602:  
check:219'0     ~
           603: ; Function Attrs: nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           604: define { i8*, i8* } @trait_option(i8* noalias noundef align 1 %x.0, i8* %x.1) unnamed_addr #0 { 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           605: start: 
check:219'0     ~~~~~~~
           606:  %0 = insertvalue { i8*, i8* } undef, i8* %x.0, 0 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           607:  %1 = insertvalue { i8*, i8* } %0, i8* %x.1, 1 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           608:  ret { i8*, i8* } %1 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~
           609: } 
check:219'0     ~~
           610:  
check:219'0     ~
           611: ; Function Attrs: nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           612: define { [0 x i16]*, i64 } @return_slice([0 x i16]* noalias noundef nonnull readonly align 2 %x.0, i64 noundef %x.1) unnamed_addr #0 { 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           613: start: 
check:219'0     ~~~~~~~
           614:  %0 = insertvalue { [0 x i16]*, i64 } undef, [0 x i16]* %x.0, 0 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           615:  %1 = insertvalue { [0 x i16]*, i64 } %0, i64 %x.1, 1 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           616:  ret { [0 x i16]*, i64 } %1 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           617: } 
check:219'0     ~~
           618:  
check:219'0     ~
           619: ; Function Attrs: nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           620: define { i16, i16 } @enum_id_1(i16 noundef %x.0, i16 %x.1) unnamed_addr #0 { 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           621: start: 
check:219'0     ~~~~~~~
           622:  %0 = insertvalue { i16, i16 } undef, i16 %x.0, 0 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           623:  %1 = insertvalue { i16, i16 } %0, i16 %x.1, 1 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           624:  ret { i16, i16 } %1 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~
           625: } 
check:219'0     ~~
           626:  
check:219'0     ~
           627: ; Function Attrs: nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           628: define { i8, i8 } @enum_id_2(i1 noundef zeroext %x.0, i8 %x.1) unnamed_addr #0 { 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           629: start: 
check:219'0     ~~~~~~~
           630:  %0 = zext i1 %x.0 to i8 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
           631:  %1 = insertvalue { i8, i8 } undef, i8 %0, 0 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           632:  %2 = insertvalue { i8, i8 } %1, i8 %x.1, 1 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           633:  ret { i8, i8 } %2 
check:219'0     ~~~~~~~~~~~~~~~~~~~
           634: } 
check:219'0     ~~
           635:  
check:219'0     ~
           636: ; Function Attrs: nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           637: declare noundef i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, %"unwind::libunwind::_Unwind_Exception"* noundef, %"unwind::libunwind::_Unwind_Context"* noundef) unnamed_addr #0 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           638:  
check:219'0     ~
           639: ; Function Attrs: argmemonly nofree nosync nounwind willreturn 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           640: declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #2 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           641:  
check:219'0     ~
           642: ; core::panicking::panic_cannot_unwind 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           643: ; Function Attrs: cold noinline noreturn nounwind nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           644: declare void @_ZN4core9panicking19panic_cannot_unwind17h46a4d5565f5f5e00E() unnamed_addr #3 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           645:  
check:219'0     ~
           646: ; Function Attrs: argmemonly nofree nosync nounwind willreturn 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           647: declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #2 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           648:  
check:219'0     ~
           649: ; core::panicking::panic_nounwind 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           650: ; Function Attrs: cold noinline noreturn nounwind nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           651: declare void @_ZN4core9panicking14panic_nounwind17h36a8b28c9df1529bE([0 x i8]* noalias noundef nonnull readonly align 1, i64 noundef) unnamed_addr #3 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           652:  
check:219'0     ~
           653: ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           654: declare i64 @llvm.ctpop.i64(i64) #4 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           655:  
check:219'0     ~
           656: ; Function Attrs: inaccessiblememonly nofree nosync nounwind willreturn 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           657: declare void @llvm.assume(i1 noundef) #5 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           658:  
check:219'0     ~
           659: ; Function Attrs: nounwind nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           660: declare void @__rust_dealloc(i8* noundef, i64 noundef, i64 noundef) unnamed_addr #6 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           661:  
check:219'0     ~
           662: ; Function Attrs: argmemonly nofree nounwind willreturn 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           663: declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #7 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           664:  
check:219'0     ~
           665: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           666: attributes #1 = { inlinehint nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           667: attributes #2 = { argmemonly nofree nosync nounwind willreturn } 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           668: attributes #3 = { cold noinline noreturn nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           669: attributes #4 = { nofree nosync nounwind readnone speculatable willreturn } 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           670: attributes #5 = { inaccessiblememonly nofree nosync nounwind willreturn } 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           671: attributes #6 = { nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           672: attributes #7 = { argmemonly nofree nounwind willreturn } 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           673: attributes #8 = { noinline } 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
             .
             .
>>>>>>
------------------------------------------
