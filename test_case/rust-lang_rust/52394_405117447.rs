plain
travis_time:start:test_run-make-fulldeps
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:18:01] 
[01:18:01] running 187 tests
[01:18:31] ..............................................F.....................................................
[01:19:25] ......................................................................................test [run-make] run-make-fulldeps/long-linker-command-lines has been running for over 60 seconds
[01:20:07] failures:
[01:20:07] 
[01:20:07] ---- [run-make] run-make-fulldeps/hir-tree stdout ----
[01:20:07] 
[01:20:07] 
[01:20:07] error: make failed
[01:20:07] status: exit code: 2
[01:20:07] command: "make"
[01:20:07] stdout:
[01:20:07] ------------------------------------------
[01:20:07] make[1]: Entering directory '/checkout/src/test/run-make-fulldeps/hir-tree'
[01:20:07] LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hir-tree/hir-tree:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib:" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hir-tree/hir-tree -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hir-tree/hir-tree  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hir-tree/hir-tree/input.hir -Z unpretty=hir-tree input.rs
[01:20:07] "/checkout/src/etc/cat-and-grep.sh" '"Hello, Rustaceans!\n"' < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/hir-tree/hir-tree/input.hir
[01:20:07] [[[ begin stdout ]]]
[01:20:07]     module: Mod {
[01:20:07]     module: Mod {
[01:20:07]         inner: Span {
[01:20:07]             lo: BytePos(
[01:20:07]             ),
[01:20:07]             ),
[01:20:07]             hi: BytePos(
[01:20:07]             ),
[01:20:07]             ),
[01:20:07]             ctxt: #0
[01           name: std,
[01:20:07]             id: NodeId(
[01:20:07]                 3
[01:20:07]             ),
[01:20:07]             ),
[01:20:07]             hir_id: HirId {
[01:20:07]                 owner: DefIndex(0:2),
[01:20:07]                 local_id: ItemLocalId(
[01:20:07]                 )
[01:20:07]             },
[01:20:07]             attrs: [
[01:20:07]                 Attribute {
[01:20:07]                 Attribute {
[01:20:07]                     id: AttrId(
[01:20:07]                         0
[01:20:07]                     ),
[01:20:07]                     style: Outer,
[01:20:07]                     path: path(macro_use),
[01:20:07]                     tokens: TokenStream {
[01:20:07]                         kind: Empty
[01:20:07]                     },
[01:20:07]                     is_sugared_doc: false,
[01:20:07]                     span: Span {
[01:20:07]                         lo: BytePos(
[01:20:07]                         ),
[01:20:07]                         ),
[01:20:07]                         hi: BytePos(
[01:20:07]                         ),
[01:20:07]                         ),
[01:20:07]                         ctxt: #0
[01:20:07]                 }
[01:20:07]             ],
[01:20:07]             ],
[01:20:07]             node: ItemExternCrate(
[01:20:07]             ),
[01:20:07]             ),
[01:20:07]             vis: Spanned {
[01:20:07]                 node: Inherited,
[01:20:07]                 span: Span {
[01:20:07]                     lo: BytePos(
[01:20:07]                     ),
[01:20:07]                          variadic: false,
[01:20:07]                          variadic: false,
[01:20:07]                     has_implicit_self: false
[01:20:07]                 },
[01:20:07]                 FnHeader {
[01:20:07]                     unsafety: Normal,
[01:20:07]                     constness: NotConst,
[01:20:07]                     asyncness: NotAsync,
[01:20:07]                     abi: Rust
[01:20:07]                 Generics {
[01:20:07]                     params: [],
[01:20:07]                     params: [],
[01:20:07]                     where_clause: WhereClause {
[01:20:07]                         id: NodeId(
[01:20:07]                         ),
[01:20:07]                         predicates: []
[01:20:07]                     },
[01:20:07]                     },
[01:20:07]                     span: Span {
[01:20:07]                         lo: BytePos(
[01:20:07]                         ),
[01:20:07]                         ),
[01:20:07]                         hi: BytePos(
[01:20:07]                         ),
[01:20:07]                         ),
[01:20:07]                         ctxt: #0
[01:20:07]                 },
[01:20:07]                 BodyId {
[01:20:07]                     node_id: NodeId(
[01:20:07]                         62
[01:20:07]                         62
[01:20:07]                     )
[01:20:07]                 }
[01:20:07]             ),
[01:20:07]             vis: Spanned {
[01:20:07]                 node: Inherited,
[01:20:07]                 span: Span {
[01:20:07]                     lo: BytePos(
[01:20:07]                     ),
[01:20:07]                     ),
[01:20:07]                     hi: ByteP                                                [],
[01:20:07]                                                                                                        }),)
[01:20:07]                                                                        {
[01:20:07]                                                                        (arg0,) =>
[01:20:07]                                                                        [<::fmt::ArgumentV1>::new(arg0,
[01:20:07]                                                                                                  ::fmt::Display::fmt)],
[01:20:07]                                                                    },
[01:20:07]                                                                   &[::fmt::rt::v1::Argument{position:
[01:20:07]                                                                                                 ::fmt::rt::v1::Position::At(0usize),
[01:20:07]                                                                                             format:
[01:20:07]                                                                                                 ::fmt::rt::v1::FormatSpec{fill:
[01:20:07]                                                                                                                               ' ',
[01:20:07]                                                                                                                           align:
[01:20:07]                                                                                                                               ::fmt::rt::v1::Alignment::Unknown,
[01:yld scalaropts selectiondag sparc sparcasmparser sparcasmprinter sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzasmprinter systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target transformutils vectorize x86 x86asmparser x86asmprinter x86codegen x86desc x86disassembler x86info x86utils xcore xcoreasmprinter xcorecodegen xcoredesc xcoredisassembler xcoreinfo" "--llvm-cxxflags" "-I/usr/lib/llvm-5.0/include -std=c++0x -fuse-ld=gold -Wl,--no-keep-files-mapped -Wl,--no-map-whole-files -fPIC -fvisibility-inlines-hidden -Werror=date-time -std=c++11 -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -pedantic -Wno-long-long -Wno-maybe-uninitialized -Wdelete-non-virtual-dtor -Wno-comment -ffunction-sections -fdata-sections -O2 -DNDEBUG -g1  -fno-exceptions -DLLVM_BUILD_GLOBAL_ISEL -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:20:07] 
[01:20:07] 
[01:20:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:20:07] Build completed unsuccessfully in 0:34:03
[01:20:07] Build completed unsuccessfully in 0:34:03
[01:20:07] make: *** [check] Error 1
[01:20:07] Makefile:58: recipe for target 'check' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d2d2826
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:2939de97:start=1531687892576772644,finish=1531687892585886277,duration=9113633
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01a19343
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
