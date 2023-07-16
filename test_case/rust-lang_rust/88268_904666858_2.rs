ignore (to-prevent-tidy-error)
   |     ^^^
   = note: error from rustc: character literal may only contain one codepoint
!!!! []
!!!! []
!!!! []
!!!! []
---
To only update this specific test, also pass `--test-args intra-doc/double-anchor.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/double-anchor.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/double-anchor" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/double-anchor/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `with#anchor#error` contains multiple anchors
   |
   |
LL | /// docs [label][with#anchor#error]
   |                             |
   |                             invalid anchor
   |
   = note: `#[warn(rustdoc::broken_intra_doc_links)]` on by default
---
---- [ui] rustdoc-ui/intra-doc/private.rs#public stdout ----
diff of stderr:

23    |
24    = note: this link will resolve properly if you pass `--document-private-items`
+ !!!! []
+ !!!! []
+ !!!! []
+ !!!! []
+ !!!! []
+ !!!! []
26 warning: 3 warnings emitted
27 
28 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/private.public/private.public.stderr
To only update this specific test, also pass `--test-args intra-doc/private.rs`


error in revision `public`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/private.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "public" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/private.public" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/private.public/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: public documentation for `DocMe` links to private item `DontDocMe`
   |
   |
LL | /// docs [DontDocMe] [DontDocMe::f] [DontDocMe::x]
   |
   = note: `#[warn(rustdoc::private_intra_doc_links)]` on by default
   = note: `#[warn(rustdoc::private_intra_doc_links)]` on by default
   = note: this link will resolve properly if you pass `--document-private-items`

warning: public documentation for `DocMe` links to private item `DontDocMe::f`
   |
   |
LL | /// docs [DontDocMe] [DontDocMe::f] [DontDocMe::x]
   |
   |
   = note: this link will resolve properly if you pass `--document-private-items`

warning: public documentation for `DocMe` links to private item `DontDocMe::x`
   |
   |
LL | /// docs [DontDocMe] [DontDocMe::f] [DontDocMe::x]
   |
   |
   = note: this link will resolve properly if you pass `--document-private-items`
!!!! []
!!!! []
!!!! []
!!!! []
---
---- [ui] rustdoc-ui/intra-doc/private.rs#private stdout ----
diff of stderr:

23    |
24    = note: this link resolves only because you passed `--document-private-items`, but will break without
+ !!!! []
+ !!!! []
+ !!!! []
+ !!!! []
---
28 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/private.private/private.private.stderr
To only update this specific test, also pass `--test-args intra-doc/private.rs`


error in revision `private`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/private.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "private" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/private.private" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--document-private-items" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/private.private/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: public documentation for `DocMe` links to private item `DontDocMe`
   |
   |
LL | /// docs [DontDocMe] [DontDocMe::f] [DontDocMe::x]
   |
   = note: `#[warn(rustdoc::private_intra_doc_links)]` on by default
   = note: `#[warn(rustdoc::private_intra_doc_links)]` on by default
   = note: this link resolves only because you passed `--document-private-items`, but will break without

warning: public documentation for `DocMe` links to private item `DontDocMe::f`
   |
   |
LL | /// docs [DontDocMe] [DontDocMe::f] [DontDocMe::x]
   |
   |
   = note: this link resolves only because you passed `--document-private-items`, but will break without

warning: public documentation for `DocMe` links to private item `DontDocMe::x`
   |
   |
LL | /// docs [DontDocMe] [DontDocMe::f] [DontDocMe::x]
   |
   |
   = note: this link resolves only because you passed `--document-private-items`, but will break without
!!!! []
!!!! []
!!!! []
!!!! []
---
---- [ui] rustdoc-ui/intra-doc/warning-crlf.rs stdout ----
diff of stderr:

31    |
32    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
+ !!!! []
+ !!!! []
+ !!!! []
+ !!!! []
---
36 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/warning-crlf/warning-crlf.stderr
To only update this specific test, also pass `--test-args intra-doc/warning-crlf.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/warning-crlf.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/warning-crlf" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/warning-crlf/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unresolved link to `error`
  --> /checkout/src/test/rustdoc-ui/intra-doc/warning-crlf.rs:7:6
   |
LL | /// [error]
   |      ^^^^^ no item named `error` in scope
   = note: `#[warn(rustdoc::broken_intra_doc_links)]` on by default
   = note: `#[warn(rustdoc::broken_intra_doc_links)]` on by default
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `error1`
  --> /checkout/src/test/rustdoc-ui/intra-doc/warning-crlf.rs:12:11
   |
   |
LL | /// docs [error1]
   |           ^^^^^^ no item named `error1` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `error2`
  --> /checkout/src/test/rustdoc-ui/intra-doc/warning-crlf.rs:15:11
   |
   |
LL | /// docs [error2]
   |           ^^^^^^ no item named `error2` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `error`
  --> /checkout/src/test/rustdoc-ui/intra-doc/warning-crlf.rs:23:20
   |
   |
LL |  * It also has an [error].
   |                    ^^^^^ no item named `error` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
!!!! []
!!!! []
!!!! []
!!!! []
---
---- [ui] rustdoc-ui/intra-doc/warning.rs stdout ----
diff of stderr:

171    |
172    = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
+ !!!! []
+ !!!! []
+ !!!! []
+ !!!! []
---
To only update this specific test, also pass `--test-args intra-doc/warning.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/warning.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/warning" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/warning/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unresolved link to `Foo::baz`
   |
   |
LL |        //! Test with [Foo::baz], [Bar::foo], ...
   |                       ^^^^^^^^ the struct `Foo` has no field or associated item named `baz`
   = note: `#[warn(rustdoc::broken_intra_doc_links)]` on by default


warning: unresolved link to `Bar::foo`
   |
   |
LL |        //! Test with [Foo::baz], [Bar::foo], ...
   |                                   ^^^^^^^^ no item named `Bar` in scope

warning: unresolved link to `Uniooon::X`
   |
   |
LL |      //! , [Uniooon::X] and [Qux::Z].
   |             ^^^^^^^^^^ no item named `Uniooon` in scope

warning: unresolved link to `Qux::Z`
   |
   |
LL |      //! , [Uniooon::X] and [Qux::Z].
   |                              ^^^^^^ no item named `Qux` in scope

warning: unresolved link to `Uniooon::X`
   |
   |
LL |       //! , [Uniooon::X] and [Qux::Z].
   |              ^^^^^^^^^^ no item named `Uniooon` in scope

warning: unresolved link to `Qux::Z`
   |
   |
LL |       //! , [Uniooon::X] and [Qux::Z].
   |                               ^^^^^^ no item named `Qux` in scope

warning: unresolved link to `Qux:Y`
   |
   |
LL |        /// [Qux:Y]
   |             ^^^^^ no item named `Qux:Y` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

warning: unresolved link to `BarA`
   |
   |
LL | /// bar [BarA] bar //~ WARNING `BarA`
   |          ^^^^ no item named `BarA` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

warning: unresolved link to `BarB`
   |
   |
LL |  * bar [BarB] bar //~ WARNING `BarB`
   |         ^^^^ no item named `BarB` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

warning: unresolved link to `BarC`
   |
   |
LL | bar [BarC] bar //~ WARNING `BarC`
   |      ^^^^ no item named `BarC` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

warning: unresolved link to `BarD`
   |
   |
LL | #[doc = "Foo\nbar [BarD] bar\nbaz"] //~ WARNING `BarD`
   |
   = note: the link appears in this line:
           
           
           bar [BarD] bar
                ^^^^
   = note: no item named `BarD` in scope
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

warning: unresolved link to `BarF`
   |
   |
LL |         #[doc = $f] //~ WARNING `BarF`
...
...
LL | f!("Foo\nbar [BarF] bar\nbaz");
   |
   = note: the link appears in this line:
           
           
           bar [BarF] bar
                ^^^^
   = note: no item named `BarF` in scope
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
   = note: this warning originates in the macro `f` (in Nightly builds, run with -Z macro-backtrace for more info)
warning: unresolved link to `error`
  --> /checkout/src/test/rustdoc-ui/intra-doc/warning.rs:58:30
   |
   |
LL |  * time to introduce a link [error]*/ //~ WARNING `error`
   |                              ^^^^^ no item named `error` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `error`
  --> /checkout/src/test/rustdoc-ui/intra-doc/warning.rs:64:30
   |
   |
LL |  * time to introduce a link [error] //~ WARNING `error`
   |                              ^^^^^ no item named `error` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `error`
  --> /checkout/src/test/rustdoc-ui/intra-doc/warning.rs:68:1
   |
   |
LL | #[doc = "single line [error]"] //~ WARNING `error`
   |
   = note: the link appears in this line:
           
           
           single line [error]
   = note: no item named `error` in scope
   = note: no item named `error` in scope
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `error`
  --> /checkout/src/test/rustdoc-ui/intra-doc/warning.rs:71:1
   |
   |
LL | #[doc = "single line with \"escaping\" [error]"] //~ WARNING `error`
   |
   = note: the link appears in this line:
           
           
           single line with "escaping" [error]
   = note: no item named `error` in scope
   = note: no item named `error` in scope
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `error`
  --> /checkout/src/test/rustdoc-ui/intra-doc/warning.rs:74:1
   |
   |
LL | / /// Item docs. //~ WARNING `error`
LL | | #[doc="Hello there!"]
LL | | /// [error]
   |
   = note: the link appears in this line:
           
           [error]
           [error]
            ^^^^^
   = note: no item named `error` in scope
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `error1`
  --> /checkout/src/test/rustdoc-ui/intra-doc/warning.rs:80:11
   |
   |
LL | /// docs [error1] //~ WARNING `error1`
   |           ^^^^^^ no item named `error1` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
warning: unresolved link to `error2`
  --> /checkout/src/test/rustdoc-ui/intra-doc/warning.rs:82:11
   |
   |
LL | /// docs [error2] //~ WARNING `error2`
   |           ^^^^^^ no item named `error2` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
!!!! []
!!!! []
!!!! []
!!!! []
---

---- [ui] rustdoc-ui/issue-74134.rs#private stdout ----
diff of stderr:

7    = note: `#[warn(rustdoc::private_intra_doc_links)]` on by default
8    = note: this link resolves only because you passed `--document-private-items`, but will break without
+ !!!! []
+ !!!! []
+ !!!! []
+ !!!! []
---
12 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-74134.private/issue-74134.private.stderr
To only update this specific test, also pass `--test-args issue-74134.rs`


error in revision `private`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/issue-74134.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "private" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-74134.private" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--document-private-items" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-74134.private/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: public documentation for `public_item` links to private item `PrivateType`
   |
   |
LL |     /// [`PrivateType`]
   |
   = note: `#[warn(rustdoc::private_intra_doc_links)]` on by default
   = note: `#[warn(rustdoc::private_intra_doc_links)]` on by default
   = note: this link resolves only because you passed `--document-private-items`, but will break without
!!!! []
!!!! []
!!!! []
!!!! []
---

---- [ui] rustdoc-ui/issue-74134.rs#public stdout ----
diff of stderr:

7    = note: `#[warn(rustdoc::private_intra_doc_links)]` on by default
8    = note: this link will resolve properly if you pass `--document-private-items`
+ !!!! []
+ !!!! []
+ !!!! []
+ !!!! []
---
12 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-74134.public/issue-74134.public.stderr
To only update this specific test, also pass `--test-args issue-74134.rs`


error in revision `public`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/issue-74134.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "public" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-74134.public" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/issue-74134.public/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: public documentation for `public_item` links to private item `PrivateType`
   |
   |
LL |     /// [`PrivateType`]
   |
   = note: `#[warn(rustdoc::private_intra_doc_links)]` on by default
   = note: `#[warn(rustdoc::private_intra_doc_links)]` on by default
   = note: this link will resolve properly if you pass `--document-private-items`
!!!! []
!!!! []
!!!! []
!!!! []
---
test result: FAILED. 115 passed; 14 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.96s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc-ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:24:09
