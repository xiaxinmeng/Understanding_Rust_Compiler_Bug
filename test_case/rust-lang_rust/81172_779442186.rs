plain
[RUSTC-TIMING] linkchecker test:false 2.902
    Finished release [optimized] target(s) in 3.74s
[TIMING] ToolBuild { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-pc-windows-gnu", file: None } }, target: TargetSelection { triple: "x86_64-pc-windows-gnu", file: None }, tool: "linkchecker", path: "src/tools/linkchecker", mode: ToolBootstrap, is_optional_tool: false, source_type: InTree, extra_features: [] } -- 3.781
[TIMING] Linkchecker { compiler: Compiler { stage: 0, host: TargetSelection { triple: "x86_64-pc-windows-gnu", file: None } }, target: TargetSelection { triple: "x86_64-pc-windows-gnu", file: None } } -- 0.000
std\keyword.Self.html:66: id is not unique: `search`
std\keyword.Self.html:66: id is not unique: `rustdoc-vars`
thread 'main' panicked at 'found some broken links', src\tools\linkchecker\main.rs:102:9



command did not execute successfully: "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\stage0-tools-bin\\linkchecker.exe" "D:\\a\\rust\\rust\\build\\x86_64-pc-windows-gnu\\doc"


failed to run: D:\a\rust\rust\build\bootstrap\debug\bootstrap test --stage 2 --exclude src/test/ui
Build completed unsuccessfully in 1:15:04
Build completed unsuccessfully in 1:15:04
make: *** [Makefile:80: ci-mingw-subset-1] Error 1
