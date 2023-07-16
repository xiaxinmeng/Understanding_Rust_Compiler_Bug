plain
failures:

---- [codegen] codegen/debuginfo-generic-closure-env-names.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debuginfo-generic-closure-env-names/debuginfo-generic-closure-env-names.ll" "/checkout/src/test/codegen/debuginfo-generic-closure-env-names.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/debuginfo-generic-closure-env-names.rs:34:13: error: NONMSVC: expected string not found in input
// NONMSVC: !DICompositeType(tag: DW_TAG_structure_type, name: "{async_fn_env#0}<debuginfo_generic_closure_env_names::Foo>", scope: ![[generic_async_function_NAMESPACE]]
            ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debuginfo-generic-closure-env-names/debuginfo-generic-closure-env-names.ll:1991:93: note: scanning from here
!709 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure_env#0}<u32>", scope: !21, file: !2, size: 32, align: 32, elements: !710, templateParams: !18, identifier: "f9a07aaa4dc01fdc4282b2fb451df555")
                                                                                            ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debuginfo-generic-closure-env-names/debuginfo-generic-closure-env-names.ll:1991:93: note: with "generic_async_function_NAMESPACE" equal to "27"
!709 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure_env#0}<u32>", scope: !21, file: !2, size: 32, align: 32, elements: !710, templateParams: !18, identifier: "f9a07aaa4dc01fdc4282b2fb451df555")

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debuginfo-generic-closure-env-names/debuginfo-generic-closure-env-names.ll
Check file: /checkout/src/test/codegen/debuginfo-generic-closure-env-names.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
         1986: !704 = !DILocation(line: 65, column: 12, scope: !696)
         1987: !705 = !DILocation(line: 66, column: 2, scope: !696)
         1988: !706 = distinct !DISubprogram(name: "function_containing_closure<u32>", linkageName: "_RINvCs3mvmSMtNbaT_35debuginfo_generic_closure_env_names27function_containing_closuremEB2_", scope: !14, file: !22, line: 59, type: !707, scopeLine: 59, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !34, templateParams: !714, retainedNodes: !712)
         1989: !707 = !DISubroutineType(types: !708)
         1990: !708 = !{!709, !187}
         1991: !709 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure_env#0}<u32>", scope: !21, file: !2, size: 32, align: 32, elements: !710, templateParams: !18, identifier: "f9a07aaa4dc01fdc4282b2fb451df555")
check:34'0                                                                                                 X~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ error: no match found
check:34'1                                                                                                                                                                                                                       with "generic_async_function_NAMESPACE" equal to "27"
         1992: !710 = !{!711}
check:34'0     ~~~~~~~~~~~~~~
         1993: !711 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !709, file: !2, baseType: !187, size: 32, align: 32)
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         1994: !712 = !{!713}
check:34'0     ~~~~~~~~~~~~~~
         1995: !713 = !DILocalVariable(name: "x", arg: 1, scope: !706, file: !22, line: 59, type: !187)
check:34'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         1996: !714 = !{!715}
check:34'0     ~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
