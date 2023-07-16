plain
failures:

---- [codegen] src/test/codegen/generator-debug.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/generator-debug/generator-debug.ll" "/checkout/src/test/codegen/generator-debug.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/generator-debug.rs:36:16: error: CHECK-SAME: expected string not found in input
// CHECK-SAME: file: [[FILE]], line: 14,
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/generator-debug/generator-debug.ll:365:66: note: scanning from here
!225 = !DIDerivedType(tag: DW_TAG_member, name: "1", scope: !221, file: !213, line: 18, baseType: !226, size: 256, align: 64, extraData: i64 1)
                                                                 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/generator-debug/generator-debug.ll:365:66: note: with "FILE" equal to "!213"
!225 = !DIDerivedType(tag: DW_TAG_member, name: "1", scope: !221, file: !213, line: 18, baseType: !226, size: 256, align: 64, extraData: i64 1)
                                                                 ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/generator-debug/generator-debug.ll:375:82: note: possible intended match here
!235 = !DICompositeType(tag: DW_TAG_structure_type, name: "String", scope: !236, file: !2, size: 192, align: 64, elements: !238, templateParams: !23, identifier: "5d0d8aa51f155d5a7d05ab909ae1d01")

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/generator-debug/generator-debug.ll
Check file: /checkout/src/test/codegen/generator-debug.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
           .
           .
           .
           .
         360: !220 = !{!221} 
         361: !221 = !DICompositeType(tag: DW_TAG_variant_part, scope: !218, file: !2, size: 256, align: 64, elements: !222, templateParams: !23, identifier: "6a7c67ee16a446579284c9476449b8fc", discriminator: !268) 
         362: !222 = !{!223, !225, !227, !229, !231} 
         363: !223 = !DIDerivedType(tag: DW_TAG_member, name: "0", scope: !221, file: !213, line: 14, baseType: !224, size: 256, align: 64, extraData: i64 0) 
         364: !224 = !DICompositeType(tag: DW_TAG_structure_type, name: "Unresumed", scope: !218, file: !2, size: 256, align: 64, elements: !23, identifier: "42576641f430e6e3d72e91358715a6da") 
         365: !225 = !DIDerivedType(tag: DW_TAG_member, name: "1", scope: !221, file: !213, line: 18, baseType: !226, size: 256, align: 64, extraData: i64 1) 
same:36'0                                                                      X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
same:36'1                                                                                                                                                      with "FILE" equal to "!213"
         366: !226 = !DICompositeType(tag: DW_TAG_structure_type, name: "Returned", scope: !218, file: !2, size: 256, align: 64, elements: !23, identifier: "8e7eba083922961692a0584d468f037c") 
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         367: !227 = !DIDerivedType(tag: DW_TAG_member, name: "2", scope: !221, file: !213, line: 18, baseType: !228, size: 256, align: 64, extraData: i64 2) 
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         368: !228 = !DICompositeType(tag: DW_TAG_structure_type, name: "Panicked", scope: !218, file: !2, size: 256, align: 64, elements: !23, identifier: "5f3d4d38166df564364453a4b14eb0aa") 
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         369: !229 = !DIDerivedType(tag: DW_TAG_member, name: "3", scope: !221, file: !213, line: 15, baseType: !230, size: 256, align: 64, extraData: i64 3) 
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         370: !230 = !DICompositeType(tag: DW_TAG_structure_type, name: "Suspend0", scope: !218, file: !2, size: 256, align: 64, elements: !23, identifier: "acebdcbffd4f3a2b9b0d97a70ec0985") 
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         371: !231 = !DIDerivedType(tag: DW_TAG_member, name: "4", scope: !221, file: !213, line: 17, baseType: !232, size: 256, align: 64, extraData: i64 4) 
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         372: !232 = !DICompositeType(tag: DW_TAG_structure_type, name: "Suspend1", scope: !218, file: !2, size: 256, align: 64, elements: !233, templateParams: !23, identifier: "a68c0640139013574d32a470bd55055") 
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         373: !233 = !{!234} 
same:36'0     ~~~~~~~~~~~~~~~
         374: !234 = !DIDerivedType(tag: DW_TAG_member, name: "s", scope: !232, file: !2, baseType: !235, size: 192, align: 64) 
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         375: !235 = !DICompositeType(tag: DW_TAG_structure_type, name: "String", scope: !236, file: !2, size: 192, align: 64, elements: !238, templateParams: !23, identifier: "5d0d8aa51f155d5a7d05ab909ae1d01") 
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
same:36'2                                                                                      ?                                                                                                                    possible intended match
         376: !236 = !DINamespace(name: "string", scope: !237) 
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         377: !237 = !DINamespace(name: "alloc", scope: null) 
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         378: !238 = !{!239} 
same:36'0     ~~~~~~~~~~~~~~~
         379: !239 = !DIDerivedType(tag: DW_TAG_member, name: "vec", scope: !235, file: !2, baseType: !240, size: 192, align: 64) 
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         380: !240 = !DICompositeType(tag: DW_TAG_structure_type, name: "Vec<u8, alloc::alloc::Global>", scope: !241, file: !2, size: 192, align: 64, elements: !242, templateParams: !265, identifier: "26fcd102c4701b75219e0574d1e66246") 
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           .
           .
>>>>>>
------------------------------------------
