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
i.....i..i............i....i..ii.................iii........i.i.i........i.............. 88/409
.....ii.................i..............i.....i...............i....i...iiii.F.....i..i... 176/409
...............ii........................i.i.ii.i.i...............i....i....i..iii...... 352/409
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.i...i......................iiiiiiii.i...................
failures:
failures:

---- [codegen] tests/codegen/function-arguments.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-14/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll" "/checkout/tests/codegen/function-arguments.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/function-arguments.rs:284:11: error: CHECK: expected string not found in input
// CHECK: { {{\{\}\*|ptr}}, {{.+}} } @dyn_star({{\{\}\*|ptr}} noundef %x.0, {{.+}} noalias noundef readonly align {{.*}} dereferenceable({{.*}}) %x.1)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll:643:63: note: scanning from here
define { i8, i8 } @enum_id_2(i1 noundef zeroext %x.0, i8 %x.1) unnamed_addr #0 {

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll
Check file: /checkout/tests/codegen/function-arguments.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
           .
           .
           .
           .
         543: } 
         544:  
         545: ; Function Attrs: nonlazybind uwtable 
         546: define void @helper(i64 noundef %_1) unnamed_addr #0 { 
         548:  ret void 
         549: } 
         550:  
         550:  
         551: ; Function Attrs: nonlazybind uwtable 
         552: define void @slice([0 x i8]* noalias noundef nonnull readonly align 1 %_1.0, i64 noundef %_1.1) unnamed_addr #0 { 
         554:  ret void 
         555: } 
         556:  
         556:  
         557: ; Function Attrs: nonlazybind uwtable 
         558: define void @mutable_slice([0 x i8]* noalias noundef nonnull align 1 %_1.0, i64 noundef %_1.1) unnamed_addr #0 { 
         560:  ret void 
         561: } 
         562:  
         562:  
         563: ; Function Attrs: nonlazybind uwtable 
         564: define void @unsafe_slice([0 x i16]* noundef nonnull align 2 %_1.0, i64 noundef %_1.1) unnamed_addr #0 { 
         566:  ret void 
         567: } 
         568:  
         568:  
         569: ; Function Attrs: nonlazybind uwtable 
         570: define void @raw_slice([0 x i8]* noundef %_1.0, i64 noundef %_1.1) unnamed_addr #0 { 
         572:  ret void 
         573: } 
         574:  
         574:  
         575: ; Function Attrs: nonlazybind uwtable 
         576: define void @str([0 x i8]* noalias noundef nonnull readonly align 1 %_1.0, i64 noundef %_1.1) unnamed_addr #0 { 
         578:  ret void 
         579: } 
         580:  
         580:  
         581: ; Function Attrs: nonlazybind uwtable 
         582: define void @trait_borrow({}* noundef nonnull align 1 %_1.0, [3 x i64]* noalias noundef readonly align 8 dereferenceable(24) %_1.1) unnamed_addr #0 { 
         584:  ret void 
         585: } 
         586:  
         586:  
         587: ; Function Attrs: nonlazybind uwtable 
         588: define void @option_trait_borrow(i8* noundef align 1 %x.0, i8* %x.1) unnamed_addr #0 { 
         590:  ret void 
         591: } 
         592:  
         592:  
         593: ; Function Attrs: nonlazybind uwtable 
         594: define void @option_trait_borrow_mut(i8* noundef align 1 %x.0, i8* %x.1) unnamed_addr #0 { 
         596:  ret void 
         597: } 
         598:  
         598:  
         599: ; Function Attrs: nonlazybind uwtable 
         600: define void @trait_raw({}* noundef %_1.0, [3 x i64]* noalias noundef readonly align 8 dereferenceable(24) %_1.1) unnamed_addr #0 { 
         602:  ret void 
         603: } 
         604:  
         604:  
         605: ; Function Attrs: nonlazybind uwtable 
         606: define void @trait_box({}* noalias noundef nonnull align 1 %0, [3 x i64]* noalias noundef readonly align 8 dereferenceable(24) %1) unnamed_addr #0 { 
         607: start: 
         608:  %_1 = alloca { {}*, [3 x i64]* }, align 8 
         609:  %2 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_1, i32 0, i32 0 
         610:  store {}* %0, {}** %2, align 8 
         611:  %3 = getelementptr inbounds { {}*, [3 x i64]* }, { {}*, [3 x i64]* }* %_1, i32 0, i32 1 
         612:  store [3 x i64]* %1, [3 x i64]** %3, align 8 
         613: ; call core::ptr::drop_in_place<alloc::boxed::Box<dyn core::ops::drop::Drop+core::marker::Unpin>> 
         614:  call void @"_ZN4core3ptr99drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$core..ops..drop..Drop$u2b$core..marker..Unpin$GT$$GT$17h415ae239c7e70498E"({ {}*, [3 x i64]* }* noundef %_1) 
         615:  ret void 
         616: } 
         617:  
         618: ; Function Attrs: nonlazybind uwtable 
         619: define { i8*, i8* } @trait_option(i8* noalias noundef align 1 %x.0, i8* %x.1) unnamed_addr #0 { 
         620: start: 
         621:  %0 = insertvalue { i8*, i8* } undef, i8* %x.0, 0 
         622:  %1 = insertvalue { i8*, i8* } %0, i8* %x.1, 1 
         623:  ret { i8*, i8* } %1 
         624: } 
         625:  
         626: ; Function Attrs: nonlazybind uwtable 
         627: define { [0 x i16]*, i64 } @return_slice([0 x i16]* noalias noundef nonnull readonly align 2 %x.0, i64 noundef %x.1) unnamed_addr #0 { 
         628: start: 
         629:  %0 = insertvalue { [0 x i16]*, i64 } undef, [0 x i16]* %x.0, 0 
         630:  %1 = insertvalue { [0 x i16]*, i64 } %0, i64 %x.1, 1 
         631:  ret { [0 x i16]*, i64 } %1 
         632: } 
         633:  
         634: ; Function Attrs: nonlazybind uwtable 
         635: define { i16, i16 } @enum_id_1(i16 noundef %x.0, i16 %x.1) unnamed_addr #0 { 
         636: start: 
         637:  %0 = insertvalue { i16, i16 } undef, i16 %x.0, 0 
         638:  %1 = insertvalue { i16, i16 } %0, i16 %x.1, 1 
         639:  ret { i16, i16 } %1 
         640: } 
         641:  
         642: ; Function Attrs: nonlazybind uwtable 
         643: define { i8, i8 } @enum_id_2(i1 noundef zeroext %x.0, i8 %x.1) unnamed_addr #0 { 
check:284                                                                   X~~~~~~~~~~~~~~~~~~ error: no match found
check:284     ~~~~~~~
check:284     ~~~~~~~
         645:  %0 = zext i1 %x.0 to i8 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~
         646:  %1 = insertvalue { i8, i8 } undef, i8 %0, 0 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         647:  %2 = insertvalue { i8, i8 } %1, i8 %x.1, 1 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         648:  ret { i8, i8 } %2 
         649: } 
check:284     ~~
         650:  
check:284     ~
check:284     ~
         651: ; Function Attrs: nonlazybind uwtable 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         652: define { {}*, [3 x i64]* } @dyn_star({}* noundef %x.0, [3 x i64]* noundef nonnull %x.1) unnamed_addr #0 { 
         653: start: 
check:284     ~~~~~~~
check:284     ~~~~~~~
         654:  %0 = insertvalue { {}*, [3 x i64]* } undef, {}* %x.0, 0 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         655:  %1 = insertvalue { {}*, [3 x i64]* } %0, [3 x i64]* %x.1, 1 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         656:  ret { {}*, [3 x i64]* } %1 
         657: } 
check:284     ~~
         658:  
check:284     ~
check:284     ~
         659: ; Function Attrs: argmemonly nofree nosync nounwind willreturn 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         660: declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #2 
         661:  
check:284     ~
check:284     ~
         662: ; Function Attrs: argmemonly nofree nosync nounwind willreturn 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         663: declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #2 
         664:  
check:284     ~
         665: ; core::panicking::panic_nounwind 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         666: ; Function Attrs: cold noinline noreturn nounwind nonlazybind uwtable 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         667: declare void @_ZN4core9panicking14panic_nounwind17hd14c3d7b8c5401bfE([0 x i8]* noalias noundef nonnull readonly align 1, i64 noundef) unnamed_addr #3 
         668:  
check:284     ~
check:284     ~
         669: ; Function Attrs: nonlazybind uwtable 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         670: declare noundef i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, %"unwind::libunwind::_Unwind_Exception"* noundef, %"unwind::libunwind::_Unwind_Context"* noundef) unnamed_addr #0 
         671:  
check:284     ~
         672: ; core::panicking::panic_cannot_unwind 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         673: ; Function Attrs: cold noinline noreturn nounwind nonlazybind uwtable 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         674: declare void @_ZN4core9panicking19panic_cannot_unwind17hed29291bf070713aE() unnamed_addr #3 
         675:  
check:284     ~
check:284     ~
         676: ; Function Attrs: nofree nosync nounwind readnone speculatable willreturn 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         677: declare i64 @llvm.ctpop.i64(i64) #4 
         678:  
check:284     ~
check:284     ~
         679: ; Function Attrs: inaccessiblememonly nofree nosync nounwind willreturn 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         680: declare void @llvm.assume(i1 noundef) #5 
         681:  
check:284     ~
check:284     ~
         682: ; Function Attrs: nounwind nonlazybind uwtable 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         683: declare void @__rust_dealloc(i8* noundef, i64 noundef, i64 noundef) unnamed_addr #6 
         684:  
check:284     ~
check:284     ~
         685: ; Function Attrs: argmemonly nofree nounwind willreturn 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         686: declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #7 
         687:  
check:284     ~
check:284     ~
         688: attributes #0 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         689: attributes #1 = { inlinehint nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         690: attributes #2 = { argmemonly nofree nosync nounwind willreturn } 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         691: attributes #3 = { cold noinline noreturn nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         692: attributes #4 = { nofree nosync nounwind readnone speculatable willreturn } 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         693: attributes #5 = { inaccessiblememonly nofree nosync nounwind willreturn } 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         694: attributes #6 = { nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" } 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         695: attributes #7 = { argmemonly nofree nounwind willreturn } 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         696: attributes #8 = { noreturn nounwind } 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         697: attributes #9 = { noinline } 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         698: attributes #10 = { noinline noreturn nounwind } 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         699: attributes #11 = { nounwind } 
         700:  
check:284     ~
check:284     ~
         701: !llvm.module.flags = !{!0, !1} 
         702:  
check:284     ~
check:284     ~
         703: !0 = !{i32 7, !"PIC Level", i32 2} 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         704: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         705: !2 = !{} 
check:284     ~~~~~~~~~
         706: !3 = !{i64 8} 
check:284     ~~~~~~~~~~~~~~
         707: !4 = !{i64 0, i64 -9223372036854775808} 
check:284     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         708: !5 = !{i64 1, i64 0} 
check:284     ~~~~~~~~~~~~~~~~~~~~~
         709: !6 = !{i64 1, i64 -9223372036854775807} 
>>>>>>
------------------------------------------


