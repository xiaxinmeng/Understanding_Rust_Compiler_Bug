plain
running 465 tests
i.i...i.i..ii............i....i..ii.................iii........ii.i.i............i......  88/465
..............ii..................i..............i.....i........i..iiii....iiiiiiii....i 176/465
..iiiii........ii.i....i.i.iiii.iiii.....ii....i.ii......i..i.......................i... 264/465
ii..i.i.....ii..i.ii..............ii.i...........................ii......F.....F........ 352/465
iiiiii...................

failures:


---- [codegen] tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs#aarch64 stdout ----

error in revision `aarch64`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-14/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.aarch64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll" "/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,aarch64" "--dump-input-context" "100"
stdout: none
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:99:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:99:12: error: CHECK: expected string not found in input
 // CHECK: call void %{{[0-9]}}({{\{\}\*|ptr}} align 1 {{%[a-z]\.0|%_[0-9]}}, {{\{\}\*|ptr}} align 1 {{%[a-z]\.0|%_[0-9]}}){{.*}}[ "kcfi"(i32 [[TYPE3:[[:print:]]+]]) ]
           ^
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.aarch64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll:105:183: note: scanning from here
define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo317hdb324cf16b626e72E({}* align 1 %a.0, [3 x i64]* align 8 %a.1) unnamed_addr #1 !<unknown kind #36> !6 {
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:108:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:108:12: error: CHECK: expected string not found in input
 // CHECK: call void %{{[0-9]}}({{\{\}\*|ptr}} align 1 {{%[a-z]\.0|%_[0-9]}}, {{\{\}\*|ptr}} align 1 {{%[a-z]\.0|%_[0-9]}}){{.*}}[ "kcfi"(i32 [[TYPE3:[[:print:]]+]]) ]
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.aarch64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll:117:142: note: scanning from here
define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4bar317hd298b3b9b9d428c0E() unnamed_addr #1 !<unknown kind #36> !1 {


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.aarch64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
           .
           .
           .
           .
           5:  
           6: %Type1 = type {} 
           7: %Type2 = type {} 
           8: %Type3 = type {} 
           9:  
          10: @vtable.0 = private unnamed_addr constant <{ i8*, [16 x i8], i8* }> <{ i8* bitcast (void ()* @"_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects78drop_in_place_fn$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$GT$17hd544d746556e563bE" to i8*), [16 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", i8* bitcast (void (%Type1*)* @"_ZN130_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h3ed069aedb940e54E" to i8*) }>, align 8 
          11: @vtable.1 = private unnamed_addr constant <{ i8*, [16 x i8], i8* }> <{ i8* bitcast (void ()* @"_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects78drop_in_place_fn$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type2$GT$17h8aa09b16cfc8ee65E" to i8*), [16 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", i8* bitcast (void (%Type2*)* @"_ZN141_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type2$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait2$LT$i32$GT$$GT$3bar17hc43b8b747eac5e80E" to i8*) }>, align 8 
          12: @vtable.2 = private unnamed_addr constant <{ i8*, [16 x i8], i8* }> <{ i8* bitcast (void ()* @"_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects78drop_in_place_fn$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type3$GT$17h3b9500d3f54315f5E" to i8*), [16 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", i8* bitcast (void (%Type3*, %Type3*)* @"_ZN86_$LT$T$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait3$LT$U$GT$$GT$3baz17h3db20bfe4fcf6962E" to i8*) }>, align 8 
          13:  
          14: ; sanitizer_kcfi_emit_type_metadata_trait_objects::drop_in_place_fn<sanitizer_kcfi_emit_type_metadata_trait_objects::Type1> 
          15: ; Function Attrs: inlinehint noredzone nounwind 
          16: define internal void @"_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects78drop_in_place_fn$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$GT$17hd544d746556e563bE"() unnamed_addr #0 !<unknown kind #36> !1 { 
          18:  ret void 
          19: } 
          20:  
          20:  
          21: ; sanitizer_kcfi_emit_type_metadata_trait_objects::drop_in_place_fn<sanitizer_kcfi_emit_type_metadata_trait_objects::Type2> 
          22: ; Function Attrs: inlinehint noredzone nounwind 
          23: define internal void @"_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects78drop_in_place_fn$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type2$GT$17h8aa09b16cfc8ee65E"() unnamed_addr #0 !<unknown kind #36> !1 { 
          25:  ret void 
          26: } 
          27:  
          27:  
          28: ; sanitizer_kcfi_emit_type_metadata_trait_objects::drop_in_place_fn<sanitizer_kcfi_emit_type_metadata_trait_objects::Type3> 
          29: ; Function Attrs: inlinehint noredzone nounwind 
          30: define internal void @"_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects78drop_in_place_fn$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type3$GT$17h3b9500d3f54315f5E"() unnamed_addr #0 !<unknown kind #36> !1 { 
          32:  ret void 
          33: } 
          34:  
          34:  
          35: ; <sanitizer_kcfi_emit_type_metadata_trait_objects::Type1 as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait1>::foo 
          36: ; Function Attrs: noredzone nounwind 
          37: define dso_local void @"_ZN130_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h3ed069aedb940e54E"(%Type1* align 1 %self) unnamed_addr #1 !<unknown kind #36> !2 { 
          39:  ret void 
          40: } 
          41:  
          41:  
          42: ; <sanitizer_kcfi_emit_type_metadata_trait_objects::Type2 as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait2<i32>>::bar 
          43: ; Function Attrs: noredzone nounwind 
          44: define dso_local void @"_ZN141_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type2$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait2$LT$i32$GT$$GT$3bar17hc43b8b747eac5e80E"(%Type2* align 1 %self) unnamed_addr #1 !<unknown kind #36> !3 { 
          46:  ret void 
          47: } 
          48:  
          48:  
          49: ; <T as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait3<U>>::baz 
          50: ; Function Attrs: noredzone nounwind 
          51: define dso_local void @"_ZN86_$LT$T$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait3$LT$U$GT$$GT$3baz17h3db20bfe4fcf6962E"(%Type3* align 1 %self, %Type3* align 1 %_2) unnamed_addr #1 !<unknown kind #36> !4 { 
          53:  ret void 
          54: } 
          55:  
          55:  
          56: ; sanitizer_kcfi_emit_type_metadata_trait_objects::foo1 
          57: ; Function Attrs: noredzone nounwind 
          58: define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo117h93502349188b3e24E({}* align 1 %a.0, [3 x i64]* align 8 %a.1) unnamed_addr #1 !<unknown kind #36> !2 { 
          59: start: 
          60:  %0 = bitcast [3 x i64]* %a.1 to void ({}*)** 
          61:  %1 = getelementptr inbounds void ({}*)*, void ({}*)** %0, i64 3 
          62:  %2 = load void ({}*)*, void ({}*)** %1, align 8, !invariant.load !5, !nonnull !5 
          63:  call void %2({}* align 1 %a.0) #2 [ "kcfi"(i32 -56908610) ] 
          64:  ret void 
          65: } 
          66:  
          67: ; sanitizer_kcfi_emit_type_metadata_trait_objects::bar1 
          68: ; Function Attrs: noredzone nounwind 
          69: define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4bar117had3ab10c0b90936dE() unnamed_addr #1 !<unknown kind #36> !1 { 
          70: start: 
          71:  %_1 = alloca %Type1, align 1 
          72:  %b.0 = bitcast %Type1* %_1 to {}* 
          73:  %0 = load void ({}*)*, void ({}*)** getelementptr inbounds (void ({}*)*, void ({}*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.0 to void ({}*)**), i64 3), align 8, !invariant.load !5, !nonnull !5 
          74:  call void %0({}* align 1 %b.0) #2 [ "kcfi"(i32 -56908610) ] 
          75:  ret void 
          76: } 
          77:  
          78: ; sanitizer_kcfi_emit_type_metadata_trait_objects::foo2 
          79: ; Function Attrs: noredzone nounwind 
          80: define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo217h8161e42dd83c0851E({}* align 1 %a.0, [3 x i64]* align 8 %a.1) unnamed_addr #1 !<unknown kind #36> !3 { 
          81: start: 
          82:  %0 = bitcast [3 x i64]* %a.1 to void ({}*)** 
          83:  %1 = getelementptr inbounds void ({}*)*, void ({}*)** %0, i64 3 
          84:  %2 = load void ({}*)*, void ({}*)** %1, align 8, !invariant.load !5, !nonnull !5 
          85:  call void %2({}* align 1 %a.0) #2 [ "kcfi"(i32 102973893) ] 
          86:  ret void 
          87: } 
          88:  
          89: ; sanitizer_kcfi_emit_type_metadata_trait_objects::bar2 
          90: ; Function Attrs: noredzone nounwind 
          91: define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4bar217h18ebf9f7b2e09ed4E() unnamed_addr #1 !<unknown kind #36> !1 { 
          92: start: 
          93:  %_1 = alloca %Type2, align 1 
          94:  %_3.0 = bitcast %Type2* %_1 to {}* 
          95: ; call sanitizer_kcfi_emit_type_metadata_trait_objects::foo2 
          96:  call void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo217h8161e42dd83c0851E({}* align 1 %_3.0, [3 x i64]* align 8 bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.1 to [3 x i64]*)) #2 
          97:  %b.0 = bitcast %Type2* %_1 to {}* 
          98:  %0 = load void ({}*)*, void ({}*)** getelementptr inbounds (void ({}*)*, void ({}*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.1 to void ({}*)**), i64 3), align 8, !invariant.load !5, !nonnull !5 
          99:  call void %0({}* align 1 %b.0) #2 [ "kcfi"(i32 102973893) ] 
         100:  ret void 
         101: } 
         102:  
         103: ; sanitizer_kcfi_emit_type_metadata_trait_objects::foo3 
         104: ; Function Attrs: noredzone nounwind 
         105: define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo317hdb324cf16b626e72E({}* align 1 %a.0, [3 x i64]* align 8 %a.1) unnamed_addr #1 !<unknown kind #36> !6 { 
check:99                                                                                                                                                                                            X~~ error: no match found
check:99      ~~~~~~~
check:99      ~~~~~~~
         107:  %_2 = alloca %Type3, align 1 
check:99      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         108:  %0 = bitcast [3 x i64]* %a.1 to void ({}*, %Type3*)** 
check:99      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         109:  %1 = getelementptr inbounds void ({}*, %Type3*)*, void ({}*, %Type3*)** %0, i64 3 
check:99      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         110:  %2 = load void ({}*, %Type3*)*, void ({}*, %Type3*)** %1, align 8, !invariant.load !5, !nonnull !5 
check:99      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         111:  call void %2({}* align 1 %a.0, %Type3* align 1 %_2) #2 [ "kcfi"(i32 -1903783964) ] 
         112:  ret void 
check:99      ~~~~~~~~~~
         113: } 
check:99      ~~
check:99      ~~
         114:  
check:99      ~
         115: ; sanitizer_kcfi_emit_type_metadata_trait_objects::bar3 
check:99      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         116: ; Function Attrs: noredzone nounwind 
check:99      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         117: define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4bar317hd298b3b9b9d428c0E() unnamed_addr #1 !<unknown kind #36> !1 { 
check:99      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:108                                                                                                                                                  X~~ error: no match found
check:108     ~~~~~~~
check:108     ~~~~~~~
         119:  %_1 = alloca %Type3, align 1 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         120:  %_3.0 = bitcast %Type3* %_1 to {}* 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         121: ; call sanitizer_kcfi_emit_type_metadata_trait_objects::foo3 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         122:  call void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo317hdb324cf16b626e72E({}* align 1 %_3.0, [3 x i64]* align 8 bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.2 to [3 x i64]*)) #2 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         123:  %b.0 = bitcast %Type3* %_1 to {}* 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         124:  %0 = load void ({}*, %Type3*)*, void ({}*, %Type3*)** getelementptr inbounds (void ({}*, %Type3*)*, void ({}*, %Type3*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.2 to void ({}*, %Type3*)**), i64 3), align 8, !invariant.load !5, !nonnull !5 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         125:  call void %0({}* align 1 %b.0, %Type3* align 1 %_1) #2 [ "kcfi"(i32 -1903783964) ] 
         126:  ret void 
check:108     ~~~~~~~~~~
         127: } 
check:108     ~~
check:108     ~~
         128:  
check:108     ~
         129: attributes #0 = { inlinehint noredzone nounwind "target-cpu"="generic" "target-features"="+v8a,+strict-align,+neon,+fp-armv8" } 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         130: attributes #1 = { noredzone nounwind "target-cpu"="generic" "target-features"="+v8a,+strict-align,+neon,+fp-armv8" } 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         131: attributes #2 = { nounwind } 
         132:  
check:108     ~
check:108     ~
         133: !llvm.module.flags = !{!0} 
         134:  
check:108     ~
check:108     ~
         135: !0 = !{i32 4, !"kcfi", i32 1} 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         136: !1 = !{i32 -1522505972} 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~
         137: !2 = !{i32 -56908610} 
check:108     ~~~~~~~~~~~~~~~~~~~~~~
         138: !3 = !{i32 102973893} 
check:108     ~~~~~~~~~~~~~~~~~~~~~~
         139: !4 = !{i32 -1903783964} 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~
         140: !5 = !{} 
check:108     ~~~~~~~~~
         141: !6 = !{i32 -1134101214} 
>>>>>>
------------------------------------------



---- [codegen] tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs#x86_64 stdout ----

error in revision `x86_64`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-14/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.x86_64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll" "/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,x86_64" "--dump-input-context" "100"
stdout: none
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:99:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:99:12: error: CHECK: expected string not found in input
 // CHECK: call void %{{[0-9]}}({{\{\}\*|ptr}} align 1 {{%[a-z]\.0|%_[0-9]}}, {{\{\}\*|ptr}} align 1 {{%[a-z]\.0|%_[0-9]}}){{.*}}[ "kcfi"(i32 [[TYPE3:[[:print:]]+]]) ]
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.x86_64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll:105:173: note: scanning from here
define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo317hdb324cf16b626e72E({}* align 1 %a.0, [3 x i64]* align 8 %a.1) unnamed_addr #1 !<unknown kind #36> !9 {
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:108:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:108:12: error: CHECK: expected string not found in input
 // CHECK: call void %{{[0-9]}}({{\{\}\*|ptr}} align 1 {{%[a-z]\.0|%_[0-9]}}, {{\{\}\*|ptr}} align 1 {{%[a-z]\.0|%_[0-9]}}){{.*}}[ "kcfi"(i32 [[TYPE3:[[:print:]]+]]) ]
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.x86_64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll:117:132: note: scanning from here
define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4bar317hd298b3b9b9d428c0E() unnamed_addr #1 !<unknown kind #36> !4 {


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.x86_64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
           .
           .
           .
           .
           5:  
           6: %Type1 = type {} 
           7: %Type2 = type {} 
           8: %Type3 = type {} 
           9:  
          10: @vtable.0 = private unnamed_addr constant <{ i8*, [16 x i8], i8* }> <{ i8* bitcast (void ()* @"_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects78drop_in_place_fn$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$GT$17hd544d746556e563bE" to i8*), [16 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", i8* bitcast (void (%Type1*)* @"_ZN130_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h3ed069aedb940e54E" to i8*) }>, align 8 
          11: @vtable.1 = private unnamed_addr constant <{ i8*, [16 x i8], i8* }> <{ i8* bitcast (void ()* @"_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects78drop_in_place_fn$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type2$GT$17h8aa09b16cfc8ee65E" to i8*), [16 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", i8* bitcast (void (%Type2*)* @"_ZN141_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type2$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait2$LT$i32$GT$$GT$3bar17hc43b8b747eac5e80E" to i8*) }>, align 8 
          12: @vtable.2 = private unnamed_addr constant <{ i8*, [16 x i8], i8* }> <{ i8* bitcast (void ()* @"_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects78drop_in_place_fn$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type3$GT$17h3b9500d3f54315f5E" to i8*), [16 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", i8* bitcast (void (%Type3*, %Type3*)* @"_ZN86_$LT$T$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait3$LT$U$GT$$GT$3baz17h3db20bfe4fcf6962E" to i8*) }>, align 8 
          13:  
          14: ; sanitizer_kcfi_emit_type_metadata_trait_objects::drop_in_place_fn<sanitizer_kcfi_emit_type_metadata_trait_objects::Type1> 
          15: ; Function Attrs: inlinehint noredzone nounwind nonlazybind 
          16: define internal void @"_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects78drop_in_place_fn$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$GT$17hd544d746556e563bE"() unnamed_addr #0 !<unknown kind #36> !4 { 
          18:  ret void 
          19: } 
          20:  
          20:  
          21: ; sanitizer_kcfi_emit_type_metadata_trait_objects::drop_in_place_fn<sanitizer_kcfi_emit_type_metadata_trait_objects::Type2> 
          22: ; Function Attrs: inlinehint noredzone nounwind nonlazybind 
          23: define internal void @"_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects78drop_in_place_fn$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type2$GT$17h8aa09b16cfc8ee65E"() unnamed_addr #0 !<unknown kind #36> !4 { 
          25:  ret void 
          26: } 
          27:  
          27:  
          28: ; sanitizer_kcfi_emit_type_metadata_trait_objects::drop_in_place_fn<sanitizer_kcfi_emit_type_metadata_trait_objects::Type3> 
          29: ; Function Attrs: inlinehint noredzone nounwind nonlazybind 
          30: define internal void @"_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects78drop_in_place_fn$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type3$GT$17h3b9500d3f54315f5E"() unnamed_addr #0 !<unknown kind #36> !4 { 
          32:  ret void 
          33: } 
          34:  
          34:  
          35: ; <sanitizer_kcfi_emit_type_metadata_trait_objects::Type1 as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait1>::foo 
          36: ; Function Attrs: noredzone nounwind nonlazybind 
          37: define void @"_ZN130_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type1$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait1$GT$3foo17h3ed069aedb940e54E"(%Type1* align 1 %self) unnamed_addr #1 !<unknown kind #36> !5 { 
          39:  ret void 
          40: } 
          41:  
          41:  
          42: ; <sanitizer_kcfi_emit_type_metadata_trait_objects::Type2 as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait2<i32>>::bar 
          43: ; Function Attrs: noredzone nounwind nonlazybind 
          44: define void @"_ZN141_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type2$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait2$LT$i32$GT$$GT$3bar17hc43b8b747eac5e80E"(%Type2* align 1 %self) unnamed_addr #1 !<unknown kind #36> !6 { 
          46:  ret void 
          47: } 
          48:  
          48:  
          49: ; <T as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait3<U>>::baz 
          50: ; Function Attrs: noredzone nounwind nonlazybind 
          51: define void @"_ZN86_$LT$T$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait3$LT$U$GT$$GT$3baz17h3db20bfe4fcf6962E"(%Type3* align 1 %self, %Type3* align 1 %_2) unnamed_addr #1 !<unknown kind #36> !7 { 
          53:  ret void 
          54: } 
          55:  
          55:  
          56: ; sanitizer_kcfi_emit_type_metadata_trait_objects::foo1 
          57: ; Function Attrs: noredzone nounwind nonlazybind 
          58: define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo117h93502349188b3e24E({}* align 1 %a.0, [3 x i64]* align 8 %a.1) unnamed_addr #1 !<unknown kind #36> !5 { 
          59: start: 
          60:  %0 = bitcast [3 x i64]* %a.1 to void ({}*)** 
          61:  %1 = getelementptr inbounds void ({}*)*, void ({}*)** %0, i64 3 
          62:  %2 = load void ({}*)*, void ({}*)** %1, align 8, !invariant.load !8, !nonnull !8 
          63:  call void %2({}* align 1 %a.0) #2 [ "kcfi"(i32 -56908610) ] 
          64:  ret void 
          65: } 
          66:  
          67: ; sanitizer_kcfi_emit_type_metadata_trait_objects::bar1 
          68: ; Function Attrs: noredzone nounwind nonlazybind 
          69: define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4bar117had3ab10c0b90936dE() unnamed_addr #1 !<unknown kind #36> !4 { 
          70: start: 
          71:  %_1 = alloca %Type1, align 1 
          72:  %b.0 = bitcast %Type1* %_1 to {}* 
          73:  %0 = load void ({}*)*, void ({}*)** getelementptr inbounds (void ({}*)*, void ({}*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.0 to void ({}*)**), i64 3), align 8, !invariant.load !8, !nonnull !8 
          74:  call void %0({}* align 1 %b.0) #2 [ "kcfi"(i32 -56908610) ] 
          75:  ret void 
          76: } 
          77:  
          78: ; sanitizer_kcfi_emit_type_metadata_trait_objects::foo2 
          79: ; Function Attrs: noredzone nounwind nonlazybind 
          80: define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo217h8161e42dd83c0851E({}* align 1 %a.0, [3 x i64]* align 8 %a.1) unnamed_addr #1 !<unknown kind #36> !6 { 
          81: start: 
          82:  %0 = bitcast [3 x i64]* %a.1 to void ({}*)** 
          83:  %1 = getelementptr inbounds void ({}*)*, void ({}*)** %0, i64 3 
          84:  %2 = load void ({}*)*, void ({}*)** %1, align 8, !invariant.load !8, !nonnull !8 
          85:  call void %2({}* align 1 %a.0) #2 [ "kcfi"(i32 102973893) ] 
          86:  ret void 
          87: } 
          88:  
          89: ; sanitizer_kcfi_emit_type_metadata_trait_objects::bar2 
          90: ; Function Attrs: noredzone nounwind nonlazybind 
          91: define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4bar217h18ebf9f7b2e09ed4E() unnamed_addr #1 !<unknown kind #36> !4 { 
          92: start: 
          93:  %_1 = alloca %Type2, align 1 
          94:  %_3.0 = bitcast %Type2* %_1 to {}* 
          95: ; call sanitizer_kcfi_emit_type_metadata_trait_objects::foo2 
          96:  call void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo217h8161e42dd83c0851E({}* align 1 %_3.0, [3 x i64]* align 8 bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.1 to [3 x i64]*)) #2 
          97:  %b.0 = bitcast %Type2* %_1 to {}* 
          98:  %0 = load void ({}*)*, void ({}*)** getelementptr inbounds (void ({}*)*, void ({}*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.1 to void ({}*)**), i64 3), align 8, !invariant.load !8, !nonnull !8 
          99:  call void %0({}* align 1 %b.0) #2 [ "kcfi"(i32 102973893) ] 
         100:  ret void 
         101: } 
         102:  
         103: ; sanitizer_kcfi_emit_type_metadata_trait_objects::foo3 
         104: ; Function Attrs: noredzone nounwind nonlazybind 
         105: define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo317hdb324cf16b626e72E({}* align 1 %a.0, [3 x i64]* align 8 %a.1) unnamed_addr #1 !<unknown kind #36> !9 { 
check:99                                                                                                                                                                                  X~~ error: no match found
check:99      ~~~~~~~
check:99      ~~~~~~~
         107:  %_2 = alloca %Type3, align 1 
check:99      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         108:  %0 = bitcast [3 x i64]* %a.1 to void ({}*, %Type3*)** 
check:99      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         109:  %1 = getelementptr inbounds void ({}*, %Type3*)*, void ({}*, %Type3*)** %0, i64 3 
check:99      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         110:  %2 = load void ({}*, %Type3*)*, void ({}*, %Type3*)** %1, align 8, !invariant.load !8, !nonnull !8 
check:99      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         111:  call void %2({}* align 1 %a.0, %Type3* align 1 %_2) #2 [ "kcfi"(i32 -1903783964) ] 
         112:  ret void 
check:99      ~~~~~~~~~~
         113: } 
check:99      ~~
check:99      ~~
         114:  
check:99      ~
         115: ; sanitizer_kcfi_emit_type_metadata_trait_objects::bar3 
check:99      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         116: ; Function Attrs: noredzone nounwind nonlazybind 
check:99      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         117: define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4bar317hd298b3b9b9d428c0E() unnamed_addr #1 !<unknown kind #36> !4 { 
check:99      ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:108                                                                                                                                        X~~ error: no match found
check:108     ~~~~~~~
check:108     ~~~~~~~
         119:  %_1 = alloca %Type3, align 1 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         120:  %_3.0 = bitcast %Type3* %_1 to {}* 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         121: ; call sanitizer_kcfi_emit_type_metadata_trait_objects::foo3 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         122:  call void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo317hdb324cf16b626e72E({}* align 1 %_3.0, [3 x i64]* align 8 bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.2 to [3 x i64]*)) #2 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         123:  %b.0 = bitcast %Type3* %_1 to {}* 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         124:  %0 = load void ({}*, %Type3*)*, void ({}*, %Type3*)** getelementptr inbounds (void ({}*, %Type3*)*, void ({}*, %Type3*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.2 to void ({}*, %Type3*)**), i64 3), align 8, !invariant.load !8, !nonnull !8 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         125:  call void %0({}* align 1 %b.0, %Type3* align 1 %_1) #2 [ "kcfi"(i32 -1903783964) ] 
         126:  ret void 
check:108     ~~~~~~~~~~
         127: } 
check:108     ~~
check:108     ~~
         128:  
check:108     ~
         129: attributes #0 = { inlinehint noredzone nounwind nonlazybind "probe-stack"="__rust_probestack" "target-cpu"="x86-64" "target-features"="-mmx,-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-3dnow,-3dnowa,-avx,-avx2,+soft-float" } 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         130: attributes #1 = { noredzone nounwind nonlazybind "probe-stack"="__rust_probestack" "target-cpu"="x86-64" "target-features"="-mmx,-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-3dnow,-3dnowa,-avx,-avx2,+soft-float" } 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         131: attributes #2 = { nounwind } 
         132:  
check:108     ~
check:108     ~
         133: !llvm.module.flags = !{!0, !1, !2, !3} 
         134:  
check:108     ~
check:108     ~
         135: !0 = !{i32 7, !"PIC Level", i32 2} 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         136: !1 = !{i32 1, !"Code Model", i32 2} 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         137: !2 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         138: !3 = !{i32 4, !"kcfi", i32 1} 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         139: !4 = !{i32 -1522505972} 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~
         140: !5 = !{i32 -56908610} 
check:108     ~~~~~~~~~~~~~~~~~~~~~~
         141: !6 = !{i32 102973893} 
check:108     ~~~~~~~~~~~~~~~~~~~~~~
         142: !7 = !{i32 -1903783964} 
check:108     ~~~~~~~~~~~~~~~~~~~~~~~~
         143: !8 = !{} 
check:108     ~~~~~~~~~
         144: !9 = !{i32 -1134101214} 
>>>>>>
------------------------------------------


