plain
[2022-07-19T23:53:15Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-19T23:53:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-19T23:53:23Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpvgGFpu#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark cargo-0.60.0 (1/7)
collector error: Failed to profile 'cargo-0.60.0' with Eprintln, recorded: expected success, got exit code: 101
stderr=LLVM Profile Warning: Unable to merge profile data: source profile file is not compatible.
LLVM Profile Error: Profile Merging of file C:/msys64/tmp/tmp-pgo/llvm-pgo/prof-5380\default_4647308148763315687_0.profraw failed: File exists
LLVM Profile Error: Failed to write file "C:/msys64/tmp/tmp-pgo/llvm-pgo/prof-5380\default_4647308148763315687_0.profraw": File exists
LLVM Profile Warning: Unable to merge profile data: source profile file is not compatible.
---
LLVM Profile Error: Failed to write file "C:/msys64/tmp/tmp-pgo/llvm-pgo/prof-3368\default_4647308148763315687_0.profraw": File exists
LLVM Profile Warning: Unable to merge profile data: source profile file is not compatible.
LLVM Profile Error: Profile Merging of file C:/msys64/tmp/tmp-pgo/llvm-pgo/prof-656\default_4647308148763315687_0.profraw failed: File exists
LLVM Profile Error: Failed to write file "C:/msys64/tmp/tmp-pgo/llvm-pgo/prof-656\default_4647308148763315687_0.profraw": File exists
"\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage2\\bin\\rustc.exe" ["--crate-name", "anyhow", "--edition=2018", "C:\\Users\\runneradmin\\.cargo\\registry\\src\\github.com-1ecc6299db9ec823\\anyhow-1.0.55\\src\\lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"std\"", "-C", "metadata=c1456bfefc49db80", "-C", "extra-filename=-c1456bfefc49db80", "--out-dir", "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpvgGFpu\\target\\debug\\deps", "-L", "dependency=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpvgGFpu\\target\\debug\\deps", "--cap-lints", "allow", "--cfg", "backtrace", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `anyhow`

 stdout=

Executing benchmark clap-3.1.6 (2/7)
---
Executing benchmark regex-1.5.5 (4/7)
Preparing regex-1.5.5
[2022-07-19T23:55:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-07-19T23:55:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-07-19T23:55:31Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpTyQvcy#regex@1.5.5" "--" "--skip-this-rustc"
[2022-07-19T23:55:31Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpeCUQKU#regex@1.5.5" "--release" "--" "--skip-this-rustc"
[2022-07-19T23:55:47Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-07-19T23:55:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-07-19T23:55:47Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmprngZRd#regex@1.5.5" "--" "--wrap-rustc-with" "Eprintln"
Running regex-1.5.5: Opt + [Full]
