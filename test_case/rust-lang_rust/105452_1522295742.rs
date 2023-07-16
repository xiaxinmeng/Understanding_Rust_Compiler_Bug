plain
test [ui] tests/ui/sanitize/badfree.rs ... ok
test [ui] tests/ui/sanitize/address.rs ... ok
test [ui] tests/ui/sanitize/leak.rs ... ok
test [ui] tests/ui/sanitize/memory-eager.rs#unoptimized ... ok
test [ui] tests/ui/sanitize/sanitizer-cfi-generalize-pointers-require-cfi.rs ... ok
test [ui] tests/ui/sanitize/sanitizer-cfi-canonical-jump-tables-require-cfi.rs ... ok
test [ui] tests/ui/rustc-rust-log.rs ... ok
test [ui] tests/ui/sanitize/sanitizer-cfi-invalid-attr-cfi-encoding.rs ... ok
test [ui] tests/ui/sanitize/sanitizer-cfi-generalize-pointers-attr-cfg.rs ... ok
test [ui] tests/ui/sanitize/memory-passing.rs#optimized ... ok
test [ui] tests/ui/sanitize/sanitizer-cfi-is-incompatible-with-saniziter-kcfi.rs#aarch64 ... ok
test [ui] tests/ui/sanitize/sanitizer-cfi-is-incompatible-with-saniziter-kcfi.rs#x86_64 ... ok
test [ui] tests/ui/sanitize/sanitizer-cfi-requires-lto.rs ... ok
test [ui] tests/ui/sanitize/sanitizer-cfi-normalize-integers-require-cfi.rs ... ok
test [ui] tests/ui/sanitize/sanitizer-cfi-normalize-integers-attr-cfg.rs ... ok
test [ui] tests/ui/sanitize/split-lto-unit-requires-lto.rs ... ok
---
failures:

---- [codegen] tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-attr-no-sanitize.rs#aarch64 stdout ----

error in revision `aarch64`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-attr-no-sanitize.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2" "--cfg" "aarch64" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-attr-no-sanitize.aarch64" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-attr-no-sanitize.aarch64/auxiliary" "--target" "aarch64-unknown-none" "-Cno-prepopulate-passes" "-Zsanitizer=kcfi" "-Copt-level=0"
stdout: none
--- stderr -------------------------------
function must have a single !kcfi_type attachment
ptr @_ZN56sanitizer_kcfi_emit_kcfi_operand_bundle_attr_no_sanitize3foo17haa6457a932183f9cE
!2 = !{i32 -1741689296}
LLVM ERROR: Broken module found, compilation aborted!



---- [codegen] tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-attr-no-sanitize.rs#x86_64 stdout ----

error in revision `x86_64`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-attr-no-sanitize.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2" "--cfg" "x86_64" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-attr-no-sanitize.x86_64" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-attr-no-sanitize.x86_64/auxiliary" "--target" "x86_64-unknown-none" "-Cno-prepopulate-passes" "-Zsanitizer=kcfi" "-Copt-level=0"
stdout: none
--- stderr -------------------------------
function must have a single !kcfi_type attachment
ptr @_ZN56sanitizer_kcfi_emit_kcfi_operand_bundle_attr_no_sanitize3foo17haa6457a932183f9cE
!5 = !{i32 -1741689296}
LLVM ERROR: Broken module found, compilation aborted!



---- [codegen] tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-generalized.rs#aarch64 stdout ----

error in revision `aarch64`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-generalized.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2" "--cfg" "aarch64" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-generalized.aarch64" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-generalized.aarch64/auxiliary" "--target" "aarch64-unknown-none" "-Cno-prepopulate-passes" "-Zsanitizer=kcfi" "-Zsanitizer-cfi-generalize-pointers"
stdout: none
--- stderr -------------------------------
function must have a single !kcfi_type attachment
ptr @_ZN67sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi_generalized3foo17hc95f402efde1cf81E
!2 = !{i32 -1741689296}
function must have a single !kcfi_type attachment
ptr @_ZN67sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi_generalized3bar17hfb161470f1c02c2dE
!6 = !{i32 489439372}
function must have a single !kcfi_type attachment
ptr @_ZN67sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi_generalized3baz17h5d2df055550a06dfE
!10 = !{i32 2026563871}
LLVM ERROR: Broken module found, compilation aborted!



---- [codegen] tests/codegen/sanitizer-cfi-normalize-integers.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-cfi-normalize-integers/sanitizer-cfi-normalize-integers.ll" "/checkout/tests/codegen/sanitizer-cfi-normalize-integers.rs" "--allow-unused-prefixes" "--check-prefixes" "CHECK,NONMSVC" "--dump-input-context" "100"
stdout: none
/checkout/tests/codegen/sanitizer-cfi-normalize-integers.rs:69:11: error: CHECK: expected string not found in input
/checkout/tests/codegen/sanitizer-cfi-normalize-integers.rs:69:11: error: CHECK: expected string not found in input
// CHECK: ![[TYPE9]] = !{i64 0, !"_ZTSFv{{u2i8|u2u8}}E.normalized"}
          ^
/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-cfi-normalize-integers/sanitizer-cfi-normalize-integers.ll:216:47: note: scanning from here
!38 = !{i64 0, !"_ZTSFvu3u64S_S_E.normalized"}
                                              ^
/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-cfi-normalize-integers/sanitizer-cfi-normalize-integers.ll:216:47: note: with "TYPE9" equal to "6"
!38 = !{i64 0, !"_ZTSFvu3u64S_S_E.normalized"}
                                              ^
/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-cfi-normalize-integers/sanitizer-cfi-normalize-integers.ll:222:3: note: possible intended match here
!44 = !{i64 0, !"_ZTSFvu2u8u2i8E.normalized"}

Input file: /checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-cfi-normalize-integers/sanitizer-cfi-normalize-integers.ll
Check file: /checkout/tests/codegen/sanitizer-cfi-normalize-integers.rs


-dump-input=help explains the following input dump.
Input was:
<<<<<<
            .
            .
            .
            .
          116: } 
          117:  
          118: ; sanitizer_cfi_normalize_integers::foo16 
          119: ; Function Attrs: nonlazybind uwtable 
          120: define void @_ZN32sanitizer_cfi_normalize_integers5foo1617h991b78edafc4fc2fE(i16 noundef %_1, i16 noundef %_2) unnamed_addr #0 !type !66 !type !67 !type !68 !type !69 { 
          122:  ret void 
          123: } 
          124:  
          124:  
          125: ; sanitizer_cfi_normalize_integers::foo17 
          126: ; Function Attrs: nonlazybind uwtable 
          127: define void @_ZN32sanitizer_cfi_normalize_integers5foo1717h3ba41407be50496bE(i16 noundef %_1, i16 noundef %_2, i16 noundef %_3) unnamed_addr #0 !type !70 !type !71 !type !72 !type !73 { 
          129:  ret void 
          130: } 
          131:  
          131:  
          132: ; sanitizer_cfi_normalize_integers::foo18 
          133: ; Function Attrs: nonlazybind uwtable 
          134: define void @_ZN32sanitizer_cfi_normalize_integers5foo1817hf6f17972b410c224E(i32 noundef %_1) unnamed_addr #0 !type !74 !type !75 !type !76 !type !77 { 
          136:  ret void 
          137: } 
          138:  
          138:  
          139: ; sanitizer_cfi_normalize_integers::foo19 
          140: ; Function Attrs: nonlazybind uwtable 
          141: define void @_ZN32sanitizer_cfi_normalize_integers5foo1917h18844fcd9e8187d1E(i32 noundef %_1, i32 noundef %_2) unnamed_addr #0 !type !78 !type !79 !type !80 !type !81 { 
          143:  ret void 
          144: } 
          145:  
          145:  
          146: ; sanitizer_cfi_normalize_integers::foo20 
          147: ; Function Attrs: nonlazybind uwtable 
          148: define void @_ZN32sanitizer_cfi_normalize_integers5foo2017h4928e4d0670e950dE(i32 noundef %_1, i32 noundef %_2, i32 noundef %_3) unnamed_addr #0 !type !82 !type !83 !type !84 !type !85 { 
          150:  ret void 
          151: } 
          152:  
          152:  
          153: ; sanitizer_cfi_normalize_integers::foo21 
          154: ; Function Attrs: nonlazybind uwtable 
          155: define void @_ZN32sanitizer_cfi_normalize_integers5foo2117h0b532340c2939738E(i16 noundef %_1) unnamed_addr #0 !type !86 !type !87 !type !88 !type !89 { 
          157:  ret void 
          158: } 
          159:  
          159:  
          160: ; sanitizer_cfi_normalize_integers::foo22 
          161: ; Function Attrs: nonlazybind uwtable 
          162: define void @_ZN32sanitizer_cfi_normalize_integers5foo2217hdaad2963c9711d28E(i16 noundef %_1, i16 noundef %_2) unnamed_addr #0 !type !90 !type !91 !type !92 !type !93 { 
          164:  ret void 
          165: } 
          166:  
          166:  
          167: ; sanitizer_cfi_normalize_integers::foo23 
          168: ; Function Attrs: nonlazybind uwtable 
          169: define void @_ZN32sanitizer_cfi_normalize_integers5foo2317h29f4c2cc593e5f25E(i16 noundef %_1, i16 noundef %_2, i16 noundef %_3) unnamed_addr #0 !type !94 !type !95 !type !96 !type !97 { 
          171:  ret void 
          172: } 
          173:  
          173:  
          174: attributes #0 = { nonlazybind uwtable "target-cpu"="generic" "target-features"="+v8a,+outline-atomics" } 
          175:  
          176: !llvm.module.flags = !{!0, !1, !2, !3} 
          177:  
          178: !0 = !{i32 8, !"PIC Level", i32 2} 
          179: !1 = !{i32 2, !"RtLibUseGOT", i32 1} 
          180: !2 = !{i32 4, !"CFI Canonical Jump Tables", i32 1} 
          181: !3 = !{i32 4, !"EnableSplitLTOUnit", i32 1} 
          182: !4 = !{i64 0, !"_ZTSFvbE"} 
          183: !5 = !{i64 0, !"_ZTSFvbE.generalized"} 
          184: !6 = !{i64 0, !"_ZTSFvu2u8E.normalized"} 
          185: !7 = !{i64 0, !"_ZTSFvu2u8E.normalized.generalized"} 
          186: !8 = !{i64 0, !"_ZTSFvbu2u8E"} 
          187: !9 = !{i64 0, !"_ZTSFvbu2u8E.generalized"} 
          188: !10 = !{i64 0, !"_ZTSFvu2u8S_E.normalized"} 
          189: !11 = !{i64 0, !"_ZTSFvu2u8S_E.normalized.generalized"} 
          190: !12 = !{i64 0, !"_ZTSFvbu2u8S_E"} 
          191: !13 = !{i64 0, !"_ZTSFvbu2u8S_E.generalized"} 
          192: !14 = !{i64 0, !"_ZTSFvu2u8S_S_E.normalized"} 
          193: !15 = !{i64 0, !"_ZTSFvu2u8S_S_E.normalized.generalized"} 
          194: !16 = !{i64 0, !"_ZTSFvu5isizeE"} 
          195: !17 = !{i64 0, !"_ZTSFvu5isizeE.generalized"} 
          196: !18 = !{i64 0, !"_ZTSFvu3i64E.normalized"} 
          197: !19 = !{i64 0, !"_ZTSFvu3i64E.normalized.generalized"} 
          198: !20 = !{i64 0, !"_ZTSFvu5isizeu3i64E"} 
          199: !21 = !{i64 0, !"_ZTSFvu5isizeu3i64E.generalized"} 
          200: !22 = !{i64 0, !"_ZTSFvu3i64S_E.normalized"} 
          201: !23 = !{i64 0, !"_ZTSFvu3i64S_E.normalized.generalized"} 
          202: !24 = !{i64 0, !"_ZTSFvu5isizeu3i64S0_E"} 
          203: !25 = !{i64 0, !"_ZTSFvu5isizeu3i64S0_E.generalized"} 
          204: !26 = !{i64 0, !"_ZTSFvu3i64S_S_E.normalized"} 
          205: !27 = !{i64 0, !"_ZTSFvu3i64S_S_E.normalized.generalized"} 
          206: !28 = !{i64 0, !"_ZTSFvu5usizeE"} 
          207: !29 = !{i64 0, !"_ZTSFvu5usizeE.generalized"} 
          208: !30 = !{i64 0, !"_ZTSFvu3u64E.normalized"} 
          209: !31 = !{i64 0, !"_ZTSFvu3u64E.normalized.generalized"} 
          210: !32 = !{i64 0, !"_ZTSFvu5usizeu3u64E"} 
          211: !33 = !{i64 0, !"_ZTSFvu5usizeu3u64E.generalized"} 
          212: !34 = !{i64 0, !"_ZTSFvu3u64S_E.normalized"} 
          213: !35 = !{i64 0, !"_ZTSFvu3u64S_E.normalized.generalized"} 
          214: !36 = !{i64 0, !"_ZTSFvu5usizeu3u64S0_E"} 
          215: !37 = !{i64 0, !"_ZTSFvu5usizeu3u64S0_E.generalized"} 
          216: !38 = !{i64 0, !"_ZTSFvu3u64S_S_E.normalized"} 
check:69'0                                                   X error: no match found
check:69'1                                                     with "TYPE9" equal to "6"
          217: !39 = !{i64 0, !"_ZTSFvu3u64S_S_E.normalized.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          218: !40 = !{i64 0, !"_ZTSFvu2u8E"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          219: !41 = !{i64 0, !"_ZTSFvu2u8E.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          220: !42 = !{i64 0, !"_ZTSFvu2u8u2i8E"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          221: !43 = !{i64 0, !"_ZTSFvu2u8u2i8E.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          222: !44 = !{i64 0, !"_ZTSFvu2u8u2i8E.normalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
check:69'2       ?                                            possible intended match
          223: !45 = !{i64 0, !"_ZTSFvu2u8u2i8E.normalized.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          224: !46 = !{i64 0, !"_ZTSFvu2u8u2i8S0_E"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          225: !47 = !{i64 0, !"_ZTSFvu2u8u2i8S0_E.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          226: !48 = !{i64 0, !"_ZTSFvu2u8u2i8S0_E.normalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          227: !49 = !{i64 0, !"_ZTSFvu2u8u2i8S0_E.normalized.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          228: !50 = !{i64 0, !"_ZTSFvu3i32E"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          229: !51 = !{i64 0, !"_ZTSFvu3i32E.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          230: !52 = !{i64 0, !"_ZTSFvu3i32E.normalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          231: !53 = !{i64 0, !"_ZTSFvu3i32E.normalized.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          232: !54 = !{i64 0, !"_ZTSFvu3i32S_E"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          233: !55 = !{i64 0, !"_ZTSFvu3i32S_E.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          234: !56 = !{i64 0, !"_ZTSFvu3i32S_E.normalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          235: !57 = !{i64 0, !"_ZTSFvu3i32S_E.normalized.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          236: !58 = !{i64 0, !"_ZTSFvu3i32S_S_E"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          237: !59 = !{i64 0, !"_ZTSFvu3i32S_S_E.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          238: !60 = !{i64 0, !"_ZTSFvu3i32S_S_E.normalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          239: !61 = !{i64 0, !"_ZTSFvu3i32S_S_E.normalized.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          240: !62 = !{i64 0, !"_ZTSFvu3i16E"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          241: !63 = !{i64 0, !"_ZTSFvu3i16E.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          242: !64 = !{i64 0, !"_ZTSFvu3i16E.normalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          243: !65 = !{i64 0, !"_ZTSFvu3i16E.normalized.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          244: !66 = !{i64 0, !"_ZTSFvu3i16S_E"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          245: !67 = !{i64 0, !"_ZTSFvu3i16S_E.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          246: !68 = !{i64 0, !"_ZTSFvu3i16S_E.normalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          247: !69 = !{i64 0, !"_ZTSFvu3i16S_E.normalized.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          248: !70 = !{i64 0, !"_ZTSFvu3i16S_S_E"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          249: !71 = !{i64 0, !"_ZTSFvu3i16S_S_E.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          250: !72 = !{i64 0, !"_ZTSFvu3i16S_S_E.normalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          251: !73 = !{i64 0, !"_ZTSFvu3i16S_S_E.normalized.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          252: !74 = !{i64 0, !"_ZTSFvu3u32E"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          253: !75 = !{i64 0, !"_ZTSFvu3u32E.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          254: !76 = !{i64 0, !"_ZTSFvu3u32E.normalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          255: !77 = !{i64 0, !"_ZTSFvu3u32E.normalized.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          256: !78 = !{i64 0, !"_ZTSFvu3u32S_E"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          257: !79 = !{i64 0, !"_ZTSFvu3u32S_E.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          258: !80 = !{i64 0, !"_ZTSFvu3u32S_E.normalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          259: !81 = !{i64 0, !"_ZTSFvu3u32S_E.normalized.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          260: !82 = !{i64 0, !"_ZTSFvu3u32S_S_E"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          261: !83 = !{i64 0, !"_ZTSFvu3u32S_S_E.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          262: !84 = !{i64 0, !"_ZTSFvu3u32S_S_E.normalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          263: !85 = !{i64 0, !"_ZTSFvu3u32S_S_E.normalized.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          264: !86 = !{i64 0, !"_ZTSFvu3u16E"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          265: !87 = !{i64 0, !"_ZTSFvu3u16E.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          266: !88 = !{i64 0, !"_ZTSFvu3u16E.normalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          267: !89 = !{i64 0, !"_ZTSFvu3u16E.normalized.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          268: !90 = !{i64 0, !"_ZTSFvu3u16S_E"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          269: !91 = !{i64 0, !"_ZTSFvu3u16S_E.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          270: !92 = !{i64 0, !"_ZTSFvu3u16S_E.normalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          271: !93 = !{i64 0, !"_ZTSFvu3u16S_E.normalized.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          272: !94 = !{i64 0, !"_ZTSFvu3u16S_S_E"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          273: !95 = !{i64 0, !"_ZTSFvu3u16S_S_E.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          274: !96 = !{i64 0, !"_ZTSFvu3u16S_S_E.normalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
          275: !97 = !{i64 0, !"_ZTSFvu3u16S_S_E.normalized.generalized"} 
check:69'0     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
------------------------------------------



---- [codegen] tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-generalized.rs#x86_64 stdout ----

error in revision `x86_64`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-generalized.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2" "--cfg" "x86_64" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-generalized.x86_64" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-generalized.x86_64/auxiliary" "--target" "x86_64-unknown-none" "-Cno-prepopulate-passes" "-Zsanitizer=kcfi" "-Zsanitizer-cfi-generalize-pointers"
stdout: none
--- stderr -------------------------------
function must have a single !kcfi_type attachment
ptr @_ZN67sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi_generalized3foo17hc95f402efde1cf81E
!5 = !{i32 -1741689296}
function must have a single !kcfi_type attachment
ptr @_ZN67sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi_generalized3bar17hfb161470f1c02c2dE
!9 = !{i32 489439372}
function must have a single !kcfi_type attachment
ptr @_ZN67sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi_generalized3baz17h5d2df055550a06dfE
!13 = !{i32 2026563871}
LLVM ERROR: Broken module found, compilation aborted!



---- [codegen] tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-normalized-generalized.rs#aarch64 stdout ----

error in revision `aarch64`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-normalized-generalized.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2" "--cfg" "aarch64" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-normalized-generalized.aarch64" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-normalized-generalized.aarch64/auxiliary" "--target" "aarch64-unknown-none" "-Cno-prepopulate-passes" "-Zsanitizer=kcfi" "-Zsanitizer-cfi-normalize-integers" "-Zsanitizer-cfi-generalize-pointers"
stdout: none
--- stderr -------------------------------
function must have a single !kcfi_type attachment
ptr @_ZN78sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi_normalized_generalized3foo17h857d665809dbf959E
!2 = !{i32 -1741689296}
function must have a single !kcfi_type attachment
ptr @_ZN78sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi_normalized_generalized3bar17h494746814e81b482E
!6 = !{i32 489439372}
function must have a single !kcfi_type attachment
ptr @_ZN78sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi_normalized_generalized3baz17h1af87e31dea4c27cE
!10 = !{i32 2026563871}
LLVM ERROR: Broken module found, compilation aborted!



---- [codegen] tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-normalized-generalized.rs#x86_64 stdout ----

error in revision `x86_64`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-normalized-generalized.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2" "--cfg" "x86_64" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-normalized-generalized.x86_64" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-normalized-generalized.x86_64/auxiliary" "--target" "x86_64-unknown-none" "-Cno-prepopulate-passes" "-Zsanitizer=kcfi" "-Zsanitizer-cfi-normalize-integers" "-Zsanitizer-cfi-generalize-pointers"
stdout: none
--- stderr -------------------------------
function must have a single !kcfi_type attachment
ptr @_ZN78sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi_normalized_generalized3foo17h857d665809dbf959E
!5 = !{i32 -1741689296}
function must have a single !kcfi_type attachment
ptr @_ZN78sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi_normalized_generalized3bar17h494746814e81b482E
!9 = !{i32 489439372}
function must have a single !kcfi_type attachment
ptr @_ZN78sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi_normalized_generalized3baz17h1af87e31dea4c27cE
!13 = !{i32 2026563871}
LLVM ERROR: Broken module found, compilation aborted!



---- [codegen] tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi.rs#x86_64 stdout ----

error in revision `x86_64`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2" "--cfg" "x86_64" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi.x86_64" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi.x86_64/auxiliary" "--target" "x86_64-unknown-none" "-Cno-prepopulate-passes" "-Zsanitizer=kcfi" "-Copt-level=0"
stdout: none
--- stderr -------------------------------
function must have a single !kcfi_type attachment
ptr @_ZN55sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi3foo17hb5ee96f2a8adc2ebE
!5 = !{i32 -1741689296}
function must have a single !kcfi_type attachment
ptr @_ZN55sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi3bar17he6ca4d7a74c00a5cE
!9 = !{i32 489439372}
function must have a single !kcfi_type attachment
ptr @_ZN55sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi3baz17h6f37a51fcf9064e7E
!13 = !{i32 2026563871}
LLVM ERROR: Broken module found, compilation aborted!



---- [codegen] tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-normalized.rs#aarch64 stdout ----

error in revision `aarch64`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-normalized.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2" "--cfg" "aarch64" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-normalized.aarch64" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-normalized.aarch64/auxiliary" "--target" "aarch64-unknown-none" "-Cno-prepopulate-passes" "-Zsanitizer=kcfi" "-Zsanitizer-cfi-normalize-integers"
stdout: none
--- stderr -------------------------------
function must have a single !kcfi_type attachment
ptr @_ZN66sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi_normalized3foo17h161614ec2b8deb84E
!2 = !{i32 -1741689296}
function must have a single !kcfi_type attachment
ptr @_ZN66sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi_normalized3bar17hbe64fea0edc5c4a8E
!6 = !{i32 489439372}
function must have a single !kcfi_type attachment
ptr @_ZN66sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi_normalized3baz17h0f3e92d1132ec79fE
!10 = !{i32 2026563871}
LLVM ERROR: Broken module found, compilation aborted!


---- [codegen] tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle.rs#aarch64 stdout ----


error in revision `aarch64`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2" "--cfg" "aarch64" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle.aarch64" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle.aarch64/auxiliary" "--target" "aarch64-unknown-none" "-Cno-prepopulate-passes" "-Zsanitizer=kcfi" "-Copt-level=0"
stdout: none
--- stderr -------------------------------
function must have a single !kcfi_type attachment
ptr @_ZN39sanitizer_kcfi_emit_kcfi_operand_bundle3foo17h24765755a0af4f5cE
!2 = !{i32 -1741689296}
LLVM ERROR: Broken module found, compilation aborted!



---- [codegen] tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle.rs#x86_64 stdout ----

error in revision `x86_64`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2" "--cfg" "x86_64" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle.x86_64" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle.x86_64/auxiliary" "--target" "x86_64-unknown-none" "-Cno-prepopulate-passes" "-Zsanitizer=kcfi" "-Copt-level=0"
stdout: none
--- stderr -------------------------------
function must have a single !kcfi_type attachment
ptr @_ZN39sanitizer_kcfi_emit_kcfi_operand_bundle3foo17h24765755a0af4f5cE
!5 = !{i32 -1741689296}
LLVM ERROR: Broken module found, compilation aborted!



---- [codegen] tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-normalized.rs#x86_64 stdout ----

error in revision `x86_64`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-normalized.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2" "--cfg" "x86_64" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-normalized.x86_64" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi-normalized.x86_64/auxiliary" "--target" "x86_64-unknown-none" "-Cno-prepopulate-passes" "-Zsanitizer=kcfi" "-Zsanitizer-cfi-normalize-integers"
stdout: none
--- stderr -------------------------------
function must have a single !kcfi_type attachment
ptr @_ZN66sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi_normalized3foo17h161614ec2b8deb84E
!5 = !{i32 -1741689296}
function must have a single !kcfi_type attachment
ptr @_ZN66sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi_normalized3bar17hbe64fea0edc5c4a8E
!9 = !{i32 489439372}
function must have a single !kcfi_type attachment
ptr @_ZN66sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi_normalized3baz17h0f3e92d1132ec79fE
!13 = !{i32 2026563871}
LLVM ERROR: Broken module found, compilation aborted!


---- [codegen] tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi.rs#aarch64 stdout ----


error in revision `aarch64`: compilation failed!
status: exit status: 101
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2" "--cfg" "aarch64" "-O" "--emit" "llvm-ir" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi.aarch64" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/codegen/sanitizer-kcfi-emit-kcfi-operand-bundle-itanium-cxx-abi.aarch64/auxiliary" "--target" "aarch64-unknown-none" "-Cno-prepopulate-passes" "-Zsanitizer=kcfi" "-Copt-level=0"
stdout: none
--- stderr -------------------------------
function must have a single !kcfi_type attachment
ptr @_ZN55sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi3foo17hb5ee96f2a8adc2ebE
!2 = !{i32 -1741689296}
function must have a single !kcfi_type attachment
ptr @_ZN55sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi3bar17he6ca4d7a74c00a5cE
!6 = !{i32 489439372}
function must have a single !kcfi_type attachment
ptr @_ZN55sanitizer_kcfi_emit_kcfi_operand_bundle_itanium_cxx_abi3baz17h6f37a51fcf9064e7E
!10 = !{i32 2026563871}
LLVM ERROR: Broken module found, compilation aborted!



failures:
