plain
.................................................................................................... 9400/11770
.................................................................................................... 9500/11770
...................................................................................................i 9600/11770
......i............................................................................................. 9700/11770
.............................................iiiiiii..iiiiii.i...................................... 9800/11770
.................................................................................................... 10000/11770
.................................................................................................... 10100/11770
.................................................................................................... 10200/11770
.................................................................................................... 10300/11770
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 31 ignored; 0 measured; 0 filtered out; finished in 0.12s

 finished in 0.195 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i...i..i.ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.34s

 finished in 2.410 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; diffs will not be generated

running 437 tests
i........................................i.................FF..F..F................................. 100/437
......................................................................F............................. 200/437
.................................................................................................... 300/437
....F.............................................i.......F.F....................................... 400/437
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [rustdoc] rustdoc/doc-cfg-traits.rs stdout ----
---- [rustdoc] rustdoc/doc-cfg-traits.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-cfg-traits" "/checkout/src/test/rustdoc/doc-cfg-traits.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
5: @count check failed
 Expected 2 occurrences but found 0
 // @count   - '//*[@class="stab portability"]' 2
6: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' '^jurisconsult$'
7: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' '^quarter$'
12: @count check failed
 Expected 6 occurrences but found 0
 // @count   - '//*[@class="stab portability"]' 6
13: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature zibib'
14: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature poriform'
15: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature ethopoeia'
16: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature lea'
17: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature unit'
18: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature quarter'
43: @count check failed
 Expected 7 occurrences but found 0
 // @count   - '//*[@class="stab portability"]' 7
44: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature jurisconsult'
45: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature lithomancy'
46: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature boodle'
47: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature mistetch'
48: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature lea'
49: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature unit'
50: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature quarter'
76: @count check failed
 Expected 8 occurrences but found 0
 // @count   - '//*[@class="stab portability"]' 8
78: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature zibib'
79: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature poriform'
80: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature ethopoeia'
82: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature jurisconsult'
83: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature lithomancy'
84: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature boodle'
85: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature mistetch'
87: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature copy'
100: @count check failed
 Expected 9 occurrences but found 0
 // @count   - '//*[@class="stab portability"]' 9
101: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature quarter'
103: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature zibib'
104: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature poriform'
105: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature ethopoeia'
107: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature jurisconsult'
108: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature lithomancy'
109: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature boodle'
110: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature mistetch'
112: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature copy'
Encountered 37 errors

------------------------------------------



---- [rustdoc] rustdoc/doc-cfg-simplification.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-cfg-simplification" "/checkout/src/test/rustdoc/doc-cfg-simplification.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
5: @count check failed
 Expected 1 occurrences but found 0
 // @count   - '//*[@class="stab portability"]' 1
6: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' '^ratel$'
9: @count check failed
 Expected 8 occurrences but found 0
 // @count   - '//*[@class="stab portability"]' 8
10: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature ratel'
11: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' '^zoonosology$'
12: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' '^yusho$'
13: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' '^nunciative$'
14: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' '^thionic$'
15: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' '^zincic$'
16: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' '^cosmotellurian$'
17: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' '^aposiopesis$'
21: @count check failed
 Expected 1 occurrences but found 0
     // @count   - '//*[@class="stab portability"]' 1
22: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate feature ratel'
26: @count check failed
 Expected 1 occurrences but found 0
     // @count   - '//*[@class="stab portability"]' 1
27: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate features ratel and zoonosology'
32: @count check failed
 Expected 1 occurrences but found 0
     // @count   - '//*[@class="stab portability"]' 1
33: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate feature ratel'
37: @count check failed
 Expected 1 occurrences but found 0
     // @count   - '//*[@class="stab portability"]' 1
38: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate features ratel and yusho'
43: @count check failed
 Expected 1 occurrences but found 0
     // @count   - '//*[@class="stab portability"]' 1
44: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate feature ratel'
48: @count check failed
 Expected 1 occurrences but found 0
     // @count   - '//*[@class="stab portability"]' 1
49: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate features ratel and nunciative'
54: @count check failed
 Expected 1 occurrences but found 0
     // @count   - '//*[@class="stab portability"]' 1
55: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate feature ratel'
59: @count check failed
 Expected 1 occurrences but found 0
     // @count   - '//*[@class="stab portability"]' 1
60: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate features ratel and thionic'
65: @count check failed
 Expected 1 occurrences but found 0
     // @count   - '//*[@class="stab portability"]' 1
66: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate feature ratel'
70: @count check failed
 Expected 2 occurrences but found 0
     // @count   - '//*[@class="stab portability"]' 2
71: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate features ratel and zincic'
72: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate feature rutherford'
82: @count check failed
 Expected 10 occurrences but found 0
     // @count   - '//*[@class="stab portability"]' 10
83: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate features ratel and cosmotellurian'
84: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate feature biotaxy'
85: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate feature xiphopagus'
86: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate feature juxtapositive'
87: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate feature fuero'
88: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate feature palaeophile'
89: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate feature broadcloth'
90: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate features broadcloth and xanthocomic'
91: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate feature broadcloth'
92: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate features broadcloth and whosoever'
137: @count check failed
 Expected 4 occurrences but found 0
     // @count   - '//*[@class="stab portability"]' 4
138: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate feature ratel'
139: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate feature unzymotic'
140: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate feature summate'
141: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate feature unctuous'
160: @count check failed
 Expected 4 occurrences but found 0
     // @count   - '//*[@class="stab portability"]' 4
161: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate features ratel and aposiopesis'
162: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate feature umbracious'
163: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate feature uakari'
164: @matches check failed
 `XPATH PATTERN` did not match
     // @matches - '//*[@class="stab portability"]' 'crate feature rotograph'
Encountered 53 errors

------------------------------------------



---- [rustdoc] rustdoc/doc-cfg.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doc-cfg" "/checkout/src/test/rustdoc/doc-cfg.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
7: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="stab portability"]' 'This is supported on Unix and ARM only.'
9: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@class="stab portability"]' 'This is supported on WASI and WebAssembly only.'
12: @has check failed
 `XPATH PATTERN` did not match
 // @has doc_cfg/unix_only/index.html '//*[@id="main"]/*[@class="item-info"]/*[@class="stab portability"]' 'This is supported on Unix only.'
15: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="module-item"]//*[@class="stab portability"]' '\AARM\Z'
16: @count check failed
 Expected 2 occurrences but found 0
 // @count - '//*[@class="stab portability"]' 2
19: @has check failed
 `XPATH PATTERN` did not match
     // @has doc_cfg/unix_only/fn.unix_only_function.html '//*[@id="main"]/*[@class="item-info"]/*[@class="stab portability"]' 'This is supported on Unix only.'
22: @count check failed
 Expected 1 occurrences but found 0
     // @count - '//*[@class="stab portability"]' 1
27: @has check failed
 `XPATH PATTERN` did not match
     // @has doc_cfg/unix_only/trait.ArmOnly.html '//*[@id="main"]/*[@class="item-info"]/*[@class="stab portability"]' 'This is supported on Unix and ARM only.'
30: @count check failed
 Expected 1 occurrences but found 0
     // @count - '//*[@class="stab portability"]' 1
42: @has check failed
 `XPATH PATTERN` did not match
 // @has doc_cfg/wasi_only/index.html '//*[@id="main"]/*[@class="item-info"]/*[@class="stab portability"]' 'This is supported on WASI only.'
45: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="module-item"]//*[@class="stab portability"]' '\AWebAssembly\Z'
46: @count check failed
 Expected 2 occurrences but found 0
 // @count - '//*[@class="stab portability"]' 2
49: @has check failed
 `XPATH PATTERN` did not match
     // @has doc_cfg/wasi_only/fn.wasi_only_function.html '//*[@id="main"]/*[@class="item-info"]/*[@class="stab portability"]' 'This is supported on WASI only.'
52: @count check failed
 Expected 1 occurrences but found 0
     // @count - '//*[@class="stab portability"]' 1
57: @has check failed
 `XPATH PATTERN` did not match
     // @has doc_cfg/wasi_only/trait.Wasm32Only.html '//*[@id="main"]/*[@class="item-info"]/*[@class="stab portability"]' 'This is supported on WASI and WebAssembly only.'
60: @count check failed
 Expected 1 occurrences but found 0
     // @count - '//*[@class="stab portability"]' 1
88: @has check failed
 `XPATH PATTERN` did not match
 // @has - '//*[@id="main"]/*[@class="item-info"]/*[@class="stab portability"]' 'This is supported with target feature avx only.'
Encountered 17 errors

------------------------------------------



---- [rustdoc] rustdoc/duplicate-cfg.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/duplicate-cfg" "/checkout/src/test/rustdoc/duplicate-cfg.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
5: @matches check failed
 `XPATH PATTERN` did not match
 // @matches '-' '//*[@class="module-item"]//*[@class="stab portability"]' '^sync$'
6: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="module-item"]//*[@class="stab portability"]/@title' 'This is supported on crate feature `sync` only'
9: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="stab portability"]' 'sync'
15: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="stab portability"]' 'This is supported on crate feature sync only.'
19: @has check failed
 `XPATH PATTERN` did not match
     // @has '-' '//*[@class="stab portability"]' 'This is supported on crate feature sync only.'
25: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="stab portability"]' 'This is supported on crate features sync and send only.'
29: @has check failed
 `XPATH PATTERN` did not match
     // @has '-' '//*[@class="stab portability"]' 'This is supported on crate features sync and send only.'
35: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="stab portability"]' 'This is supported on crate feature sync only.'
39: @has check failed
 `XPATH PATTERN` did not match
     // @has '-' '//*[@class="stab portability"]' 'This is supported on crate features sync and send only.'
45: @has check failed
 `XPATH PATTERN` did not match
 // @has '-' '//*[@class="stab portability"]' 'This is supported on crate feature sync and crate feature send and foo only.'
49: @has check failed
 `XPATH PATTERN` did not match
     // @has '-' '//*[@class="stab portability"]' 'This is supported on crate feature sync and crate feature send and foo and bar only.'
Encountered 11 errors

------------------------------------------



---- [rustdoc] rustdoc/invalid-doc-cfg.rs stdout ----
error: rustdoc failed!
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/invalid-doc-cfg/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/invalid-doc-cfg" "/checkout/src/test/rustdoc/invalid-doc-cfg.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected item after attributes
 --> /checkout/src/test/rustdoc/invalid-doc-cfg.rs:1:1
  |
1 | #[doc(cfg(x, y))] //~ ERROR

error: aborting due to previous error



------------------------------------------


---- [rustdoc] rustdoc/issue-79201.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-79201" "/checkout/src/test/rustdoc/issue-79201.rs"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
4: @count check failed
 Expected 6 occurrences but found 0
 // @count   - '//*[@class="stab portability"]' 6
5: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature foo-root'
6: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature foo-public-mod'
7: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature foo-private-mod'
8: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature foo-fn'
9: @matches check failed
 `XPATH PATTERN` did not match
 // @matches - '//*[@class="stab portability"]' 'crate feature foo-method'
Encountered 6 errors

------------------------------------------



---- [rustdoc] rustdoc/reexport-stability-tags-deprecated-and-portability.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/reexport-stability-tags-deprecated-and-portability" "/checkout/src/test/rustdoc/reexport-stability-tags-deprecated-and-portability.rs"
------------------------------------------

------------------------------------------
---
test result: FAILED. 426 passed; 8 failed; 3 ignored; 0 measured; 0 filtered out; finished in 37.98s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:17:04
