plain
running 463 tests
i....i.i...ii...........i....i..ii.................iii........ii.i.i............i.......  88/463
.............ii.................i................i....i........i..iiii...iiii.iiii....i. 176/463
.iiiii.......i.i.i....i.i.iiii.iiii.....ii....i.ii......i...i......................i...i 264/463
i..i.i.....ii..i.ii..............ii.i........................F..ii......F............... 352/463
iiii...................

failures:


---- [codegen] tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs#aarch64 stdout ----

error in revision `aarch64`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-14/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.aarch64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll" "/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,aarch64" "--dump-input-context" "100"
stdout: none
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:56:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:56:12: error: CHECK: expected string not found in input
 // CHECK: call void %0(ptr align 1 %_1){{.*}}[ "kcfi"(i32 [[TYPE1:[[:print:]]+]]) ]
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.aarch64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll:36:141: note: scanning from here
Build completed unsuccessfully in 0:13:23
define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects3bar17hcd008b5f757bbd00E() unnamed_addr #1 !<unknown kind #36> !1 {
                                                                                                                                            ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.aarch64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll:41:2: note: possible intended match here
 call void %0({}* align 1 %b.0) #2 [ "kcfi"(i32 -56908610) ]
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:66:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:66:12: error: CHECK: expected string not found in input
 // CHECK: call void %0(ptr align 1 %_1){{.*}}[ "kcfi"(i32 [[TYPE1:[[:print:]]+]]) ]
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.aarch64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll:51:128: note: scanning from here
; call <sanitizer_kcfi_emit_type_metadata_trait_objects::Type1 as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait1>::foo
                                                                                                                               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.aarch64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll:54:2: note: possible intended match here
 call void %0({}* align 1 %b.0) #2 [ "kcfi"(i32 -56908610) ]


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.aarch64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'sanitizer_kcfi_emit_type_metadata_trait_objects.74fbb9ecb80d2934-cgu.0' 
            2: source_filename = "sanitizer_kcfi_emit_type_metadata_trait_objects.74fbb9ecb80d2934-cgu.0" 
            3: target datalayout = "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128" 
            4: target triple = "aarch64-unknown-none" 
            5:  
            6: %Type1 = type {} 
            7:  
            8: @vtable.0 = private unnamed_addr constant <{ i8*, [16 x i8], i8* }> <{ i8* bitcast (void ()* @"_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects78drop_in_place_fn$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$GT$17hd544d746556e563bE" to i8*), [16 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", i8* bitcast (void (%Type1*)* @"_ZN130_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h3ed069aedb940e54E" to i8*) }>, align 8 
            9:  
           10: ; sanitizer_kcfi_emit_type_metadata_trait_objects::drop_in_place_fn<sanitizer_kcfi_emit_type_metadata_trait_objects::Type1> 
           11: ; Function Attrs: inlinehint noredzone nounwind 
           12: define internal void @"_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects78drop_in_place_fn$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$GT$17hd544d746556e563bE"() unnamed_addr #0 !<unknown kind #36> !1 { 
           14:  ret void 
           15: } 
           16:  
           16:  
           17: ; <sanitizer_kcfi_emit_type_metadata_trait_objects::Type1 as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait1>::foo 
           18: ; Function Attrs: noredzone nounwind 
           19: define dso_local void @"_ZN130_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h3ed069aedb940e54E"(%Type1* align 1 %self) unnamed_addr #1 !<unknown kind #36> !2 { 
           21:  ret void 
           22: } 
           23:  
           23:  
           24: ; sanitizer_kcfi_emit_type_metadata_trait_objects::foo 
           25: ; Function Attrs: noredzone nounwind 
           26: define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects3foo17he4aee4a0b2685663E() unnamed_addr #1 !<unknown kind #36> !1 { 
           27: start: 
           28:  %_1 = alloca %Type1, align 1 
           29: ; call <sanitizer_kcfi_emit_type_metadata_trait_objects::Type1 as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait1>::foo 
           30:  call void @"_ZN130_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h3ed069aedb940e54E"(%Type1* align 1 %_1) #2 
           31:  ret void 
           32: } 
           33:  
           34: ; sanitizer_kcfi_emit_type_metadata_trait_objects::bar 
           35: ; Function Attrs: noredzone nounwind 
           36: define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects3bar17hcd008b5f757bbd00E() unnamed_addr #1 !<unknown kind #36> !1 { 
check:56'0                                                                                                                                                 X~~ error: no match found
           37: start: 
check:56'0     ~~~~~~~
           38:  %_1 = alloca %Type1, align 1 
check:56'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           39:  %b.0 = bitcast %Type1* %_1 to {}* 
check:56'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           40:  %0 = load void ({}*)*, void ({}*)** getelementptr inbounds (void ({}*)*, void ({}*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.0 to void ({}*)**), i64 3), align 8, !invariant.load !3, !nonnull !3 
check:56'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           41:  call void %0({}* align 1 %b.0) #2 [ "kcfi"(i32 -56908610) ] 
check:56'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:56'1      ?                                                            possible intended match
           42:  ret void 
check:56'0     ~~~~~~~~~~
           43: } 
check:56'0     ~~
           44:  
check:56'0     ~
           45: ; sanitizer_kcfi_emit_type_metadata_trait_objects::baz 
check:56'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           46: ; Function Attrs: noredzone nounwind 
check:56'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           47: define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects3baz17hf719ac674d79536dE() unnamed_addr #1 !<unknown kind #36> !1 { 
check:56'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           48: start: 
           49:  %_1 = alloca %Type1, align 1 
           50:  %b.0 = bitcast %Type1* %_1 to {}* 
           51: ; call <sanitizer_kcfi_emit_type_metadata_trait_objects::Type1 as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait1>::foo 
check:66'0                                                                                                                                    X error: no match found
           52:  call void @"_ZN130_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h3ed069aedb940e54E"(%Type1* align 1 %_1) #2 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           53:  %0 = load void ({}*)*, void ({}*)** getelementptr inbounds (void ({}*)*, void ({}*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.0 to void ({}*)**), i64 3), align 8, !invariant.load !3, !nonnull !3 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           54:  call void %0({}* align 1 %b.0) #2 [ "kcfi"(i32 -56908610) ] 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:66'1      ?                                                            possible intended match
           55:  ret void 
check:66'0     ~~~~~~~~~~
           56: } 
check:66'0     ~~
           57:  
check:66'0     ~
           58: attributes #0 = { inlinehint noredzone nounwind "target-cpu"="generic" "target-features"="+v8a,+strict-align,+neon,+fp-armv8" } 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           59: attributes #1 = { noredzone nounwind "target-cpu"="generic" "target-features"="+v8a,+strict-align,+neon,+fp-armv8" } 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           60: attributes #2 = { nounwind } 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           61:  
check:66'0     ~
           62: !llvm.module.flags = !{!0} 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
           63:  
check:66'0     ~
           64: !0 = !{i32 4, !"kcfi", i32 1} 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           65: !1 = !{i32 -1522505972} 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~
           66: !2 = !{i32 -56908610} 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~
           67: !3 = !{} 
check:66'0     ~~~~~~~~~
------------------------------------------



---- [codegen] tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs#x86_64 stdout ----

error in revision `x86_64`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-14/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.x86_64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll" "/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,x86_64" "--dump-input-context" "100"
stdout: none
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:56:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:56:12: error: CHECK: expected string not found in input
 // CHECK: call void %0(ptr align 1 %_1){{.*}}[ "kcfi"(i32 [[TYPE1:[[:print:]]+]]) ]
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.x86_64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll:36:131: note: scanning from here
define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects3bar17hcd008b5f757bbd00E() unnamed_addr #1 !<unknown kind #36> !4 {
                                                                                                                                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.x86_64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll:41:2: note: possible intended match here
 call void %0({}* align 1 %b.0) #2 [ "kcfi"(i32 -56908610) ]
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:66:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:66:12: error: CHECK: expected string not found in input
 // CHECK: call void %0(ptr align 1 %_1){{.*}}[ "kcfi"(i32 [[TYPE1:[[:print:]]+]]) ]
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.x86_64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll:51:128: note: scanning from here
; call <sanitizer_kcfi_emit_type_metadata_trait_objects::Type1 as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait1>::foo
                                                                                                                               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.x86_64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll:54:2: note: possible intended match here
 call void %0({}* align 1 %b.0) #2 [ "kcfi"(i32 -56908610) ]


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.x86_64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
<<<<<<
            1: ; ModuleID = 'sanitizer_kcfi_emit_type_metadata_trait_objects.74fbb9ecb80d2934-cgu.0' 
            2: source_filename = "sanitizer_kcfi_emit_type_metadata_trait_objects.74fbb9ecb80d2934-cgu.0" 
            3: target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128" 
            4: target triple = "x86_64-unknown-none-elf" 
            5:  
            6: %Type1 = type {} 
            7:  
            8: @vtable.0 = private unnamed_addr constant <{ i8*, [16 x i8], i8* }> <{ i8* bitcast (void ()* @"_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects78drop_in_place_fn$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$GT$17hd544d746556e563bE" to i8*), [16 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", i8* bitcast (void (%Type1*)* @"_ZN130_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h3ed069aedb940e54E" to i8*) }>, align 8 
            9:  
           10: ; sanitizer_kcfi_emit_type_metadata_trait_objects::drop_in_place_fn<sanitizer_kcfi_emit_type_metadata_trait_objects::Type1> 
           11: ; Function Attrs: inlinehint noredzone nounwind nonlazybind 
           12: define internal void @"_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects78drop_in_place_fn$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$GT$17hd544d746556e563bE"() unnamed_addr #0 !<unknown kind #36> !4 { 
           14:  ret void 
           15: } 
           16:  
           16:  
           17: ; <sanitizer_kcfi_emit_type_metadata_trait_objects::Type1 as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait1>::foo 
           18: ; Function Attrs: noredzone nounwind nonlazybind 
           19: define void @"_ZN130_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h3ed069aedb940e54E"(%Type1* align 1 %self) unnamed_addr #1 !<unknown kind #36> !5 { 
           21:  ret void 
           22: } 
           23:  
           23:  
           24: ; sanitizer_kcfi_emit_type_metadata_trait_objects::foo 
           25: ; Function Attrs: noredzone nounwind nonlazybind 
           26: define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects3foo17he4aee4a0b2685663E() unnamed_addr #1 !<unknown kind #36> !4 { 
           27: start: 
           28:  %_1 = alloca %Type1, align 1 
           29: ; call <sanitizer_kcfi_emit_type_metadata_trait_objects::Type1 as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait1>::foo 
           30:  call void @"_ZN130_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h3ed069aedb940e54E"(%Type1* align 1 %_1) #2 
           31:  ret void 
           32: } 
           33:  
           34: ; sanitizer_kcfi_emit_type_metadata_trait_objects::bar 
           35: ; Function Attrs: noredzone nounwind nonlazybind 
           36: define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects3bar17hcd008b5f757bbd00E() unnamed_addr #1 !<unknown kind #36> !4 { 
check:56'0                                                                                                                                       X~~ error: no match found
           37: start: 
check:56'0     ~~~~~~~
           38:  %_1 = alloca %Type1, align 1 
check:56'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           39:  %b.0 = bitcast %Type1* %_1 to {}* 
check:56'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           40:  %0 = load void ({}*)*, void ({}*)** getelementptr inbounds (void ({}*)*, void ({}*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.0 to void ({}*)**), i64 3), align 8, !invariant.load !6, !nonnull !6 
check:56'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           41:  call void %0({}* align 1 %b.0) #2 [ "kcfi"(i32 -56908610) ] 
check:56'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:56'1      ?                                                            possible intended match
           42:  ret void 
check:56'0     ~~~~~~~~~~
           43: } 
check:56'0     ~~
           44:  
check:56'0     ~
           45: ; sanitizer_kcfi_emit_type_metadata_trait_objects::baz 
check:56'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           46: ; Function Attrs: noredzone nounwind nonlazybind 
check:56'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           47: define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects3baz17hf719ac674d79536dE() unnamed_addr #1 !<unknown kind #36> !4 { 
check:56'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           48: start: 
           49:  %_1 = alloca %Type1, align 1 
           50:  %b.0 = bitcast %Type1* %_1 to {}* 
           51: ; call <sanitizer_kcfi_emit_type_metadata_trait_objects::Type1 as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait1>::foo 
check:66'0                                                                                                                                    X error: no match found
           52:  call void @"_ZN130_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h3ed069aedb940e54E"(%Type1* align 1 %_1) #2 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           53:  %0 = load void ({}*)*, void ({}*)** getelementptr inbounds (void ({}*)*, void ({}*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.0 to void ({}*)**), i64 3), align 8, !invariant.load !6, !nonnull !6 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           54:  call void %0({}* align 1 %b.0) #2 [ "kcfi"(i32 -56908610) ] 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:66'1      ?                                                            possible intended match
           55:  ret void 
check:66'0     ~~~~~~~~~~
           56: } 
check:66'0     ~~
           57:  
check:66'0     ~
           58: attributes #0 = { inlinehint noredzone nounwind nonlazybind "probe-stack"="__rust_probestack" "target-cpu"="x86-64" "target-features"="-mmx,-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-3dnow,-3dnowa,-avx,-avx2,+soft-float" } 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           59: attributes #1 = { noredzone nounwind nonlazybind "probe-stack"="__rust_probestack" "target-cpu"="x86-64" "target-features"="-mmx,-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-3dnow,-3dnowa,-avx,-avx2,+soft-float" } 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           60: attributes #2 = { nounwind } 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           61:  
check:66'0     ~
           62: !llvm.module.flags = !{!0, !1, !2, !3} 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           63:  
check:66'0     ~
           64: !0 = !{i32 7, !"PIC Level", i32 2} 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           65: !1 = !{i32 1, !"Code Model", i32 2} 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           66: !2 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           67: !3 = !{i32 4, !"kcfi", i32 1} 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           68: !4 = !{i32 -1522505972} 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~~~
           69: !5 = !{i32 -56908610} 
check:66'0     ~~~~~~~~~~~~~~~~~~~~~~
           70: !6 = !{} 
check:66'0     ~~~~~~~~~
------------------------------------------



