plain
running 466 tests
i.i...i.i..ii............i....i..ii.................iii.........iii.i............i......  88/466
...............ii................i...............i.....i........i..iiii...iiii.iiii..... 176/466
i..iiiii.......i.i..i...i.i.iiii.iiii.....ii....i.ii......i..i.......................i.. 264/466
.ii..i.i.....ii..i.ii..............ii.i...........................ii........F.F......... 352/466
iiiiiii...................

failures:


---- [codegen] tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs#x86_64 stdout ----

error in revision `x86_64`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-14/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.x86_64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll" "/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,x86_64" "--dump-input-context" "100"
stdout: none
--- stderr -------------------------------
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:129:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:129:12: error: CHECK: expected string not found in input
 // CHECK: call align 4 {{ptr|i32*}} %{{[0-9]}}({{\{\}\*|ptr}} align 1 {{%[a-z]\.0|%_[0-9]}}, {{\{\}\*|ptr|%Type4\*}} align 1 {{%[a-z]\.0|%_[0-9]}}){{.*}}[ "kcfi"(i32 [[TYPE4:[[:print:]]+]]) ]
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.x86_64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll:148:174: note: scanning from here
define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo417hb59657770e96a860E({}* align 1 %a.0, [3 x i64]* align 8 %a.1) unnamed_addr #1 !<unknown kind #36> !11 {
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:138:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:138:12: error: CHECK: expected string not found in input
 // CHECK: call align 4 {{ptr|i32*}} %{{[0-9]}}({{\{\}\*|ptr}} align 1 {{%[a-z]\.0|%_[0-9]}}, {{\{\}\*|ptr|%Type4\*}} align 1 {{%[a-z]\.0|%_[0-9]}}){{.*}}[ "kcfi"(i32 [[TYPE4:[[:print:]]+]]) ]
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.x86_64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll:160:132: note: scanning from here
define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4bar417hf2f55f3071851d4eE() unnamed_addr #1 !<unknown kind #36> !4 {


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.x86_64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
           .
           .
           .
           .
          48: start: 
          49:  ret void 
          50: } 
          51:  
          52: ; <sanitizer_kcfi_emit_type_metadata_trait_objects::Type2 as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait2<i32>>::bar 
          53: ; Function Attrs: noredzone nounwind nonlazybind 
          54: define void @"_ZN141_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type2$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait2$LT$i32$GT$$GT$3bar17hc43b8b747eac5e80E"(%Type2* align 1 %self) unnamed_addr #1 !<unknown kind #36> !6 { 
          56:  ret void 
          57: } 
          58:  
          58:  
          59: ; <T as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait3<U>>::baz 
          60: ; Function Attrs: noredzone nounwind nonlazybind 
          61: define void @"_ZN86_$LT$T$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait3$LT$U$GT$$GT$3baz17h3db20bfe4fcf6962E"(%Type3* align 1 %self, %Type3* align 1 %_2) unnamed_addr #1 !<unknown kind #36> !7 { 
          63:  ret void 
          64: } 
          65:  
          65:  
          66: ; <T as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait4<U>>::qux 
          67: ; Function Attrs: noredzone nounwind nonlazybind 
          68: define align 4 i32* @"_ZN86_$LT$T$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait4$LT$U$GT$$GT$3qux17h9ce3df977b9ae6feE"(%Type4* align 1 %self, %Type4* align 1 %_2) unnamed_addr #1 !<unknown kind #36> !8 { 
          69: start: 
          70:  ret i32* bitcast (<{ [4 x i8] }>* @alloc_83ea17bf0c4f4a5a5a13d3ae7955acd0 to i32*) 
          71: } 
          72:  
          73: ; sanitizer_kcfi_emit_type_metadata_trait_objects::foo1 
          74: ; Function Attrs: noredzone nounwind nonlazybind 
          75: define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo117h93502349188b3e24E({}* align 1 %a.0, [3 x i64]* align 8 %a.1) unnamed_addr #1 !<unknown kind #36> !5 { 
          76: start: 
          77:  %0 = bitcast [3 x i64]* %a.1 to void ({}*)** 
          78:  %1 = getelementptr inbounds void ({}*)*, void ({}*)** %0, i64 3 
          79:  %2 = load void ({}*)*, void ({}*)** %1, align 8, !invariant.load !9, !nonnull !9 
          80:  call void %2({}* align 1 %a.0) #2 [ "kcfi"(i32 -56908610) ] 
          81:  ret void 
          82: } 
          83:  
          84: ; sanitizer_kcfi_emit_type_metadata_trait_objects::bar1 
          85: ; Function Attrs: noredzone nounwind nonlazybind 
          86: define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4bar117had3ab10c0b90936dE() unnamed_addr #1 !<unknown kind #36> !4 { 
          87: start: 
          88:  %_1 = alloca %Type1, align 1 
          89:  %b.0 = bitcast %Type1* %_1 to {}* 
          90:  %0 = load void ({}*)*, void ({}*)** getelementptr inbounds (void ({}*)*, void ({}*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.0 to void ({}*)**), i64 3), align 8, !invariant.load !9, !nonnull !9 
          91:  call void %0({}* align 1 %b.0) #2 [ "kcfi"(i32 -56908610) ] 
          92:  ret void 
          93: } 
          94:  
          95: ; sanitizer_kcfi_emit_type_metadata_trait_objects::foo2 
          96: ; Function Attrs: noredzone nounwind nonlazybind 
          97: define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo217h8161e42dd83c0851E({}* align 1 %a.0, [3 x i64]* align 8 %a.1) unnamed_addr #1 !<unknown kind #36> !6 { 
          98: start: 
          99:  %0 = bitcast [3 x i64]* %a.1 to void ({}*)** 
         100:  %1 = getelementptr inbounds void ({}*)*, void ({}*)** %0, i64 3 
         101:  %2 = load void ({}*)*, void ({}*)** %1, align 8, !invariant.load !9, !nonnull !9 
         102:  call void %2({}* align 1 %a.0) #2 [ "kcfi"(i32 102973893) ] 
         103:  ret void 
         104: } 
         105:  
         106: ; sanitizer_kcfi_emit_type_metadata_trait_objects::bar2 
         107: ; Function Attrs: noredzone nounwind nonlazybind 
         108: define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4bar217h18ebf9f7b2e09ed4E() unnamed_addr #1 !<unknown kind #36> !4 { 
         109: start: 
         110:  %_1 = alloca %Type2, align 1 
         111:  %_3.0 = bitcast %Type2* %_1 to {}* 
         112: ; call sanitizer_kcfi_emit_type_metadata_trait_objects::foo2 
         113:  call void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo217h8161e42dd83c0851E({}* align 1 %_3.0, [3 x i64]* align 8 bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.1 to [3 x i64]*)) #2 
         114:  %b.0 = bitcast %Type2* %_1 to {}* 
         115:  %0 = load void ({}*)*, void ({}*)** getelementptr inbounds (void ({}*)*, void ({}*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.1 to void ({}*)**), i64 3), align 8, !invariant.load !9, !nonnull !9 
         116:  call void %0({}* align 1 %b.0) #2 [ "kcfi"(i32 102973893) ] 
         117:  ret void 
         118: } 
         119:  
         120: ; sanitizer_kcfi_emit_type_metadata_trait_objects::foo3 
         121: ; Function Attrs: noredzone nounwind nonlazybind 
         122: define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo317hdb324cf16b626e72E({}* align 1 %a.0, [3 x i64]* align 8 %a.1) unnamed_addr #1 !<unknown kind #36> !10 { 
         123: start: 
         124:  %_2 = alloca %Type3, align 1 
         125:  %0 = bitcast [3 x i64]* %a.1 to void ({}*, %Type3*)** 
         126:  %1 = getelementptr inbounds void ({}*, %Type3*)*, void ({}*, %Type3*)** %0, i64 3 
         127:  %2 = load void ({}*, %Type3*)*, void ({}*, %Type3*)** %1, align 8, !invariant.load !9, !nonnull !9 
         128:  call void %2({}* align 1 %a.0, %Type3* align 1 %_2) #2 [ "kcfi"(i32 -1903783964) ] 
         129:  ret void 
         130: } 
         131:  
         132: ; sanitizer_kcfi_emit_type_metadata_trait_objects::bar3 
         133: ; Function Attrs: noredzone nounwind nonlazybind 
         134: define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4bar317hd298b3b9b9d428c0E() unnamed_addr #1 !<unknown kind #36> !4 { 
         135: start: 
         136:  %_1 = alloca %Type3, align 1 
         137:  %_3.0 = bitcast %Type3* %_1 to {}* 
         138: ; call sanitizer_kcfi_emit_type_metadata_trait_objects::foo3 
         139:  call void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo317hdb324cf16b626e72E({}* align 1 %_3.0, [3 x i64]* align 8 bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.2 to [3 x i64]*)) #2 
         140:  %b.0 = bitcast %Type3* %_1 to {}* 
         141:  %0 = load void ({}*, %Type3*)*, void ({}*, %Type3*)** getelementptr inbounds (void ({}*, %Type3*)*, void ({}*, %Type3*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.2 to void ({}*, %Type3*)**), i64 3), align 8, !invariant.load !9, !nonnull !9 
         142:  call void %0({}* align 1 %b.0, %Type3* align 1 %_1) #2 [ "kcfi"(i32 -1903783964) ] 
         143:  ret void 
         144: } 
         145:  
         146: ; sanitizer_kcfi_emit_type_metadata_trait_objects::foo4 
         147: ; Function Attrs: noredzone nounwind nonlazybind 
         148: define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo417hb59657770e96a860E({}* align 1 %a.0, [3 x i64]* align 8 %a.1) unnamed_addr #1 !<unknown kind #36> !11 { 
check:129                                                                                                                                                                                  X~~ error: no match found
check:129     ~~~~~~~
check:129     ~~~~~~~
         150:  %_2 = alloca %Type4, align 1 
check:129     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         151:  %0 = bitcast [3 x i64]* %a.1 to i32* ({}*, %Type4*)** 
check:129     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         152:  %1 = getelementptr inbounds i32* ({}*, %Type4*)*, i32* ({}*, %Type4*)** %0, i64 3 
check:129     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         153:  %2 = load i32* ({}*, %Type4*)*, i32* ({}*, %Type4*)** %1, align 8, !invariant.load !9, !nonnull !9 
check:129     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         154:  %_3 = call align 4 i32* %2({}* align 1 %a.0, %Type4* align 1 %_2) #2 [ "kcfi"(i32 1825868617) ] 
         155:  ret void 
check:129     ~~~~~~~~~~
         156: } 
check:129     ~~
check:129     ~~
         157:  
check:129     ~
         158: ; sanitizer_kcfi_emit_type_metadata_trait_objects::bar4 
check:129     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         159: ; Function Attrs: noredzone nounwind nonlazybind 
check:129     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         160: define void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4bar417hf2f55f3071851d4eE() unnamed_addr #1 !<unknown kind #36> !4 { 
check:129     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:138                                                                                                                                        X~~ error: no match found
check:138     ~~~~~~~
check:138     ~~~~~~~
         162:  %_1 = alloca %Type4, align 1 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         163:  %_3.0 = bitcast %Type4* %_1 to {}* 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         164: ; call sanitizer_kcfi_emit_type_metadata_trait_objects::foo4 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         165:  call void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo417hb59657770e96a860E({}* align 1 %_3.0, [3 x i64]* align 8 bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.3 to [3 x i64]*)) #2 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         166:  %b.0 = bitcast %Type4* %_1 to {}* 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         167:  %0 = load i32* ({}*, %Type4*)*, i32* ({}*, %Type4*)** getelementptr inbounds (i32* ({}*, %Type4*)*, i32* ({}*, %Type4*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.3 to i32* ({}*, %Type4*)**), i64 3), align 8, !invariant.load !9, !nonnull !9 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         168:  %_7 = call align 4 i32* %0({}* align 1 %b.0, %Type4* align 1 %_1) #2 [ "kcfi"(i32 1825868617) ] 
         169:  ret void 
check:138     ~~~~~~~~~~
         170: } 
check:138     ~~
check:138     ~~
         171:  
check:138     ~
         172: attributes #0 = { inlinehint noredzone nounwind nonlazybind "probe-stack"="__rust_probestack" "target-cpu"="x86-64" "target-features"="-mmx,-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-3dnow,-3dnowa,-avx,-avx2,+soft-float" } 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         173: attributes #1 = { noredzone nounwind nonlazybind "probe-stack"="__rust_probestack" "target-cpu"="x86-64" "target-features"="-mmx,-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-3dnow,-3dnowa,-avx,-avx2,+soft-float" } 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         174: attributes #2 = { nounwind } 
         175:  
check:138     ~
check:138     ~
         176: !llvm.module.flags = !{!0, !1, !2, !3} 
         177:  
check:138     ~
check:138     ~
         178: !0 = !{i32 7, !"PIC Level", i32 2} 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         179: !1 = !{i32 1, !"Code Model", i32 2} 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         180: !2 = !{i32 2, !"RtLibUseGOT", i32 1} 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         181: !3 = !{i32 4, !"kcfi", i32 1} 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         182: !4 = !{i32 -1522505972} 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~
         183: !5 = !{i32 -56908610} 
check:138     ~~~~~~~~~~~~~~~~~~~~~~
         184: !6 = !{i32 102973893} 
check:138     ~~~~~~~~~~~~~~~~~~~~~~
         185: !7 = !{i32 -1903783964} 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~
         186: !8 = !{i32 1825868617} 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~
         187: !9 = !{} 
check:138     ~~~~~~~~~
         188: !10 = !{i32 -1134101214} 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~~
         189: !11 = !{i32 2137817990} 
>>>>>>
------------------------------------------



---- [codegen] tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs#aarch64 stdout ----

error in revision `aarch64`: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-14/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.aarch64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll" "/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC,aarch64" "--dump-input-context" "100"
stdout: none
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:129:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:129:12: error: CHECK: expected string not found in input
 // CHECK: call align 4 {{ptr|i32*}} %{{[0-9]}}({{\{\}\*|ptr}} align 1 {{%[a-z]\.0|%_[0-9]}}, {{\{\}\*|ptr|%Type4\*}} align 1 {{%[a-z]\.0|%_[0-9]}}){{.*}}[ "kcfi"(i32 [[TYPE4:[[:print:]]+]]) ]
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.aarch64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll:148:183: note: scanning from here
define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo417hb59657770e96a860E({}* align 1 %a.0, [3 x i64]* align 8 %a.1) unnamed_addr #1 !<unknown kind #36> !8 {
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:138:12: error: CHECK: expected string not found in input
/checkout/tests/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.rs:138:12: error: CHECK: expected string not found in input
 // CHECK: call align 4 {{ptr|i32*}} %{{[0-9]}}({{\{\}\*|ptr}} align 1 {{%[a-z]\.0|%_[0-9]}}, {{\{\}\*|ptr|%Type4\*}} align 1 {{%[a-z]\.0|%_[0-9]}}){{.*}}[ "kcfi"(i32 [[TYPE4:[[:print:]]+]]) ]
           ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.aarch64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll:160:142: note: scanning from here
define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4bar417hf2f55f3071851d4eE() unnamed_addr #1 !<unknown kind #36> !1 {


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-type-metadata-trait-objects.aarch64/sanitizer-kcfi-emit-type-metadata-trait-objects.ll


-dump-input=help explains the following input dump.
Input was:
<<<<<<
           .
           .
           .
           .
          48: start: 
          49:  ret void 
          50: } 
          51:  
          52: ; <sanitizer_kcfi_emit_type_metadata_trait_objects::Type2 as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait2<i32>>::bar 
          53: ; Function Attrs: noredzone nounwind 
          54: define dso_local void @"_ZN141_$LT$sanitizer_kcfi_emit_type_metadata_trait_objects..Type2$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait2$LT$i32$GT$$GT$3bar17hc43b8b747eac5e80E"(%Type2* align 1 %self) unnamed_addr #1 !<unknown kind #36> !3 { 
          56:  ret void 
          57: } 
          58:  
          58:  
          59: ; <T as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait3<U>>::baz 
          60: ; Function Attrs: noredzone nounwind 
          61: define dso_local void @"_ZN86_$LT$T$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait3$LT$U$GT$$GT$3baz17h3db20bfe4fcf6962E"(%Type3* align 1 %self, %Type3* align 1 %_2) unnamed_addr #1 !<unknown kind #36> !4 { 
          63:  ret void 
          64: } 
          65:  
          65:  
          66: ; <T as sanitizer_kcfi_emit_type_metadata_trait_objects::Trait4<U>>::qux 
          67: ; Function Attrs: noredzone nounwind 
          68: define dso_local align 4 i32* @"_ZN86_$LT$T$u20$as$u20$sanitizer_kcfi_emit_type_metadata_trait_objects..Trait4$LT$U$GT$$GT$3qux17h9ce3df977b9ae6feE"(%Type4* align 1 %self, %Type4* align 1 %_2) unnamed_addr #1 !<unknown kind #36> !5 { 
          69: start: 
          70:  ret i32* bitcast (<{ [4 x i8] }>* @alloc_83ea17bf0c4f4a5a5a13d3ae7955acd0 to i32*) 
          71: } 
          72:  
          73: ; sanitizer_kcfi_emit_type_metadata_trait_objects::foo1 
          74: ; Function Attrs: noredzone nounwind 
          75: define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo117h93502349188b3e24E({}* align 1 %a.0, [3 x i64]* align 8 %a.1) unnamed_addr #1 !<unknown kind #36> !2 { 
          76: start: 
          77:  %0 = bitcast [3 x i64]* %a.1 to void ({}*)** 
          78:  %1 = getelementptr inbounds void ({}*)*, void ({}*)** %0, i64 3 
          79:  %2 = load void ({}*)*, void ({}*)** %1, align 8, !invariant.load !6, !nonnull !6 
          80:  call void %2({}* align 1 %a.0) #2 [ "kcfi"(i32 -56908610) ] 
          81:  ret void 
          82: } 
          83:  
          84: ; sanitizer_kcfi_emit_type_metadata_trait_objects::bar1 
          85: ; Function Attrs: noredzone nounwind 
          86: define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4bar117had3ab10c0b90936dE() unnamed_addr #1 !<unknown kind #36> !1 { 
          87: start: 
          88:  %_1 = alloca %Type1, align 1 
          89:  %b.0 = bitcast %Type1* %_1 to {}* 
          90:  %0 = load void ({}*)*, void ({}*)** getelementptr inbounds (void ({}*)*, void ({}*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.0 to void ({}*)**), i64 3), align 8, !invariant.load !6, !nonnull !6 
          91:  call void %0({}* align 1 %b.0) #2 [ "kcfi"(i32 -56908610) ] 
          92:  ret void 
          93: } 
          94:  
          95: ; sanitizer_kcfi_emit_type_metadata_trait_objects::foo2 
          96: ; Function Attrs: noredzone nounwind 
          97: define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo217h8161e42dd83c0851E({}* align 1 %a.0, [3 x i64]* align 8 %a.1) unnamed_addr #1 !<unknown kind #36> !3 { 
          98: start: 
          99:  %0 = bitcast [3 x i64]* %a.1 to void ({}*)** 
         100:  %1 = getelementptr inbounds void ({}*)*, void ({}*)** %0, i64 3 
         101:  %2 = load void ({}*)*, void ({}*)** %1, align 8, !invariant.load !6, !nonnull !6 
         102:  call void %2({}* align 1 %a.0) #2 [ "kcfi"(i32 102973893) ] 
         103:  ret void 
         104: } 
         105:  
         106: ; sanitizer_kcfi_emit_type_metadata_trait_objects::bar2 
         107: ; Function Attrs: noredzone nounwind 
         108: define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4bar217h18ebf9f7b2e09ed4E() unnamed_addr #1 !<unknown kind #36> !1 { 
         109: start: 
         110:  %_1 = alloca %Type2, align 1 
         111:  %_3.0 = bitcast %Type2* %_1 to {}* 
         112: ; call sanitizer_kcfi_emit_type_metadata_trait_objects::foo2 
         113:  call void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo217h8161e42dd83c0851E({}* align 1 %_3.0, [3 x i64]* align 8 bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.1 to [3 x i64]*)) #2 
         114:  %b.0 = bitcast %Type2* %_1 to {}* 
         115:  %0 = load void ({}*)*, void ({}*)** getelementptr inbounds (void ({}*)*, void ({}*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.1 to void ({}*)**), i64 3), align 8, !invariant.load !6, !nonnull !6 
         116:  call void %0({}* align 1 %b.0) #2 [ "kcfi"(i32 102973893) ] 
         117:  ret void 
         118: } 
         119:  
         120: ; sanitizer_kcfi_emit_type_metadata_trait_objects::foo3 
         121: ; Function Attrs: noredzone nounwind 
         122: define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo317hdb324cf16b626e72E({}* align 1 %a.0, [3 x i64]* align 8 %a.1) unnamed_addr #1 !<unknown kind #36> !7 { 
         123: start: 
         124:  %_2 = alloca %Type3, align 1 
         125:  %0 = bitcast [3 x i64]* %a.1 to void ({}*, %Type3*)** 
         126:  %1 = getelementptr inbounds void ({}*, %Type3*)*, void ({}*, %Type3*)** %0, i64 3 
         127:  %2 = load void ({}*, %Type3*)*, void ({}*, %Type3*)** %1, align 8, !invariant.load !6, !nonnull !6 
         128:  call void %2({}* align 1 %a.0, %Type3* align 1 %_2) #2 [ "kcfi"(i32 -1903783964) ] 
         129:  ret void 
         130: } 
         131:  
         132: ; sanitizer_kcfi_emit_type_metadata_trait_objects::bar3 
         133: ; Function Attrs: noredzone nounwind 
         134: define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4bar317hd298b3b9b9d428c0E() unnamed_addr #1 !<unknown kind #36> !1 { 
         135: start: 
         136:  %_1 = alloca %Type3, align 1 
         137:  %_3.0 = bitcast %Type3* %_1 to {}* 
         138: ; call sanitizer_kcfi_emit_type_metadata_trait_objects::foo3 
         139:  call void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo317hdb324cf16b626e72E({}* align 1 %_3.0, [3 x i64]* align 8 bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.2 to [3 x i64]*)) #2 
         140:  %b.0 = bitcast %Type3* %_1 to {}* 
         141:  %0 = load void ({}*, %Type3*)*, void ({}*, %Type3*)** getelementptr inbounds (void ({}*, %Type3*)*, void ({}*, %Type3*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.2 to void ({}*, %Type3*)**), i64 3), align 8, !invariant.load !6, !nonnull !6 
         142:  call void %0({}* align 1 %b.0, %Type3* align 1 %_1) #2 [ "kcfi"(i32 -1903783964) ] 
         143:  ret void 
         144: } 
         145:  
         146: ; sanitizer_kcfi_emit_type_metadata_trait_objects::foo4 
         147: ; Function Attrs: noredzone nounwind 
         148: define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo417hb59657770e96a860E({}* align 1 %a.0, [3 x i64]* align 8 %a.1) unnamed_addr #1 !<unknown kind #36> !8 { 
check:129                                                                                                                                                                                           X~~ error: no match found
check:129     ~~~~~~~
check:129     ~~~~~~~
         150:  %_2 = alloca %Type4, align 1 
check:129     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         151:  %0 = bitcast [3 x i64]* %a.1 to i32* ({}*, %Type4*)** 
check:129     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         152:  %1 = getelementptr inbounds i32* ({}*, %Type4*)*, i32* ({}*, %Type4*)** %0, i64 3 
check:129     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         153:  %2 = load i32* ({}*, %Type4*)*, i32* ({}*, %Type4*)** %1, align 8, !invariant.load !6, !nonnull !6 
check:129     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         154:  %_3 = call align 4 i32* %2({}* align 1 %a.0, %Type4* align 1 %_2) #2 [ "kcfi"(i32 1825868617) ] 
         155:  ret void 
check:129     ~~~~~~~~~~
         156: } 
check:129     ~~
check:129     ~~
         157:  
check:129     ~
         158: ; sanitizer_kcfi_emit_type_metadata_trait_objects::bar4 
check:129     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         159: ; Function Attrs: noredzone nounwind 
check:129     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         160: define dso_local void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4bar417hf2f55f3071851d4eE() unnamed_addr #1 !<unknown kind #36> !1 { 
check:129     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:138                                                                                                                                                  X~~ error: no match found
check:138     ~~~~~~~
check:138     ~~~~~~~
         162:  %_1 = alloca %Type4, align 1 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         163:  %_3.0 = bitcast %Type4* %_1 to {}* 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         164: ; call sanitizer_kcfi_emit_type_metadata_trait_objects::foo4 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         165:  call void @_ZN47sanitizer_kcfi_emit_type_metadata_trait_objects4foo417hb59657770e96a860E({}* align 1 %_3.0, [3 x i64]* align 8 bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.3 to [3 x i64]*)) #2 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         166:  %b.0 = bitcast %Type4* %_1 to {}* 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         167:  %0 = load i32* ({}*, %Type4*)*, i32* ({}*, %Type4*)** getelementptr inbounds (i32* ({}*, %Type4*)*, i32* ({}*, %Type4*)** bitcast (<{ i8*, [16 x i8], i8* }>* @vtable.3 to i32* ({}*, %Type4*)**), i64 3), align 8, !invariant.load !6, !nonnull !6 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         168:  %_7 = call align 4 i32* %0({}* align 1 %b.0, %Type4* align 1 %_1) #2 [ "kcfi"(i32 1825868617) ] 
         169:  ret void 
check:138     ~~~~~~~~~~
         170: } 
check:138     ~~
check:138     ~~
         171:  
check:138     ~
         172: attributes #0 = { inlinehint noredzone nounwind "target-cpu"="generic" "target-features"="+v8a,+strict-align,+neon,+fp-armv8" } 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         173: attributes #1 = { noredzone nounwind "target-cpu"="generic" "target-features"="+v8a,+strict-align,+neon,+fp-armv8" } 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         174: attributes #2 = { nounwind } 
         175:  
check:138     ~
check:138     ~
         176: !llvm.module.flags = !{!0} 
         177:  
check:138     ~
check:138     ~
         178: !0 = !{i32 4, !"kcfi", i32 1} 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         179: !1 = !{i32 -1522505972} 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~
         180: !2 = !{i32 -56908610} 
check:138     ~~~~~~~~~~~~~~~~~~~~~~
         181: !3 = !{i32 102973893} 
check:138     ~~~~~~~~~~~~~~~~~~~~~~
         182: !4 = !{i32 -1903783964} 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~
         183: !5 = !{i32 1825868617} 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~
         184: !6 = !{} 
check:138     ~~~~~~~~~
         185: !7 = !{i32 -1134101214} 
check:138     ~~~~~~~~~~~~~~~~~~~~~~~~
         186: !8 = !{i32 2137817990} 
>>>>>>
------------------------------------------


