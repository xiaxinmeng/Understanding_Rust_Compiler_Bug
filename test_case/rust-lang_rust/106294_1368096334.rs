plain
failures:

---- [codegen] src/test/codegen/function-arguments.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll" "/checkout/src/test/codegen/function-arguments.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/function-arguments.rs:156:11: error: CHECK: expected string not found in input
// CHECK: @raw_option_nonnull_struct({{%i32\*|ptr}} noundef %_1)
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll:565:41: note: scanning from here
define void @raw_struct(%S* noundef %_1) unnamed_addr #0 {
                                        ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll:571:3: note: possible intended match here
define void @raw_option_nonnull_struct(i32* noundef %_1) unnamed_addr #0 {

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/function-arguments/function-arguments.ll
Check file: /checkout/src/test/codegen/function-arguments.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
             .
             .
             .
             .
           465:  ret i32 %x 
           466: } 
           467:  
           468: ; Function Attrs: nonlazybind uwtable 
           469: define i32 @maybeuninit_char(i32 %x) unnamed_addr #0 { 
           470: start: 
           471:  ret i32 %x 
           472: } 
           473:  
           474: ; Function Attrs: nonlazybind uwtable 
           475: define noundef i64 @int(i64 noundef %x) unnamed_addr #0 { 
           476: start: 
           477:  ret i64 %x 
           478: } 
           479:  
           480: ; Function Attrs: nonlazybind uwtable 
           481: define noundef i64 @nonzero_int(i64 noundef %x) unnamed_addr #0 { 
           482: start: 
           483:  ret i64 %x 
           484: } 
           485:  
           486: ; Function Attrs: nonlazybind uwtable 
           487: define noundef i64 @option_nonzero_int(i64 noundef %x) unnamed_addr #0 { 
           488: start: 
           489:  ret i64 %x 
           490: } 
           491:  
           492: ; Function Attrs: nonlazybind uwtable 
           493: define void @readonly_borrow(i32* noalias noundef readonly align 4 dereferenceable(4) %_1) unnamed_addr #0 { 
           495:  ret void 
           496: } 
           497:  
           497:  
           498: ; Function Attrs: nonlazybind uwtable 
           499: define void @static_borrow(i32* noalias noundef readonly align 4 dereferenceable(4) %_1) unnamed_addr #0 { 
           501:  ret void 
           502: } 
           503:  
           503:  
           504: ; Function Attrs: nonlazybind uwtable 
           505: define void @named_borrow(i32* noalias noundef readonly align 4 dereferenceable(4) %_1) unnamed_addr #0 { 
           507:  ret void 
           508: } 
           509:  
           509:  
           510: ; Function Attrs: nonlazybind uwtable 
           511: define void @unsafe_borrow(i16* noundef nonnull align 2 %_1) unnamed_addr #0 { 
           513:  ret void 
           514: } 
           515:  
           515:  
           516: ; Function Attrs: nonlazybind uwtable 
           517: define void @mutable_unsafe_borrow(i16* noalias noundef align 2 dereferenceable(2) %_1) unnamed_addr #0 { 
           519:  ret void 
           520: } 
           521:  
           521:  
           522: ; Function Attrs: nonlazybind uwtable 
           523: define void @mutable_borrow(i32* noalias noundef align 4 dereferenceable(4) %_1) unnamed_addr #0 { 
           525:  ret void 
           526: } 
           527:  
           527:  
           528: ; Function Attrs: nonlazybind uwtable 
           529: define void @mutable_notunpin_borrow(i32* noundef align 4 dereferenceable(4) %_1) unnamed_addr #0 { 
           531:  ret void 
           532: } 
           533:  
           533:  
           534: ; Function Attrs: nonlazybind uwtable 
           535: define void @notunpin_borrow(i32* noalias noundef readonly align 4 dereferenceable(4) %_1) unnamed_addr #0 { 
           537:  ret void 
           538: } 
           539:  
           539:  
           540: ; Function Attrs: nonlazybind uwtable 
           541: define void @indirect_struct(%S* noalias nocapture noundef readonly dereferenceable(32) %_1) unnamed_addr #0 { 
           543:  ret void 
           544: } 
           545:  
           545:  
           546: ; Function Attrs: nonlazybind uwtable 
           547: define void @borrowed_struct(%S* noalias noundef readonly align 4 dereferenceable(32) %_1) unnamed_addr #0 { 
           549:  ret void 
           550: } 
           551:  
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
check:156'0                                             X~~~~~~~~~~~~~~~~~~ error: no match found
           566: start: 
check:156'0     ~~~~~~~
           567:  ret void 
check:156'0     ~~~~~~~~~~
           568: } 
check:156'0     ~~
           569:  
check:156'0     ~
           570: ; Function Attrs: nonlazybind uwtable 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           571: define void @raw_option_nonnull_struct(i32* noundef %_1) unnamed_addr #0 { 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:156'1       ?                                                                         possible intended match
           572: start: 
check:156'0     ~~~~~~~
           573:  ret void 
check:156'0     ~~~~~~~~~~
           574: } 
check:156'0     ~~
           575:  
check:156'0     ~
           576: ; Function Attrs: nonlazybind uwtable 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           577: define noalias noundef nonnull align 4 i32* @_box(i32* noalias noundef nonnull align 4 %x) unnamed_addr #0 { 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           578: start: 
check:156'0     ~~~~~~~
           579:  ret i32* %x 
check:156'0     ~~~~~~~~~~~~~
           580: } 
check:156'0     ~~
           581:  
check:156'0     ~
           582: ; Function Attrs: nonlazybind uwtable 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           583: define void @struct_return(%S* noalias nocapture noundef sret(%S) dereferenceable(32) %0) unnamed_addr #0 { 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           584: start: 
check:156'0     ~~~~~~~
           585:  %_1 = alloca [8 x i32], align 4 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           586:  %1 = bitcast [8 x i32]* %_1 to i8* 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           587:  call void @llvm.lifetime.start.p0i8(i64 32, i8* %1) 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           588:  %2 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 0 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           589:  store i32 0, i32* %2, align 4 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           590:  %3 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 1 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           591:  store i32 0, i32* %3, align 4 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           592:  %4 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 2 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           593:  store i32 0, i32* %4, align 4 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           594:  %5 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 3 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           595:  store i32 0, i32* %5, align 4 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           596:  %6 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 4 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           597:  store i32 0, i32* %6, align 4 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           598:  %7 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 5 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           599:  store i32 0, i32* %7, align 4 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           600:  %8 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 6 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           601:  store i32 0, i32* %8, align 4 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           602:  %9 = getelementptr inbounds [8 x i32], [8 x i32]* %_1, i64 0, i64 7 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           603:  store i32 0, i32* %9, align 4 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           604:  %10 = bitcast %S* %0 to [8 x i32]* 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           605:  %11 = bitcast [8 x i32]* %10 to i8* 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           606:  %12 = bitcast [8 x i32]* %_1 to i8* 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           607:  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %11, i8* align 4 %12, i64 32, i1 false) 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           608:  %13 = bitcast [8 x i32]* %_1 to i8* 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           609:  call void @llvm.lifetime.end.p0i8(i64 32, i8* %13) 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           610:  ret void 
check:156'0     ~~~~~~~~~~
           611: } 
check:156'0     ~~
           612:  
check:156'0     ~
           613: ; Function Attrs: nonlazybind uwtable 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           614: define void @helper(i64 noundef %_1) unnamed_addr #0 { 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           615: start: 
check:156'0     ~~~~~~~
           616:  ret void 
check:156'0     ~~~~~~~~~~
           617: } 
check:156'0     ~~
           618:  
check:156'0     ~
           619: ; Function Attrs: nonlazybind uwtable 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           620: define void @slice([0 x i8]* noalias noundef nonnull readonly align 1 %_1.0, i64 noundef %_1.1) unnamed_addr #0 { 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           621: start: 
check:156'0     ~~~~~~~
           622:  ret void 
check:156'0     ~~~~~~~~~~
           623: } 
check:156'0     ~~
           624:  
check:156'0     ~
           625: ; Function Attrs: nonlazybind uwtable 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           626: define void @mutable_slice([0 x i8]* noalias noundef nonnull align 1 %_1.0, i64 noundef %_1.1) unnamed_addr #0 { 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           627: start: 
check:156'0     ~~~~~~~
           628:  ret void 
check:156'0     ~~~~~~~~~~
           629: } 
check:156'0     ~~
           630:  
check:156'0     ~
           631: ; Function Attrs: nonlazybind uwtable 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           632: define void @unsafe_slice([0 x i16]* noundef nonnull align 2 %_1.0, i64 noundef %_1.1) unnamed_addr #0 { 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           633: start: 
check:156'0     ~~~~~~~
           634:  ret void 
check:156'0     ~~~~~~~~~~
           635: } 
check:156'0     ~~
           636:  
check:156'0     ~
           637: ; Function Attrs: nonlazybind uwtable 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           638: define void @raw_slice([0 x i8]* noundef %_1.0, i64 noundef %_1.1) unnamed_addr #0 { 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           639: start: 
check:156'0     ~~~~~~~
           640:  ret void 
check:156'0     ~~~~~~~~~~
           641: } 
check:156'0     ~~
           642:  
check:156'0     ~
           643: ; Function Attrs: nonlazybind uwtable 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           644: define void @str([0 x i8]* noalias noundef nonnull readonly align 1 %_1.0, i64 noundef %_1.1) unnamed_addr #0 { 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           645: start: 
check:156'0     ~~~~~~~
           646:  ret void 
check:156'0     ~~~~~~~~~~
           647: } 
check:156'0     ~~
           648:  
check:156'0     ~
           649: ; Function Attrs: nonlazybind uwtable 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           650: define void @trait_borrow({}* noundef nonnull align 1 %_1.0, [3 x i64]* noalias noundef readonly align 8 dereferenceable(24) %_1.1) unnamed_addr #0 { 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           651: start: 
check:156'0     ~~~~~~~
           652:  ret void 
check:156'0     ~~~~~~~~~~
           653: } 
check:156'0     ~~
           654:  
check:156'0     ~
           655: ; Function Attrs: nonlazybind uwtable 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           656: define void @option_trait_borrow(i8* noundef align 1 %x.0, i8* %x.1) unnamed_addr #0 { 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           657: start: 
check:156'0     ~~~~~~~
           658:  ret void 
check:156'0     ~~~~~~~~~~
           659: } 
check:156'0     ~~
           660:  
check:156'0     ~
           661: ; Function Attrs: nonlazybind uwtable 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           662: define void @option_trait_borrow_mut(i8* noundef align 1 %x.0, i8* %x.1) unnamed_addr #0 { 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           663: start: 
check:156'0     ~~~~~~~
           664:  ret void 
check:156'0     ~~~~~~~~~~
           665: } 
check:156'0     ~~
           666:  
check:156'0     ~
           667: ; Function Attrs: nonlazybind uwtable 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           668: define void @trait_raw({}* noundef %_1.0, [3 x i64]* noalias noundef readonly align 8 dereferenceable(24) %_1.1) unnamed_addr #0 { 
check:156'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           669: start: 
check:156'0     ~~~~~~~
           670:  ret void 
check:156'0     ~~~~~~~~~~
           671: } 
check:156'0     ~~
             .
             .
>>>>>>
------------------------------------------
