plain
[2023-02-15T04:25:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-15T04:25:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-15T04:25:48Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpNwSjyV#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-15T04:25:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-02-15T04:25:50Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpNwSjyV#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpNwSjyV\\incremental-state"
[2023-02-15T04:25:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-02-15T04:25:54Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpNwSjyV#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpNwSjyV\\incremental-state"
[2023-02-15T04:25:55Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpNwSjyV"
[2023-02-15T04:25:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-02-15T04:25:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-02-15T04:25:55Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpNwSjyV#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpNwSjyV\\incremental-state"
[2023-02-15T04:25:56Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-15T04:25:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-02-15T04:25:56Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpYbICPC#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-15T04:25:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-02-15T04:25:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-02-15T04:25:59Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpYbICPC#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpYbICPC\\incremental-state"
[2023-02-15T04:26:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-02-15T04:26:03Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpYbICPC#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpYbICPC\\incremental-state"
[2023-02-15T04:26:04Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpYbICPC"
[2023-02-15T04:26:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-02-15T04:26:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-02-15T04:26:04Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpYbICPC#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpYbICPC\\incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2023-02-15T04:26:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-02-15T04:26:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2023-02-15T04:28:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-15T04:28:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-02-15T04:28:41Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpFHawg9#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-15T04:29:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-02-15T04:29:05Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpFHawg9#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpFHawg9\\incremental-state"
[2023-02-15T04:29:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-02-15T04:29:32Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpFHawg9#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpFHawg9\\incremental-state"
[2023-02-15T04:29:37Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpFHawg9"
[2023-02-15T04:29:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-02-15T04:29:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-02-15T04:29:37Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpFHawg9#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpFHawg9\\incremental-state"
[2023-02-15T04:29:46Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-15T04:29:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-15T04:29:48Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpiVo6BR#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-15T04:30:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-02-15T04:30:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-02-15T04:30:48Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpiVo6BR#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpiVo6BR\\incremental-state"
[2023-02-15T04:31:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-02-15T04:31:59Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpiVo6BR#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpiVo6BR\\incremental-state"
[2023-02-15T04:32:12Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpiVo6BR"
[2023-02-15T04:32:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-02-15T04:32:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-02-15T04:32:12Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpiVo6BR#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpiVo6BR\\incremental-state"
[2023-02-15T04:32:28Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-15T04:32:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-02-15T04:32:31Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp4px9WD#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-15T04:33:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2023-02-15T04:35:50Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-15T04:35:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-02-15T04:35:50Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpnaRZhI#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-15T04:35:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-02-15T04:35:59Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpnaRZhI#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpnaRZhI\\incremental-state"
[2023-02-15T04:36:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-02-15T04:36:11Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpnaRZhI#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpnaRZhI\\incremental-state"
[2023-02-15T04:36:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-15T04:36:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-15T04:36:11Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpswM6Zn#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-15T04:36:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-02-15T04:36:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-02-15T04:36:21Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpswM6Zn#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpswM6Zn\\incremental-state"
[2023-02-15T04:36:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-02-15T04:36:32Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpswM6Zn#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpswM6Zn\\incremental-state"
[2023-02-15T04:36:33Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-15T04:36:33Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-02-15T04:36:33Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpQOsU7A#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-15T04:36:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2023-02-15T04:37:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-15T04:37:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-02-15T04:37:19Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpOmtdWV#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-15T04:37:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-02-15T04:37:36Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpOmtdWV#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpOmtdWV\\incremental-state"
[2023-02-15T04:37:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-02-15T04:37:57Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpOmtdWV#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpOmtdWV\\incremental-state"
[2023-02-15T04:38:01Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpOmtdWV"
[2023-02-15T04:38:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-02-15T04:38:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-02-15T04:38:01Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpOmtdWV#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpOmtdWV\\incremental-state"
[2023-02-15T04:38:06Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-15T04:38:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-15T04:38:06Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpdcgv2e#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-15T04:38:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-02-15T04:38:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-02-15T04:38:27Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpdcgv2e#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpdcgv2e\\incremental-state"
[2023-02-15T04:38:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-02-15T04:38:51Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpdcgv2e#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpdcgv2e\\incremental-state"
[2023-02-15T04:38:56Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpdcgv2e"
[2023-02-15T04:38:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-02-15T04:38:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-02-15T04:38:56Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpdcgv2e#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpdcgv2e\\incremental-state"
[2023-02-15T04:39:01Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-15T04:39:02Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-02-15T04:39:02Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpN22uVy#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-15T04:39:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2023-02-15T04:39:58Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-15T04:39:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-02-15T04:39:58Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpNvYwX2#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-15T04:39:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-02-15T04:39:59Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpNvYwX2#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpNvYwX2\\incremental-state"
[2023-02-15T04:40:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-02-15T04:40:00Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpNvYwX2#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpNvYwX2\\incremental-state"
[2023-02-15T04:40:01Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-15T04:40:01Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-15T04:40:01Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp22Tnvc#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-15T04:40:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2023-02-15T04:40:26Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-15T04:40:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-02-15T04:40:26Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpiM7Wsc#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-15T04:40:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-02-15T04:40:29Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpiM7Wsc#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpiM7Wsc\\incremental-state"
[2023-02-15T04:40:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-02-15T04:40:33Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpiM7Wsc#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpiM7Wsc\\incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2023-02-15T04:40:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-02-15T04:40:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2023-02-15T04:40:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-15T04:40:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-02-15T04:40:42Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpzF3MZL#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-15T04:40:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-02-15T04:40:48Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpzF3MZL#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpzF3MZL\\incremental-state"
[2023-02-15T04:40:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-02-15T04:40:55Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpzF3MZL#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpzF3MZL\\incremental-state"
[2023-02-15T04:40:57Z DEBUG collector::execute] applying new row to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpzF3MZL"
[2023-02-15T04:40:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-02-15T04:40:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-02-15T04:40:57Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpzF3MZL#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpzF3MZL\\incremental-state"
[2023-02-15T04:41:04Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-15T04:41:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-15T04:41:04Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpDHQLN2#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-15T04:41:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-02-15T04:41:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-02-15T04:41:11Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpDHQLN2#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpDHQLN2\\incremental-state"
[2023-02-15T04:41:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-02-15T04:41:18Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpDHQLN2#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpDHQLN2\\incremental-state"
[2023-02-15T04:41:20Z DEBUG collector::execute] applying new row to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpDHQLN2"
[2023-02-15T04:41:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-02-15T04:41:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-02-15T04:41:20Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpDHQLN2#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpDHQLN2\\incremental-state"
[2023-02-15T04:41:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-15T04:41:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-02-15T04:41:27Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmppmpWNh#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-15T04:41:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
Finished benchmark tuple-stress (8/8)
stage-build DEBUG: Reverting working dir to `D:\a\rust\rust`
stage-build INFO: Merging Rustc PGO profiles to D:\a\rust\rust\opt-artifacts\rustc-pgo.profdata
stage-build INFO: Executing `D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\bin\llvm-profdata merge -o D:\a\rust\rust\opt-artifacts\rustc-pgo.profdata D:\a\rust\rust\opt-artifacts\rustc-pgo`
warning: D:\a\rust\rust\opt-artifacts\rustc-pgo\default_2064842429685484369_0.profraw: malformed instrumentation profile data: not enough space for another header
error: no profile can be merged
stage-build INFO: Section `Stage 2 (rustc PGO) > Gather profiles` ended: FAIL (1055.01s)
stage-build INFO: Section `Stage 2 (rustc PGO)` ended: FAIL (1922.63s)
stage-build ERROR: The multi-stage build has failed
------------------------------------------------
Stage 1 (LLVM PGO):            2512.24s (56.65%)
  Build rustc and LLVM:        1875.58s (42.29%)
    LLVM:                       289.26s ( 6.52%)
---
Total duration:                       1h 13m 55s
------------------------------------------------
root INFO: Free disk space: 35.17 GiB out of total 56.00 GiB (37.20% used)
Traceback (most recent call last):
  File "D:\a\rust\rust\src\ci\stage-build.py", line 837, in <module>
    raise e
  File "D:\a\rust\rust\src\ci\stage-build.py", line 834, in <module>
    execute_build_pipeline(timer, pipeline, build_args)
  File "D:\a\rust\rust\src\ci\stage-build.py", line 784, in execute_build_pipeline
    gather_rustc_profiles(pipeline)
  File "D:\a\rust\rust\src\ci\stage-build.py", line 629, in gather_rustc_profiles
    cmd([
  File "D:\a\rust\rust\src\ci\stage-build.py", line 452, in cmd
    return subprocess.run(args, env=environment, check=True)
  File "C:\hostedtoolcache\windows\Python\3.11.1\x64\Lib\subprocess.py", line 571, in run
  File "C:\hostedtoolcache\windows\Python\3.11.1\x64\Lib\subprocess.py", line 571, in run
    raise CalledProcessError(retcode, process.args,
subprocess.CalledProcessError: Command '['D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\bin\\llvm-profdata', 'merge', '-o', 'D:\\a\\rust\\rust\\opt-artifacts\\rustc-pgo.profdata', 'D:\\a\\rust\\rust\\opt-artifacts\\rustc-pgo']' returned non-zero exit status 1.
