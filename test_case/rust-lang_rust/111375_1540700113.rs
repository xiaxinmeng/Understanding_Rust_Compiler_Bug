plain
running 463 tests
i....i.i..ii............i....i..ii.................iii........ii.i.i............i.......  88/463
.............ii.................i...............i.....i........i..iiii...iiii.iiii....i. 176/463
.iiiii.......i.i.i....i.i.iiii.iiii.....ii....i.ii......i..i.......................i...i 264/463
i..i.i.....ii..i.ii..............ii.i...........................ii..F..F................ 352/463
iiii...................

failures:


---- [codegen] tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs#x86_64 stdout ----

error in revision `x86_64`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-14/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.x86_64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll" "/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,x86_64" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:47:18: error: CHECK-LABEL: expected string not found in input
 // CHECK-LABEL: define{{.*}}foo{{.*}}!kcfi_type !{{[0-9]+}}
                 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.x86_64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll:1:1: note: scanning from here
Build completed unsuccessfully in 0:13:12
; ModuleID = 'sanitizer_kcfi_emit_type_metadata_trait_objects.74fbb9ecb80d2934-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.x86_64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll:26:1: note: possible intended match here
define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects3foo17he4aee4a0b2685663E() unnamed_addr #1 !<unknown kind #36> !4 {


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.x86_64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'sanitizer_kcfi_emit_type_metadata_trait_objects.74fbb9ecb80d2934-cgu.0' 
label:47'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "sanitizer_kcfi_emit_type_metadata_trait_objects.74fbb9ecb80d2934-cgu.0" 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "x86_64-unknown-none-elf" 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
label:47'0     ~
            6: %Type1 = type {} 
label:47'0     ~~~~~~~~~~~~~~~~~
            7:  
label:47'0     ~
            8: @vtable.0 = private unnamed_addr constant <{ i8*, [16 x i8], i8* }> <{ i8* bitcast (void ()* @"_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects78drop_in_place_fn$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$GT$17hd544d746556e563bE" to i8*), [16 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", i8* bitcast (void (%Type1*)* @"_ZN130_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h3ed069aedb940e54E" to i8*) }>, align 8 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            9:  
label:47'0     ~
           10: ; sanitizer_kcfi_emit_type_metadata_trait_objects::drop_in_place_fn<sanitizer_kcfi_emit_type_metadata_trait_objects::Type1> 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           11: ; Function Attrs: inlinehint noredzone nounwind nonlazybind 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           12: define internal void @"_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects78drop_in_place_fn$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$GT$17hd544d746556e563bE"() unnamed_addr #0 !<unknown kind #36> !4 { 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           13: start: 
label:47'0     ~~~~~~~
           14:  ret void 
label:47'0     ~~~~~~~~~~
           15: } 
label:47'0     ~~
           16:  
label:47'0     ~
           17: ; <sanitizer_kcfi_emit_type_metadata_trait_objects::Type1 as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait1>::foo 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           18: ; Function Attrs: noredzone nounwind nonlazybind 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           19: define void @"_ZN130_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h3ed069aedb940e54E"(%Type1* align 1 %self) unnamed_addr #1 !<unknown kind #36> !5 { 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           20: start: 
label:47'0     ~~~~~~~
           21:  ret void 
label:47'0     ~~~~~~~~~~
           22: } 
label:47'0     ~~
           23:  
label:47'0     ~
           24: ; sanitizer_kcfi_emit_type_metadata_trait_objects::foo 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           25: ; Function Attrs: noredzone nounwind nonlazybind 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           26: define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects3foo17he4aee4a0b2685663E() unnamed_addr #1 !<unknown kind #36> !4 { 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
label:47'1     ?                                                                                                                                     possible intended match
           27: start: 
label:47'0     ~~~~~~~
           28:  %_1 = alloca %Type1, align 1 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           29: ; call <sanitizer_kcfi_emit_type_metadata_trait_objects::Type1 as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait1>::foo 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           30:  call void @"_ZN130_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h3ed069aedb940e54E"(%Type1* align 1 %_1) #2 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           31:  ret void 
label:47'0     ~~~~~~~~~~
           32: } 
label:47'0     ~~
           33:  
label:47'0     ~
           34: ; sanitizer_kcfi_emit_type_metadata_trait_objects::bar 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           35: ; Function Attrs: noredzone nounwind nonlazybind 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           36: define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects3bar17hcd008b5f757bbd00E() unnamed_addr #1 !<unknown kind #36> !4 { 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           37: start: 
label:47'0     ~~~~~~~
           38:  %_1 = alloca %Type1, align 1 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           39:  %b.0 = bitcast %Type1* %_1 to {}* 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           40:  %0 = load void ({}*)*, void ({}*)** getelementptr inbounds (void ({}*)*, void ({}*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.0 to void ({}*)**), i64 3), align 8, !invariant.load !6, !nonnull !6 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           41:  call void %0({}* align 1 %b.0) #2 [ "kcfi"(i32 -56908610) ] 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           42:  ret void 
label:47'0     ~~~~~~~~~~
           43: } 
label:47'0     ~~
           44:  
label:47'0     ~
           45: ; sanitizer_kcfi_emit_type_metadata_trait_objects::baz 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           46: ; Function Attrs: noredzone nounwind nonlazybind 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           47: define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects3baz17hf719ac674d79536dE() unnamed_addr #1 !<unknown kind #36> !4 { 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           48: start: 
label:47'0     ~~~~~~~
           49:  %_1 = alloca %Type1, align 1 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           50:  %b.0 = bitcast %Type1* %_1 to {}* 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           51: ; call <sanitizer_kcfi_emit_type_metadata_trait_objects::Type1 as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait1>::foo 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           52:  call void @"_ZN130_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h3ed069aedb940e54E"(%Type1* align 1 %_1) #2 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           53:  %0 = load void ({}*)*, void ({}*)** getelementptr inbounds (void ({}*)*, void ({}*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.0 to void ({}*)**), i64 3), align 8, !invariant.load !6, !nonnull !6 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           54:  call void %0({}* align 1 %b.0) #2 [ "kcfi"(i32 -56908610) ] 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           55:  ret void 
label:47'0     ~~~~~~~~~~
           56: } 
label:47'0     ~~
           57:  
label:47'0     ~
           58: attributes #0 = { inlinehint noredzone nounwind nonlazybind "probe-stack"="__rust_probestack" "target-cpu"="x86-64" "target-features"="-mmx,-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-3dnow,-3dnowa,-avx,-avx2,+soft-float" } 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           59: attributes #1 = { noredzone nounwind nonlazybind "probe-stack"="__rust_probestack" "target-cpu"="x86-64" "target-features"="-mmx,-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-3dnow,-3dnowa,-avx,-avx2,+soft-float" } 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           60: attributes #2 = { nounwind } 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           61:  
label:47'0     ~
           62: !llvm.module.flags = !{!0, !1, !2, !3} 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           63:  
label:47'0     ~
           64: !0 = !{i32 7, !"PIC Level", i32 2} 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           65: !1 = !{i32 1, !"Code Model", i32 2} 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           66: !2 = !{i32 2, !"RtLibUseGOT", i32 1} 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           67: !3 = !{i32 4, !"kcfi", i32 1} 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           68: !4 = !{i32 -1522505972} 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~
           69: !5 = !{i32 -56908610} 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~
           70: !6 = !{} 
label:47'0     ~~~~~~~~~
------------------------------------------


---- [codegen] tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs#aarch64 stdout ----
---- [codegen] tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs#aarch64 stdout ----

error in revision `aarch64`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-14/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.aarch64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll" "/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,aarch64" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:47:18: error: CHECK-LABEL: expected string not found in input
 // CHECK-LABEL: define{{.*}}foo{{.*}}!kcfi_type !{{[0-9]+}}
                 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.aarch64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll:1:1: note: scanning from here
; ModuleID = 'sanitizer_kcfi_emit_type_metadata_trait_objects.74fbb9ecb80d2934-cgu.0'
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.aarch64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll:41:22: note: possible intended match here
 call void %0({}* align 1 %b.0) #2 [ "kcfi"(i32 -56908610) ]


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.aarch64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'sanitizer_kcfi_emit_type_metadata_trait_objects.74fbb9ecb80d2934-cgu.0' 
label:47'0     X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
            2: source_filename = "sanitizer_kcfi_emit_type_metadata_trait_objects.74fbb9ecb80d2934-cgu.0" 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            3: target datalayout = "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128" 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            4: target triple = "aarch64-unknown-none" 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            5:  
label:47'0     ~
            6: %Type1 = type {} 
label:47'0     ~~~~~~~~~~~~~~~~~
            7:  
label:47'0     ~
            8: @vtable.0 = private unnamed_addr constant <{ i8*, [16 x i8], i8* }> <{ i8* bitcast (void ()* @"_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects78drop_in_place_fn$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$GT$17hd544d746556e563bE" to i8*), [16 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", i8* bitcast (void (%Type1*)* @"_ZN130_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h3ed069aedb940e54E" to i8*) }>, align 8 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            9:  
label:47'0     ~
           10: ; sanitizer_kcfi_emit_type_metadata_trait_objects::drop_in_place_fn<sanitizer_kcfi_emit_type_metadata_trait_objects::Type1> 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           11: ; Function Attrs: inlinehint noredzone nounwind 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           12: define internal void @"_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects78drop_in_place_fn$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$GT$17hd544d746556e563bE"() unnamed_addr #0 !<unknown kind #36> !1 { 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           13: start: 
label:47'0     ~~~~~~~
           14:  ret void 
label:47'0     ~~~~~~~~~~
           15: } 
label:47'0     ~~
           16:  
label:47'0     ~
           17: ; <sanitizer_kcfi_emit_type_metadata_trait_objects::Type1 as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait1>::foo 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           18: ; Function Attrs: noredzone nounwind 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           19: define dso_local void @"_ZN130_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h3ed069aedb940e54E"(%Type1* align 1 %self) unnamed_addr #1 !<unknown kind #36> !2 { 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           20: start: 
label:47'0     ~~~~~~~
           21:  ret void 
label:47'0     ~~~~~~~~~~
           22: } 
label:47'0     ~~
           23:  
label:47'0     ~
           24: ; sanitizer_kcfi_emit_type_metadata_trait_objects::foo 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           25: ; Function Attrs: noredzone nounwind 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           26: define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects3foo17he4aee4a0b2685663E() unnamed_addr #1 !<unknown kind #36> !1 { 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           27: start: 
label:47'0     ~~~~~~~
           28:  %_1 = alloca %Type1, align 1 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           29: ; call <sanitizer_kcfi_emit_type_metadata_trait_objects::Type1 as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait1>::foo 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           30:  call void @"_ZN130_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h3ed069aedb940e54E"(%Type1* align 1 %_1) #2 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           31:  ret void 
label:47'0     ~~~~~~~~~~
           32: } 
label:47'0     ~~
           33:  
label:47'0     ~
           34: ; sanitizer_kcfi_emit_type_metadata_trait_objects::bar 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           35: ; Function Attrs: noredzone nounwind 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           36: define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects3bar17hcd008b5f757bbd00E() unnamed_addr #1 !<unknown kind #36> !1 { 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           37: start: 
label:47'0     ~~~~~~~
           38:  %_1 = alloca %Type1, align 1 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           39:  %b.0 = bitcast %Type1* %_1 to {}* 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           40:  %0 = load void ({}*)*, void ({}*)** getelementptr inbounds (void ({}*)*, void ({}*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.0 to void ({}*)**), i64 3), align 8, !invariant.load !3, !nonnull !3 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           41:  call void %0({}* align 1 %b.0) #2 [ "kcfi"(i32 -56908610) ] 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
label:47'1                          ?                                        possible intended match
           42:  ret void 
label:47'0     ~~~~~~~~~~
           43: } 
label:47'0     ~~
           44:  
label:47'0     ~
           45: ; sanitizer_kcfi_emit_type_metadata_trait_objects::baz 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           46: ; Function Attrs: noredzone nounwind 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           47: define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects3baz17hf719ac674d79536dE() unnamed_addr #1 !<unknown kind #36> !1 { 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           48: start: 
label:47'0     ~~~~~~~
           49:  %_1 = alloca %Type1, align 1 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           50:  %b.0 = bitcast %Type1* %_1 to {}* 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           51: ; call <sanitizer_kcfi_emit_type_metadata_trait_objects::Type1 as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait1>::foo 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           52:  call void @"_ZN130_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h3ed069aedb940e54E"(%Type1* align 1 %_1) #2 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           53:  %0 = load void ({}*)*, void ({}*)** getelementptr inbounds (void ({}*)*, void ({}*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.0 to void ({}*)**), i64 3), align 8, !invariant.load !3, !nonnull !3 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           54:  call void %0({}* align 1 %b.0) #2 [ "kcfi"(i32 -56908610) ] 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           55:  ret void 
label:47'0     ~~~~~~~~~~
           56: } 
label:47'0     ~~
           57:  
label:47'0     ~
           58: attributes #0 = { inlinehint noredzone nounwind "target-cpu"="generic" "target-features"="+v8a,+strict-align,+neon,+fp-armv8" } 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           59: attributes #1 = { noredzone nounwind "target-cpu"="generic" "target-features"="+v8a,+strict-align,+neon,+fp-armv8" } 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           60: attributes #2 = { nounwind } 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           61:  
label:47'0     ~
           62: !llvm.module.flags = !{!0} 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
           63:  
label:47'0     ~
           64: !0 = !{i32 4, !"kcfi", i32 1} 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           65: !1 = !{i32 -1522505972} 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~~~
           66: !2 = !{i32 -56908610} 
label:47'0     ~~~~~~~~~~~~~~~~~~~~~~
           67: !3 = !{} 
label:47'0     ~~~~~~~~~
------------------------------------------



