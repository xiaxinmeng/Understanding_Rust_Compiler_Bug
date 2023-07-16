plain
    Finished release [optimized] target(s) in 23.36s
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 12310 tests
...............................F.................................................................... 100/12310
........................................iiiiiiiiiii......F...F.i...........ii...ii.................. 200/12310
.................................................................................................... 400/12310
.................................................................................................... 500/12310
.................................................................................................... 600/12310
..........................................................................i......................... 700/12310
---

---- [ui] ui/abi/unsupported.rs#aarch64 stdout ----
diff of stderr:

+ '+i8mm' is not a recognized feature for this target (ignoring feature)
+ '+i8mm' is not a recognized feature for this target (ignoring feature)
1 error[E0570]: `"ptx-kernel"` is not a supported ABI for the current target
3    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.aarch64/unsupported.aarch64.stderr
To only update this specific test, also pass `--test-args abi/unsupported.rs`


error in revision `aarch64`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/unsupported.rs" "-Zthreads=1" "--cfg" "aarch64" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.aarch64" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=aarch64-unknown-linux-gnu" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.aarch64/auxiliary"
------------------------------------------

------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stderr:
------------------------------------------
'+i8mm' is not a recognized feature for this target (ignoring feature)
'+i8mm' is not a recognized feature for this target (ignoring feature)
error[E0570]: `"ptx-kernel"` is not a supported ABI for the current target
   |
   |
LL | extern "ptx-kernel" fn ptx() {}


error[E0570]: `"amdgpu-kernel"` is not a supported ABI for the current target
   |
   |
LL | extern "amdgpu-kernel" fn amdgpu() {}


error[E0570]: `"wasm"` is not a supported ABI for the current target
   |
   |
LL | extern "wasm" fn wasm() {}


error[E0570]: `"aapcs"` is not a supported ABI for the current target
   |
   |
LL | extern "aapcs" fn aapcs() {}


error[E0570]: `"msp430-interrupt"` is not a supported ABI for the current target
   |
   |
LL | extern "msp430-interrupt" fn msp430() {}


error[E0570]: `"avr-interrupt"` is not a supported ABI for the current target
   |
   |
LL | extern "avr-interrupt" fn avr() {}


error[E0570]: `"x86-interrupt"` is not a supported ABI for the current target
   |
   |
LL | extern "x86-interrupt" fn x86() {}

warning: use of calling convention not supported on this target
  --> /checkout/src/test/ui/abi/unsupported.rs:43:1
   |
   |
LL | extern "stdcall" fn stdcall() {}
   |
   = note: `#[warn(unsupported_calling_conventions)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #87678 <https://github.com/rust-lang/rust/issues/87678>
   = note: for more information, see issue #87678 <https://github.com/rust-lang/rust/issues/87678>

warning: use of calling convention not supported on this target
  --> /checkout/src/test/ui/abi/unsupported.rs:50:1
   |
LL | extern "thiscall" fn thiscall() {}
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #87678 <https://github.com/rust-lang/rust/issues/87678>

---

---- [ui] ui/asm/bad-template.rs#aarch64_mirunsafeck stdout ----
diff of stderr:

+ '+i8mm' is not a recognized feature for this target (ignoring feature)
+ '+i8mm' is not a recognized feature for this target (ignoring feature)
1 error: invalid reference to argument at index 0
3    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_mirunsafeck/bad-template.aarch64_mirunsafeck.stderr
To only update this specific test, also pass `--test-args asm/bad-template.rs`


error in revision `aarch64_mirunsafeck`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/bad-template.rs" "-Zthreads=1" "--cfg" "aarch64_mirunsafeck" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_mirunsafeck" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "aarch64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_mirunsafeck/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
'+i8mm' is not a recognized feature for this target (ignoring feature)
'+i8mm' is not a recognized feature for this target (ignoring feature)
error: invalid reference to argument at index 0
   |
   |
LL |         asm!("{}");
   |               ^^ from here
   = note: no arguments were given

error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:33:15
  --> /checkout/src/test/ui/asm/bad-template.rs:33:15
   |
LL |         asm!("{1}", in(reg) foo);
   |               ^^^ from here
   = note: there is 1 argument

error: argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:33:21
  --> /checkout/src/test/ui/asm/bad-template.rs:33:21
   |
LL |         asm!("{1}", in(reg) foo);
   |                     ^^^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {0} */"`
error: there is no argument named `a`
  --> /checkout/src/test/ui/asm/bad-template.rs:36:15
   |
   |
LL |         asm!("{a}");

error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:38:15
   |
   |
LL |         asm!("{}", a = in(reg) foo);
   |               ^^   --------------- named argument
   |               from here
   |
   = note: no positional arguments were given
note: named arguments cannot be referenced by position
note: named arguments cannot be referenced by position
  --> /checkout/src/test/ui/asm/bad-template.rs:38:20
   |
LL |         asm!("{}", a = in(reg) foo);

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:38:20
   |
   |
LL |         asm!("{}", a = in(reg) foo);
   |                    ^^^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:41:15
   |
   |
LL |         asm!("{1}", a = in(reg) foo);
   |               ^^^ from here
   = note: no positional arguments were given

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:41:21
  --> /checkout/src/test/ui/asm/bad-template.rs:41:21
   |
LL |         asm!("{1}", a = in(reg) foo);
   |                     ^^^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:48:15
   |
   |
LL |         asm!("{}", in("x0") foo);
   |               ^^   ------------ explicit register argument
   |               from here
   |
   = note: no positional arguments were given
note: explicit register arguments cannot be used in the asm template
note: explicit register arguments cannot be used in the asm template
  --> /checkout/src/test/ui/asm/bad-template.rs:48:20
   |
LL |         asm!("{}", in("x0") foo);


error: asm template modifier must be a single character
   |
   |
LL |         asm!("{:foo}", in(reg) foo);


error: multiple unused asm arguments
   |
   |
LL |         asm!("", in(reg) 0, in(reg) 1);
   |                  ^^^^^^^^^  ^^^^^^^^^ argument never used
   |                  argument never used
   |
   |
   = help: if these arguments are intentionally unused, consider using them in an asm comment: `"/* {0} {1} */"`
error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:58:14
   |
   |
LL | global_asm!("{}");
   |              ^^ from here
   = note: no arguments were given

error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:60:14
  --> /checkout/src/test/ui/asm/bad-template.rs:60:14
   |
LL | global_asm!("{1}", const FOO);
   |              ^^^ from here
   = note: there is 1 argument

error: argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:60:20
  --> /checkout/src/test/ui/asm/bad-template.rs:60:20
   |
LL | global_asm!("{1}", const FOO);
   |                    ^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {0} */"`
error: there is no argument named `a`
  --> /checkout/src/test/ui/asm/bad-template.rs:63:14
   |
   |
LL | global_asm!("{a}");

error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:65:14
   |
   |
LL | global_asm!("{}", a = const FOO);
   |              ^^   ------------- named argument
   |              from here
   |
   = note: no positional arguments were given
note: named arguments cannot be referenced by position
note: named arguments cannot be referenced by position
  --> /checkout/src/test/ui/asm/bad-template.rs:65:19
   |
LL | global_asm!("{}", a = const FOO);

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:65:19
   |
   |
LL | global_asm!("{}", a = const FOO);
   |                   ^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:68:14
   |
   |
LL | global_asm!("{1}", a = const FOO);
   |              ^^^ from here
   = note: no positional arguments were given

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:68:20
  --> /checkout/src/test/ui/asm/bad-template.rs:68:20
   |
LL | global_asm!("{1}", a = const FOO);
   |                    ^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`

error: asm template modifier must be a single character
   |
   |
LL | global_asm!("{:foo}", const FOO);


error: multiple unused asm arguments
   |
   |
LL | global_asm!("", const FOO, const FOO);
   |                 ^^^^^^^^^  ^^^^^^^^^ argument never used
   |                 argument never used
   |
   |
   = help: if these arguments are intentionally unused, consider using them in an asm comment: `"/* {0} {1} */"`
error: aborting due to 21 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/asm/bad-template.rs#aarch64_thirunsafeck stdout ----
diff of stderr:

+ '+i8mm' is not a recognized feature for this target (ignoring feature)
+ '+i8mm' is not a recognized feature for this target (ignoring feature)
1 error: invalid reference to argument at index 0
3    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_thirunsafeck/bad-template.aarch64_thirunsafeck.stderr
To only update this specific test, also pass `--test-args asm/bad-template.rs`


error in revision `aarch64_thirunsafeck`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/bad-template.rs" "-Zthreads=1" "--cfg" "aarch64_thirunsafeck" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_thirunsafeck" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "thir-unsafeck" "--target" "aarch64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/asm/bad-template.aarch64_thirunsafeck/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
'+i8mm' is not a recognized feature for this target (ignoring feature)
'+i8mm' is not a recognized feature for this target (ignoring feature)
error: invalid reference to argument at index 0
   |
   |
LL |         asm!("{}");
   |               ^^ from here
   = note: no arguments were given

error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:33:15
  --> /checkout/src/test/ui/asm/bad-template.rs:33:15
   |
LL |         asm!("{1}", in(reg) foo);
   |               ^^^ from here
   = note: there is 1 argument

error: argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:33:21
  --> /checkout/src/test/ui/asm/bad-template.rs:33:21
   |
LL |         asm!("{1}", in(reg) foo);
   |                     ^^^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {0} */"`
error: there is no argument named `a`
  --> /checkout/src/test/ui/asm/bad-template.rs:36:15
   |
   |
LL |         asm!("{a}");

error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:38:15
   |
   |
LL |         asm!("{}", a = in(reg) foo);
   |               ^^   --------------- named argument
   |               from here
   |
   = note: no positional arguments were given
note: named arguments cannot be referenced by position
note: named arguments cannot be referenced by position
  --> /checkout/src/test/ui/asm/bad-template.rs:38:20
   |
LL |         asm!("{}", a = in(reg) foo);

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:38:20
   |
   |
LL |         asm!("{}", a = in(reg) foo);
   |                    ^^^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:41:15
   |
   |
LL |         asm!("{1}", a = in(reg) foo);
   |               ^^^ from here
   = note: no positional arguments were given

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:41:21
  --> /checkout/src/test/ui/asm/bad-template.rs:41:21
   |
LL |         asm!("{1}", a = in(reg) foo);
   |                     ^^^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:48:15
   |
   |
LL |         asm!("{}", in("x0") foo);
   |               ^^   ------------ explicit register argument
   |               from here
   |
   = note: no positional arguments were given
note: explicit register arguments cannot be used in the asm template
note: explicit register arguments cannot be used in the asm template
  --> /checkout/src/test/ui/asm/bad-template.rs:48:20
   |
LL |         asm!("{}", in("x0") foo);


error: asm template modifier must be a single character
   |
   |
LL |         asm!("{:foo}", in(reg) foo);


error: multiple unused asm arguments
   |
   |
LL |         asm!("", in(reg) 0, in(reg) 1);
   |                  ^^^^^^^^^  ^^^^^^^^^ argument never used
   |                  argument never used
   |
   |
   = help: if these arguments are intentionally unused, consider using them in an asm comment: `"/* {0} {1} */"`
error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:58:14
   |
   |
LL | global_asm!("{}");
   |              ^^ from here
   = note: no arguments were given

error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:60:14
  --> /checkout/src/test/ui/asm/bad-template.rs:60:14
   |
LL | global_asm!("{1}", const FOO);
   |              ^^^ from here
   = note: there is 1 argument

error: argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:60:20
  --> /checkout/src/test/ui/asm/bad-template.rs:60:20
   |
LL | global_asm!("{1}", const FOO);
   |                    ^^^^^^^^^ argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {0} */"`
error: there is no argument named `a`
  --> /checkout/src/test/ui/asm/bad-template.rs:63:14
   |
   |
LL | global_asm!("{a}");

error: invalid reference to argument at index 0
  --> /checkout/src/test/ui/asm/bad-template.rs:65:14
   |
   |
LL | global_asm!("{}", a = const FOO);
   |              ^^   ------------- named argument
   |              from here
   |
   = note: no positional arguments were given
note: named arguments cannot be referenced by position
note: named arguments cannot be referenced by position
  --> /checkout/src/test/ui/asm/bad-template.rs:65:19
   |
LL | global_asm!("{}", a = const FOO);

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:65:19
   |
   |
LL | global_asm!("{}", a = const FOO);
   |                   ^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`
error: invalid reference to argument at index 1
  --> /checkout/src/test/ui/asm/bad-template.rs:68:14
   |
   |
LL | global_asm!("{1}", a = const FOO);
   |              ^^^ from here
   = note: no positional arguments were given

error: named argument never used
  --> /checkout/src/test/ui/asm/bad-template.rs:68:20
  --> /checkout/src/test/ui/asm/bad-template.rs:68:20
   |
LL | global_asm!("{1}", a = const FOO);
   |                    ^^^^^^^^^^^^^ named argument never used
   |
   = help: if this argument is intentionally unused, consider using it in an asm comment: `"/* {a} */"`

error: asm template modifier must be a single character
   |
   |
LL | global_asm!("{:foo}", const FOO);


error: multiple unused asm arguments
   |
   |
LL | global_asm!("", const FOO, const FOO);
   |                 ^^^^^^^^^  ^^^^^^^^^ argument never used
   |                 argument never used
   |
   |
   = help: if these arguments are intentionally unused, consider using them in an asm comment: `"/* {0} {1} */"`
error: aborting due to 21 previous errors


------------------------------------------
---
test result: FAILED. 12190 passed; 3 failed; 117 ignored; 0 measured; 0 filtered out; finished in 134.24s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:19
