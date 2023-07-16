
591:[00:06:50] [TIMING] StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu" } -- 0.321
592:[00:06:51] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu", tool: "fabricate", path: "src/tools/rust-installer", mode: Libstd, is_ext_tool: false, extra_features: [] } -- 0.142
593:[00:06:51] [TIMING] Mingw { host: "x86_64-pc-windows-gnu" } -- 0.145
594:[00:06:52] [TIMING] Src -- 0.907
595:[00:06:52] [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu", tool: "cargo", path: "src/tools/cargo", mode: Librustc, is_ext_tool: false, extra_features: [] } -- 0.149
596:[00:06:52] [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu", tool: "rustfmt", path: "src/tools/rustfmt", mode: Librustc, is_ext_tool: true, extra_features: [] } -- 0.103
597:[00:06:52] [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu", tool: "cargo-fmt", path: "src/tools/rustfmt", mode: Librustc, is_ext_tool: true, extra_features: [] } -- 0.106
598:[00:06:52] [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu", tool: "clippy-driver", path: "src/tools/clippy", mode: Librustc, is_ext_tool: true, extra_features: [] } -- 0.164
600:[00:06:53] [TIMING] StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu" } -- 0.199
628:[00:10:04] [TIMING] Std { target: "x86_64-pc-windows-gnu", compiler: Compiler { stage: 0, host: "x86_64-pc-windows-gnu" } } -- 191.096
638:[00:10:22] [TIMING] Test { target: "x86_64-pc-windows-gnu", compiler: Compiler { stage: 0, host: "x86_64-pc-windows-gnu" } } -- 17.897
787:[00:36:45] [TIMING] Rustc { target: "x86_64-pc-windows-gnu", compiler: Compiler { stage: 0, host: "x86_64-pc-windows-gnu" } } -- 1583.251
4627:[00:55:00] [TIMING] Llvm { target: "x86_64-pc-windows-gnu", emscripten: false } -- 1095.358
4639:[00:57:53] [TIMING] CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu", backend: "llvm" } -- 172.926
7489:[01:06:01] [TIMING] Llvm { target: "x86_64-pc-windows-gnu", emscripten: true } -- 487.764
7496:[01:08:20] [TIMING] CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu", backend: "emscripten" } -- 138.957
7704:[01:09:22] [TIMING] Lld { target: "x86_64-pc-windows-gnu" } -- 62.500
7733:[01:11:35] [TIMING] Std { target: "x86_64-pc-windows-gnu", compiler: Compiler { stage: 1, host: "x86_64-pc-windows-gnu" } } -- 132.673
7743:[01:11:56] [TIMING] Test { target: "x86_64-pc-windows-gnu", compiler: Compiler { stage: 1, host: "x86_64-pc-windows-gnu" } } -- 20.767
7892:[01:37:11] [TIMING] Rustc { target: "x86_64-pc-windows-gnu", compiler: Compiler { stage: 1, host: "x86_64-pc-windows-gnu" } } -- 1515.080
7904:[01:39:12] [TIMING] CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu", backend: "llvm" } -- 120.948
7911:[01:40:55] [TIMING] CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu", backend: "emscripten" } -- 103.268
7938:[01:42:32] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu", tool: "unstable-book-gen", path: "src/tools/unstable-book-gen", mode: Libstd, is_ext_tool: false, extra_features: [] } -- 97.191
7939:[01:42:34] [TIMING] UnstableBookGen { target: "x86_64-pc-windows-gnu" } -- 1.923
8106:[01:48:23] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu", tool: "rustbook", path: "src/tools/rustbook", mode: Librustc, is_ext_tool: false, extra_features: [] } -- 349.028
8108:[01:48:27] [TIMING] RustbookSrc { target: "x86_64-pc-windows-gnu", name: "unstable-book", src: "C:\\projects\\rust\\build\\x86_64-pc-windows-gnu\\md-doc" } -- 3.242
8110:[01:48:29] [TIMING] RustbookSrc { target: "x86_64-pc-windows-gnu", name: "book/first-edition", src: "C:\\projects\\rust\\src/doc" } -- 1.994
8112:[01:48:32] [TIMING] RustbookSrc { target: "x86_64-pc-windows-gnu", name: "book/second-edition", src: "C:\\projects\\rust\\src/doc" } -- 2.857
8146:[01:52:20] [TIMING] Rustdoc { host: "x86_64-pc-windows-gnu" } -- 228.543
8147:[01:52:21] [TIMING] Standalone { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu" } -- 1.313
8150:[01:52:24] [TIMING] TheBook { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu", name: "book" } -- 2.524
8177:[01:54:29] [TIMING] Std { stage: 2, target: "x86_64-pc-windows-gnu" } -- 125.386
8227:[01:55:43] [TIMING] WhitelistedRustc { stage: 2, target: "x86_64-pc-windows-gnu" } -- 73.282
8235:[01:56:08] [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu", tool: "error_index_generator", path: "src/tools/error_index_generator", mode: Librustc, is_ext_tool: false, extra_features: [] } -- 25.371
8236:[01:56:08] [TIMING] ErrorIndex { target: "x86_64-pc-windows-gnu" } -- 0.225
8238:[01:56:10] [TIMING] RustbookSrc { target: "x86_64-pc-windows-gnu", name: "nomicon", src: "C:\\projects\\rust\\src/doc" } -- 1.466
8240:[01:56:11] [TIMING] RustbookSrc { target: "x86_64-pc-windows-gnu", name: "reference", src: "C:\\projects\\rust\\src/doc" } -- 1.469
8242:[01:56:11] [TIMING] RustbookSrc { target: "x86_64-pc-windows-gnu", name: "rustdoc", src: "C:\\projects\\rust\\src/doc" } -- 0.210
8244:[01:56:14] [TIMING] RustbookSrc { target: "x86_64-pc-windows-gnu", name: "rust-by-example", src: "C:\\projects\\rust\\src/doc" } -- 2.670
8246:[01:56:15] [TIMING] CargoBook { target: "x86_64-pc-windows-gnu", name: "cargo" } -- 0.463
8304:[01:59:32] [TIMING] ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu", tool: "fabricate", path: "src/tools/rust-installer", mode: Libstd, is_ext_tool: false, extra_features: [] } -- 190.611
8305:[02:01:01] [TIMING] Docs { stage: 2, host: "x86_64-pc-windows-gnu" } -- 96.084
8309:[02:01:16] [TIMING] Mingw { host: "x86_64-pc-windows-gnu" } -- 14.611
8311:[02:05:11] [TIMING] Rustc { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-gnu" } } -- 234.835
8313:[02:07:13] [TIMING] Std { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu" } -- 122.391
8316:[02:07:17] [TIMING] Analysis { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu" } -- 4.282
8318:[02:07:36] [TIMING] Src -- 19.133
8500:[02:24:33] [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu", tool: "cargo", path: "src/tools/cargo", mode: Librustc, is_ext_tool: false, extra_features: [] } -- 1016.431
8501:[02:24:50] [TIMING] Cargo { stage: 2, target: "x86_64-pc-windows-gnu" } -- 16.725
8581:[02:31:13] [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu", tool: "rustfmt", path: "src/tools/rustfmt", mode: Librustc, is_ext_tool: true, extra_features: [] } -- 382.840
8590:[02:32:45] [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu", tool: "cargo-fmt", path: "src/tools/rustfmt", mode: Librustc, is_ext_tool: true, extra_features: [] } -- 92.627
8591:[02:32:57] [TIMING] Rustfmt { stage: 2, target: "x86_64-pc-windows-gnu" } -- 11.539
8763:[02:34:36] [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu", tool: "clippy-driver", path: "src/tools/clippy", mode: Librustc, is_ext_tool: true, extra_features: [] } -- 99.115
8852:[02:43:53] [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-pc-windows-gnu" }, target: "x86_64-pc-windows-gnu", tool: "rls", path: "src/tools/rls", mode: Librustc, is_ext_tool: true, extra_features: [] } -- 556.950
8853:[02:44:17] [TIMING] Rls { stage: 2, target: "x86_64-pc-windows-gnu" } -- 24.551
