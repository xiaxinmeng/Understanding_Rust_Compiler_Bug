plain
configure: rust.channel         := nightly
configure: rust.debug-assertions := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    Finished release [optimized] target(s) in 28.56s
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> wasm32-unknown-emscripten)

running 11837 tests
iiiFiFFFFFFiFFFFFFF.FFi.iiiFFFFFFFF.F.FF..FF..iiF.F..FFF..F.FFFF.....iii.....iFFFFFFFF.F.FF.FiFFF.FF 100/11837
FF.ii.F..FF.FF.FiFFFFF.FFFFi..FF.FFF.iiiiiF.iii.iiiiiiiii.FF.FFF......F....FF.FFFF.FF.F..FFFF..FFF.F 200/11837
FFFFF.....F............F..FF..FF....F.F.....F........F...FFFF.F....F.FF.FF.F..F..FFFF....FF.F.F.FFFF 300/11837
F..F..FFFF.FF.FF..FFF...F..F...F...FFF.FFFFFFFFF.FF..FF.F.F..F.....FF.........F.......F....F........ 400/11837
.F..F................................................................F....F.ii.....F.....F...FF..... 500/11837
F..FF........FF..F.F..F..F.....F...........................FF......F.F..F.FF..Fiii.........F..i..... 600/11837
......F..F....F.FF...FF...F.....iiF........................F.FFFF..............FF.F....F..FF...FF..F 700/11837
FFFFFFiiiiFF.F.FFF.FFFFF..FFFFFFFFFFFFFFF.FFFFFF..FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF 800/11837
FFFFFFFFFFFFFFFFF.FiFFF..iFFF..FF..F.F...F....F.F...................F..i..F.F....F.......F.F.......F 900/11837
................F....F..F....F.............FF......F....ii.F...............F.F....F...F...........F. 1000/11837
.................F........F...........F.........F..F..FFFF..F........F.............F.......F...F..FF 1100/11837
.....F.............i.........................F....FF.....F..F....................FF.F.F.F.FF....FF.. 1200/11837
.FFF..F....F...F...F.F...F..F.FFFFFFFiFFFFFFFFFFF.FFFFF..F.F.F.F..............FFi....F..iFiFFFF..... 1300/11837
FF....F.....FFFF.F......FF.........F........................F......F.FFFFFF..FF..FF.F..F.F...F...FF. 1400/11837
....F.i.....F...................FF..Fiiii.iiFi.F...F.FF.F..i...............F.F.F.FFFFF.FFF.........F 1500/11837
..FF....FFF.............FF...F...........FF....F.......F...FF......F.....F.......F......FF.F........ 1600/11837
....iiiiiii..........................................i....................F...F...F....F.FF......FFF 1700/11837
FFFFF.F..FFFFFF.....FF.F.F...F....F...FFF..F.F.....F.FF....FFF.F..FFFF.F.....FF.FF.FF.F.F.FF.FFF.F.. 1800/11837
F.F...FF.FFF......F...FF.......F....FF...F.......FF............FF............F......F....iF.F.....F. 1900/11837
....F.......FF...FFFF....F.............F......F..FF...F......F...F..........F....................... 2000/11837
........F..F..F..F.................FF.F...F........FF..F.......FF..F.F....F.F.FFF...FF.F...FF..FFFFF 2100/11837
F.FFF.F...FF....F......F....F..............FFFFFFFF.FF.FFFF.....F..FFF.FFF.FFFFFFFFFFFFFFFFFF..F...F 2200/11837
......FFF.i.F....FF.F.........F.F.......................F.....F........F..F.....FF.............Fii.. 2300/11837
.F........F.FF....F..F....FFFF.F...FFF.FFFF..FFFFFF.F.F.F...F..........FFFFF.FFF....F...FFFFFFF...F. 2400/11837
....FFFFF.F.F.FFFFF...F..FF...F..FFFF.F......F...FF.F.F....FFF....F.....FFF....F..F.i..FF.F...FF..F. 2500/11837
F.......FF..FFFF.....FF...F.FFF.F.FFi.i.....F.F.F...F.FFFF.F...Fi....FFFF.F.....F..F.F............Fi 2600/11837
.F......F...F......F.....FFFFFFFFFF.....FF...FFF...FF......F...FFFFFF.FF......i.FF...F.i.FFFF..F..FF 2700/11837
FFFFFFFi.Fi....FF.FF....FFFiiF..FFFFF...FF..F..F..F...F.......F.F.....FF..F.FF......FF...F.......... 2800/11837
.F............F........F.F........FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF.....FF..FF.F.F.F..F.......FF..... 2900/11837
.F..F..........F...iiiii...F.....F............i.F.....FFFFFFFFFF..F........F.FF..F..FF....FF..F..F.. 3000/11837
.F.F....F.............F.FFFFFFFFFFFFFFFFFFF...F.FF..F...F...F.....F.F....F..F..F......FF...F..F..... 3100/11837
F..F....F.FFiiF.....F...F.F..F....F.F........................F.F...........F........................ 3200/11837
......................F............................................................................. 3300/11837
.......................................................iiii.........................i............... 3400/11837
.......FFFFFFFFFFFF.F..Fi.i.i.iFF.F..FF...F.F........F.....FF......F....F.FFFFFFF.FFFF..FF....Fiiiii 3500/11837
iiiiiiiiiiF....F.iFFF.F.i..FF.....Fii..F...F...FF.........................F......................... 3600/11837
..................................F................................................................. 3700/11837
........F.Fi..............F.F.FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF..FFF.....FF.Fi..FF.F.FF...FFFFF 3800/11837
FFFFFFFFFFFFFFFFFFFFFFFFFFFF...F.F.FFFFF.Fi....F.F.F..F...FFF.F.F..FF..iFF..F.F..FF..FFi.FFii..F.FFF 3900/11837
.FFF...F..FFFF.FF.......FFFF...F.F.......F......F.............FF...F........F....F.................. 4000/11837
.....F.FF......FFFFFFFFFFFF...FFFF.FF...FFF..FF.F.F...F..i..i.......F..FFFFFFFFFF.FFFF..FFF......... 4100/11837
............F.F.....F..............F...............F..................F...F.......FFF..F..F.F...F.FF 4200/11837
FFFFFFFFF......FF..F...F....FF.F.F....F....FFF..F.........................FF........F............F.. 4300/11837
.........F...FF.FFFFF........FFFF..F.F..F..F....F.F..FFFFFFFiF..FF.F...F........F.FF.FFF.F.FFF.FF... 4400/11837
....F.......FF.F...FF...FF..F............F................F...................FFF.ii................ 4500/11837
....FF.......ii....FFF....F.FFFFFF.F...F..FFF.F.i....F....F.....FF...F....F.F........i..F.F..FFFF.F. 4600/11837
FF....FF.FFFFF.F..FF..F.FFF.FF....F......F..F.i.F.F....FFF.FF.F.Fi..FFFF.i....iFFF..FFF........FF.F. 4700/11837
F....F.F...FFF.F.F...F.FFi.F..FF.F..FF.F.F.FiF..F.FFF....FF..F.FFF.FFF.F.FF.FFFF...FFFi..FF.F..i.... 4800/11837
FFFFFFFF..FFFFFF.F.F.FF.F...F..FFF.FF.F...FF.......FFF..FF....F.FF..........F....F.FF..FF.FF......F. 4900/11837
F..F.....Fi..F.FFF.FiF...F.F.FF.FF..F.F...F...F...F...F.F..FF..F.F.FFF.......F.......i.F...FFF..F... 5000/11837
.F...F.....F.F.FFF.FF..F...Fi..F.FF.FFi....F.F..FF.F..F....F....FiF......F..FFF.F.FFF...F..F..F..F.F 5100/11837
.F.F..F.i...F.F.F...F.F..F.F..........FF..F..iiFFFiF.F..FFFFF.FF.FFFFFFFFFF..F.FFFFFi..FF...F.i....F 5200/11837
F.F.iFFFFF..FFFFiF..F.FFFF.FF.F..FFF.FFFFF.F...F.F....FF...FF.FF..F..F.i.F.FF....i.Fi.F.F.F..FF....F 5300/11837
F.FF.F...FFFF.i.F..iF..F...FFFF....FF..F.F......F...F.FiFFi.F...FF.....FF..FFF...FFiiFF...F..F..i..F 5400/11837
.F..i.F.F..F..F.F.Fi....i.F..i.F...FF.F.F.FF......F.FF.F.F.F.F.F....FF.F..FFF.....F...F.F......FF.FF 5500/11837
..FF.FFiF.iF...F..FF.F.F.......FF.F.F.F...FF.FF.F....F.F..F.F...FFFFF..F...F..FF.FF..FF.FF.F..FFF.FF 5600/11837
.F...FFiFi.F..FF..FF.F..F...Fi...FFF....F.FFFi...F.F...FFFi...FFFF..F.F...F.F.FFF..FFF.......F.F..F. 5700/11837
.F..FFF...F..F......F.F..F....F.FF....FF..FF.F.FF..FFF..F...F..FFF..F...FFF.....FFF.......FF.i..iiii 5800/11837
..F.i.F...FF..FFFF.F.F.....FFF...F.......F.F....F.FFF.....F.F.F......FF...F.FFF........FFF.Fi...F... 5900/11837
.F.......F..F.FFF.......F.FF.....F...FF.......FF..F...FF......F.F..F.FFF....FF......F...F..F..F.F.F. 6000/11837
.FFi...FF...F.F...F....F.FF........FFF...F...F.F.F..FFFF..F.F.F....F..F.......FF.F.F.F.i..F...F.Fi.. 6100/11837
.F.F...............F..FF.F.FF.F.i......F...F...FFF...F.....F....F..F..ii......F.....FF..........F... 6200/11837
....F.FF....FF.F.......F.......F...F...F...FFF..FF...F.........i..F..F.F..i.F.FFF.FFF.F...FiFFF...FF 6300/11837
FFFFFi.i.FF.FFFFF.FFFF.FF.FF.F.F.F...FFFFFFFFFF.F.iF..F.F......FF...............F..FF...F..........F 6400/11837
.F....FFFF.FF....F..F.F.F.F........F.F.................FF....................i....i..iii.F.F......i. 6500/11837
................i....F.......FF.F..........i......FF..F..F.......F........F............F............ 6600/11837
..........F................F.................................F.............i.......F................ 6700/11837
...........F..F.................i....F..............iiii.i.iiiiiii...iiiii......i.iF.F...FFFF..F.F.. 6800/11837
F.i..F..........iii..Fiiiiiii.FFF.........F.iii.F.FF....F.F..F...FF.......F.Fii..FF....F......F..... 6900/11837
.F.F.FFF.FFFFF.....FFFF.FFiiFF..F..FF.FFFF.F...FFFF.F.F.F...F.FFFF.FF...FF.FF...F.FFF.F..FF..FF.FF.. 7000/11837
FFFFFFi...iFFF...F.F....F.F.FFF.FF.FFFiFiiiiFF...FF...F.F..........F...F.ii.......ii......F...F..... 7100/11837
..i.F.....F.ii.F.FF.............F..FFF..FFF.FFFFFFFFFF.FF.FFF.F..FFF.FF.FFFF.F..FF.....F.FFFF.F.F... 7200/11837
.FFFF.F.FF.FFFFFFFFFFFFFFFFFFFFFiiiiFFFFFF.FFFiiiiFFFiiiiF.FFFFFFFFFFFFFFF....F.........FFFFFF...... 7300/11837
..F.........ii.................i.i..ii.FFFFFFFFFFF.F...F...F.F.......FiF.FF..F.i.iF......FF.FF...... 7400/11837
..F..F.FF..FFF.FF..F.F....FF.........F...FiF.F..FFFFFFFFFFFF.FFF..FFF.F.F..............F.....F.F.... 7500/11837
........F..........F.......F.........F.........i........................F..F.F..F....FF.......F....F 7600/11837
..............F................F.................................F.................................. 7700/11837
.............................................i.F.........F..i..i..ii.F.F..F.F..F....Fi.....FFFFFFiFF 7800/11837
FFFFFFiiiiFFi.FFFiiiiFF...FiFFi.........FFFFF...F.F.FFFF..F........F.F.................FF...FF...... 7900/11837
..F.F......F...F..FFFFFFFFF..F..F.......F...F.F...........i.F.FFF..FFFFFFFFFFFFFFFFFFFFF.iiiF.FFFFFF 8000/11837
FFFFF...F..F.Fiiii.F..ii..FFiiiii.F.FiFiiiiiiii.iiiiiiiiiiiiiiiiiiiiiiiii.Fii...F.......FF.......... 8100/11837
..................F.......i..F......F..................................................F............ 8200/11837
..............................................................i..................................... 8300/11837
................................F.................................................F............F.... 8400/11837
......................i..............................F............................................FF 8500/11837
.........F..........................F.....................................................i......... 8600/11837
.......F......F.......F.FFF.........FFF....FFF..........F..F.F.........................F..F......... 8700/11837
.........................FF................F....F......F.....FFF.Fi...F..FFFFFFF.FFFF..FF.F.F...FF.. 8800/11837
....FF..................F.....FF....F..FFFF...F......FFFFFFF..F.F.FFFFF......iF..........F.F...F.FFF 8900/11837
F.FF...F.FFF...FFF.Fiiii.iiiii............FF.........F.....F..F....F.F....F............i....FFii.... 9000/11837
....FF.F.F.i.F.F......F....F.F.........iiiiiiiiiiiF.FF...F.................F..F.....F.F........FFF.. 9100/11837
..F...F.FFF............F.F........F.....F.......F...F..........F........F..F.F.F.....F..F......FF..F 9200/11837
...FFFF..F.FF..F..F..F...F...FFFF.FFFF..F.F..F.FF..............F..F...FF...FF...F.....FFFFF.F.F.F.F. 9300/11837
F.FF.........F.FF.........F......F..FF.F....F.FF.....FFFF..F..F....F....F......F..F......F.......... 9400/11837
.........F.......F........F....................................F.........................F...FFFFF.. 9500/11837
F.FFFFFF..iiii......FFF..F..........F.FF.F.......F..........F....F.F......FFFFFFFFFFFFFF.......F..FF 9600/11837
..F..F..F...F..F.........F.......F...F.....F.......F.i......i......F...........F..F........FFFFFFFFF 9700/11837
FF.FFFFFFFFFFFiiiiiiiFFFFFFFFF.FF.FFi...F....F..F.F.............F.F.F.F..F........FF...............i 9800/11837
iiiiii..iiiiii.i........................F..FFFF..F.F.FF...FFFF......F...F...........F..........FFF.F 9900/11837
FFF...FF..F.FF.FF.FFi.F.FFFF.FF..iiiF..F.iF.FFii....iiiiiiiiiii.iF.........FF.F.F...........FF..FFF. 10000/11837
..........FF.....F...........F.......FF.F........................................................... 10100/11837
.....................FF....F..F..F..F.......F......F..F.....FFFFFFFFFFF.FF..F...........F...F.F..F.. 10200/11837
.................FFi.iiF..FFFFiFF..F..F..FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFiFFFFFFFFF 10300/11837
FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFiFFFFiF.FFF..F.F..........F.F...F...F...FF. 10400/11837
..............F........................................F............................................ 10500/11837
......................................................FF.....F...........iiii......F..i.F........i.. 10600/11837
....FFFFF.iiii.iii.FFFiiFFFFiiF.i.iFF..FiiiiiiFFFFFiiiiiFiiiiiFFFiiFiiiiiFFiiiii.F......FFF..FF...FF 10700/11837
....FF....F...FF.F.F.....FF.F.F...F...FiF.F.F.FFF..FF..FFF...F....FFFFFF.FFFFFFFF.FFF.FFF.....FF.FF. 10800/11837
...FF..F.FFFFFFFFFFFFFFFFFFFFFF.FFFFFFFFFFFFF..FFF..FFFF..........FFF..F..F.FF...F...........F...FFF 10900/11837
.FF.F...F....FFF.F.FF...F.FFFF..FF.F...F.F....F...F.F.FF....F....FFF..F.FFF...F.......F..FFFF..F..iF 11000/11837
.FFF...FF..F...F...FFFF..F.......F.......F....F.F..F..............FF..F..F....F............F......FF 11100/11837
F........................F.............iiF..F.......................................F..F............ 11200/11837
.........FF...FF..FF.F.F..FFF....F................F...................FFF......F.....F...FF......... 11300/11837
......................FF.....F.F..FFF.FFFFFFF..FF.FFF..F...FFFF.F.FFFFFFF..FF.FFF.FFFF.F......F...F. 11400/11837
..FFF........F.....F.....FF.F.....FF.FFF.FFF.F..F.F....F.F.F...F.FFF.F.FF...FF.FF..FFFFFFFFFFFFFFFFF 11500/11837
FFFFFFFFFFFFFFFiFFF.F..F.F.FFF..F..FF.F..FF.F.........F..F...F...F.FFF..F.F.......F..............F.F 11600/11837
FFFF..F...........F.FF..F....................ii........F.FFF....FFFF...F.F....F..i.....F............ 11700/11837
...F....................i...F..i...F.FF..FF.....FF.......F......F.F......F.........F................ 11800/11837
...F.FFF.FFFFii.FFFF....FFFFFFFFFFFFF

---- [ui] ui/abi/anon-extern-mod.rs stdout ----


error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/anon-extern-mod.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/anon-extern-mod/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/anon-extern-mod/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/anon-extern-mod/a.anon_extern_mod.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/anon-extern-mod/a.3kazdqopfnzbjaux.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/anon-extern-mod/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "rust_test_helpers" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/anon-extern-mod/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/abi/c-stack-as-value.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/c-stack-as-value.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/c-stack-as-value/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/c-stack-as-value/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/c-stack-as-value/a.c_stack_as_value.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/c-stack-as-value/a.56rjqkulkqb4r925.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/c-stack-as-value/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "rust_test_helpers" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/c-stack-as-value/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/abi/cabi-int-widening.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/cabi-int-widening.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/cabi-int-widening/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/cabi-int-widening/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/cabi-int-widening/a.cabi_int_widening.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/cabi-int-widening/a.4vhf6uiifohppse0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/cabi-int-widening/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "rust_test_helpers" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/cabi-int-widening/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/abi/extern/extern-call-direct.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/extern/extern-call-direct.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-call-direct/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-call-direct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-call-direct/a.extern_call_direct.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-call-direct/a.2ixv6uibry0h6td0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-call-direct/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-call-direct/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/abi/abi-sysv64-register-usage.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/abi-sysv64-register-usage.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/abi-sysv64-register-usage/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/abi-sysv64-register-usage/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/abi-sysv64-register-usage/a.abi_sysv64_register_usage.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/abi-sysv64-register-usage/a.1l7ms05cu4qje2na.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/abi-sysv64-register-usage/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/abi-sysv64-register-usage/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/abi/extern/extern-pass-double.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/extern/extern-pass-double.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-double/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-double/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-double/a.extern_pass_double.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-double/a.201o3ijt9aftijqr.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-double/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "rust_test_helpers" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-double/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/abi/extern/extern-pass-TwoU16s.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/extern/extern-pass-TwoU16s.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU16s/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU16s/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU16s/a.extern_pass_TwoU16s.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU16s/a.18hp4sgvscyvdp4d.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU16s/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "rust_test_helpers" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU16s/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/abi/extern/extern-call-indirect.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/extern/extern-call-indirect.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-call-indirect/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-call-indirect/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-call-indirect/a.extern_call_indirect.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-call-indirect/a.1ybwo33dozutw10t.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-call-indirect/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "rust_test_helpers" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-call-indirect/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/abi/extern/extern-pass-char.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/extern/extern-pass-char.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-char/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-char/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-char/a.extern_pass_char.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-char/a.dcwa1ju0q5nnc7v.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-char/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "rust_test_helpers" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-char/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/abi/extern/extern-pass-TwoU32s.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/extern/extern-pass-TwoU32s.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU32s/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU32s/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU32s/a.extern_pass_TwoU32s.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU32s/a.30b0pm9qm1x41dsb.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU32s/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "rust_test_helpers" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU32s/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/abi/abi-sysv64-arg-passing.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/abi-sysv64-arg-passing.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/abi-sysv64-arg-passing/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/abi-sysv64-arg-passing/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/abi-sysv64-arg-passing/a.abi_sysv64_arg_passing.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/abi-sysv64-arg-passing/a.5fq38qfg6v0howf8.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/abi-sysv64-arg-passing/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/abi-sysv64-arg-passing/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/abi/extern/extern-pass-TwoU8s.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/extern/extern-pass-TwoU8s.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU8s/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU8s/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU8s/a.extern_pass_TwoU8s.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU8s/a.25227y4tf49fjcpd.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU8s/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "rust_test_helpers" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU8s/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/abi/cross-crate/anon-extern-mod-cross-crate-2.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/cross-crate/anon-extern-mod-cross-crate-2.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/cross-crate/anon-extern-mod-cross-crate-2/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/cross-crate/anon-extern-mod-cross-crate-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/cross-crate/anon-extern-mod-cross-crate-2/a.anon_extern_mod_cross_crate_2.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/cross-crate/anon-extern-mod-cross-crate-2/a.9vbhihyx15wh56w.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/cross-crate/anon-extern-mod-cross-crate-2/auxiliary/libanonexternmod.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/cross-crate/anon-extern-mod-cross-crate-2/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/cross-crate/anon-extern-mod-cross-crate-2/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/abi/extern/extern-pass-TwoU64s.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/extern/extern-pass-TwoU64s.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU64s/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU64s/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU64s/a.extern_pass_TwoU64s.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU64s/a.4adpbfhsvmzo3pbx.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU64s/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "rust_test_helpers" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-TwoU64s/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/abi/duplicated-external-mods.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/duplicated-external-mods.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/duplicated-external-mods/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/duplicated-external-mods/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/duplicated-external-mods/a.duplicated_external_mods.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/duplicated-external-mods/a.2ls3y8kjeiuj3eej.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/duplicated-external-mods/auxiliary/libanonexternmod.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/duplicated-external-mods/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/duplicated-external-mods/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/abi/extern/extern-crosscrate.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/extern/extern-crosscrate.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-crosscrate/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-crosscrate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-crosscrate/a.extern_crosscrate.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-crosscrate/a.519crnftny0l5yqj.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-crosscrate/auxiliary/libexterncallback.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-crosscrate/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-crosscrate/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/abi/extern/extern-pass-u32.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/extern/extern-pass-u32.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-u32/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/extern/extern-pass-u32/auxiliary"
------------------------------------------

---


---- [ui] ui/consts/const-eval/enum_discr.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/enum_discr.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/enum_discr/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/enum_discr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/enum_discr/a.enum_discr.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/enum_discr/a.r4hpjf3um5hru5u.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/enum_discr/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/enum_discr/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/consts/const-eval/const_transmute.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_transmute.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_transmute/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_transmute/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_transmute/a.const_transmute.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_transmute/a.o0rdmwnmq9o13gc.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_transmute/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_transmute/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/consts/const-eval/heap/alloc_intrinsic_transient.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/heap/alloc_intrinsic_transient.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_transient/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_transient/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_transient/a.alloc_intrinsic_transient.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_transient/a.558qlfarj0wllixz.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_transient/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_transient/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/consts/const-eval/heap/alloc_intrinsic_nontransient.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/heap/alloc_intrinsic_nontransient.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_nontransient/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_nontransient/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_nontransient/a.alloc_intrinsic_nontransient.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_nontransient/a.9haqaz2ma64im8g.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_nontransient/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/heap/alloc_intrinsic_nontransient/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/consts/const-eval/issue-64970.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-64970.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-64970/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-64970/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-64970/a.issue_64970.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-64970/a.ylazk0vas24l76.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-64970/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-64970/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/consts/const-eval/issue-64908.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-64908.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-64908/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-64908/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-64908/a.issue_64908.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-64908/a.1h6ud6pc6fzjxhna.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-64908/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-64908/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/consts/const-eval/promoted_errors.rs#noopt stdout ----

error in revision `noopt`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/promoted_errors.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--cfg" "noopt" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.noopt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-C" "opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.noopt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: any use of this value will cause an error
   |
LL |       0 - 1
   |       ^^^^^
   |       |
   |       |
   |       attempt to compute `0_u32 - 1_u32`, which would overflow
   |       inside `overflow` at /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:13:5
   |       inside `X` at /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:33:29
...
LL | / const X: () = {
LL | |     let _x: &'static u32 = &overflow();
LL | |     //[opt_with_overflow_checks,noopt]~^ WARN any use of this value will cause an error
LL | |     //[opt_with_overflow_checks,noopt]~| WARN this was previously accepted by the compiler
...  |
LL | |     let _x: &'static i32 = &oob();
LL | | };
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:9:9
   |
   |
LL | #![warn(const_err, arithmetic_overflow, unconditional_panic)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


warning: any use of this value will cause an error
   |
   |
LL | / const X: () = {
LL | |     let _x: &'static u32 = &overflow();
   | |                            ^^^^^^^^^^^ referenced constant has errors
LL | |     //[opt_with_overflow_checks,noopt]~^ WARN any use of this value will cause an error
LL | |     //[opt_with_overflow_checks,noopt]~| WARN this was previously accepted by the compiler
...  |
LL | |     let _x: &'static i32 = &oob();
LL | | };
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.noopt/promoted_errors.promoted_errors.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.noopt/promoted_errors.yqutxs91otg8fcf.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.noopt/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.noopt/promoted_errors.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O0" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error; 2 warnings emitted



------------------------------------------


---- [ui] ui/consts/const-eval/nrvo.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/nrvo.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/nrvo/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/nrvo/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/nrvo/a.nrvo.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/nrvo/a.sedigq9xezzfwtz.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/nrvo/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/nrvo/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/consts/const-eval/promoted_errors.rs#opt stdout ----

error in revision `opt`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/promoted_errors.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--cfg" "opt" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.opt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.opt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: any use of this value will cause an error
   |
LL |       1 / 0
   |       ^^^^^
   |       |
   |       |
   |       attempt to divide `1_i32` by zero
   |       inside `div_by_zero1` at /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:18:5
   |       inside `X` at /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:36:29
...
LL | / const X: () = {
LL | |     let _x: &'static u32 = &overflow();
LL | |     //[opt_with_overflow_checks,noopt]~^ WARN any use of this value will cause an error
LL | |     //[opt_with_overflow_checks,noopt]~| WARN this was previously accepted by the compiler
...  |
LL | |     let _x: &'static i32 = &oob();
LL | | };
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:9:9
   |
   |
LL | #![warn(const_err, arithmetic_overflow, unconditional_panic)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


warning: any use of this value will cause an error
   |
   |
LL | / const X: () = {
LL | |     let _x: &'static u32 = &overflow();
LL | |     //[opt_with_overflow_checks,noopt]~^ WARN any use of this value will cause an error
LL | |     //[opt_with_overflow_checks,noopt]~| WARN this was previously accepted by the compiler
LL | |     let _x: &'static i32 = &div_by_zero1();
   | |                            ^^^^^^^^^^^^^^^ referenced constant has errors
...  |
LL | |     let _x: &'static i32 = &oob();
LL | | };
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.opt/promoted_errors.promoted_errors.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.opt/promoted_errors.yqutxs91otg8fcf.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.opt/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.opt/promoted_errors.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error; 2 warnings emitted



------------------------------------------


---- [ui] ui/consts/const-eval/promoted_errors.rs#opt_with_overflow_checks stdout ----

error in revision `opt_with_overflow_checks`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/promoted_errors.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--cfg" "opt_with_overflow_checks" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.opt_with_overflow_checks" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-C" "overflow-checks=on" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.opt_with_overflow_checks/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: any use of this value will cause an error
   |
LL |       0 - 1
   |       ^^^^^
   |       |
   |       |
   |       attempt to compute `0_u32 - 1_u32`, which would overflow
   |       inside `overflow` at /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:13:5
   |       inside `X` at /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:33:29
...
LL | / const X: () = {
LL | |     let _x: &'static u32 = &overflow();
LL | |     //[opt_with_overflow_checks,noopt]~^ WARN any use of this value will cause an error
LL | |     //[opt_with_overflow_checks,noopt]~| WARN this was previously accepted by the compiler
...  |
LL | |     let _x: &'static i32 = &oob();
LL | | };
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:9:9
   |
   |
LL | #![warn(const_err, arithmetic_overflow, unconditional_panic)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


warning: any use of this value will cause an error
   |
   |
LL | / const X: () = {
LL | |     let _x: &'static u32 = &overflow();
   | |                            ^^^^^^^^^^^ referenced constant has errors
LL | |     //[opt_with_overflow_checks,noopt]~^ WARN any use of this value will cause an error
LL | |     //[opt_with_overflow_checks,noopt]~| WARN this was previously accepted by the compiler
...  |
LL | |     let _x: &'static i32 = &oob();
LL | | };
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.opt_with_overflow_checks/promoted_errors.promoted_errors.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.opt_with_overflow_checks/promoted_errors.yqutxs91otg8fcf.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.opt_with_overflow_checks/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.opt_with_overflow_checks/promoted_errors.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error; 2 warnings emitted



------------------------------------------


---- [ui] ui/consts/const-eval/simd/insert_extract.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/simd/insert_extract.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/simd/insert_extract/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/simd/insert_extract/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/simd/insert_extract/a.insert_extract.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/simd/insert_extract/a.ivefvtqf95z7ykv.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/simd/insert_extract/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/simd/insert_extract/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/consts/const-eval/write-to-uninhabited-enum-variant.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/write-to-uninhabited-enum-variant.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/write-to-uninhabited-enum-variant/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/write-to-uninhabited-enum-variant/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/write-to-uninhabited-enum-variant/a.write_to_uninhabited_enum_variant.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/write-to-uninhabited-enum-variant/a.3xhuv3989nxuqwth.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/write-to-uninhabited-enum-variant/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/write-to-uninhabited-enum-variant/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/consts/const-eval/strlen.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/strlen.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/strlen/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/strlen/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/strlen/a.strlen.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/strlen/a.2m5pb35moupbffcz.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/strlen/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/strlen/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/consts/const-expr-in-fixed-length-vec.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-expr-in-fixed-length-vec.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-expr-in-fixed-length-vec/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-expr-in-fixed-length-vec/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-expr-in-fixed-length-vec/a.const_expr_in_fixed_length_vec.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-expr-in-fixed-length-vec/a.39evz2yjzzvxmeof.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-expr-in-fixed-length-vec/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-expr-in-fixed-length-vec/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/consts/const-expr-addr-operator.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-expr-addr-operator.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-expr-addr-operator" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-expr-addr-operator/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-expr-addr-operator/const-expr-addr-operator.const_expr_addr_operator.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-expr-addr-operator/const-expr-addr-operator.3x69u2lb0zynevw9.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-expr-addr-operator/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-expr-addr-operator/const-expr-addr-operator.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/consts/const-expr-in-vec-repeat.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-expr-in-vec-repeat.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-expr-in-vec-repeat/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-expr-in-vec-repeat/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-expr-in-vec-repeat/a.const_expr_in_vec_repeat.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-expr-in-vec-repeat/a.pb252fn6gi8d3ka.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-expr-in-vec-repeat/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-expr-in-vec-repeat/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---


---- [ui] ui/rfc-2091-track-caller/caller-location-intrinsic.rs#default stdout ----

error in revision `default`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/caller-location-intrinsic.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--cfg" "default" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/caller-location-intrinsic.default/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/caller-location-intrinsic.default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/caller-location-intrinsic.default/a.caller_location_intrinsic.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/caller-location-intrinsic.default/a.2n6opkjckaso369a.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/caller-location-intrinsic.default/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/caller-location-intrinsic.default/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/rfc-2091-track-caller/caller-location-intrinsic.rs#mir-opt stdout ----

error in revision `mir-opt`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/caller-location-intrinsic.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--cfg" "mir_opt" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/caller-location-intrinsic.mir-opt/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-Zmir-opt-level=4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/caller-location-intrinsic.mir-opt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/caller-location-intrinsic.mir-opt/a.caller_location_intrinsic.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/caller-location-intrinsic.mir-opt/a.2n6opkjckaso369a.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/caller-location-intrinsic.mir-opt/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/caller-location-intrinsic.mir-opt/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/rfc-2091-track-caller/const-caller-location.rs#mir-opt stdout ----

error in revision `mir-opt`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/const-caller-location.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--cfg" "mir_opt" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/const-caller-location.mir-opt/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-Zmir-opt-level=4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/const-caller-location.mir-opt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/const-caller-location.mir-opt/a.const_caller_location.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/const-caller-location.mir-opt/a.3rixkt2a95asba1f.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/const-caller-location.mir-opt/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/const-caller-location.mir-opt/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/rfc-2091-track-caller/const-caller-location.rs#default stdout ----

error in revision `default`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/const-caller-location.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--cfg" "default" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/const-caller-location.default/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/const-caller-location.default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/const-caller-location.default/a.const_caller_location.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/const-caller-location.default/a.3rixkt2a95asba1f.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/const-caller-location.default/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/const-caller-location.default/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/rfc-2091-track-caller/diverging-caller-location.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/diverging-caller-location.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/diverging-caller-location/a.js" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/diverging-caller-location/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/diverging-caller-location/a.diverging_caller_location.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/diverging-caller-location/a.3vfltjqxiaxd8txv.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/diverging-caller-location/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/diverging-caller-location/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/rfc-2091-track-caller/pass.rs#default stdout ----

error in revision `default`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/pass.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--cfg" "default" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/pass.default/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/pass.default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/pass.default/a.pass.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/pass.default/a.1tq93dbmsst9knvd.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/pass.default/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/pass.default/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/rfc-2091-track-caller/intrinsic-wrapper.rs#mir-opt stdout ----

error in revision `mir-opt`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/intrinsic-wrapper.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--cfg" "mir_opt" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/intrinsic-wrapper.mir-opt/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-Zmir-opt-level=4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/intrinsic-wrapper.mir-opt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/intrinsic-wrapper.mir-opt/a.intrinsic_wrapper.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/intrinsic-wrapper.mir-opt/a.1r81euzfirofubsz.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/intrinsic-wrapper.mir-opt/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/intrinsic-wrapper.mir-opt/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/rfc-2091-track-caller/intrinsic-wrapper.rs#default stdout ----

error in revision `default`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/intrinsic-wrapper.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--cfg" "default" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/intrinsic-wrapper.default/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/intrinsic-wrapper.default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/intrinsic-wrapper.default/a.intrinsic_wrapper.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/intrinsic-wrapper.default/a.1r81euzfirofubsz.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/intrinsic-wrapper.default/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/intrinsic-wrapper.default/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/rfc-2091-track-caller/pass.rs#mir-opt stdout ----

error in revision `mir-opt`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/pass.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--cfg" "mir_opt" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/pass.mir-opt/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-Zmir-opt-level=4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/pass.mir-opt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/pass.mir-opt/a.pass.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/pass.mir-opt/a.1tq93dbmsst9knvd.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/pass.mir-opt/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/pass.mir-opt/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/rfc-2027-object-safe-for-dispatch/manual-self-impl-for-unsafe-obj.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2027-object-safe-for-dispatch/manual-self-impl-for-unsafe-obj.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2027-object-safe-for-dispatch/manual-self-impl-for-unsafe-obj/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2027-object-safe-for-dispatch/manual-self-impl-for-unsafe-obj/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2027-object-safe-for-dispatch/manual-self-impl-for-unsafe-obj/a.manual_self_impl_for_unsafe_obj.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2027-object-safe-for-dispatch/manual-self-impl-for-unsafe-obj/a.1vo7visb1ach6i2s.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2027-object-safe-for-dispatch/manual-self-impl-for-unsafe-obj/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2027-object-safe-for-dispatch/manual-self-impl-for-unsafe-obj/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/rfc-2091-track-caller/track-caller-ffi.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/track-caller-ffi.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/track-caller-ffi/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/track-caller-ffi/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_track_caller_ffi_test_tracked\",\"_rust_track_caller_ffi_test_untracked\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/track-caller-ffi/a.track_caller_ffi.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/track-caller-ffi/a.3r4fv89d66ssqkjw.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/track-caller-ffi/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/track-caller-ffi/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/rfc-2091-track-caller/track-caller-attribute.rs#mir-opt stdout ----

error in revision `mir-opt`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/track-caller-attribute.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--cfg" "mir_opt" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/track-caller-attribute.mir-opt/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-Zmir-opt-level=4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/track-caller-attribute.mir-opt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/track-caller-attribute.mir-opt/a.track_caller_attribute.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/track-caller-attribute.mir-opt/a.m2cttco2ctkqp74.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/track-caller-attribute.mir-opt/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/track-caller-attribute.mir-opt/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/rfc-2091-track-caller/track-caller-attribute.rs#default stdout ----

error in revision `default`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/track-caller-attribute.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--cfg" "default" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/track-caller-attribute.default/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/track-caller-attribute.default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/track-caller-attribute.default/a.track_caller_attribute.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/track-caller-attribute.default/a.m2cttco2ctkqp74.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/track-caller-attribute.default/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/track-caller-attribute.default/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/rfc-2091-track-caller/tracked-fn-ptr-with-arg.rs#default stdout ----

error in revision `default`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/tracked-fn-ptr-with-arg.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--cfg" "default" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr-with-arg.default/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr-with-arg.default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr-with-arg.default/a.tracked_fn_ptr_with_arg.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr-with-arg.default/a.4u3pjv936dum2xkn.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr-with-arg.default/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr-with-arg.default/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/rfc-2091-track-caller/tracked-fn-ptr-with-arg.rs#mir-opt stdout ----

error in revision `mir-opt`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/tracked-fn-ptr-with-arg.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--cfg" "mir_opt" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr-with-arg.mir-opt/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-Zmir-opt-level=4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr-with-arg.mir-opt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr-with-arg.mir-opt/a.tracked_fn_ptr_with_arg.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr-with-arg.mir-opt/a.4u3pjv936dum2xkn.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr-with-arg.mir-opt/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr-with-arg.mir-opt/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/rfc-2091-track-caller/tracked-fn-ptr.rs#mir-opt stdout ----

error in revision `mir-opt`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/tracked-fn-ptr.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--cfg" "mir_opt" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr.mir-opt/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-Zmir-opt-level=4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr.mir-opt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr.mir-opt/a.tracked_fn_ptr.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr.mir-opt/a.24tjxhdzmdgj004q.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr.mir-opt/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr.mir-opt/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/rfc-2091-track-caller/tracked-fn-ptr.rs#default stdout ----

error in revision `default`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/tracked-fn-ptr.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--cfg" "default" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr.default/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr.default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr.default/a.tracked_fn_ptr.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr.default/a.24tjxhdzmdgj004q.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr.default/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-fn-ptr.default/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/rfc-2091-track-caller/tracked-trait-obj.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/tracked-trait-obj.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-trait-obj/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-trait-obj/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-trait-obj/a.tracked_trait_obj.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-trait-obj/a.1azscyrxwbipjk8g.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-trait-obj/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-trait-obj/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/rfc-2091-track-caller/tracked-trait-impls.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2091-track-caller/tracked-trait-impls.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-trait-impls/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-trait-impls/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-trait-impls/a.tracked_trait_impls.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-trait-impls/a.4ukhowd359zog10.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-trait-impls/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/tracked-trait-impls/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/repr/repr-no-niche.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/repr/repr-no-niche.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/repr-no-niche/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/repr-no-niche/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/repr-no-niche/a.repr_no_niche.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/repr-no-niche/a.23kr846e3v6xmsn0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/repr-no-niche/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repr/repr-no-niche/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/rfc-0107-bind-by-move-pattern-guards/bind-by-move-no-guards.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-0107-bind-by-move-pattern-guards/bind-by-move-no-guards.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-0107-bind-by-move-pattern-guards/bind-by-move-no-guards/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-0107-bind-by-move-pattern-guards/bind-by-move-no-guards/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-0107-bind-by-move-pattern-guards/bind-by-move-no-guards/a.bind_by_move_no_guards.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-0107-bind-by-move-pattern-guards/bind-by-move-no-guards/a.2t998z058e8q2smo.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-0107-bind-by-move-pattern-guards/bind-by-move-no-guards/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-0107-bind-by-move-pattern-guards/bind-by-move-no-guards/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)

error: aborting due to previous error



------------------------------------------


---- [ui] ui/rfc-2093-infer-outlives/privacy.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2093-infer-outlives/privacy.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/privacy/a.js" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/privacy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\",\"_rust_eh_personality\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/privacy/a.privacy.7rcbfp3g-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/privacy/a.n3wge5qqugm3mcy.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-53cc6bb299d58d8e.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-0cd812f46f74eb20.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-daa00a42abbbc36d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-401bf781cc1196ee.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-29d3ba9853ac18a0.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-49e775b9dd83f3a7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-fafb55636bf5fb8b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-ac4c80204a37e493.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-3664d9e68cfd7c8d.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-f7cce79cc0c524d2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-c004ac4a3060e29a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-4a8579e38c7df750.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-031f3285cf1d1f57.rlib" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/privacy/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-l" "c" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/privacy/a.js" "-s" "DISABLE_EXCEPTION_CATCHING=0" "--fallback-arch" "generic" "-O2" "--memory-init-file" "0" "-g0" "-s" "DEFAULT_LIBRARY_FUNCS_TO_INCLUDE=[]" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-s" "ERROR_ON_UNDEFINED_SYMBOLS=1" "-s" "ASSERTIONS=1" "-s" "ABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: emcc: error: generic: No such file or directory ("generic" was expected to be an input file, based on the commandline arguments provided)
---
test result: FAILED. 7778 passed; 3558 failed; 501 ignored; 0 measured; 0 filtered out; finished in 114.40s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-emscripten" "--suite" "ui" "--mode" "ui" "--target" "wasm32-unknown-emscripten" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/emsdk-portable/node/latest/bin/node" "--npm" "/emsdk-portable/node/latest/bin/npm" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.1-rust-1.54.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 --host= --target wasm32-unknown-emscripten --exclude library/alloc
Build completed unsuccessfully in 0:20:27
