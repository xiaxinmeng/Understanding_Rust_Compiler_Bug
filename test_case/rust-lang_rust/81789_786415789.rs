plain

---- [ui] ui/lint/lint-invalid-ptr-to-int-cast.rs stdout ----
diff of fixed:

1 // check-pass
- // run-rustfix 
+ // run-rustfix
3 // compile-flags: -W invalid-ptr-to-int-cast
5 fn main() {


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-invalid-ptr-to-int-cast/lint-invalid-ptr-to-int-cast.fixed
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/lint-invalid-ptr-to-int-cast.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-invalid-ptr-to-int-cast.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-invalid-ptr-to-int-cast" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-W" "invalid-ptr-to-int-cast" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-invalid-ptr-to-int-cast/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: casting pointer to `u32`
  --> /checkout/src/test/ui/lint/lint-invalid-ptr-to-int-cast.rs:7:13
   |
LL |     let _ = u16::max as u32; //~ casting pointer to `u32` [invalid_ptr_to_int_cast]
   |             ^^^^^^^^^^^^^^^ help: to cast to `u32`, cast to `usize` first: `u16::max as usize as u32`
   |
   = note: requested on the command line with `-W invalid-ptr-to-int-cast`
   = help: pointers should only be cast to `usize`
warning: casting pointer to `u8`
  --> /checkout/src/test/ui/lint/lint-invalid-ptr-to-int-cast.rs:12:13
   |
   |
LL |     let _ = ptr as u8; //~ casting pointer to `u8` [invalid_ptr_to_int_cast]
   |             ^^^^^^^^^ help: to cast to `u8`, cast to `usize` first: `ptr as usize as u8`
   |
   = help: pointers should only be cast to `usize`
warning: casting pointer to `u16`
  --> /checkout/src/test/ui/lint/lint-invalid-ptr-to-int-cast.rs:13:13
   |
   |
LL |     let _ = ptr as u16; //~ casting pointer to `u16` [invalid_ptr_to_int_cast]
   |             ^^^^^^^^^^ help: to cast to `u16`, cast to `usize` first: `ptr as usize as u16`
   |
   = help: pointers should only be cast to `usize`
warning: casting pointer to `u32`
  --> /checkout/src/test/ui/lint/lint-invalid-ptr-to-int-cast.rs:14:13
   |
   |
LL |     let _ = ptr as u32; //~ casting pointer to `u32` [invalid_ptr_to_int_cast]
   |             ^^^^^^^^^^ help: to cast to `u32`, cast to `usize` first: `ptr as usize as u32`
   |
   = help: pointers should only be cast to `usize`
warning: casting pointer to `u64`
  --> /checkout/src/test/ui/lint/lint-invalid-ptr-to-int-cast.rs:15:13
   |
   |
LL |     let _ = ptr as u64; //~ casting pointer to `u64` [invalid_ptr_to_int_cast]
   |             ^^^^^^^^^^ help: to cast to `u64`, cast to `usize` first: `ptr as usize as u64`
   |
   = help: pointers should only be cast to `usize`
warning: casting pointer to `u128`
  --> /checkout/src/test/ui/lint/lint-invalid-ptr-to-int-cast.rs:16:13
   |
   |
LL |     let _ = ptr as u128; //~ casting pointer to `u128` [invalid_ptr_to_int_cast]
   |             ^^^^^^^^^^^ help: to cast to `u128`, cast to `usize` first: `ptr as usize as u128`
   |
   = help: pointers should only be cast to `usize`
warning: casting pointer to `i8`
  --> /checkout/src/test/ui/lint/lint-invalid-ptr-to-int-cast.rs:17:13
   |
   |
LL |     let _ = ptr as i8; //~ casting pointer to `i8` [invalid_ptr_to_int_cast]
   |             ^^^^^^^^^ help: to cast to `i8`, cast to `usize` first: `ptr as usize as i8`
   |
   = help: pointers should only be cast to `usize`
warning: casting pointer to `i16`
  --> /checkout/src/test/ui/lint/lint-invalid-ptr-to-int-cast.rs:18:13
   |
   |
LL |     let _ = ptr as i16; //~ casting pointer to `i16` [invalid_ptr_to_int_cast]
   |             ^^^^^^^^^^ help: to cast to `i16`, cast to `usize` first: `ptr as usize as i16`
   |
   = help: pointers should only be cast to `usize`
warning: casting pointer to `i32`
  --> /checkout/src/test/ui/lint/lint-invalid-ptr-to-int-cast.rs:19:13
   |
   |
LL |     let _ = ptr as i32; //~ casting pointer to `i32` [invalid_ptr_to_int_cast]
   |             ^^^^^^^^^^ help: to cast to `i32`, cast to `usize` first: `ptr as usize as i32`
   |
   = help: pointers should only be cast to `usize`
warning: casting pointer to `i64`
  --> /checkout/src/test/ui/lint/lint-invalid-ptr-to-int-cast.rs:20:13
   |
   |
LL |     let _ = ptr as i64; //~ casting pointer to `i64` [invalid_ptr_to_int_cast]
   |             ^^^^^^^^^^ help: to cast to `i64`, cast to `usize` first: `ptr as usize as i64`
   |
   = help: pointers should only be cast to `usize`
warning: casting pointer to `i128`
  --> /checkout/src/test/ui/lint/lint-invalid-ptr-to-int-cast.rs:21:13
   |
   |
LL |     let _ = ptr as i128; //~ casting pointer to `i128` [invalid_ptr_to_int_cast]
   |             ^^^^^^^^^^^ help: to cast to `i128`, cast to `usize` first: `ptr as usize as i128`
   |
   = help: pointers should only be cast to `usize`
warning: casting pointer to `u8`
  --> /checkout/src/test/ui/lint/lint-invalid-ptr-to-int-cast.rs:27:13
   |
   |
LL |     let _ = test as u8; //~ casting pointer to `u8` [invalid_ptr_to_int_cast]
   |             ^^^^^^^^^^ help: to cast to `u8`, cast to `usize` first: `test as usize as u8`
   |
   = help: pointers should only be cast to `usize`
warning: casting pointer to `u16`
  --> /checkout/src/test/ui/lint/lint-invalid-ptr-to-int-cast.rs:28:13
   |
   |
LL |     let _ = test as u16; //~ casting pointer to `u16` [invalid_ptr_to_int_cast]
   |             ^^^^^^^^^^^ help: to cast to `u16`, cast to `usize` first: `test as usize as u16`
   |
   = help: pointers should only be cast to `usize`
warning: casting pointer to `u32`
  --> /checkout/src/test/ui/lint/lint-invalid-ptr-to-int-cast.rs:29:13
   |
   |
LL |     let _ = test as u32; //~ casting pointer to `u32` [invalid_ptr_to_int_cast]
   |             ^^^^^^^^^^^ help: to cast to `u32`, cast to `usize` first: `test as usize as u32`
   |
   = help: pointers should only be cast to `usize`
warning: casting pointer to `u64`
  --> /checkout/src/test/ui/lint/lint-invalid-ptr-to-int-cast.rs:30:13
   |
   |
LL |     let _ = test as u64; //~ casting pointer to `u64` [invalid_ptr_to_int_cast]
   |             ^^^^^^^^^^^ help: to cast to `u64`, cast to `usize` first: `test as usize as u64`
   |
   = help: pointers should only be cast to `usize`
warning: casting pointer to `u128`
  --> /checkout/src/test/ui/lint/lint-invalid-ptr-to-int-cast.rs:31:13
   |
   |
LL |     let _ = test as u128; //~ casting pointer to `u128` [invalid_ptr_to_int_cast]
   |             ^^^^^^^^^^^^ help: to cast to `u128`, cast to `usize` first: `test as usize as u128`
   |
   = help: pointers should only be cast to `usize`
warning: casting pointer to `i8`
  --> /checkout/src/test/ui/lint/lint-invalid-ptr-to-int-cast.rs:32:13
   |
   |
LL |     let _ = test as i8; //~ casting pointer to `i8` [invalid_ptr_to_int_cast]
   |             ^^^^^^^^^^ help: to cast to `i8`, cast to `usize` first: `test as usize as i8`
   |
   = help: pointers should only be cast to `usize`
warning: casting pointer to `i16`
  --> /checkout/src/test/ui/lint/lint-invalid-ptr-to-int-cast.rs:33:13
   |
   |
LL |     let _ = test as i16; //~ casting pointer to `i16` [invalid_ptr_to_int_cast]
   |             ^^^^^^^^^^^ help: to cast to `i16`, cast to `usize` first: `test as usize as i16`
   |
   = help: pointers should only be cast to `usize`
warning: casting pointer to `i32`
  --> /checkout/src/test/ui/lint/lint-invalid-ptr-to-int-cast.rs:34:13
   |
   |
LL |     let _ = test as i32; //~ casting pointer to `i32` [invalid_ptr_to_int_cast]
   |             ^^^^^^^^^^^ help: to cast to `i32`, cast to `usize` first: `test as usize as i32`
   |
   = help: pointers should only be cast to `usize`
warning: casting pointer to `i64`
  --> /checkout/src/test/ui/lint/lint-invalid-ptr-to-int-cast.rs:35:13
   |
   |
LL |     let _ = test as i64; //~ casting pointer to `i64` [invalid_ptr_to_int_cast]
   |             ^^^^^^^^^^^ help: to cast to `i64`, cast to `usize` first: `test as usize as i64`
   |
   = help: pointers should only be cast to `usize`
warning: casting pointer to `i128`
  --> /checkout/src/test/ui/lint/lint-invalid-ptr-to-int-cast.rs:36:13
   |
   |
LL |     let _ = test as i128; //~ casting pointer to `i128` [invalid_ptr_to_int_cast]
   |             ^^^^^^^^^^^^ help: to cast to `i128`, cast to `usize` first: `test as usize as i128`
   |
   = help: pointers should only be cast to `usize`
warning: 21 warnings emitted


------------------------------------------
---
test result: FAILED. 11408 passed; 1 failed; 93 ignored; 0 measured; 0 filtered out; finished in 136.62s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:03
