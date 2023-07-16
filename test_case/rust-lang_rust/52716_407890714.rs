plain
[00:00:00] Attempting with retry: sh -c rm -f download-src-doc-book.tar.gz &&         curl -sSL -o download-src-doc-book.tar.gz https://github.com/rust-lang/book/archive/f475da63a18d50217459a601cbef69a4bcac5e71.tar.gz
[00:00:00] rm 'src/doc/rust-by-example'
[00:00:00] Attempting with retry: sh -c rm -f download-src-doc-rust-by-example.tar.gz &&         curl -sSL -o download-src-doc-rust-by-example.tar.gz https://github.com/rust-lang/rust-by-example/archive/d2a64395a5210a61d3512a3a5c615f5c47699443.tar.gz
[00:00:00] rm 'src/llvm-emscripten'
[00:00:00] Attempting with retry: sh -c git submodule deinit -f  src/jemalloc src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/tools/lld src/libbacktrace src/lldb src/clang &&     git submodule sync &&     git submodule update -j 16 --init --recursive  src/jemalloc src/tools/rust-installer src/liblibc src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/libcompiler_builtins src/tools/clippy src/tools/rustfmt src/tools/miri src/dlmalloc src/stdsimd src/tools/lld src/libbacktrace src/lldb src/clang
[00:00:00] Cleared directory 'src/clang'
[00:00:00] Cleared directory 'src/dlmalloc'
[00:00:00] Cleared directory 'src/doc/nomicon'
[00:00:00] Cleared directory 'src/doc/reference'
---
[00:00:00] Cleared directory 'src/tools/miri'
[00:00:00] Cleared directory 'src/tools/rls'
[00:00:00] Cleared directory 'src/tools/rust-installer'
[00:00:00] Cleared directory 'src/tools/rustfmt'
[00:00:00] Submodule 'src/clang' (https://github.com/rust-lang-nursery/clang/) registered for path 'src/clang'
[00:00:00] Submodule 'src/doc/nomicon' (https://github.com/rust-lang-nursery/nomicon.git) registered for path 'src/doc/nomicon'
[00:00:00] Submodule 'src/doc/reference' (https://github.com/rust-lang-nursery/reference.git) registered for path 'src/doc/reference'
[00:00:00] Submodule 'src/jemalloc' (https://github.com/rust-lang/jemalloc.git) registered for path 'src/jemalloc'
[00:00:00] Submodule 'src/libbacktrace' (https://github.com/rust-lang-nursery/libbacktrace) registered for path 'src/libbacktrace'
[00:00:00] Submodule 'src/libbacktrace' (https://github.com/rust-lang-nursery/libbacktrace) registered for path 'src/libbacktrace'
[00:00:00] Submodule 'src/libcompiler_builtins' (https://github.com/rust-lang-nursery/compiler-builtins) registered for path 'src/libcompiler_builtins'
[00:00:00] Submodule 'src/liblibc' (https://github.com/rust-lang/libc.git) registered for path 'src/liblibc'
[00:00:00] Submodule 'src/lldb' (https://github.com/rust-lang-nursery/lldb/) registered for path 'src/lldb'
[00:00:00] Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'src/tools/cargo'
[00:00:00] Submodule 'src/tools/clippy' (https://github.com/rust-lang-nursery/rust-clippy.git) registered for path 'src/tools/clippy'
[00:00:00] Submodule 'src/tools/lld' (https://github.com/rust-lang/lld.git) registered for path 'src/tools/lld'
[00:00:00] Submodule 'src/tools/miri' (https://github.com/solson/miri.git) registered for path 'src/tools/miri'
---
[00:00:53] Cloning into '/home/travis/build/rust-lang/rust/src/tools/rustfmt'...
[00:00:53] Cloning into '/home/travis/build/rust-lang/rust/src/liblibc'...
[00:00:53] Cloning into '/home/travis/build/rust-lang/rust/src/tools/cargo'...
[00:00:53] Cloning into '/home/travis/build/rust-lang/rust/src/tools/lld'...
[00:00:53] Cloning into '/home/travis/build/rust-lang/rust/src/lldb'...
[00:00:54] Submodule path 'src/clang': checked out '6fda594059bd48b6b2ddcb34eda0a278aee2214e'
[00:00:54] Submodule path 'src/doc/nomicon': checked out '66ef7373409d1979c2839db8886ac2ec9b6a58cd'
[00:00:54] Submodule path 'src/doc/reference': checked out '0f63519ea10c028f48b2dbf7d0a2454203b68b0b'
[00:00:54] Submodule path 'src/jemalloc': checked out '1f5a28755e301ac581e2048011e4e0ff3da482ef'
[00:00:54] Submodule path 'src/libbacktrace': checked out 'f4d02bbdbf8a2c5a31f0801dfef597a86caad9e3'
---
[00:00:57] Cloning into '/home/travis/build/rust-lang/rust/src/libcompiler_builtins/libm'...
[00:00:58] Submodule path 'src/libcompiler_builtins/compiler-rt': checked out '40151c4c1cf77e593e3654e66e25ea423116aaae'
[00:00:58] Submodule path 'src/libcompiler_builtins/libm': checked out 'd65f60f24289ba212f5d47792f7236857efb2339'
[00:00:58] Submodule path 'src/liblibc': checked out 'b6d23ed45d72918239c0bfac11dc547895e59b81'
[00:00:59] Submodule path 'src/lldb': checked out '3dbe998969d457c5cef245f61b48bdaed0f5c059'
[00:00:59] Submodule path 'src/tools/cargo': checked out '506eea76edbf7198258265ddabcf320365bc4c5c'
[00:00:59] Submodule path 'src/tools/clippy': checked out 'afd91248eda02cf2968e4e02c77b6c10ecd3fd4f'
[00:00:59] Submodule path 'src/tools/lld': checked out '8214ccf861d538671b0a1436dbf4538dc4a64d09'
[00:00:59] Submodule path 'src/tools/miri': checked out '911aedf736992e907d11cb494167f41f28d02368'
---

[00:04:51] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:53] tidy error: /checkout/src/clang/www/builtins.py:148: line longer than 100 chars
[00:04:53] tidy error: /checkout/src/clang/www/builtins.py:151: line longer than 100 chars
[00:04:53] tidy error: /checkout/src/clang/www/builtins.py: incorrect license
[00:04:53] tidy error: /checkout/src/clang/www/analyzer/scripts/menu.js: incorrect license
[00:04:53] tidy error: /checkout/src/clang/www/analyzer/scripts/expandcollapse.js:1: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/www/analyzer/scripts/expandcollapse.js:25: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/www/analyzer/scripts/expandcollapse.js:26: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/www/analyzer/scripts/expandcollapse.js:27: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/www/analyzer/scripts/expandcollapse.js:55: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/www/analyzer/scripts/expandcollapse.js:64: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/www/analyzer/scripts/expandcollapse.js:83: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/www/analyzer/scripts/expandcollapse.js:85: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/www/analyzer/scripts/expandcollapse.js:89: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/www/analyzer/scripts/expandcollapse.js:90: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/www/analyzer/scripts/expandcollapse.js:94: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/www/analyzer/scripts/expandcollapse.js:95: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/www/analyzer/scripts/expandcollapse.js:130: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/www/analyzer/scripts/expandcollapse.js:158: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/www/analyzer/scripts/expandcollapse.js:184: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/www/analyzer/scripts/expandcollapse.js:185: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/www/analyzer/scripts/expandcollapse.js: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Driver/MultilibTest.cpp:276: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/Driver/MultilibTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Driver/DistroTest.cpp:73: line longer than 100 chars
[00:04:53] tidy error: /checkout/src/clang/unittests/Driver/DistroTest.cpp:110: line longer than 100 chars
[00:04:53] tidy error: /checkout/src/clang/unittests/Driver/DistroTest.cpp:161: line longer than 100 chars
[00:04:53] tidy error: /checkout/src/clang/unittests/Driver/DistroTest.cpp:186: line longer than 100 chars
[00:04:53] tidy error: /checkout/src/clang/unittests/Driver/DistroTest.cpp:212: line longer than 100 chars
[00:04:53] tidy error: /checkout/src/clang/unittests/Driver/DistroTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Driver/ModuleCacheTest.cpp:26: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/Driver/ModuleCacheTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Driver/ToolChainTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/StaticAnalyzer/RegisterCustomCheckersTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/StaticAnalyzer/AnalyzerOptionsTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/CodeGen/IncrementalProcessingTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/CodeGen/CodeGenExternalTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/CodeGen/BufferSourceTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/CodeGen/IRMatchers.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/CodeGen/TBAAMetadataTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Lex/HeaderSearchTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Lex/LexerTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Lex/HeaderMapTest.cpp:89: \
[00:04:53] C++ code used llvm_unreachable, which triggers undefined behavior
[00:04:53] when executed when assertions are disabled.
[00:04:53] Use llvm::report_fatal_error for increased robustness.
[00:04:53] tidy error: /checkout/src/clang/unittests/Lex/HeaderMapTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Lex/PPCallbacksTest.cpp:119: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/Lex/PPCallbacksTest.cpp:142: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/Lex/PPCallbacksTest.cpp:201: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/Lex/PPCallbacksTest.cpp:225: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/Lex/PPCallbacksTest.cpp:248: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/Lex/PPCallbacksTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Lex/PPConditionalDirectiveRecordTest.cpp:103: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/Lex/PPConditionalDirectiveRecordTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Analysis/CFGTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Analysis/CloneDetectionTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/libclang/LibclangTest.cpp:444: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/libclang/LibclangTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTestComments.cpp:680: XXX is deprecated; use FIXME
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTestComments.cpp:682: XXX is deprecated; use FIXME
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTestComments.cpp:684: XXX is deprecated; use FIXME
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTestComments.cpp:686: XXX is deprecated; use FIXME
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTestComments.cpp:1922: TODO is deprecated; use FIXME
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTestComments.cpp:1925: TODO is deprecated; use FIXME
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTestComments.cpp:1927: TODO is deprecated; use FIXME
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTestComments.cpp:1941: XXX is deprecated; use FIXME
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTestComments.cpp:1943: XXX is deprecated; use FIXME
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTestComments.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTestSelective.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/SortIncludesTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTestTextProto.cpp:203: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTestTextProto.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTestJava.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/UsingDeclarationsSorterTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/CleanupTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/SortImportsTestJS.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTestRawStrings.cpp:401: TODO is deprecated; use FIXME
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTestRawStrings.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTestObjC.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTestUtils.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTestProto.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTestJS.cpp:635: TODO is deprecated; use FIXME
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTestJS.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTest.cpp:1103: line longer than 100 chars
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTest.cpp:1112: line longer than 100 chars
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTest.cpp:1654: tab character
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTest.cpp:1655: tab character
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTest.cpp:3618: tab character
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTest.cpp:3624: tab character
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTest.cpp:3629: tab character
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTest.cpp:3633: tab character
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTest.cpp:3640: tab character
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTest.cpp:3645: tab character
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTest.cpp:3650: tab character
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTest.cpp:3658: tab character
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTest.cpp:3666: tab character
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTest.cpp:3716: tab character
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTest.cpp:3726: tab character
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTest.cpp:3737: tab character
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTest.cpp:3738: tab character
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTest.cpp:3742: tab character
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTest.cpp:5544: line longer than 100 chars
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTest.cpp:5562: line longer than 100 chars
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTest.cpp:6854: tab character
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/FormatTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Format/NamespaceEndCommentsFixerTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/StmtPrinterTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/CommentLexer.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/DeclTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/MatchVerifier.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/DeclMatcher.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/ASTVectorTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/ExternalASTSourceTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/ASTImporterTest.cpp:2477: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/ASTImporterTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/CommentParser.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/CommentTextTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/EvaluateAsRValueTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/StructuralEquivalenceTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/ASTTypeTraitsTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/Language.cpp:40: \
[00:04:53] C++ code used llvm_unreachable, which triggers undefined behavior
[00:04:53] when executed when assertions are disabled.
[00:04:53] Use llvm::report_fatal_error for increased robustness.
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/Language.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/Language.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/NamedDeclPrinterTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/SourceLocationTest.cpp:272: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/SourceLocationTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/ASTContextParentMapTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/DeclPrinterTest.cpp:790: TODO is deprecated; use FIXME
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/DeclPrinterTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/AST/DataCollectionTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/ASTMatchers/ASTMatchersInternalTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/ASTMatchers/ASTMatchersNarrowingTest.cpp:550: line longer than 100 chars
[00:04:53] tidy error: /checkout/src/clang/unittests/ASTMatchers/ASTMatchersNarrowingTest.cpp:553: line longer than 100 chars
[00:04:53] tidy error: /checkout/src/clang/unittests/ASTMatchers/ASTMatchersNarrowingTest.cpp:567: line longer than 100 chars
[00:04:53] tidy error: /checkout/src/clang/unittests/ASTMatchers/ASTMatchersNarrowingTest.cpp:1919: line longer than 100 chars
[00:04:53] tidy error: /checkout/src/clang/unittests/ASTMatchers/ASTMatchersNarrowingTest.cpp:2085: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/ASTMatchers/ASTMatchersNarrowingTest.cpp:2131: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/ASTMatchers/ASTMatchersNarrowingTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/ASTMatchers/Dynamic/VariantValueTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/ASTMatchers/Dynamic/ParserTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/ASTMatchers/Dynamic/RegistryTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/ASTMatchers/ASTMatchersTest.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/ASTMatchers/ASTMatchersTraversalTest.cpp:1526: line longer than 100 chars
[00:04:53] tidy error: /checkout/src/clang/unittests/ASTMatchers/ASTMatchersTraversalTest.cpp:1547: line longer than 100 chars
[00:04:53] tidy error: /checkout/src/clang/unittests/ASTMatchers/ASTMatchersTraversalTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/ASTMatchers/ASTMatchersNodeTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/ReplacementsYamlTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/LookupTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/DiagnosticsYamlTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/ReplacementTest.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/CommentHandlerTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RefactoringTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RecursiveASTVisitorTests/InitListExprPreOrderNoQueue.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RecursiveASTVisitorTests/TemplateArgumentLocTraverser.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RecursiveASTVisitorTests/ConstructExpr.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RecursiveASTVisitorTests/ImplicitCtor.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RecursiveASTVisitorTests/Class.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RecursiveASTVisitorTests/LambdaExpr.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RecursiveASTVisitorTests/DeclRefExpr.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RecursiveASTVisitorTests/InitListExprPreOrder.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RecursiveASTVisitorTests/NestedNameSpecifiers.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RecursiveASTVisitorTests/ParenExpr.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RecursiveASTVisitorTests/CXXBoolLiteralExpr.cpp:16: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RecursiveASTVisitorTests/CXXBoolLiteralExpr.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RecursiveASTVisitorTests/CXXMemberCall.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RecursiveASTVisitorTests/Attr.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RecursiveASTVisitorTests/IntegerLiteral.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RecursiveASTVisitorTests/CXXOperatorCallExprTraverser.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RecursiveASTVisitorTests/InitListExprPostOrder.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RecursiveASTVisitorTests/LambdaDefaultCapture.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RecursiveASTVisitorTests/InitListExprPostOrderNoQueue.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RefactoringActionRulesTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/ToolingTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/LexicallyOrderedRecursiveASTVisitorTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RecursiveASTVisitorTestDeclVisitor.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/ExecutionTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/FixItTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/HeaderIncludesTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/CastExprTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RewriterTestContext.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/ASTSelectionTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RewriterTest.cpp:24: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RewriterTest.cpp:37: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RewriterTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RecursiveASTVisitorTestTypeLocVisitor.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/QualTypeNamesTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RefactoringCallbacksTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/CompilationDatabaseTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/RecursiveASTVisitorTestPostOrderVisitor.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Tooling/TestVisitor.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Rewrite/RewriteBufferTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Sema/CodeCompleteTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Sema/ExternalSemaSourceTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Rename/RenameEnumTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Rename/ClangRenameTest.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Rename/RenameMemberTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Rename/RenameFunctionTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Rename/RenameAliasTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Rename/RenameClassTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Frontend/ParsedSourceLocationTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Frontend/PCHPreambleTest.cpp:173: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/Frontend/PCHPreambleTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Frontend/CodeGenActionTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Frontend/CompilerInstanceTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Frontend/ASTUnitTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Frontend/FrontendActionTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Frontend/OutputStreamTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Basic/FileManagerTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Basic/SourceManagerTest.cpp:88: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/Basic/SourceManagerTest.cpp:92: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/Basic/SourceManagerTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Basic/MemoryBufferCacheTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Basic/DiagnosticTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Basic/VirtualFileSystemTest.cpp:32: \
[00:04:53] C++ code used llvm_unreachable, which triggers undefined behavior
[00:04:53] when executed when assertions are disabled.
[00:04:53] Use llvm::report_fatal_error for increased robustness.
[00:04:53] tidy error: /checkout/src/clang/unittests/Basic/VirtualFileSystemTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/Basic/CharInfoTest.cpp:169: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/Basic/CharInfoTest.cpp:190: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/Basic/CharInfoTest.cpp:211: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/Basic/CharInfoTest.cpp:233: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/unittests/Basic/CharInfoTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/unittests/CrossTU/CrossTranslationUnitTest.cpp: incorrect license
[00:04:53] tidy error: /checkout/src/clang/docs/analyzer/conf.py: incorrect license
[00:04:53] tidy error: /checkout/src/clang/docs/tools/dump_format_style.py: incorrect license
[00:04:53] tidy error: /checkout/src/clang/docs/tools/dump_ast_matchers.py:17: line longer than 100 chars
[00:04:53] tidy error: /checkout/src/clang/docs/tools/dump_ast_matchers.py:86: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/docs/tools/dump_ast_matchers.py:90: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/docs/tools/dump_ast_matchers.py:325: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/docs/tools/dump_ast_matchers.py:360: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/docs/tools/dump_ast_matchers.py: incorrect license
[00:04:53] tidy error: /checkout/src/clang/docs/conf.py: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTWriter.h:419: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTWriter.h:427: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTWriter.h:478: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTWriter.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/SerializationDiagnostic.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/GlobalModuleIndex.h:118: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/GlobalModuleIndex.h:170: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/GlobalModuleIndex.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTDeserializationListener.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTReader.h:594: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTReader.h:665: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTReader.h:668: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTReader.h:681: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTReader.h:686: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTReader.h:702: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTReader.h:703: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTReader.h:706: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTReader.h:1059: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTReader.h:1116: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTReader.h:1123: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTReader.h:1624: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTReader.h:1653: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTReader.h:1740: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTReader.h:1838: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTReader.h:1843: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTReader.h:1846: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTReader.h:2066: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTReader.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/Module.h:176: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/Module.h:361: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/Module.h:365: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/Module.h:368: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/Module.h:371: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/Module.h:434: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/Module.h:437: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/Module.h:440: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/Module.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:58: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:62: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:66: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:67: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:101: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:108: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:127: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:158: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:159: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:173: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:314: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:709: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:712: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:717: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:740: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:744: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:1206: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:1214: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:1282: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:1542: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:1744: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:1803: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:1865: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:1867: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:1881: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:1891: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:2001: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:2006: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:2011: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:2016: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:2030: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:2035: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:2040: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h:2045: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ASTBitCodes.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ModuleManager.h:85: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ModuleManager.h:153: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ModuleManager.h:159: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ModuleManager.h:175: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ModuleManager.h:179: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ModuleManager.h:182: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ModuleManager.h:194: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ModuleManager.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ModuleFileExtension.h:29: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ModuleFileExtension.h:131: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ModuleFileExtension.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ContinuousRangeMap.h:75: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ContinuousRangeMap.h:82: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ContinuousRangeMap.h:109: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ContinuousRangeMap.h:119: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ContinuousRangeMap.h:131: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Serialization/ContinuousRangeMap.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Driver/Util.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Driver/SanitizerArgs.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Driver/Job.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Driver/Types.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Driver/Distro.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Driver/Driver.h:259: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Driver/Driver.h:368: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Driver/Driver.h:373: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Driver/Driver.h:496: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Driver/Driver.h:505: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Driver/Driver.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Driver/Phases.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Driver/Action.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Driver/Tool.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Driver/ToolChain.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Driver/Compilation.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Driver/DriverDiagnostic.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Driver/Multilib.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Driver/XRayArgs.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Driver/Options.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Edit/EditsReceiver.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Edit/Commit.h:60: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Edit/Commit.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Edit/Rewriters.h:32: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Edit/Rewriters.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Edit/FileOffset.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/Edit/EditedSource.h:88: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/Edit/EditedSource.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerOptInfo.h:31: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerOptInfo.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerManager.h:89: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerManager.h:355: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerManager.h:357: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerManager.h:360: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerManager.h:363: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerManager.h:366: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerManager.h:383: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerManager.h:421: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerManager.h:427: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerManager.h:431: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerManager.h:435: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerManager.h:442: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerManager.h:448: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerManager.h:451: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerManager.h:453: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerManager.h:461: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerManager.h:467: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerManager.h:471: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerManager.h:532: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerManager.h:637: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerManager.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/Checker.h:66: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/Checker.h:334: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/Checker.h:373: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/Checker.h:413: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/Checker.h:439: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/Checker.h:507: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/Checker.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerRegistry.h:50: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/CheckerRegistry.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/BugReporterVisitors.h:249: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/BugReporterVisitors.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/CommonBugCategories.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/BugReporter.h:77: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/BugReporter.h:105: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/BugReporter.h:223: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/BugReporter.h:228: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/BugReporter.h:254: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/BugReporter.h:258: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/BugReporter.h:293: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/BugReporter.h:313: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/BugReporter.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:74: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:77: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:84: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:97: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:130: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:316: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:321: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:348: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:372: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:395: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:398: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:402: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:482: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:557: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:562: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:585: TODO is deprecated; use FIXME
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:599: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:605: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:608: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:719: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:773: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:790: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:793: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:800: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:805: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h:892: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/PathDiagnostic.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/BugType.h:55: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/BugReporter/BugType.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/IssueHash.h:32: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/IssueHash.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/AnalyzerOptions.h:133: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/AnalyzerOptions.h:140: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/AnalyzerOptions.h:146: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/AnalyzerOptions.h:149: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/AnalyzerOptions.h:173: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/AnalyzerOptions.h:179: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/AnalyzerOptions.h:183: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/AnalyzerOptions.h:187: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/AnalyzerOptions.h:214: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/AnalyzerOptions.h:224: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/AnalyzerOptions.h:242: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/AnalyzerOptions.h:725: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/AnalyzerOptions.h:727: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/AnalyzerOptions.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/BlockCounter.h:38: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/BlockCounter.h:48: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/BlockCounter.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/SummaryManager.h:24: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/SummaryManager.h:26: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/SummaryManager.h:37: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/SummaryManager.h:39: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/SummaryManager.h:43: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/SummaryManager.h:45: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/SummaryManager.h:48: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/SummaryManager.h:51: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/SummaryManager.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/AnalysisManager.h:48: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/AnalysisManager.h:53: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/AnalysisManager.h:63: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/AnalysisManager.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/APSIntType.h:91: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/APSIntType.h:105: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/APSIntType.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/StoreRef.h:21: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/StoreRef.h:23: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/StoreRef.h:29: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/StoreRef.h:39: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/StoreRef.h:46: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/StoreRef.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/SimpleConstraintManager.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/SMTConstraintManager.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/Store.h:152: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/Store.h:179: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/Store.h:180: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/Store.h:198: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/Store.h:224: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/Store.h:308: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/Store.h:312: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/Store.h:317: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/Store.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/SValBuilder.h:58: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/SValBuilder.h:72: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/SValBuilder.h:140: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/SValBuilder.h:160: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/SValBuilder.h:164: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/SValBuilder.h:240: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/SValBuilder.h:255: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/SValBuilder.h: incorrect license
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/CallEvent.h:120: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/CallEvent.h:122: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/CallEvent.h:123: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/CallEvent.h:125: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/CallEvent.h:144: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/CallEvent.h:145: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/CallEvent.h:146: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/CallEvent.h:149: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/CallEvent.h:150: trailing whitespace
[00:04:53] tidy error: /checkout/src/clang/include/clang/StaticAnalyzer/Core/PathSensitive/CallEvent.h:631: trailing whitespace
---
travis_time:end:2640dcb0:start=1532551237240243961,finish=1532551237248668817,duration=8424856
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01954e60
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b69e1e4
travis_time:start:0b69e1e4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:025fda00
$ dmesg | grep -i kill
