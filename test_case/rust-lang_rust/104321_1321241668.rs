plain
 finished in 6.881 seconds
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 383 tests
i.....i..............i....i...ii................iii........ii.i........i..F............. 88/383
i.........i.iii.....i..................i...i...i.....ii..i.ii.................ii........ 264/383
................i.i..ii.ii............i..i....i......iii.......i...ii................... 352/383
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
..iiiiiiii.i...................
..iiiiiiii.i...................
failures:

---- [codegen] src/test/codegen/async-fn-debug-awaitee-field.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/async-fn-debug-awaitee-field/async-fn-debug-awaitee-field.ll" "/checkout/src/test/codegen/async-fn-debug-awaitee-field.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/async-fn-debug-awaitee-field.rs:18:13: error: NONMSVC: expected string not found in input
// NONMSVC: [[AWAITEE_TYPE]] = !DICompositeType(tag: DW_TAG_structure_type, name: "async_fn_debug_awaitee_field::foo::{async_fn_env#0}",
            ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/async-fn-debug-awaitee-field/async-fn-debug-awaitee-field.ll:306:100: note: scanning from here
!163 = !DIDerivedType(tag: DW_TAG_member, name: "__awaitee", scope: !161, file: !2, baseType: !164, size: 8, align: 8, offset: 8)
                                                                                                   ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/async-fn-debug-awaitee-field/async-fn-debug-awaitee-field.ll:306:100: note: with "AWAITEE_TYPE" equal to "!164"
!163 = !DIDerivedType(tag: DW_TAG_member, name: "__awaitee", scope: !161, file: !2, baseType: !164, size: 8, align: 8, offset: 8)
                                                                                                   ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/async-fn-debug-awaitee-field/async-fn-debug-awaitee-field.ll:307:3: note: possible intended match here
!164 = !DICompositeType(tag: DW_TAG_structure_type, name: "{async_fn_env#0}", scope: !165, file: !2, size: 8, align: 8, elements: !166, templateParams: !23, identifier: "fc5c7c428ba260b5e550e492531a6410")

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/async-fn-debug-awaitee-field/async-fn-debug-awaitee-field.ll
Check file: /checkout/src/test/codegen/async-fn-debug-awaitee-field.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
          301: !158 = !DIDerivedType(tag: DW_TAG_member, name: "2", scope: !152, file: !144, line: 12, baseType: !159, size: 16, align: 8, extraData: i64 2) 
          302: !159 = !DICompositeType(tag: DW_TAG_structure_type, name: "Panicked", scope: !149, file: !2, size: 16, align: 8, elements: !23, identifier: "60d20a5c259ef18931f3a6d28e02c23d") 
          303: !160 = !DIDerivedType(tag: DW_TAG_member, name: "3", scope: !152, file: !144, line: 11, baseType: !161, size: 16, align: 8, extraData: i64 3) 
          304: !161 = !DICompositeType(tag: DW_TAG_structure_type, name: "Suspend0", scope: !149, file: !2, size: 16, align: 8, elements: !162, templateParams: !23, identifier: "dd26c3c7e95d3b14be06904441662544") 
          305: !162 = !{!163} 
          306: !163 = !DIDerivedType(tag: DW_TAG_member, name: "__awaitee", scope: !161, file: !2, baseType: !164, size: 8, align: 8, offset: 8) 
check:18'0                                                                                                        X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
check:18'1                                                                                                                                        with "AWAITEE_TYPE" equal to "!164"
          307: !164 = !DICompositeType(tag: DW_TAG_structure_type, name: "{async_fn_env#0}", scope: !165, file: !2, size: 8, align: 8, elements: !166, templateParams: !23, identifier: "fc5c7c428ba260b5e550e492531a6410") 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:18'2       ?                                                                                                                                                                                                           possible intended match
          308: !165 = !DINamespace(name: "foo", scope: !145) 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          309: !166 = !{!167} 
check:18'0     ~~~~~~~~~~~~~~~
          310: !167 = !DICompositeType(tag: DW_TAG_variant_part, scope: !164, file: !2, size: 8, align: 8, elements: !168, templateParams: !23, identifier: "6ce0ae470bb762954c3e3e086aaf2e87", discriminator: !175) 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          311: !168 = !{!169, !171, !173} 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~
          312: !169 = !DIDerivedType(tag: DW_TAG_member, name: "0", scope: !167, file: !144, line: 8, baseType: !170, size: 8, align: 8, extraData: i64 0) 
check:18'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
