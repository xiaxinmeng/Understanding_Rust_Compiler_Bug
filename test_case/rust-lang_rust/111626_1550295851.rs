plain
---- [run-make] tests/run-make/emit-to-stdout stdout ----

error: make failed
status: exit status: 2
command: cd "/checkout/tests/run-make/emit-to-stdout" && AR="ar" CC="cc -ffunction-sections -fdata-sections -fPIC -m64" CXX="c++ -ffunction-sections -fdata-sections -fPIC -m64" HOST_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" LD_LIB_PATH_ENVVAR="LD_LIBRARY_PATH" LLVM_BIN_DIR="/usr/lib/llvm-14/bin" LLVM_COMPONENTS="aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgputargetmca amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils ve veasmparser vecodegen vectorize vedesc vedisassembler veinfo webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86targetmca xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" LLVM_FILECHECK="/usr/lib/llvm-14/bin/FileCheck" NODE="/usr/bin/node" PYTHON="/usr/bin/python3" RUSTC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" RUSTDOC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_BUILD_STAGE="stage2-x86_64-unknown-linux-gnu" RUST_DEMANGLER="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" S="/checkout" TARGET="x86_64-unknown-linux-gnu" TARGET_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" TMPDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout" "make"
mkdir -p /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout/out
mkdir -p /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout/out
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout  --emit asm=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout/out/asm test.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout  --emit asm=- test.rs | diff - /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout/out/asm
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout  --emit llvm-ir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout/out/llvm-ir test.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout  --emit llvm-ir=- test.rs | diff - /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout/out/llvm-ir
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout  -Z dep-info-omit-d-target=yes --emit dep-info=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout/out/dep-info test.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout  --emit dep-info=- test.rs | diff - /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout/out/dep-info
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout  --emit mir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout/out/mir test.rs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout  --emit mir=- test.rs | diff - /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout/out/mir
ls -l /dev
crw-r--r--  1 root root     10, 235 May 16 19:36 autofs
drwxr-xr-x  2 root root          80 May 16 19:36 bsg
drwxr-xr-x  2 root root          80 May 16 19:36 bsg
crw-------  1 root root     10, 234 May 16 19:36 btrfs-control
lrwxrwxrwx  1 root root          11 May 16 19:36 core -> /proc/kcore
drwxr-xr-x 18 root root         360 May 16 19:36 cpu
crw-------  1 root root     10, 123 May 16 19:36 cpu_dma_latency
crw-------  1 root root     10, 203 May 16 19:36 cuse
drwxr-xr-x  2 root root          60 May 16 19:36 dma_heap
drwxr-xr-x  2 root root          60 May 16 19:36 dri
crw-------  1 root root     10, 126 May 16 19:36 ecryptfs
crw-rw----  1 root video    29,   0 May 16 19:36 fb0
lrwxrwxrwx  1 root root          13 May 16 19:36 fd -> /proc/self/fd
crw-rw-rw-  1 root root      1,   7 May 16 19:36 full
crw-rw-rw-  1 root root     10, 229 May 16 19:36 fuse
crw-------  1 root root     10, 228 May 16 19:36 hpet
crw-------  1 root root     10, 183 May 16 19:36 hwrng
drwxr-xr-x  2 root root         180 May 16 19:36 input
crw-r--r--  1 root root      1,  11 May 16 19:36 kmsg
crw-rw----  1 root     108  10, 232 May 16 19:36 kvm
crw-rw----  1 root disk     10, 237 May 16 19:36 loop-control
brw-rw----  1 root disk      7,   0 May 16 19:36 loop0
brw-rw----  1 root disk      7,   1 May 16 19:36 loop1
brw-rw----  1 root disk      7,   2 May 16 19:36 loop2
brw-rw----  1 root disk      7,   3 May 16 19:36 loop3
brw-rw----  1 root disk      7,   4 May 16 19:36 loop4
brw-rw----  1 root disk      7,   5 May 16 19:36 loop5
brw-rw----  1 root disk      7,   6 May 16 19:36 loop6
brw-rw----  1 root disk      7,   7 May 16 19:36 loop7
drwxr-xr-x  2 root root          60 May 16 19:36 mapper
crw-------  1 root root     10, 227 May 16 19:36 mcelog
crw-r-----  1 root kmem      1,   1 May 16 19:36 mem
drwxrwxrwt  2 root root          40 May 16 19:36 mqueue
drwxr-xr-x  2 root root          60 May 16 19:36 net
crw-rw-rw-  1 root root      1,   3 May 16 19:36 null
crw-------  1 root root     10, 144 May 16 19:36 nvram
crw-r-----  1 root kmem      1,   4 May 16 19:36 port
crw-------  1 root root    108,   0 May 16 19:36 ppp
crw-------  1 root root     10,   1 May 16 19:36 psaux
lrwxrwxrwx  1 root root           8 May 16 19:36 ptmx -> pts/ptmx
crw-------  1 root root    246,   0 May 16 19:36 ptp0
drwxr-xr-x  2 root root           0 May 16 19:36 pts
crw-rw-rw-  1 root root      1,   8 May 16 19:36 random
crw-rw-r--  1 root root     10, 242 May 16 19:36 rfkill
brw-------  1 root root      8,   1 May 16 19:36 root
crw-------  1 root root    248,   0 May 16 19:36 rtc0
brw-rw----  1 root disk      8,   0 May 16 19:36 sda
brw-rw----  1 root disk      8,   1 May 16 19:36 sda1
brw-rw----  1 root disk      8,  14 May 16 19:36 sda14
brw-rw----  1 root disk      8,  15 May 16 19:36 sda15
crw-rw----  1 root disk     21,   0 May 16 19:36 sg0
crw-rw----  1 root cdrom    21,   1 May 16 19:36 sg1
drwxrwxrwt  2 root root          40 May 16 19:36 shm
crw-------  1 root root     10, 231 May 16 19:36 snapshot
drwxr-xr-x  2 root root          80 May 16 19:36 snd
brw-rw----  1 root cdrom    11,   0 May 16 19:36 sr0
lrwxrwxrwx  1 root root          15 May 16 19:36 stderr -> /proc/self/fd/2
lrwxrwxrwx  1 root root          15 May 16 19:36 stdin -> /proc/self/fd/0
lrwxrwxrwx  1 root root          15 May 16 19:36 stdout -> /proc/self/fd/1
crw-rw-rw-  1 root tty       5,   0 May 16 19:36 tty
crw--w----  1 root tty       4,   0 May 16 19:36 tty0
crw--w----  1 root tty       4,   1 May 16 19:36 tty1
crw--w----  1 root tty       4,  10 May 16 19:36 tty10
crw--w----  1 root tty       4,  11 May 16 19:36 tty11
crw--w----  1 root tty       4,  12 May 16 19:36 tty12
crw--w----  1 root tty       4,  13 May 16 19:36 tty13
crw--w----  1 root tty       4,  14 May 16 19:36 tty14
crw--w----  1 root tty       4,  15 May 16 19:36 tty15
crw--w----  1 root tty       4,  16 May 16 19:36 tty16
crw--w----  1 root tty       4,  17 May 16 19:36 tty17
crw--w----  1 root tty       4,  18 May 16 19:36 tty18
crw--w----  1 root tty       4,  19 May 16 19:36 tty19
crw--w----  1 root tty       4,   2 May 16 19:36 tty2
crw--w----  1 root tty       4,  20 May 16 19:36 tty20
crw--w----  1 root tty       4,  21 May 16 19:36 tty21
crw--w----  1 root tty       4,  22 May 16 19:36 tty22
crw--w----  1 root tty       4,  23 May 16 19:36 tty23
crw--w----  1 root tty       4,  24 May 16 19:36 tty24
crw--w----  1 root tty       4,  25 May 16 19:36 tty25
crw--w----  1 root tty       4,  26 May 16 19:36 tty26
crw--w----  1 root tty       4,  27 May 16 19:36 tty27
crw--w----  1 root tty       4,  28 May 16 19:36 tty28
crw--w----  1 root tty       4,  29 May 16 19:36 tty29
crw--w----  1 root tty       4,   3 May 16 19:36 tty3
crw--w----  1 root tty       4,  30 May 16 19:36 tty30
crw--w----  1 root tty       4,  31 May 16 19:36 tty31
crw--w----  1 root tty       4,  32 May 16 19:36 tty32
crw--w----  1 root tty       4,  33 May 16 19:36 tty33
crw--w----  1 root tty       4,  34 May 16 19:36 tty34
crw--w----  1 root tty       4,  35 May 16 19:36 tty35
crw--w----  1 root tty       4,  36 May 16 19:36 tty36
crw--w----  1 root tty       4,  37 May 16 19:36 tty37
crw--w----  1 root tty       4,  38 May 16 19:36 tty38
crw--w----  1 root tty       4,  39 May 16 19:36 tty39
crw--w----  1 root tty       4,   4 May 16 19:36 tty4
crw--w----  1 root tty       4,  40 May 16 19:36 tty40
crw--w----  1 root tty       4,  41 May 16 19:36 tty41
crw--w----  1 root tty       4,  42 May 16 19:36 tty42
crw--w----  1 root tty       4,  43 May 16 19:36 tty43
crw--w----  1 root tty       4,  44 May 16 19:36 tty44
crw--w----  1 root tty       4,  45 May 16 19:36 tty45
crw--w----  1 root tty       4,  46 May 16 19:36 tty46
crw--w----  1 root tty       4,  47 May 16 19:36 tty47
crw--w----  1 root tty       4,  48 May 16 19:36 tty48
crw--w----  1 root tty       4,  49 May 16 19:36 tty49
crw--w----  1 root tty       4,   5 May 16 19:36 tty5
crw--w----  1 root tty       4,  50 May 16 19:36 tty50
crw--w----  1 root tty       4,  51 May 16 19:36 tty51
crw--w----  1 root tty       4,  52 May 16 19:36 tty52
crw--w----  1 root tty       4,  53 May 16 19:36 tty53
crw--w----  1 root tty       4,  54 May 16 19:36 tty54
crw--w----  1 root tty       4,  55 May 16 19:36 tty55
crw--w----  1 root tty       4,  56 May 16 19:36 tty56
crw--w----  1 root tty       4,  57 May 16 19:36 tty57
crw--w----  1 root tty       4,  58 May 16 19:36 tty58
crw--w----  1 root tty       4,  59 May 16 19:36 tty59
crw--w----  1 root tty       4,   6 May 16 19:36 tty6
crw--w----  1 root tty       4,  60 May 16 19:36 tty60
crw--w----  1 root tty       4,  61 May 16 19:36 tty61
crw--w----  1 root tty       4,  62 May 16 19:36 tty62
crw--w----  1 root tty       4,  63 May 16 19:36 tty63
crw--w----  1 root tty       4,   7 May 16 19:36 tty7
crw--w----  1 root tty       4,   8 May 16 19:36 tty8
crw--w----  1 root tty       4,   9 May 16 19:36 tty9
crw--w----  1 root tty       4,  64 May 16 19:36 ttyS0
crw-rw----  1 root dialout   4,  65 May 16 19:36 ttyS1
crw-rw----  1 root dialout   4,  74 May 16 19:36 ttyS10
crw-rw----  1 root dialout   4,  75 May 16 19:36 ttyS11
crw-rw----  1 root dialout   4,  76 May 16 19:36 ttyS12
crw-rw----  1 root dialout   4,  77 May 16 19:36 ttyS13
crw-rw----  1 root dialout   4,  78 May 16 19:36 ttyS14
crw-rw----  1 root dialout   4,  79 May 16 19:36 ttyS15
crw-rw----  1 root dialout   4,  80 May 16 19:36 ttyS16
crw-rw----  1 root dialout   4,  81 May 16 19:36 ttyS17
crw-rw----  1 root dialout   4,  82 May 16 19:36 ttyS18
crw-rw----  1 root dialout   4,  83 May 16 19:36 ttyS19
crw-rw----  1 root dialout   4,  66 May 16 19:36 ttyS2
crw-rw----  1 root dialout   4,  84 May 16 19:36 ttyS20
crw-rw----  1 root dialout   4,  85 May 16 19:36 ttyS21
crw-rw----  1 root dialout   4,  86 May 16 19:36 ttyS22
crw-rw----  1 root dialout   4,  87 May 16 19:36 ttyS23
crw-rw----  1 root dialout   4,  88 May 16 19:36 ttyS24
crw-rw----  1 root dialout   4,  89 May 16 19:36 ttyS25
crw-rw----  1 root dialout   4,  90 May 16 19:36 ttyS26
crw-rw----  1 root dialout   4,  91 May 16 19:36 ttyS27
crw-rw----  1 root dialout   4,  92 May 16 19:36 ttyS28
crw-rw----  1 root dialout   4,  93 May 16 19:36 ttyS29
crw-rw----  1 root dialout   4,  67 May 16 19:36 ttyS3
crw-rw----  1 root dialout   4,  94 May 16 19:36 ttyS30
crw-rw----  1 root dialout   4,  95 May 16 19:36 ttyS31
crw-rw----  1 root dialout   4,  68 May 16 19:36 ttyS4
crw-rw----  1 root dialout   4,  69 May 16 19:36 ttyS5
crw-rw----  1 root dialout   4,  70 May 16 19:36 ttyS6
crw-rw----  1 root dialout   4,  71 May 16 19:36 ttyS7
crw-rw----  1 root dialout   4,  72 May 16 19:36 ttyS8
crw-rw----  1 root dialout   4,  73 May 16 19:36 ttyS9
crw-------  1 root root      5,   3 May 16 19:36 ttyprintk
crw-rw----  1 root     108  10, 125 May 16 19:36 udmabuf
crw-------  1 root root     10, 223 May 16 19:36 uinput
crw-rw-rw-  1 root root      1,   9 May 16 19:36 urandom
crw-rw----  1 root tty       7,   0 May 16 19:36 vcs
crw-rw----  1 root tty       7,   1 May 16 19:36 vcs1
crw-rw----  1 root tty       7,   2 May 16 19:36 vcs2
crw-rw----  1 root tty       7,   3 May 16 19:36 vcs3
crw-rw----  1 root tty       7,   4 May 16 19:36 vcs4
crw-rw----  1 root tty       7,   5 May 16 19:36 vcs5
crw-rw----  1 root tty       7,   6 May 16 19:36 vcs6
crw-rw----  1 root tty       7, 128 May 16 19:36 vcsa
crw-rw----  1 root tty       7, 129 May 16 19:36 vcsa1
crw-rw----  1 root tty       7, 130 May 16 19:36 vcsa2
crw-rw----  1 root tty       7, 131 May 16 19:36 vcsa3
crw-rw----  1 root tty       7, 132 May 16 19:36 vcsa4
crw-rw----  1 root tty       7, 133 May 16 19:36 vcsa5
crw-rw----  1 root tty       7, 134 May 16 19:36 vcsa6
crw-rw----  1 root tty       7,  64 May 16 19:36 vcsu
crw-rw----  1 root tty       7,  65 May 16 19:36 vcsu1
crw-rw----  1 root tty       7,  66 May 16 19:36 vcsu2
crw-rw----  1 root tty       7,  67 May 16 19:36 vcsu3
crw-rw----  1 root tty       7,  68 May 16 19:36 vcsu4
crw-rw----  1 root tty       7,  69 May 16 19:36 vcsu5
crw-rw----  1 root tty       7,  70 May 16 19:36 vcsu6
drwxr-xr-x  2 root root          60 May 16 19:36 vfio
crw-------  1 root root     10, 127 May 16 19:36 vga_arbiter
crw-------  1 root root     10, 238 May 16 19:36 vhost-net
crw-------  1 root root     10, 241 May 16 19:36 vhost-vsock
drwxr-xr-x  2 root root          60 May 16 19:36 vmbus
crw-rw-rw-  1 root root      1,   5 May 16 19:36 zero
crw-------  1 root root     10, 249 May 16 19:36 zfs
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout  --emit llvm-bc=- test.rs 1>/dev/tty 2>/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout/out/llvm-bc || true
diff /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout/out/llvm-bc emit-llvm-bc.stderr
--- stderr -------------------------------
--- stderr -------------------------------
/bin/dash: 1: cannot create /dev/tty: No such device or address
diff: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/emit-to-stdout/emit-to-stdout/out/llvm-bc: No such file or directory
make: *** [Makefile:24: llvm-bc] Error 2



failures:
