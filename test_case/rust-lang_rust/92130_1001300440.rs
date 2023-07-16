plain
test [run-make] run-make/rustdoc-scrape-examples-ordering ... ok
test [run-make] run-make/rustdoc-scrape-examples-multiple ... ok
test [run-make] run-make/emit-shared-files ... ok
test [run-make] run-make/thumb-none-qemu ... ok
test [run-make] run-make/thumb-none-cortex-m has been running for over 60 seconds
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=thumbv6m-none-eabi

failures:


---- [run-make] run-make/thumb-none-cortex-m stdout ----
error: make failed
status: exit status: 2
command: "make"
stdout:
stdout:
------------------------------------------
env
SRC=/checkout
CC_armv7a_none_eabihf=arm-none-eabi-gcc
DOC_RUST_LANG_ORG_CHANNEL=https://doc.rust-lang.org/nightly
MAIL=/var/mail/user
LLVM_FILECHECK=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck
CFLAGS_arm_unknown_linux_musleabihf=-march=armv6 -marm -mfpu=vfp
RUST_BUILD_STAGE=stage2-thumbv6m-none-eabi
SCRIPT=python3 ../x.py --stage 2 test --host= --target thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf src/test/run-make &&       python3 ../x.py dist --host= --target asmjs-unknown-emscripten,wasm32-unknown-emscripten,mips-unknown-linux-musl,mipsel-unknown-linux-musl,mips64-unknown-linux-muslabi64,mips64el-unknown-linux-muslabi64,arm-unknown-linux-musleabi,arm-unknown-linux-musleabihf,armv5te-unknown-linux-gnueabi,armv5te-unknown-linux-musleabi,armv7-unknown-linux-musleabihf,aarch64-unknown-none,aarch64-unknown-none-softfloat,sparc64-unknown-linux-gnu,x86_64-unknown-redox,thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf,thumbv8m.base-none-eabi,thumbv8m.main-none-eabi,thumbv8m.main-none-eabihf,riscv32i-unknown-none-elf,riscv32imc-unknown-none-elf,riscv32imac-unknown-none-elf,riscv64imac-unknown-none-elf,riscv64gc-unknown-none-elf,armebv7r-none-eabi,armebv7r-none-eabihf,armv7r-none-eabi,armv7r-none-eabihf,thumbv7neon-unknown-linux-gnueabihf,armv7a-none-eabi
CI=true
CI=true
RUST_CONFIGURE_ARGS=--musl-root-armv5te=/musl-armv5te       --musl-root-arm=/musl-arm       --musl-root-armhf=/musl-armhf       --musl-root-armv7hf=/musl-armv7hf       --musl-root-mips=/musl-mips       --musl-root-mipsel=/musl-mipsel       --musl-root-mips64=/musl-mips64       --musl-root-mips64el=/musl-mips64el       --disable-docs --set build.print-step-timings --enable-verbose-tests --enable-sccache --disable-manage-submodules --enable-locked-deps --enable-cargo-native-static --set rust.codegen-units-std=1 --dist-compression-formats=xz --disable-dist-src --release-channel=nightly --enable-llvm-static-stdcpp --set rust.remap-debuginfo --debuginfo-level-std=1 --enable-missing-tools
CC_mips64_unknown_linux_muslabi64=mips64-linux-gnuabi64-gcc
HOST_RPATH_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib
HOSTNAME=9b064d347092
TARGET=thumbv6m-none-eabi
---
CC_mipsel_unknown_linux_musl=mipsel-openwrt-linux-gcc
BOOTSTRAP_PARENT_ID=1963
CFLAGS_armv7a_none_eabi=-march=armv7-a
RUSTC_BOOTSTRAP=1
LD_LIB_PATH_ENVVAR=LD_LIBRARY_PATH
TMPDIR=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m
CC_x86_64_unknown_redox=x86_64-unknown-redox-gcc
_=/usr/bin/sh
CXX_thumbv7neon_unknown_linux_gnueabihf=arm-linux-gnueabihf-g++
LLVM_BIN_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin
LLVM_BIN_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin
AR=arm-none-eabi-ar
CC_mips64el_unknown_linux_muslabi64=mips64el-linux-gnuabi64-gcc
TERM=xterm
SCCACHE_BUCKET=rust-lang-ci-sccache2
BASE_COMMIT=f8abed9ed48bace6be0087bcd44ed534e239b8d8
CFLAGS_armv7_unknown_linux_musleabihf=-march=armv7-a
PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
TARGET_RPATH_DIR=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib
LLVM_COMPONENTS=aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray
RUSTFLAGS=--cap-lints=warn
MAKELEVEL=1
MIRRORS_BASE=https://ci-mirrors.rust-lang.org/rustc
CC_thumbv7neon_unknown_linux_gnueabihf=arm-linux-gnueabihf-gcc
CFLAGS_armv7a_none_eabihf=-march=armv7-a+vfpv3
CFLAGS_armv7a_none_eabihf=-march=armv7-a+vfpv3
TOOLSTATE_PUBLISH=1
__COMPAT_LAYER=RunAsInvoker
STAGING_DIR=/tmp
COMPILETEST_NEEDS_ALL_LLVM_COMPONENTS=1
COMPILETEST_NEEDS_ALL_LLVM_COMPONENTS=1
RUSTDOC=LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc' -L /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib -Clinker='arm-none-eabi-gcc'
RUST_TEST_THREADS=16
CFLAGS_armv5te_unknown_linux_musleabi=-march=armv5te -marm -mfloat-abi=soft
DEPLOY=1
DEPLOY=1
CXX=arm-none-eabi-g++ -ffunction-sections -fdata-sections -mthumb -march=armv6s-m
AWS_ACCESS_KEY_ID=AKIA46X5W6CZI5DHEBFL
AWS_SECRET_ACCESS_KEY=***
TOOLSTATE_REPO_ACCESS_TOKEN=***
BOOTSTRAP_CONFIG=config.toml
BOOTSTRAP_CONFIG=config.toml
CC_aarch64_unknown_none_softfloat=aarch64-none-elf-gcc
CFLAGS_arm_unknown_linux_musleabi=-march=armv6 -marm
GITHUB_REF=refs/heads/auto
BOOTSTRAP_CARGO=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo
***
CFLAGS_aarch64_unknown_none=-mstrict-align
CFLAGS_aarch64_unknown_none=-mstrict-align
BOOTSTRAP_PYTHON=/usr/bin/python3
CARGO_HOME=/cargo
RUN_MAKE_TARGETS=thumbv6m-none-eabi,thumbv7m-none-eabi,thumbv7em-none-eabi,thumbv7em-none-eabihf
RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc
RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc
CC=arm-none-eabi-gcc -ffunction-sections -fdata-sections -mthumb -march=armv6s-m
CC_mips_unknown_linux_musl=mips-openwrt-linux-gcc
CI_JOB_NAME=dist-various-1
RUSTC_LINKER=arm-none-eabi-gcc
mkdir -p /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m
mkdir -p /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m
cd /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m && rm -rf cortex-m
cd /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-cortex-m/thumb-none-cortex-m && bash -x /checkout/src/test/run-make/thumb-none-cortex-m/../git_clone_sha1.sh cortex-m https://github.com/rust-embedded/cortex-m a448e9156e2cb1e556e5441fd65426952ef4b927 

------------------------------------------
stderr:
------------------------------------------
------------------------------------------
+ PROJECT_NAME=cortex-m
+ URL=https://github.com/rust-embedded/cortex-m
+ SHA1=a448e9156e2cb1e556e5441fd65426952ef4b927
+ git clone https://github.com/rust-embedded/cortex-m cortex-m
Cloning into 'cortex-m'...
fatal: the remote end hung up unexpectedly
fatal: early EOF
fatal: index-pack failed
+ err_exit
+ err_exit
+ echo ERROR:
+ exit 1
make: *** [Makefile:34: all] Error 1
------------------------------------------




failures:
    [run-make] run-make/thumb-none-cortex-m

test result: FAILED. 22 passed; 1 failed; 25 ignored; 0 measured; 0 filtered out; finished in 409.59s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/thumbv6m-none-eabi/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-thumbv6m-none-eabi" "--suite" "run-make" "--mode" "run-make" "--target" "thumbv6m-none-eabi" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "arm-none-eabi-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/thumbv6m-none-eabi/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--llvm-version" "13.0.0-rust-1.59.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--llvm-bin-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin" "--cc" "arm-none-eabi-gcc" "--cxx" "arm-none-eabi-g++" "--cflags" "-ffunction-sections -fdata-sections -mthumb -march=armv6s-m" "--ar" "arm-none-eabi-ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:22:26
