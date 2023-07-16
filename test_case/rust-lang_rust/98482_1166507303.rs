plain
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 339 tests
ii...i.........i....i..i.................iii........ii.i.......i.................ii..... 88/339
............i..............i................i..Fiii........i..i......i........i.......i. 176/339
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
....i.......i.ii.........i.....................iiiiiii.i...................
failures:


---- [codegen] src/test/codegen/generator-debug.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/generator-debug/generator-debug.ll" "/checkout/src/test/codegen/generator-debug.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/generator-debug.rs:36:16: error: CHECK-SAME: expected string not found in input
// CHECK-SAME: file: [[FILE]], line: 18,
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/generator-debug/generator-debug.ll:364:67: note: scanning from here
!224 = !DIDerivedType(tag: DW_TAG_member, name: "1", scope: !220, file: !212, line: 14, baseType: !225, size: 256, align: 64, extraData: i64 1)
                                                                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/generator-debug/generator-debug.ll:364:67: note: with "FILE" equal to "!212"
!224 = !DIDerivedType(tag: DW_TAG_member, name: "1", scope: !220, file: !212, line: 14, baseType: !225, size: 256, align: 64, extraData: i64 1)
                                                                  ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/generator-debug/generator-debug.ll:374:82: note: possible intended match here
!234 = !DICompositeType(tag: DW_TAG_structure_type, name: "String", scope: !235, file: !2, size: 192, align: 64, elements: !237, templateParams: !23, identifier: "d4d27b65589154eb51bd5c68a98152a4")

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/generator-debug/generator-debug.ll
Check file: /checkout/src/test/codegen/generator-debug.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
           .
           .
           .
           .
         359: !219 = !{!220}
         360: !220 = !DICompositeType(tag: DW_TAG_variant_part, scope: !217, file: !2, size: 256, align: 64, elements: !221, templateParams: !23, identifier: "4b2f8d19d610d925da4e52b92fe592ec", discriminator: !267)
         361: !221 = !{!222, !224, !226, !228, !230}
         362: !222 = !DIDerivedType(tag: DW_TAG_member, name: "0", scope: !220, file: !212, line: 14, baseType: !223, size: 256, align: 64, extraData: i64 0)
         363: !223 = !DICompositeType(tag: DW_TAG_structure_type, name: "Unresumed", scope: !217, file: !2, size: 256, align: 64, elements: !23, identifier: "2e7558f3c8645ae1a629cf077dc25fd4")
         364: !224 = !DIDerivedType(tag: DW_TAG_member, name: "1", scope: !220, file: !212, line: 14, baseType: !225, size: 256, align: 64, extraData: i64 1)
same:36'0                                                                       X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
same:36'1                                                                                                                                                     with "FILE" equal to "!212"
         365: !225 = !DICompositeType(tag: DW_TAG_structure_type, name: "Returned", scope: !217, file: !2, size: 256, align: 64, elements: !23, identifier: "12ed742f7f4961838245404a94fddd37")
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         366: !226 = !DIDerivedType(tag: DW_TAG_member, name: "2", scope: !220, file: !212, line: 14, baseType: !227, size: 256, align: 64, extraData: i64 2)
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         367: !227 = !DICompositeType(tag: DW_TAG_structure_type, name: "Panicked", scope: !217, file: !2, size: 256, align: 64, elements: !23, identifier: "124d7055b0f1345d5dc601b9e05d6131")
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         368: !228 = !DIDerivedType(tag: DW_TAG_member, name: "3", scope: !220, file: !212, line: 15, baseType: !229, size: 256, align: 64, extraData: i64 3)
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         369: !229 = !DICompositeType(tag: DW_TAG_structure_type, name: "Suspend0", scope: !217, file: !2, size: 256, align: 64, elements: !23, identifier: "dd20e0623d8a75a5acc278275d767b76")
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         370: !230 = !DIDerivedType(tag: DW_TAG_member, name: "4", scope: !220, file: !212, line: 17, baseType: !231, size: 256, align: 64, extraData: i64 4)
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         371: !231 = !DICompositeType(tag: DW_TAG_structure_type, name: "Suspend1", scope: !217, file: !2, size: 256, align: 64, elements: !232, templateParams: !23, identifier: "5f31e980054fba10f77f71e7d6525c72")
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         372: !232 = !{!233}
same:36'0     ~~~~~~~~~~~~~~
         373: !233 = !DIDerivedType(tag: DW_TAG_member, name: "s", scope: !231, file: !2, baseType: !234, size: 192, align: 64)
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         374: !234 = !DICompositeType(tag: DW_TAG_structure_type, name: "String", scope: !235, file: !2, size: 192, align: 64, elements: !237, templateParams: !23, identifier: "d4d27b65589154eb51bd5c68a98152a4")
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
same:36'2                                                                                      ?                                                                                                                    possible intended match
         375: !235 = !DINamespace(name: "string", scope: !236)
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         376: !236 = !DINamespace(name: "alloc", scope: null)
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         377: !237 = !{!238}
same:36'0     ~~~~~~~~~~~~~~
         378: !238 = !DIDerivedType(tag: DW_TAG_member, name: "vec", scope: !234, file: !2, baseType: !239, size: 192, align: 64)
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         379: !239 = !DICompositeType(tag: DW_TAG_structure_type, name: "Vec<u8, alloc::alloc::Global>", scope: !240, file: !2, size: 192, align: 64, elements: !241, templateParams: !264, identifier: "e96a36a43dcfb1abea796677c2b7ce45")
same:36'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
           .
           .
>>>>>>
------------------------------------------
