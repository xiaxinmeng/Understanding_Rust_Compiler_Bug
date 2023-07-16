plain
failures:

---- [codegen] src/test/codegen/debuginfo-generic-closure-env-names.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-13/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debuginfo-generic-closure-env-names/debuginfo-generic-closure-env-names.ll" "/checkout/src/test/codegen/debuginfo-generic-closure-env-names.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/debuginfo-generic-closure-env-names.rs:26:11: error: CHECK: expected string not found in input
// CHECK: ![[generic_async_function_NAMESPACE:[0-9]+]] = !DINamespace(name: "generic_async_function"
          ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debuginfo-generic-closure-env-names/debuginfo-generic-closure-env-names.ll:2170:57: note: scanning from here
!1000 = !DINamespace(name: "function_containing_closure", scope: !14)
                                                        ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debuginfo-generic-closure-env-names/debuginfo-generic-closure-env-names.ll:2180:5: note: possible intended match here
!1010 = distinct !DISubprogram(name: "generic_async_function<debuginfo_generic_closure_env_names::Foo>", linkageName: "_RINvCs3mvmSMtNbaT_35debuginfo_generic_closure_env_names22generic_async_functionNtB2_3FooEB2_", scope: !14, file: !118, line: 68, type: !1011, scopeLine: 68, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !23, templateParams: !991, retainedNodes: !1013)

Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/debuginfo-generic-closure-env-names/debuginfo-generic-closure-env-names.ll
Check file: /checkout/src/test/codegen/debuginfo-generic-closure-env-names.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
         2165: !995 = !DILocation(line: 66, column: 2, scope: !986) 
         2166: !996 = distinct !DISubprogram(name: "function_containing_closure<u32>", linkageName: "_RINvCs3mvmSMtNbaT_35debuginfo_generic_closure_env_names27function_containing_closuremEB2_", scope: !14, file: !118, line: 59, type: !997, scopeLine: 59, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !23, templateParams: !1005, retainedNodes: !1003) 
         2167: !997 = !DISubroutineType(types: !998) 
         2168: !998 = !{!999, !179} 
         2169: !999 = !DICompositeType(tag: DW_TAG_structure_type, name: "{closure_env#0}<u32>", scope: !1000, file: !2, size: 32, align: 32, elements: !1001, templateParams: !18, identifier: "4abc8377f7bf03d89300ff28b5728df4") 
         2170: !1000 = !DINamespace(name: "function_containing_closure", scope: !14) 
check:26'0                                                             X~~~~~~~~~~~~~ error: no match found
         2171: !1001 = !{!1002} 
check:26'0     ~~~~~~~~~~~~~~~~~
         2172: !1002 = !DIDerivedType(tag: DW_TAG_member, name: "x", scope: !999, file: !2, baseType: !179, size: 32, align: 32) 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         2173: !1003 = !{!1004} 
check:26'0     ~~~~~~~~~~~~~~~~~
         2174: !1004 = !DILocalVariable(name: "x", arg: 1, scope: !996, file: !118, line: 59, type: !179) 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         2175: !1005 = !{!1006} 
check:26'0     ~~~~~~~~~~~~~~~~~
         2176: !1006 = !DITemplateTypeParameter(name: "T", type: !179) 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         2177: !1007 = !DILocation(line: 59, column: 44, scope: !996) 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         2178: !1008 = !DILocation(line: 65, column: 12, scope: !996) 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         2179: !1009 = !DILocation(line: 66, column: 2, scope: !996) 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         2180: !1010 = distinct !DISubprogram(name: "generic_async_function<debuginfo_generic_closure_env_names::Foo>", linkageName: "_RINvCs3mvmSMtNbaT_35debuginfo_generic_closure_env_names22generic_async_functionNtB2_3FooEB2_", scope: !14, file: !118, line: 68, type: !1011, scopeLine: 68, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !23, templateParams: !991, retainedNodes: !1013) 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:26'1         ?                                                                                                                                                                                                                                                                                                                                                                                                                    possible intended match
         2181: !1011 = !DISubroutineType(types: !1012) 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         2182: !1012 = !{!143, !17} 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~
         2183: !1013 = !{!1014} 
check:26'0     ~~~~~~~~~~~~~~~~~
         2184: !1014 = !DILocalVariable(name: "x", arg: 1, scope: !1010, file: !118, line: 68, type: !17) 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
         2185: !1015 = !DILocation(line: 68, column: 45, scope: !1010) 
check:26'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            .
            .
>>>>>>
------------------------------------------
