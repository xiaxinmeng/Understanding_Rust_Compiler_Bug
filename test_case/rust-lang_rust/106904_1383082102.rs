plain
---- [run-make] checkout/tests/run-make-fulldeps/split-debuginfo stdout ----

error: make failed
status: exit status: 2
command: cd "/checkout/tests/run-make-fulldeps/split-debuginfo" && AR="ar" CC="cc -ffunction-sections -fdata-sections -fPIC -m64" CXX="c++ -ffunction-sections -fdata-sections -fPIC -m64" HOST_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" LD_LIB_PATH_ENVVAR="LD_LIBRARY_PATH" LLVM_BIN_DIR="/usr/lib/llvm-13/bin" LLVM_COMPONENTS="aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" LLVM_FILECHECK="/usr/lib/llvm-13/bin/FileCheck" NODE="/usr/bin/node" PYTHON="/usr/bin/python3" RUSTC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" RUSTDOC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_BUILD_STAGE="stage2-x86_64-unknown-linux-gnu" RUST_DEMANGLER="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" S="/checkout" TARGET="x86_64-unknown-linux-gnu" TARGET_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" TMPDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo" "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo  foo.rs -g -C  split-debuginfo=off
[ ! -f /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwp ]
[ ! -f /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwo ]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo  foo.rs -g
[ ! -f /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwp ]
[ ! -f /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwo ]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo  foo.rs -g  -C split-debuginfo=packed -Zsplit-dwarf-kind=split
ls /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.o && exit 1 || exit 0
ls /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwo && exit 1 || exit 0
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/foo.dwp
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/foo
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo  foo.rs -g  -C split-debuginfo=packed -Zsplit-dwarf-kind=single
ls /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.o && exit 1 || exit 0
ls /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwo && exit 1 || exit 0
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/foo.dwp
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/foo
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo  baz.rs -g  -Csplit-debuginfo=packed -Zsplit-dwarf-kind=split \
 --crate-type=rlib -Clinker-plugin-lto
ls /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.o && exit 1 || exit 0
ls /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwo && exit 1 || exit 0
ls /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwp && exit 1 || exit 0
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/libbaz.rlib
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo  baz.rs -g  -Csplit-debuginfo=packed -Zsplit-dwarf-kind=single \
 --crate-type=rlib -Clinker-plugin-lto
ls /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.o && exit 1 || exit 0
ls /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwo && exit 1 || exit 0
ls /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwp && exit 1 || exit 0
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/libbaz.rlib
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo   -C split-debuginfo=packed -C debuginfo=2 \
 -Z split-dwarf-kind=split --remap-path-prefix /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo=/a foo.rs -g
objdump -Wi /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/foo | grep DW_AT_GNU_dwo_name | (! grep /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo) || exit 1
ls /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.o && exit 1 || exit 0
ls /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwo && exit 1 || exit 0
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/foo.dwp
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/foo
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo   -C split-debuginfo=packed -C debuginfo=2 \
 -Z split-dwarf-kind=single --remap-path-prefix /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo=/a foo.rs -g
objdump -Wi /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/foo | grep DW_AT_GNU_dwo_name | (! grep /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo) || exit 1
ls /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.o && exit 1 || exit 0
ls /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwo && exit 1 || exit 0
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/foo.dwp
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/foo
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo  --crate-type lib  -C split-debuginfo=packed \
 -Zsplit-dwarf-kind=split -C debuginfo=2 -g bar.rs
ls /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.rlib
/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/libbar.rlib
ls /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.o && exit 1 || exit 0
ls /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwo && exit 1 || exit 0
/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/bar.bar.a24afc20-cgu.0.rcgu.dwo
/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/bar.bar.a24afc20-cgu.1.rcgu.dwo
--- stderr -------------------------------
--- stderr -------------------------------
ls: cannot access '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.o': No such file or directory
ls: cannot access '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwo': No such file or directory
ls: cannot access '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.o': No such file or directory
ls: cannot access '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwo': No such file or directory
ls: cannot access '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.o': No such file or directory
ls: cannot access '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwo': No such file or directory
ls: cannot access '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwp': No such file or directory
ls: cannot access '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.o': No such file or directory
ls: cannot access '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwo': No such file or directory
ls: cannot access '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwp': No such file or directory
objdump: Warning: Unable to load dwo file: /a/foo.foo.e33ba30d-cgu.4.rcgu.dwo
objdump: Warning: Unable to load dwo file: /a/foo.foo.e33ba30d-cgu.3.rcgu.dwo
objdump: Warning: Unable to load dwo file: /a/foo.foo.e33ba30d-cgu.2.rcgu.dwo
objdump: Warning: Unable to load dwo file: /a/foo.foo.e33ba30d-cgu.1.rcgu.dwo
objdump: Warning: Unable to load dwo file: /a/foo.foo.e33ba30d-cgu.0.rcgu.dwo
ls: cannot access '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.o': No such file or directory
ls: cannot access '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwo': No such file or directory
objdump: Warning: Unable to load dwo file: /a/foo.foo.e33ba30d-cgu.4.rcgu.o
objdump: Warning: Unable to load dwo file: /a/foo.foo.e33ba30d-cgu.3.rcgu.o
objdump: Warning: Unable to load dwo file: /a/foo.foo.e33ba30d-cgu.2.rcgu.o
objdump: Warning: Unable to load dwo file: /a/foo.foo.e33ba30d-cgu.1.rcgu.o
objdump: Warning: Unable to load dwo file: /a/foo.foo.e33ba30d-cgu.0.rcgu.o
ls: cannot access '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.o': No such file or directory
ls: cannot access '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwo': No such file or directory
ls: cannot access '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.o': No such file or directory
make: *** [Makefile:150: packed-crosscrate-split] Error 1



failures:
