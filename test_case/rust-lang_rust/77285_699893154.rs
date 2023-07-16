plain
Initialized empty Git repository in /home/runner/work/rust/rust/.git/
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/77285/merge:refs/remotes/pull/77285/merge
---
 finished in 4.302
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 222 tests
ii............iii....i..i............i...........i............................i...iii......ii....... 100/222
.......i.............i...i....ii...iiii......F................................i.....i............iii 200/222
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:354:22
.ii...................
failures:


---- [codegen] codegen/issue-75742-format_without_fmt_args.rs stdout ----

error: verification with 'FileCheck' failed
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75742-format_without_fmt_args/issue-75742-format_without_fmt_args.ll" "/checkout/src/test/codegen/issue-75742-format_without_fmt_args.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
/checkout/src/test/codegen/issue-75742-format_without_fmt_args.rs:10:16: error: CHECK-NEXT: is not on the line after the previous match
// CHECK-NEXT: {{"_ZN[^:]+"}}:
               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75742-format_without_fmt_args/issue-75742-format_without_fmt_args.ll:195:1: note: 'next' match was here
"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17hcbba748b2e896a43E.exit.i.i.i.i25": ; preds = %bb8
^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75742-format_without_fmt_args/issue-75742-format_without_fmt_args.ll:176:32: note: previous match ended here
define void @format_wo_fmt_args() unnamed_addr #2 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
                               ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75742-format_without_fmt_args/issue-75742-format_without_fmt_args.ll:177:1: note: non-matching line after previous match is here
^


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/issue-75742-format_without_fmt_args/issue-75742-format_without_fmt_args.ll
Check file: /checkout/src/test/codegen/issue-75742-format_without_fmt_args.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
         .
         .
         .
         .
       190:  %_4.i.i.i.i.i22 = icmp eq i64 %_1.sroa.4.0.copyload, 0
       191:  %.not.i.i.i.i23 = icmp eq i8* %_1.sroa.0.0.copyload, null
       192:  %or.cond.i.i.i.i24 = or i1 %.not.i.i.i.i23, %_4.i.i.i.i.i22
       193:  br i1 %or.cond.i.i.i.i24, label %bb17, label %"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17hcbba748b2e896a43E.exit.i.i.i.i25"
       194: 
       195: "_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17hcbba748b2e896a43E.exit.i.i.i.i25": ; preds = %bb8
next:10     !~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~                error: match on wrong line
       196:  tail call void @__rust_dealloc(i8* nonnull %_1.sroa.0.0.copyload, i64 %_1.sroa.4.0.copyload, i64 1) #7
       197:  br label %bb17
       198: 
       199: bb17: ; preds = %"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17hcbba748b2e896a43E.exit.i.i.i.i25", %bb8
       200:  %1 = bitcast %"std::string::String"* %r1 to i8*
         .
         .
>>>>>>

---
test result: FAILED. 190 passed; 1 failed; 31 ignored; 0 measured; 0 filtered out



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "codegen" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "11.0.0-rust-1.48.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:22:12
Build completed unsuccessfully in 0:22:12
== printing ll file ==
; ModuleID = 'issue_75742_format_without_fmt_args.3a1fbbbh-cgu.0'
source_filename = "issue_75742_format_without_fmt_args.3a1fbbbh-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"std::vec::Vec<u8>" = type { [0 x i64], { i8*, i64 }, [0 x i64], i64, [0 x i64] }
%"std::string::String" = type { [0 x i64], %"std::vec::Vec<u8>", [0 x i64] }
%"std::panic::Location" = type { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }
%"std::fmt::Arguments" = type { [0 x i64], { [0 x { [0 x i8]*, i64 }]*, i64 }, [0 x i64], { i64*, i64 }, [0 x i64], { [0 x { i8*, i64* }]*, i64 }, [0 x i64] }
%"unwind::libunwind::_Unwind_Exception" = type { [0 x i64], i64, [0 x i64], void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [0 x i64], [6 x i64], [0 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }
%"std::fmt::Formatter" = type { [0 x i64], { i64, i64 }, [0 x i64], { i64, i64 }, [0 x i64], { {}*, [3 x i64]* }, [0 x i32], i32, [0 x i32], i32, [0 x i8], i8, [7 x i8] }

@alloc45 = private unnamed_addr constant <{ [0 x i8] }> zeroinitializer, align 8
@alloc101 = private unnamed_addr constant <{ [39 x i8] }> <{ [39 x i8] c"/checkout/library/core/src/slice/raw.rs" }>, align 1
@alloc100 = private unnamed_addr constant <{ [64 x i8] }> <{ [64 x i8] c"attempt to create slice covering at least half the address space" }>, align 1
@alloc102 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [39 x i8] }>, <{ [39 x i8] }>* @alloc101, i32 0, i32 0, i32 0), [16 x i8] c"'\00\00\00\00\00\00\00[\00\00\00\05\00\00\00" }>, align 8
@alloc105 = private unnamed_addr constant <{ [72 x i8] }> <{ [72 x i8] c"`new_layout.size()` must be greater than or equal to `old_layout.size()`" }>, align 1
@alloc106 = private unnamed_addr constant <{ [36 x i8] }> <{ [36 x i8] c"/checkout/library/alloc/src/alloc.rs" }>, align 1
@alloc107 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [36 x i8] }>, <{ [36 x i8] }>* @alloc106, i32 0, i32 0, i32 0), [16 x i8] c"$\00\00\00\00\00\00\00\A9\00\00\00\09\00\00\00" }>, align 8
@alloc42 = private unnamed_addr constant <{ [12 x i8] }> <{ [12 x i8] c"a long story" }>, align 1
@alloc10 = private unnamed_addr constant <{ [14 x i8] }> <{ [14 x i8] c"a long story {" }>, align 1
@alloc48 = private unnamed_addr constant <{ [14 x i8] }> <{ [14 x i8] c"a long story: " }>, align 1
@alloc49 = private unnamed_addr constant <{ i8*, [8 x i8] }> <{ i8* getelementptr inbounds (<{ [14 x i8] }>, <{ [14 x i8] }>* @alloc48, i32 0, i32 0, i32 0), [8 x i8] c"\0E\00\00\00\00\00\00\00" }>, align 8
@alloc54 = private unnamed_addr constant <{ [13 x i8] }> <{ [13 x i8] c"a long story " }>, align 1
@alloc55 = private unnamed_addr constant <{ i8*, [8 x i8] }> <{ i8* getelementptr inbounds (<{ [13 x i8] }>, <{ [13 x i8] }>* @alloc54, i32 0, i32 0, i32 0), [8 x i8] c"\0D\00\00\00\00\00\00\00" }>, align 8
@alloc64 = private unnamed_addr constant <{ [56 x i8] }> <{ [56 x i8] c"\00\00\00\00\00\00\00\00\02\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00 \00\00\00\00\00\00\00\03\00\00\00\00\00\00\00" }>, align 8
; core::ptr::drop_in_place
; core::ptr::drop_in_place
; Function Attrs: nounwind nonlazybind uwtable
define internal fastcc void @_ZN4core3ptr13drop_in_place17h79b2a16b1371a9d9E(%"std::vec::Vec<u8>"* nocapture readonly %_1) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
bb4:
  %.idx = bitcast %"std::vec::Vec<u8>"* %_1 to i8**
  %.idx.val = load i8*, i8** %.idx, align 8
  %0 = getelementptr %"std::vec::Vec<u8>", %"std::vec::Vec<u8>"* %_1, i64 0, i32 1, i32 1
  %.idx4.val = load i64, i64* %0, align 8
  %_4.i.i.i = icmp eq i64 %.idx4.val, 0
  %.not.i.i = icmp eq i8* %.idx.val, null
  %or.cond.i.i = or i1 %.not.i.i, %_4.i.i.i
  br i1 %or.cond.i.i, label %_ZN4core3ptr13drop_in_place17h977a96c92db9b078E.exit, label %"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17hcbba748b2e896a43E.exit.i.i"

"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17hcbba748b2e896a43E.exit.i.i": ; preds = %bb4
  tail call void @__rust_dealloc(i8* nonnull %.idx.val, i64 %.idx4.val, i64 1) #7
  br label %_ZN4core3ptr13drop_in_place17h977a96c92db9b078E.exit

_ZN4core3ptr13drop_in_place17h977a96c92db9b078E.exit: ; preds = %bb4, %"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17hcbba748b2e896a43E.exit.i.i"
  ret void

; alloc::str::<impl alloc::borrow::ToOwned for str>::to_owned
; alloc::str::<impl alloc::borrow::ToOwned for str>::to_owned
; Function Attrs: inlinehint nonlazybind uwtable
define internal fastcc void @"_ZN5alloc3str56_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$str$GT$8to_owned17hbc7ea6a9d21ebdedE"(%"std::string::String"* noalias nocapture dereferenceable(24) %0, [0 x i8]* noalias nonnull readonly align 1 %self.0, i64 %self.1) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %_2 = alloca %"std::vec::Vec<u8>", align 8
  %1 = bitcast %"std::vec::Vec<u8>"* %_2 to i8*
  call void @llvm.lifetime.start.p0i8(i64 24, i8* nonnull %1)
  %2 = icmp eq i64 %self.1, 0
  br i1 %2, label %"_ZN5alloc3vec12Vec$LT$T$GT$13with_capacity17he575f9c5052d2356E.exit.i.i.i", label %"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$5alloc17hfc96c285a7cc4383E.exit.i.i.i.i.i.i.i"

"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$5alloc17hfc96c285a7cc4383E.exit.i.i.i.i.i.i.i": ; preds = %start
  %3 = tail call i8* @__rust_alloc(i64 %self.1, i64 1) #7, !noalias !2
  %4 = insertvalue { i8*, i64 } undef, i8* %3, 0
  %5 = insertvalue { i8*, i64 } %4, i64 %self.1, 1
  %6 = icmp eq i8* %3, null
  br i1 %6, label %bb21.i.i.i.i.i.i.i, label %"_ZN5alloc3vec12Vec$LT$T$GT$13with_capacity17he575f9c5052d2356E.exit.i.i.i"

bb21.i.i.i.i.i.i.i:                               ; preds = %"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$5alloc17hfc96c285a7cc4383E.exit.i.i.i.i.i.i.i"
; call alloc::alloc::handle_alloc_error
  tail call void @_ZN5alloc5alloc18handle_alloc_error17h0bb0cccd73d589d8E(i64 %self.1, i64 1) #7, !noalias !2
  unreachable

"_ZN5alloc3vec12Vec$LT$T$GT$13with_capacity17he575f9c5052d2356E.exit.i.i.i": ; preds = %"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$5alloc17hfc96c285a7cc4383E.exit.i.i.i.i.i.i.i", %start
  %7 = phi { i8*, i64 } [ %5, %"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$5alloc17hfc96c285a7cc4383E.exit.i.i.i.i.i.i.i" ], [ { i8* inttoptr (i64 1 to i8*), i64 0 }, %start ]
  %_2.0.i.i.i.i = extractvalue { i8*, i64 } %7, 0
  %_2.1.i.i.i.i = extractvalue { i8*, i64 } %7, 1
  %8 = bitcast %"std::vec::Vec<u8>"* %_2 to i8**
  store i8* %_2.0.i.i.i.i, i8** %8, align 8, !alias.scope !2, !noalias !11
  %9 = getelementptr inbounds %"std::vec::Vec<u8>", %"std::vec::Vec<u8>"* %_2, i64 0, i32 1, i32 1
  store i64 %_2.1.i.i.i.i, i64* %9, align 8, !alias.scope !2, !noalias !11
  %10 = getelementptr inbounds %"std::vec::Vec<u8>", %"std::vec::Vec<u8>"* %_2, i64 0, i32 3
  store i64 0, i64* %10, align 8, !alias.scope !2, !noalias !11
  %11 = getelementptr [0 x i8], [0 x i8]* %self.0, i64 0, i64 0
  %_14.i.i.i.i.i.i.i = ptrtoint [0 x i8]* %self.0 to i64
  %_7.i.i.i.i.i.i.i.i = icmp slt i64 %self.1, 0
  br i1 %_7.i.i.i.i.i.i.i.i, label %bb7.i.i.i.i.i.i.i.i, label %"_ZN4core5slice4iter13Iter$LT$T$GT$8as_slice17h5707e9e03c3f627bE.exit.i.i.i.i.i"

bb7.i.i.i.i.i.i.i.i:                              ; preds = %"_ZN5alloc3vec12Vec$LT$T$GT$13with_capacity17he575f9c5052d2356E.exit.i.i.i"
; invoke core::panicking::panic
  invoke void @_ZN4core9panicking5panic17hd33b8bec11e2d3e4E([0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [64 x i8] }>* @alloc100 to [0 x i8]*), i64 64, %"std::panic::Location"* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc102 to %"std::panic::Location"*))
          to label %.noexc.i.i.i unwind label %cleanup.i.i.i, !noalias !15

.noexc.i.i.i:                                     ; preds = %bb7.i.i.i.i.i.i.i.i
  unreachable

"_ZN4core5slice4iter13Iter$LT$T$GT$8as_slice17h5707e9e03c3f627bE.exit.i.i.i.i.i": ; preds = %"_ZN5alloc3vec12Vec$LT$T$GT$13with_capacity17he575f9c5052d2356E.exit.i.i.i"
  %12 = icmp ult i64 %_2.1.i.i.i.i, %self.1
  br i1 %12, label %bb10.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i

bb10.i.i.i.i.i.i.i.i.i.i:                         ; preds = %"_ZN4core5slice4iter13Iter$LT$T$GT$8as_slice17h5707e9e03c3f627bE.exit.i.i.i.i.i"
  %_22.i.i.i.i.i.i.i.i.i.i = shl i64 %_2.1.i.i.i.i, 1
  %13 = icmp ugt i64 %_22.i.i.i.i.i.i.i.i.i.i, %self.1
  %.0.sroa.speculated.i.i.i.i.i.i.i.i.i.i.i.i.i = select i1 %13, i64 %_22.i.i.i.i.i.i.i.i.i.i, i64 %self.1
  %14 = icmp ugt i64 %.0.sroa.speculated.i.i.i.i.i.i.i.i.i.i.i.i.i, 8
  %.0.sroa.speculated.i.i.i15.i.i.i.i.i.i.i.i.i.i = select i1 %14, i64 %.0.sroa.speculated.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 8
  %_4.i.i.i.i.i.i.i.i.i.i.i = icmp eq i64 %_2.1.i.i.i.i, 0
  %.not.i.i.i.i.i.i.i.i.i.i.i = icmp eq i8* %_2.0.i.i.i.i, null
  %or.cond.i.i.i = or i1 %_4.i.i.i.i.i.i.i.i.i.i.i, %.not.i.i.i.i.i.i.i.i.i.i.i
  br i1 %or.cond.i.i.i, label %bb6.i.i7.i.i.i.i.i.i.i.i.i.i.i, label %bb22.i.i.i.i.i.i.i.i.i.i.i

bb6.i.i7.i.i.i.i.i.i.i.i.i.i.i:                   ; preds = %bb10.i.i.i.i.i.i.i.i.i.i
  %15 = tail call i8* @__rust_alloc(i64 %.0.sroa.speculated.i.i.i15.i.i.i.i.i.i.i.i.i.i, i64 1) #7, !noalias !16
  br label %bb32.i.i.i.i.i.i.i.i.i.i.i

bb22.i.i.i.i.i.i.i.i.i.i.i:                       ; preds = %bb10.i.i.i.i.i.i.i.i.i.i
  %_7.not.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp ugt i64 %_2.1.i.i.i.i, %.0.sroa.speculated.i.i.i15.i.i.i.i.i.i.i.i.i.i
  br i1 %_7.not.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb11.i.i.i.i.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i.i.i.i.i:                    ; preds = %bb22.i.i.i.i.i.i.i.i.i.i.i
; invoke core::panicking::panic
  invoke void @_ZN4core9panicking5panic17hd33b8bec11e2d3e4E([0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [72 x i8] }>* @alloc105 to [0 x i8]*), i64 72, %"std::panic::Location"* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc107 to %"std::panic::Location"*))
          to label %.noexc4.i.i.i unwind label %cleanup.i.i.i, !noalias !15

.noexc4.i.i.i:                                    ; preds = %bb4.i.i.i.i.i.i.i.i.i.i.i.i.i
  unreachable

bb11.i.i.i.i.i.i.i.i.i.i.i.i.i:                   ; preds = %bb22.i.i.i.i.i.i.i.i.i.i.i
  %16 = tail call i8* @__rust_realloc(i8* nonnull %_2.0.i.i.i.i, i64 %_2.1.i.i.i.i, i64 1, i64 %.0.sroa.speculated.i.i.i15.i.i.i.i.i.i.i.i.i.i) #7, !noalias !16
  br label %bb32.i.i.i.i.i.i.i.i.i.i.i

bb32.i.i.i.i.i.i.i.i.i.i.i:                       ; preds = %bb11.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb6.i.i7.i.i.i.i.i.i.i.i.i.i.i
  %memory.sroa.0.0.i.i.i.i.i.i.i.i.i.i.i = phi i8* [ %15, %bb6.i.i7.i.i.i.i.i.i.i.i.i.i.i ], [ %16, %bb11.i.i.i.i.i.i.i.i.i.i.i.i.i ]
  %17 = icmp eq i8* %memory.sroa.0.0.i.i.i.i.i.i.i.i.i.i.i, null
  br i1 %17, label %bb5.i.i.i.i.i.i.i.i.i, label %bb29.i.i.i.i.i.i.i.i.i.i

bb29.i.i.i.i.i.i.i.i.i.i:                         ; preds = %bb32.i.i.i.i.i.i.i.i.i.i.i
  store i8* %memory.sroa.0.0.i.i.i.i.i.i.i.i.i.i.i, i8** %8, align 8, !alias.scope !15, !noalias !24
  store i64 %.0.sroa.speculated.i.i.i15.i.i.i.i.i.i.i.i.i.i, i64* %9, align 8, !alias.scope !15, !noalias !24
  br label %bb2.i.i.i.i.i.i.i

bb5.i.i.i.i.i.i.i.i.i:                            ; preds = %bb32.i.i.i.i.i.i.i.i.i.i.i
; call alloc::alloc::handle_alloc_error
  tail call void @_ZN5alloc5alloc18handle_alloc_error17h0bb0cccd73d589d8E(i64 %.0.sroa.speculated.i.i.i15.i.i.i.i.i.i.i.i.i.i, i64 1), !noalias !15
  unreachable

bb2.i.i.i.i.i.i.i:                                ; preds = %bb29.i.i.i.i.i.i.i.i.i.i, %"_ZN4core5slice4iter13Iter$LT$T$GT$8as_slice17h5707e9e03c3f627bE.exit.i.i.i.i.i"
  %_2.idx.val.i.i.i.i.i.i.i = phi i8* [ %memory.sroa.0.0.i.i.i.i.i.i.i.i.i.i.i, %bb29.i.i.i.i.i.i.i.i.i.i ], [ %_2.0.i.i.i.i, %"_ZN4core5slice4iter13Iter$LT$T$GT$8as_slice17h5707e9e03c3f627bE.exit.i.i.i.i.i" ]
  %dst_usize.i.i.i.i.i.i.i.i = ptrtoint i8* %_2.idx.val.i.i.i.i.i.i.i to i64
  %_13.i.i.i.i.i.i.i.i = icmp ult i8* %_2.idx.val.i.i.i.i.i.i.i, %11
  %18 = sub i64 %_14.i.i.i.i.i.i.i, %dst_usize.i.i.i.i.i.i.i.i
  %19 = sub i64 %dst_usize.i.i.i.i.i.i.i.i, %_14.i.i.i.i.i.i.i
  %diff.0.i.i.i.i.i.i.i.i = select i1 %_13.i.i.i.i.i.i.i.i, i64 %18, i64 %19
  %.not.i.i.i.i.i.i.i = icmp ult i64 %diff.0.i.i.i.i.i.i.i.i, %self.1
  br i1 %.not.i.i.i.i.i.i.i, label %bb11.i.i.i.i.i.i.i, label %"_ZN5alloc5slice64_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$$u5b$T$u5d$$GT$8to_owned17h24e8c4b9377b39f9E.exit"

bb11.i.i.i.i.i.i.i:                               ; preds = %bb2.i.i.i.i.i.i.i
  tail call void @llvm.trap() #7, !noalias !15
  unreachable

cleanup.i.i.i:                                    ; preds = %bb4.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb7.i.i.i.i.i.i.i.i
  %20 = landingpad { i8*, i32 }
; call core::ptr::drop_in_place
; call core::ptr::drop_in_place
  call fastcc void @_ZN4core3ptr13drop_in_place17h79b2a16b1371a9d9E(%"std::vec::Vec<u8>"* nonnull %_2) #8
  resume { i8*, i32 } %20

"_ZN5alloc5slice64_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$$u5b$T$u5d$$GT$8to_owned17h24e8c4b9377b39f9E.exit": ; preds = %bb2.i.i.i.i.i.i.i
  tail call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 1 %_2.idx.val.i.i.i.i.i.i.i, i8* nonnull align 1 %11, i64 %self.1, i1 false) #7, !noalias !15
  store i64 %self.1, i64* %10, align 8, !alias.scope !15, !noalias !27
  %21 = bitcast %"std::string::String"* %0 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 8 dereferenceable(24) %21, i8* nonnull align 8 dereferenceable(24) %1, i64 24, i1 false) #7, !alias.scope !28
  call void @llvm.lifetime.end.p0i8(i64 24, i8* nonnull %1)
  ret void


; Function Attrs: nonlazybind uwtable
define void @format_wo_fmt_args() unnamed_addr #2 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
bb8:
  %r3 = alloca %"std::string::String", align 8
  %r1 = alloca %"std::string::String", align 8
  %r = alloca %"std::string::String", align 8
  %0 = bitcast %"std::string::String"* %r to i8*
  call void @llvm.lifetime.start.p0i8(i64 24, i8* nonnull %0)
; call alloc::str::<impl alloc::borrow::ToOwned for str>::to_owned
  call fastcc void @"_ZN5alloc3str56_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$str$GT$8to_owned17hbc7ea6a9d21ebdedE"(%"std::string::String"* noalias nocapture nonnull dereferenceable(24) %r, [0 x i8]* noalias nonnull readonly align 1 getelementptr inbounds (<{ [0 x i8] }>, <{ [0 x i8] }>* @alloc45, i32 0, i32 0), i64 0)
  %_1.sroa.0.0..sroa_cast63 = bitcast %"std::string::String"* %r to i8**
  %_1.sroa.0.0.copyload = load i8*, i8** %_1.sroa.0.0..sroa_cast63, align 8
  %_1.sroa.4.0..sroa_idx65 = getelementptr inbounds %"std::string::String", %"std::string::String"* %r, i64 0, i32 1, i32 1, i32 1
  %_1.sroa.4.0.copyload = load i64, i64* %_1.sroa.4.0..sroa_idx65, align 8
  call void @llvm.lifetime.end.p0i8(i64 24, i8* nonnull %0)
  %_4.i.i.i.i.i22 = icmp eq i64 %_1.sroa.4.0.copyload, 0
  %.not.i.i.i.i23 = icmp eq i8* %_1.sroa.0.0.copyload, null
  %or.cond.i.i.i.i24 = or i1 %.not.i.i.i.i23, %_4.i.i.i.i.i22
  br i1 %or.cond.i.i.i.i24, label %bb17, label %"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17hcbba748b2e896a43E.exit.i.i.i.i25"

"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17hcbba748b2e896a43E.exit.i.i.i.i25": ; preds = %bb8
  tail call void @__rust_dealloc(i8* nonnull %_1.sroa.0.0.copyload, i64 %_1.sroa.4.0.copyload, i64 1) #7
  br label %bb17

bb17:                                             ; preds = %"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17hcbba748b2e896a43E.exit.i.i.i.i25", %bb8
  %1 = bitcast %"std::string::String"* %r1 to i8*
  call void @llvm.lifetime.start.p0i8(i64 24, i8* nonnull %1)
; call alloc::str::<impl alloc::borrow::ToOwned for str>::to_owned
  call fastcc void @"_ZN5alloc3str56_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$str$GT$8to_owned17hbc7ea6a9d21ebdedE"(%"std::string::String"* noalias nocapture nonnull dereferenceable(24) %r1, [0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [12 x i8] }>* @alloc42 to [0 x i8]*), i64 12)
  %_17.sroa.0.0..sroa_cast94 = bitcast %"std::string::String"* %r1 to i8**
  %_17.sroa.0.0.copyload = load i8*, i8** %_17.sroa.0.0..sroa_cast94, align 8
  %_17.sroa.4.0..sroa_idx96 = getelementptr inbounds %"std::string::String", %"std::string::String"* %r1, i64 0, i32 1, i32 1, i32 1
  %_17.sroa.4.0.copyload = load i64, i64* %_17.sroa.4.0..sroa_idx96, align 8
  call void @llvm.lifetime.end.p0i8(i64 24, i8* nonnull %1)
  %_4.i.i.i.i.i44 = icmp eq i64 %_17.sroa.4.0.copyload, 0
  %.not.i.i.i.i45 = icmp eq i8* %_17.sroa.0.0.copyload, null
  %or.cond.i.i.i.i46 = or i1 %.not.i.i.i.i45, %_4.i.i.i.i.i44
  br i1 %or.cond.i.i.i.i46, label %bb26, label %"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17hcbba748b2e896a43E.exit.i.i.i.i47"

"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17hcbba748b2e896a43E.exit.i.i.i.i47": ; preds = %bb17
  tail call void @__rust_dealloc(i8* nonnull %_17.sroa.0.0.copyload, i64 %_17.sroa.4.0.copyload, i64 1) #7
  br label %bb26

bb26:                                             ; preds = %"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17hcbba748b2e896a43E.exit.i.i.i.i47", %bb17
  %2 = bitcast %"std::string::String"* %r3 to i8*
  call void @llvm.lifetime.start.p0i8(i64 24, i8* nonnull %2)
; call alloc::str::<impl alloc::borrow::ToOwned for str>::to_owned
  call fastcc void @"_ZN5alloc3str56_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$str$GT$8to_owned17hbc7ea6a9d21ebdedE"(%"std::string::String"* noalias nocapture nonnull dereferenceable(24) %r3, [0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [14 x i8] }>* @alloc10 to [0 x i8]*), i64 14)
  %_33.sroa.0.0..sroa_cast125 = bitcast %"std::string::String"* %r3 to i8**
  %_33.sroa.0.0.copyload = load i8*, i8** %_33.sroa.0.0..sroa_cast125, align 8
  %_33.sroa.4.0..sroa_idx127 = getelementptr inbounds %"std::string::String", %"std::string::String"* %r3, i64 0, i32 1, i32 1, i32 1
  %_33.sroa.4.0.copyload = load i64, i64* %_33.sroa.4.0..sroa_idx127, align 8
  call void @llvm.lifetime.end.p0i8(i64 24, i8* nonnull %2)
  %_4.i.i.i.i.i = icmp eq i64 %_33.sroa.4.0.copyload, 0
  %.not.i.i.i.i = icmp eq i8* %_33.sroa.0.0.copyload, null
  %or.cond.i.i.i.i = or i1 %.not.i.i.i.i, %_4.i.i.i.i.i
  br i1 %or.cond.i.i.i.i, label %_ZN4core3ptr13drop_in_place17he6fd0cfa44fc843cE.exit, label %"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17hcbba748b2e896a43E.exit.i.i.i.i"

"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17hcbba748b2e896a43E.exit.i.i.i.i": ; preds = %bb26
  tail call void @__rust_dealloc(i8* nonnull %_33.sroa.0.0.copyload, i64 %_33.sroa.4.0.copyload, i64 1) #7
  br label %_ZN4core3ptr13drop_in_place17he6fd0cfa44fc843cE.exit

_ZN4core3ptr13drop_in_place17he6fd0cfa44fc843cE.exit: ; preds = %bb26, %"_ZN62_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..AllocRef$GT$7dealloc17hcbba748b2e896a43E.exit.i.i.i.i"
  ret void


; Function Attrs: nonlazybind uwtable
define void @format_wo_fmt_args_ret(%"std::string::String"* noalias nocapture sret dereferenceable(24) %r) unnamed_addr #2 {
; call alloc::str::<impl alloc::borrow::ToOwned for str>::to_owned
; call alloc::str::<impl alloc::borrow::ToOwned for str>::to_owned
  tail call fastcc void @"_ZN5alloc3str56_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$str$GT$8to_owned17hbc7ea6a9d21ebdedE"(%"std::string::String"* noalias nocapture nonnull dereferenceable(24) %r, [0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [12 x i8] }>* @alloc42 to [0 x i8]*), i64 12)
  ret void


; Function Attrs: nonlazybind uwtable
define void @format_w_fmt_args_ret_1(%"std::string::String"* noalias nocapture sret dereferenceable(24) %r, i64 %0) unnamed_addr #2 {
bb9:
  %_22 = alloca %"std::fmt::Arguments", align 8
  %_9 = alloca [1 x { i8*, i64* }], align 8
  %n = alloca i64, align 8
  store i64 %0, i64* %n, align 8
  %1 = bitcast [1 x { i8*, i64* }]* %_9 to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* nonnull %1)
  %2 = bitcast [1 x { i8*, i64* }]* %_9 to i64**
  store i64* %n, i64** %2, align 8
  %3 = getelementptr inbounds [1 x { i8*, i64* }], [1 x { i8*, i64* }]* %_9, i64 0, i64 0, i32 1
  store i64* bitcast (i1 (i64*, %"std::fmt::Formatter"*)* @"_ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17hfadb3de06e46535aE" to i64*), i64** %3, align 8
  %4 = bitcast %"std::fmt::Arguments"* %_22 to i8*
  call void @llvm.lifetime.start.p0i8(i64 48, i8* nonnull %4)
  %args.sroa.0.0..sroa_cast17 = bitcast %"std::fmt::Arguments"* %_22 to [0 x { [0 x i8]*, i64 }]**
  store [0 x { [0 x i8]*, i64 }]* bitcast (<{ i8*, [8 x i8] }>* @alloc49 to [0 x { [0 x i8]*, i64 }]*), [0 x { [0 x i8]*, i64 }]** %args.sroa.0.0..sroa_cast17, align 8
  %args.sroa.5.0..sroa_idx19 = getelementptr inbounds %"std::fmt::Arguments", %"std::fmt::Arguments"* %_22, i64 0, i32 1, i32 1
  store i64 1, i64* %args.sroa.5.0..sroa_idx19, align 8
  %args.sroa.7.0..sroa_idx21 = getelementptr inbounds %"std::fmt::Arguments", %"std::fmt::Arguments"* %_22, i64 0, i32 3, i32 0
  store i64* null, i64** %args.sroa.7.0..sroa_idx21, align 8
  %args.sroa.9.0..sroa_idx25 = getelementptr inbounds %"std::fmt::Arguments", %"std::fmt::Arguments"* %_22, i64 0, i32 5, i32 0
  %5 = bitcast [0 x { i8*, i64* }]** %args.sroa.9.0..sroa_idx25 to [1 x { i8*, i64* }]**
  store [1 x { i8*, i64* }]* %_9, [1 x { i8*, i64* }]** %5, align 8
  %args.sroa.10.0..sroa_idx27 = getelementptr inbounds %"std::fmt::Arguments", %"std::fmt::Arguments"* %_22, i64 0, i32 5, i32 1
  store i64 1, i64* %args.sroa.10.0..sroa_idx27, align 8
; call alloc::fmt::format
  call void @_ZN5alloc3fmt6format17hfa56c9fdd760adc1E(%"std::string::String"* noalias nocapture nonnull sret dereferenceable(24) %r, %"std::fmt::Arguments"* noalias nocapture nonnull dereferenceable(48) %_22)
  call void @llvm.lifetime.end.p0i8(i64 48, i8* nonnull %4)
  call void @llvm.lifetime.end.p0i8(i64 16, i8* nonnull %1)
  ret void


; Function Attrs: nonlazybind uwtable
define void @format_w_fmt_args_ret_2(%"std::string::String"* noalias nocapture sret dereferenceable(24) %r, i64 %0, i64 %1) unnamed_addr #2 {
bb10:
  %_30 = alloca %"std::fmt::Arguments", align 8
  %_10 = alloca [2 x { i8*, i64* }], align 8
  %width = alloca i64, align 8
  %n = alloca i64, align 8
  store i64 %0, i64* %n, align 8
  store i64 %1, i64* %width, align 8
  %2 = bitcast [2 x { i8*, i64* }]* %_10 to i8*
  call void @llvm.lifetime.start.p0i8(i64 32, i8* nonnull %2)
; call core::fmt::ArgumentV1::from_usize
  %3 = call { i8*, i64* } @_ZN4core3fmt10ArgumentV110from_usize17h91c3b2b6934d766aE(i64* noalias nonnull readonly align 8 dereferenceable(8) %width)
  %_19.0 = extractvalue { i8*, i64* } %3, 0
  %_19.1 = extractvalue { i8*, i64* } %3, 1
  %4 = bitcast [2 x { i8*, i64* }]* %_10 to i64**
  store i64* %n, i64** %4, align 8
  %5 = getelementptr inbounds [2 x { i8*, i64* }], [2 x { i8*, i64* }]* %_10, i64 0, i64 0, i32 1
  store i64* bitcast (i1 (i64*, %"std::fmt::Formatter"*)* @"_ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17hfadb3de06e46535aE" to i64*), i64** %5, align 8
  %6 = getelementptr inbounds [2 x { i8*, i64* }], [2 x { i8*, i64* }]* %_10, i64 0, i64 1, i32 0
  store i8* %_19.0, i8** %6, align 8
  %7 = getelementptr inbounds [2 x { i8*, i64* }], [2 x { i8*, i64* }]* %_10, i64 0, i64 1, i32 1
  store i64* %_19.1, i64** %7, align 8
  %8 = bitcast %"std::fmt::Arguments"* %_30 to i8*
  call void @llvm.lifetime.start.p0i8(i64 48, i8* nonnull %8)
  %args.sroa.0.0..sroa_cast17 = bitcast %"std::fmt::Arguments"* %_30 to [0 x { [0 x i8]*, i64 }]**
  store [0 x { [0 x i8]*, i64 }]* bitcast (<{ i8*, [8 x i8] }>* @alloc55 to [0 x { [0 x i8]*, i64 }]*), [0 x { [0 x i8]*, i64 }]** %args.sroa.0.0..sroa_cast17, align 8
  %args.sroa.5.0..sroa_idx19 = getelementptr inbounds %"std::fmt::Arguments", %"std::fmt::Arguments"* %_30, i64 0, i32 1, i32 1
  store i64 1, i64* %args.sroa.5.0..sroa_idx19, align 8
  %args.sroa.7.0..sroa_idx21 = getelementptr inbounds %"std::fmt::Arguments", %"std::fmt::Arguments"* %_30, i64 0, i32 3, i32 0
  store i64* bitcast (<{ [56 x i8] }>* @alloc64 to i64*), i64** %args.sroa.7.0..sroa_idx21, align 8
  %args.sroa.8.0..sroa_idx23 = getelementptr inbounds %"std::fmt::Arguments", %"std::fmt::Arguments"* %_30, i64 0, i32 3, i32 1
  store i64 1, i64* %args.sroa.8.0..sroa_idx23, align 8
  %args.sroa.9.0..sroa_idx25 = getelementptr inbounds %"std::fmt::Arguments", %"std::fmt::Arguments"* %_30, i64 0, i32 5, i32 0
  %9 = bitcast [0 x { i8*, i64* }]** %args.sroa.9.0..sroa_idx25 to [2 x { i8*, i64* }]**
  store [2 x { i8*, i64* }]* %_10, [2 x { i8*, i64* }]** %9, align 8
  %args.sroa.10.0..sroa_idx27 = getelementptr inbounds %"std::fmt::Arguments", %"std::fmt::Arguments"* %_30, i64 0, i32 5, i32 1
  store i64 2, i64* %args.sroa.10.0..sroa_idx27, align 8
; call alloc::fmt::format
  call void @_ZN5alloc3fmt6format17hfa56c9fdd760adc1E(%"std::string::String"* noalias nocapture nonnull sret dereferenceable(24) %r, %"std::fmt::Arguments"* noalias nocapture nonnull dereferenceable(48) %_30)
  call void @llvm.lifetime.end.p0i8(i64 48, i8* nonnull %8)
  call void @llvm.lifetime.end.p0i8(i64 32, i8* nonnull %2)
  ret void

; core::panicking::panic
; core::panicking::panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking5panic17hd33b8bec11e2d3e4E([0 x i8]* noalias nonnull readonly align 1, i64, %"std::panic::Location"* noalias readonly align 8 dereferenceable(24)) unnamed_addr #3

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #4

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #4

; Function Attrs: cold noreturn nounwind
declare void @llvm.trap() #5

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #4

; Function Attrs: nounwind nonlazybind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #0

; core::fmt::num::imp::<impl core::fmt::Display for usize>::fmt
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN4core3fmt3num3imp54_$LT$impl$u20$core..fmt..Display$u20$for$u20$usize$GT$3fmt17hfadb3de06e46535aE"(i64* noalias readonly align 8 dereferenceable(8), %"std::fmt::Formatter"* align 8 dereferenceable(64)) unnamed_addr #2

; Function Attrs: nounwind nonlazybind uwtable
declare noalias i8* @__rust_alloc(i64, i64) unnamed_addr #0

; Function Attrs: nounwind nonlazybind uwtable
declare void @__rust_dealloc(i8*, i64, i64) unnamed_addr #0

; Function Attrs: nounwind nonlazybind uwtable
declare i8* @__rust_realloc(i8*, i64, i64, i64) unnamed_addr #0
; alloc::alloc::handle_alloc_error
; alloc::alloc::handle_alloc_error
; Function Attrs: noreturn nounwind nonlazybind uwtable
declare void @_ZN5alloc5alloc18handle_alloc_error17h0bb0cccd73d589d8E(i64, i64) unnamed_addr #6
; alloc::fmt::format
; alloc::fmt::format
; Function Attrs: nonlazybind uwtable
declare void @_ZN5alloc3fmt6format17hfa56c9fdd760adc1E(%"std::string::String"* noalias nocapture sret dereferenceable(24), %"std::fmt::Arguments"* noalias nocapture dereferenceable(48)) unnamed_addr #2

; core::fmt::ArgumentV1::from_usize
; Function Attrs: nonlazybind uwtable
declare { i8*, i64* } @_ZN4core3fmt10ArgumentV110from_usize17h91c3b2b6934d766aE(i64* noalias readonly align 8 dereferenceable(8)) unnamed_addr #2

attributes #0 = { nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { inlinehint nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #2 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #3 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #4 = { argmemonly nounwind willreturn }
attributes #5 = { cold noreturn nounwind }
attributes #6 = { noreturn nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #7 = { nounwind }
attributes #8 = { noinline }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{!3, !5, !7, !9}
!3 = distinct !{!3, !4, !"_ZN5alloc3vec12Vec$LT$T$GT$13with_capacity17he575f9c5052d2356E: argument 0"}
!4 = distinct !{!4, !"_ZN5alloc3vec12Vec$LT$T$GT$13with_capacity17he575f9c5052d2356E"}
!5 = distinct !{!5, !6, !"_ZN5alloc5slice4hack6to_vec17h78baae15b8be3debE: %vec"}
!6 = distinct !{!6, !"_ZN5alloc5slice4hack6to_vec17h78baae15b8be3debE"}
!7 = distinct !{!7, !8, !"_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6to_vec17he013a9cfa5928325E: argument 0"}
!8 = distinct !{!8, !"_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6to_vec17he013a9cfa5928325E"}
!9 = distinct !{!9, !10, !"_ZN5alloc5slice64_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$$u5b$T$u5d$$GT$8to_owned17h24e8c4b9377b39f9E: argument 0"}
!10 = distinct !{!10, !"_ZN5alloc5slice64_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$$u5b$T$u5d$$GT$8to_owned17h24e8c4b9377b39f9E"}
!11 = !{!12, !13, !14}
!12 = distinct !{!12, !6, !"_ZN5alloc5slice4hack6to_vec17h78baae15b8be3debE: %s.0"}
!13 = distinct !{!13, !8, !"_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6to_vec17he013a9cfa5928325E: %self.0"}
!14 = distinct !{!14, !10, !"_ZN5alloc5slice64_$LT$impl$u20$alloc..borrow..ToOwned$u20$for$u20$$u5b$T$u5d$$GT$8to_owned17h24e8c4b9377b39f9E: %self.0"}
!15 = !{!5, !7, !9}
!16 = !{!17, !19, !20, !22, !5, !7, !9}
!17 = distinct !{!17, !18, !"_ZN5alloc7raw_vec11finish_grow17h276683138c302eb8E: argument 0"}
!18 = distinct !{!18, !"_ZN5alloc7raw_vec11finish_grow17h276683138c302eb8E"}
!19 = distinct !{!19, !18, !"_ZN5alloc7raw_vec11finish_grow17h276683138c302eb8E: %current_memory"}
!20 = distinct !{!20, !21, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14grow_amortized17hbaca4f45a3988bb2E: argument 0"}
!21 = distinct !{!21, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$14grow_amortized17hbaca4f45a3988bb2E"}
!22 = distinct !{!22, !23, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11try_reserve17h336995be05635d2dE: argument 0"}
!23 = distinct !{!23, !"_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11try_reserve17h336995be05635d2dE"}
!24 = !{!20, !22, !25, !12, !13, !14}
!25 = distinct !{!25, !26, !"_ZN5alloc3vec12Vec$LT$T$GT$17extend_from_slice17ha921f1ef1b039a54E: %other.0"}
!26 = distinct !{!26, !"_ZN5alloc3vec12Vec$LT$T$GT$17extend_from_slice17ha921f1ef1b039a54E"}
!27 = !{!25, !12, !13, !14}
!28 = !{!29, !31}
!29 = distinct !{!29, !30, !"_ZN5alloc6string6String19from_utf8_unchecked17h15b09e970de26a26E: argument 0"}
!30 = distinct !{!30, !"_ZN5alloc6string6String19from_utf8_unchecked17h15b09e970de26a26E"}
!31 = distinct !{!31, !30, !"_ZN5alloc6string6String19from_utf8_unchecked17h15b09e970de26a26E: %bytes"}
== end ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (6365) (node)
Terminate orphan process: pid (6374) (python)
