plain
Get:138 http://archive.ubuntu.com/ubuntu focal-updates/main amd64 libc-ares2 amd64 1.15.0-1ubuntu0.1 [38.2 kB]
Get:139 http://archive.ubuntu.com/ubuntu focal/universe amd64 libnode64 amd64 10.19.0~dfsg-3ubuntu1 [5765 kB]
Get:140 http://archive.ubuntu.com/ubuntu focal-updates/universe amd64 libpython2.7-stdlib amd64 2.7.18-1~20.04.1 [1887 kB]
Get:141 http://archive.ubuntu.com/ubuntu focal-updates/main amd64 libssl-dev amd64 1.1.1f-1ubuntu2.8 [1584 kB]
Get:142 http://archive.ubuntu.com/ubuntu focal-updates/universe amd64 llvm-11-runtime amd64 1:11.0.0-2~ubuntu20.04.1 [169 kB]
Get:143 http://archive.ubuntu.com/ubuntu focal/main amd64 libpfm4 amd64 4.10.1+git20-g7700f49-2 [266 kB]
Get:144 http://archive.ubuntu.com/ubuntu focal-updates/universe amd64 llvm-11 amd64 1:11.0.0-2~ubuntu20.04.1 [8395 kB]
Get:146 http://archive.ubuntu.com/ubuntu focal-updates/main amd64 python3-pygments all 2.3.1+dfsg-1ubuntu2.2 [579 kB]
Get:147 http://archive.ubuntu.com/ubuntu focal-updates/universe amd64 llvm-11-tools amd64 1:11.0.0-2~ubuntu20.04.1 [333 kB]
Get:148 http://archive.ubuntu.com/ubuntu focal/universe amd64 libz3-4 amd64 4.8.7-4build1 [6792 kB]
Get:149 http://archive.ubuntu.com/ubuntu focal/universe amd64 libz3-dev amd64 4.8.7-4build1 [67.5 kB]
---
Selecting previously unselected package llvm-11-runtime.
Preparing to unpack .../128-llvm-11-runtime_1%3a11.0.0-2~ubuntu20.04.1_amd64.deb ...
Unpacking llvm-11-runtime (1:11.0.0-2~ubuntu20.04.1) ...
Selecting previously unselected package libpfm4:amd64.
Preparing to unpack .../129-libpfm4_4.10.1+git20-g7700f49-2_amd64.deb ...
Unpacking libpfm4:amd64 (4.10.1+git20-g7700f49-2) ...
Preparing to unpack .../130-llvm-11_1%3a11.0.0-2~ubuntu20.04.1_amd64.deb ...
Unpacking llvm-11 (1:11.0.0-2~ubuntu20.04.1) ...
Selecting previously unselected package libffi-dev:amd64.
Preparing to unpack .../131-libffi-dev_3.3-4_amd64.deb ...
---
Run 'dpkg-reconfigure tzdata' if you wish to change it.

Setting up libz3-4:amd64 (4.8.7-4build1) ...
Setting up libuv1:amd64 (1.34.2-1ubuntu1.3) ...
Setting up libpfm4:amd64 (4.10.1+git20-g7700f49-2) ...
Setting up make (4.2.1-1.2) ...
Setting up libmpfr6:amd64 (4.0.2-1) ...
Setting up librtmp1:amd64 (2.4+20151223.gitfa8646d.1-2build1) ...
Setting up python2.7-minimal (2.7.18-1~20.04.1) ...
---
 ---> 021b01ffee27
Successfully built 021b01ffee27
Successfully tagged rust-ci:latest
Built container sha256:021b01ffee2732ae85126d3da2cbeb335a3acc4f7793b29e704e7c324d03d1da
Uploading finished image to https://ci-caches.rust-lang.org/docker/3d9226355e5d4cbddf3ea2f3cfce13c773da0ee32f6c394faf4360005a329a0ca12c1bfd109518dab03d308c9061a8c906840a7eb29cd9e0afaecf547fd169da
upload failed: - to s3://rust-lang-ci-sccache2/docker/3d9226355e5d4cbddf3ea2f3cfce13c773da0ee32f6c394faf4360005a329a0ca12c1bfd109518dab03d308c9061a8c906840a7eb29cd9e0afaecf547fd169da Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-11]
---
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 227 tests
..........i.ii...ii................................................................................. 100/227
................i...................iiiiii......i..................iii.............F........F....... 200/227
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [run-make] run-make-fulldeps/split-dwarf stdout ----


error: make failed
status: exit status: 2
command: "make"
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-dwarf/split-dwarf:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-dwarf/split-dwarf -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-dwarf/split-dwarf  -Z unstable-options -C split-debuginfo=packed -C debuginfo=2 foo.rs -g
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-dwarf/split-dwarf/foo.dwp
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
error: linking dwarf objects with `rust-llvm-dwp` failed: exit status: 1
  |
  = note: "rust-llvm-dwp" "-e" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-dwarf/split-dwarf/foo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-dwarf/split-dwarf/foo.dwp"
  = note: error: No such file or directory
          

error: aborting due to previous error
error: aborting due to previous error

rm: cannot remove '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-dwarf/split-dwarf/foo.dwp': No such file or directory
make: *** [Makefile:7: all] Error 1
------------------------------------------


---- [run-make] run-make-fulldeps/split-debuginfo stdout ----
---- [run-make] run-make-fulldeps/split-debuginfo stdout ----

error: make failed
status: exit status: 2
command: "make"
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo  foo.rs -g -C split-debuginfo=off -Z unstable-options
[ ! -f /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwp ]
[ ! -f /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwo ]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo  foo.rs -g
[ ! -f /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwp ]
[ ! -f /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwo ]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo  foo.rs -g -C split-debuginfo=packed -Z unstable-options
ls /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwp
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
error: linking dwarf objects with `rust-llvm-dwp` failed: exit status: 1
  |
  = note: "rust-llvm-dwp" "-e" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/foo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/foo.dwp"
  = note: error: No such file or directory
          

error: aborting due to previous error
error: aborting due to previous error

ls: cannot access '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/split-debuginfo/split-debuginfo/*.dwp': No such file or directory
make: *** [Makefile:49: packed] Error 2
------------------------------------------



---
test result: FAILED. 207 passed; 2 failed; 18 ignored; 0 measured; 0 filtered out; finished in 29.02s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--rust-demangler-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make-fulldeps" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-11/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "11.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--llvm-bin-dir" "/usr/lib/llvm-11/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:31:55
