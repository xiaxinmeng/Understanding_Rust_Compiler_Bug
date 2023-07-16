plain
.................................................................................................... 9300/11531
.................................................................................................... 9400/11531
.......................................................................i......i..................... 9500/11531
.................................................................................................... 9600/11531
..........iiiiiii..iiiiii.i......................................................................... 9700/11531
.................................................................................................... 9900/11531
.................................................................................................... 10000/11531
.................................................................................................... 10100/11531
.................................................................................................... 10200/11531
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.051 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 1.85s

Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
 finished in 1.907 seconds
---
Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
   Compiling once_cell v1.4.1
   Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
    Finished release [optimized] target(s) in 1.38s
std/arch/index.html:154: broken link - std/arch/x86/index.html
std/arch/index.html:156: broken link - std/arch/arm/index.html
std/arch/index.html:157: broken link - std/arch/aarch64/index.html
std/arch/index.html:158: broken link - std/arch/mips/index.html
std/arch/index.html:159: broken link - std/arch/mips64/index.html
std/arch/index.html:160: broken link - std/arch/powerpc/index.html
std/arch/index.html:161: broken link - std/arch/powerpc64/index.html
std/arch/index.html:162: broken link - std/arch/nvptx/index.html
std/arch/index.html:163: broken link - std/arch/wasm32/index.html
thread 'main' panicked at 'found some broken links', src/tools/linkchecker/main.rs:102:9


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/linkchecker" "/checkout/obj/build/x86_64-unknown-linux-gnu/doc"
expected success, got: exit code: 101
