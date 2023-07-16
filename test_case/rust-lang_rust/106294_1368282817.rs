plain
failures:

---- [codegen] src/test/codegen/function-arguments.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll" "/checkout/src/test/codegen/function-arguments.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/function-arguments.rs:219:11: error: CHECK: expected string not found in input
// CHECK: @option_trait_borrow({{i8\*|ptr}} noundef align 1 %x.0, ptr %x.1)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll:650:132: note: scanning from here
define void @trait_borrow({}* noundef nonnull align 1 %_1.0, [3 x i64]* noalias noundef readonly align 8 dereferenceable(24) %_1.1) unnamed_addr #0 {
                                                                                                                                   ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll:662:6: note: possible intended match here
define void @option_trait_borrow_mut(i8* noundef align 1 %x.0, i8* %x.1) unnamed_addr #0 {

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll
Check file: /checkout/src/test/codegen/function-arguments.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
             .
             .
             .
             .
           550: } 
           551:  
           552: ; Function Attrs: nonlazybind uwtable 
           553: define void @option_borrow(i32* noalias noundef readonly align 4 dereferenceable_or_null(4) %x) unnamed_addr #0 { 
           555:  ret void 
           556: } 
           557:  
           557:  
           558: ; Function Attrs: nonlazybind uwtable 
           559: define void @option_borrow_mut(i32* noalias noundef align 4 dereferenceable_or_null(4) %x) unnamed_addr #0 { 
           561:  ret void 
           562: } 
           563:  
           563:  
           564: ; Function Attrs: nonlazybind uwtable 
           565: define void @raw_struct(%S* noundef %_1) unnamed_addr #0 { 
           567:  ret void 
           568: } 
           569:  
           569:  
           570: ; Function Attrs: nonlazybind uwtable 
           571: define void @raw_option_nonnull_struct(i32* noundef %_1) unnamed_addr #0 { 
           573:  ret void 
           574: } 
           575:  
           575:  
           576: ; Function Attrs: nonlazybind uwtable 
           577: define noalias noundef nonnull align 4 i32* @_box(i32* noalias noundef nonnull align 4 %x) unnamed_addr #0 { 
           578: start: 
           579:  ret i32* %x 
           580: } 
           581:  
           582: ; Function Attrs: nonlazybind uwtable 
           583: define void @struct_return(%S* noalias nocapture noundef sret(%S) dereferenceable(32) %0) unnamed_addr #0 { 
           584: start: 
           585:  %_1 = alloca [8 x i32], align 4 
           586:  %1 = bitcast [8 x i32]* %_1 to i8* 
           587:  call void @llvm.lifetime.start.p0i8(i64 32, i8* %1) 
           588:  %2 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 0 
           589:  store i32 0, i32* %2, align 4 
           590:  %3 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 1 
           591:  store i32 0, i32* %3, align 4 
           592:  %4 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 2 
           593:  store i32 0, i32* %4, align 4 
           594:  %5 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 3 
           595:  store i32 0, i32* %5, align 4 
           596:  %6 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 4 
           597:  store i32 0, i32* %6, align 4 
           598:  %7 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 5 
           599:  store i32 0, i32* %7, align 4 
           600:  %8 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 6 
           601:  store i32 0, i32* %8, align 4 
           602:  %9 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 7 
           603:  store i32 0, i32* %9, align 4 
           604:  %10 = bitcast %S* %0 to [8 x i32]* 
           605:  %11 = bitcast [8 x i32]* %10 to i8* 
           606:  %12 = bitcast [8 x i32]* %_1 to i8* 
           607:  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %11, i8* align 4 %12, i64 32, i1 false) 
           608:  %13 = bitcast [8 x i32]* %_1 to i8* 
           609:  call void @llvm.lifetime.end.p0i8(i64 32, i8* %13) 
           610:  ret void 
           611: } 
           612:  
           613: ; Function Attrs: nonlazybind uwtable 
           614: define void @helper(i64 noundef %_1) unnamed_addr #0 { 
           616:  ret void 
           617: } 
           618:  
           618:  
           619: ; Function Attrs: nonlazybind uwtable 
           620: define void @slice([0 x i8]* noalias noundef nonnull readonly align 1 %_1.0, i64 noundef %_1.1) unnamed_addr #0 { 
           622:  ret void 
           623: } 
           624:  
           624:  
           625: ; Function Attrs: nonlazybind uwtable 
           626: define void @mutable_slice([0 x i8]* noalias noundef nonnull align 1 %_1.0, i64 noundef %_1.1) unnamed_addr #0 { 
           628:  ret void 
           629: } 
           630:  
           630:  
           631: ; Function Attrs: nonlazybind uwtable 
           632: define void @unsafe_slice([0 x i16]* noundef nonnull align 2 %_1.0, i64 noundef %_1.1) unnamed_addr #0 { 
           634:  ret void 
           635: } 
           636:  
           636:  
           637: ; Function Attrs: nonlazybind uwtable 
           638: define void @raw_slice([0 x i8]* noundef %_1.0, i64 noundef %_1.1) unnamed_addr #0 { 
           640:  ret void 
           641: } 
           642:  
           642:  
           643: ; Function Attrs: nonlazybind uwtable 
           644: define void @str([0 x i8]* noalias noundef nonnull readonly align 1 %_1.0, i64 noundef %_1.1) unnamed_addr #0 { 
           646:  ret void 
           647: } 
           648:  
           648:  
           649: ; Function Attrs: nonlazybind uwtable 
           650: define void @trait_borrow({}* noundef nonnull align 1 %_1.0, [3 x i64]* noalias noundef readonly align 8 dereferenceable(24) %_1.1) unnamed_addr #0 { 
check:219'0                                                                                                                                        X~~~~~~~~~~~~~~~~~~ error: no match found
           651: start: 
check:219'0     ~~~~~~~
           652:  ret void 
check:219'0     ~~~~~~~~~~
           653: } 
check:219'0     ~~
           654:  
check:219'0     ~
           655: ; Function Attrs: nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           656: define void @option_trait_borrow(i8* noundef align 1 %x.0, i8* %x.1) unnamed_addr #0 { 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           657: start: 
check:219'0     ~~~~~~~
           658:  ret void 
check:219'0     ~~~~~~~~~~
           659: } 
check:219'0     ~~
           660:  
check:219'0     ~
           661: ; Function Attrs: nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           662: define void @option_trait_borrow_mut(i8* noundef align 1 %x.0, i8* %x.1) unnamed_addr #0 { 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:219'1          ?                                                                                      possible intended match
           663: start: 
check:219'0     ~~~~~~~
           664:  ret void 
check:219'0     ~~~~~~~~~~
           665: } 
check:219'0     ~~
           666:  
check:219'0     ~
           667: ; Function Attrs: nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           668: define void @trait_raw({}* noundef %_1.0, [3 x i64]* noalias noundef readonly align 8 dereferenceable(24) %_1.1) unnamed_addr #0 { 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           669: start: 
check:219'0     ~~~~~~~
           670:  ret void 
check:219'0     ~~~~~~~~~~
           671: } 
check:219'0     ~~
           672:  
check:219'0     ~
           673: ; Function Attrs: nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           674: define void @trait_box({}* noalias noundef nonnull align 1 %0, [3 x i64]* noalias noundef readonly align 8 dereferenceable(24) %1) unnamed_addr #0 { 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           675: start: 
check:219'0     ~~~~~~~
           676:  %_1 = alloca { {}*, [3 x i64]* }, align 8 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           677:  %2 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_1, i32 0, i32 0 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           678:  store {}* %0, {}** %2, align 8 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           679:  %3 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_1, i32 0, i32 1 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           680:  store [3 x i64]* %1, [3 x i64]** %3, align 8 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           681: ; call core::ptr::drop_in_place<alloc::boxed::Box<dyn core::ops::drop::Drop>> 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           682:  call void @"_ZN4core3ptr75drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$core..ops..drop..Drop$GT$$GT$17hfbf7b1f799100650E"({ {}*, [3 x i64]* }* noundef %_1) 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           683:  ret void 
check:219'0     ~~~~~~~~~~
           684: } 
check:219'0     ~~
           685:  
check:219'0     ~
           686: ; Function Attrs: nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           687: define { i8*, i8* } @trait_option(i8* noalias noundef align 1 %x.0, i8* %x.1) unnamed_addr #0 { 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           688: start: 
check:219'0     ~~~~~~~
           689:  %0 = insertvalue { i8*, i8* } undef, i8* %x.0, 0 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           690:  %1 = insertvalue { i8*, i8* } %0, i8* %x.1, 1 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           691:  ret { i8*, i8* } %1 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~
           692: } 
check:219'0     ~~
           693:  
check:219'0     ~
           694: ; Function Attrs: nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           695: define { [0 x i16]*, i64 } @return_slice([0 x i16]* noalias noundef nonnull readonly align 2 %x.0, i64 noundef %x.1) unnamed_addr #0 { 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           696: start: 
check:219'0     ~~~~~~~
           697:  %0 = insertvalue { [0 x i16]*, i64 } undef, [0 x i16]* %x.0, 0 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           698:  %1 = insertvalue { [0 x i16]*, i64 } %0, i64 %x.1, 1 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           699:  ret { [0 x i16]*, i64 } %1 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           700: } 
check:219'0     ~~
           701:  
check:219'0     ~
           702: ; Function Attrs: nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           703: define { i16, i16 } @enum_id_1(i16 noundef %x.0, i16 %x.1) unnamed_addr #0 { 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           704: start: 
check:219'0     ~~~~~~~
           705:  %0 = insertvalue { i16, i16 } undef, i16 %x.0, 0 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           706:  %1 = insertvalue { i16, i16 } %0, i16 %x.1, 1 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           707:  ret { i16, i16 } %1 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~
           708: } 
check:219'0     ~~
           709:  
check:219'0     ~
           710: ; Function Attrs: nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           711: define { i8, i8 } @enum_id_2(i1 noundef zeroext %x.0, i8 %x.1) unnamed_addr #0 { 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           712: start: 
check:219'0     ~~~~~~~
           713:  %0 = zext i1 %x.0 to i8 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~
           714:  %1 = insertvalue { i8, i8 } undef, i8 %0, 0 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           715:  %2 = insertvalue { i8, i8 } %1, i8 %x.1, 1 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           716:  ret { i8, i8 } %2 
check:219'0     ~~~~~~~~~~~~~~~~~~~
           717: } 
check:219'0     ~~
           718:  
check:219'0     ~
           719: ; Function Attrs: nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           720: declare noundef i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, %"unwind::libunwind::_Unwind_Exception"* noundef, %"unwind::libunwind::_Unwind_Context"* noundef) unnamed_addr #0 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           721:  
check:219'0     ~
           722: ; Function Attrs: argmemonly nofree nosync nounwind willreturn 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           723: declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #2 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           724:  
check:219'0     ~
           725: ; core::panicking::panic_cannot_unwind 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           726: ; Function Attrs: cold noinline noreturn nounwind nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           727: declare void @_ZN4core9panicking19panic_cannot_unwind17ha81c36ceb84ce9b9E() unnamed_addr #3 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           728:  
check:219'0     ~
           729: ; Function Attrs: argmemonly nofree nosync nounwind willreturn 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           730: declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #2 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           731:  
check:219'0     ~
           732: ; Function Attrs: argmemonly nofree nounwind willreturn 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           733: declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #4 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           734:  
check:219'0     ~
           735: ; core::panicking::panic_nounwind 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           736: ; Function Attrs: cold noinline noreturn nounwind nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           737: declare void @_ZN4core9panicking14panic_nounwind17h81b6feaf8f0c9c2fE([0 x i8]* noalias noundef nonnull readonly align 1, i64 noundef) unnamed_addr #3 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           738:  
check:219'0     ~
           739: ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           740: declare i64 @llvm.ctpop.i64(i64) #5 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           741:  
check:219'0     ~
           742: ; Function Attrs: inaccessiblememonly nofree nosync nounwind willreturn 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           743: declare void @llvm.assume(i1 noundef) #6 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           744:  
check:219'0     ~
           745: ; Function Attrs: nounwind nonlazybind uwtable 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           746: declare void @__rust_dealloc(i8* noundef, i64 noundef, i64 noundef) unnamed_addr #7 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           747:  
check:219'0     ~
           748: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           749: attributes #1 = { inlinehint nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           750: attributes #2 = { argmemonly nofree nosync nounwind willreturn } 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           751: attributes #3 = { cold noinline noreturn nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           752: attributes #4 = { argmemonly nofree nounwind willreturn } 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           753: attributes #5 = { nofree nosync nounwind readnone speculatable willreturn } 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           754: attributes #6 = { inaccessiblememonly nofree nosync nounwind willreturn } 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           755: attributes #7 = { nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           756: attributes #8 = { noinline } 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           757: attributes #9 = { noinline noreturn nounwind } 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           758: attributes #10 = { noreturn nounwind } 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           759: attributes #11 = { nounwind } 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           760:  
check:219'0     ~
           761: !llvm.module.flags = !{!0, !1} 
check:219'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           762:  
check:219'0     ~
             .
             .
>>>>>>
------------------------------------------
------------------------------------------


---- [codegen] src/test/codegen/loads.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll" "/checkout/src/test/codegen/loads.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/loads.rs:54:12: error: CHECK: expected string not found in input
 // CHECK: load {{i32\*|ptr}}, {{i32\*\*|ptr}} %x, align [[PTR_ALIGNMENT]], !noundef !2{{$}}
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll:497:38: note: scanning from here
define noundef i32* @load_raw_pointer(i32** noalias noundef readonly align 8 dereferenceable(8) %x) unnamed_addr #0 {
                                     ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll:497:38: note: with "PTR_ALIGNMENT" equal to "8"
define noundef i32* @load_raw_pointer(i32** noalias noundef readonly align 8 dereferenceable(8) %x) unnamed_addr #0 {
                                     ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll:499:7: note: possible intended match here
 %0 = load i32*, i32** %x, align 8
/checkout/src/test/codegen/loads.rs:96:12: error: CHECK: expected string not found in input
/checkout/src/test/codegen/loads.rs:96:12: error: CHECK: expected string not found in input
 // CHECK: load i16, {{i16\*|ptr}} %x, align 2, !noundef !2{{$}}
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll:546:29: note: scanning from here
define noundef i16 @load_int(i16* noalias noundef readonly align 2 dereferenceable(2) %x) unnamed_addr #0 {
                            ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll:548:7: note: possible intended match here
 %0 = load i16, i16* %x, align 2
/checkout/src/test/codegen/loads.rs:110:12: error: CHECK: expected string not found in input
/checkout/src/test/codegen/loads.rs:110:12: error: CHECK: expected string not found in input
 // CHECK: load i16, {{i16\*|ptr}} %x, align 2, !noundef !2{{$}}
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll:560:44: note: scanning from here
define noundef i16 @load_option_nonzero_int(i16* noalias noundef readonly align 2 dereferenceable(2) %x) unnamed_addr #0 {
                                           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll:562:7: note: possible intended match here
 %0 = load i16, i16* %x, align 2

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/loads/loads.ll
Check file: /checkout/src/test/codegen/loads.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
             .
             .
             .
             .
           397:  %26 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0 
           398:  %_18.0 = load i64, i64* %26, align 8 
           399:  %27 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1 
           400:  %_18.1 = load i64, i64* %27, align 8, !range !5, !noundef !2 
           401: ; invoke <alloc::alloc::Global as core::alloc::Allocator>::deallocate 
           402:  invoke void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h22bc9270fc70ba61E"(%"alloc::alloc::Global"* noalias noundef nonnull readonly align 1 %alloc, i8* noundef nonnull %_15, i64 noundef %_18.0, i64 noundef %_18.1) 
           403:  to label %bb3 unwind label %cleanup 
           404:  
           405: bb3: ; preds = %bb9 
           406:  %28 = bitcast { i64, i64 }* %layout to i8* 
           407:  call void @llvm.lifetime.end.p0i8(i64 16, i8* %28) 
           408:  ret void 
           409: } 
           411: ; <alloc::alloc::Global as core::alloc::Allocator>::deallocate 
           411: ; <alloc::alloc::Global as core::alloc::Allocator>::deallocate 
           412: ; Function Attrs: inlinehint nonlazybind uwtable 
           413: define internal void @"_ZN63_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$10deallocate17h22bc9270fc70ba61E"(%"alloc::alloc::Global"* noalias noundef nonnull readonly align 1 %self, i8* noundef nonnull %ptr, i64 noundef %0, i64 noundef %1) unnamed_addr #1 { 
           414: start: 
           415:  %_16 = alloca i64, align 8 
           416:  %layout1 = alloca { i64, i64 }, align 8 
           417:  %layout = alloca { i64, i64 }, align 8 
           418:  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0 
           419:  store i64 %0, i64* %2, align 8 
           420:  %3 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1 
           421:  store i64 %1, i64* %3, align 8 
           422:  %4 = bitcast { i64, i64 }* %layout to i64* 
           423:  %_4 = load i64, i64* %4, align 8 
           424:  %5 = icmp eq i64 %_4, 0 
           425:  br i1 %5, label %bb2, label %bb1 
           426:  
           427: bb2: ; preds = %start 
           428:  br label %bb3 
           429:  
           430: bb1: ; preds = %start 
           431:  %6 = bitcast { i64, i64 }* %layout1 to i8* 
           432:  call void @llvm.lifetime.start.p0i8(i64 16, i8* %6) 
           433:  %7 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 0 
           434:  %8 = load i64, i64* %7, align 8 
           435:  %9 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout, i32 0, i32 1 
           436:  %10 = load i64, i64* %9, align 8, !range !5, !noundef !2 
           437:  %11 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout1, i32 0, i32 0 
           438:  store i64 %8, i64* %11, align 8 
           439:  %12 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout1, i32 0, i32 1 
           440:  store i64 %10, i64* %12, align 8 
           441:  %13 = bitcast { i64, i64 }* %layout1 to i64* 
           442:  %_11 = load i64, i64* %13, align 8 
           443:  %14 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %layout1, i32 0, i32 1 
           444:  %self2 = load i64, i64* %14, align 8, !range !5, !noundef !2 
           445:  %15 = bitcast i64* %_16 to i8* 
           446:  call void @llvm.lifetime.start.p0i8(i64 8, i8* %15) 
           447:  store i64 %self2, i64* %_16, align 8 
           448:  %_17 = load i64, i64* %_16, align 8, !range !5, !noundef !2 
           449:  %_18 = icmp uge i64 -9223372036854775808, %_17 
           450:  call void @llvm.assume(i1 %_18) 
           451:  %_19 = icmp ule i64 1, %_17 
           452:  call void @llvm.assume(i1 %_19) 
           453:  %16 = bitcast i64* %_16 to i8* 
           454:  call void @llvm.lifetime.end.p0i8(i64 8, i8* %16) 
           455:  call void @__rust_dealloc(i8* noundef %ptr, i64 noundef %_11, i64 noundef %_17) #11 
           456:  %17 = bitcast { i64, i64 }* %layout1 to i8* 
           457:  call void @llvm.lifetime.end.p0i8(i64 16, i8* %17) 
           458:  br label %bb3 
           459:  
           460: bb3: ; preds = %bb2, %bb1 
           461:  ret void 
           462: } 
           463:  
           464: ; Function Attrs: nonlazybind uwtable 
           465: define void @ptr_alignment_helper({}** noalias noundef readonly align 8 dereferenceable(8) %x) unnamed_addr #0 { 
           467:  ret void 
           468: } 
           469:  
           469:  
           470: ; Function Attrs: nonlazybind uwtable 
           471: define noundef align 4 dereferenceable(4) i32* @load_ref(i32** noalias noundef readonly align 8 dereferenceable(8) %x) unnamed_addr #0 { 
           472: start: 
           473:  %0 = load i32*, i32** %x, align 8, !nonnull !2, !align !6, !noundef !2 
           474:  ret i32* %0 
           475: } 
           476:  
           477: ; Function Attrs: nonlazybind uwtable 
           478: define noundef align 16 dereferenceable(16) %Align16* @load_ref_higher_alignment(%Align16** noalias noundef readonly align 8 dereferenceable(8) %x) unnamed_addr #0 { 
           479: start: 
           480:  %0 = load %Align16*, %Align16** %x, align 8, !nonnull !2, !align !7, !noundef !2 
           481:  ret %Align16* %0 
           482: } 
           483:  
           484: ; Function Attrs: nonlazybind uwtable 
           485: define { i32*, i64* } @load_scalar_pair({ i32*, i64* }* noalias noundef readonly align 8 dereferenceable(16) %x) unnamed_addr #0 { 
