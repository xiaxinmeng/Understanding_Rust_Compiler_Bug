plain
[2022-08-29T15:03:56Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:03:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-08-29T15:03:56Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp8dZerR#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:03:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-08-29T15:03:59Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp8dZerR#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmp8dZerR\\incremental-state"
[2022-08-29T15:04:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-08-29T15:04:04Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp8dZerR#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmp8dZerR\\incremental-state"
[2022-08-29T15:04:05Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmp8dZerR"
[2022-08-29T15:04:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-08-29T15:04:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-08-29T15:04:05Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp8dZerR#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmp8dZerR\\incremental-state"
[2022-08-29T15:04:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:04:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-08-29T15:04:07Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpQaLVtK#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:04:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-08-29T15:04:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-08-29T15:04:11Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpQaLVtK#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpQaLVtK\\incremental-state"
[2022-08-29T15:04:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-08-29T15:04:16Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpQaLVtK#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpQaLVtK\\incremental-state"
[2022-08-29T15:04:17Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpQaLVtK"
[2022-08-29T15:04:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-08-29T15:04:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-08-29T15:04:17Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpQaLVtK#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpQaLVtK\\incremental-state"
[2022-08-29T15:04:19Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:04:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-08-29T15:04:19Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp2jGCR3#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:04:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-08-29T15:08:01Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:08:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-08-29T15:08:04Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpAb3zBO#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:08:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-08-29T15:08:36Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpAb3zBO#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpAb3zBO\\incremental-state"
[2022-08-29T15:09:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-08-29T15:09:15Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpAb3zBO#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpAb3zBO\\incremental-state"
[2022-08-29T15:09:21Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpAb3zBO"
[2022-08-29T15:09:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-08-29T15:09:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-08-29T15:09:21Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpAb3zBO#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpAb3zBO\\incremental-state"
[2022-08-29T15:09:41Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:09:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-08-29T15:09:45Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpPoRwAA#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:11:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-08-29T15:11:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-08-29T15:11:08Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpPoRwAA#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpPoRwAA\\incremental-state"
[2022-08-29T15:12:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-08-29T15:12:50Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpPoRwAA#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpPoRwAA\\incremental-state"
[2022-08-29T15:13:08Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpPoRwAA"
[2022-08-29T15:13:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-08-29T15:13:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-08-29T15:13:08Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpPoRwAA#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpPoRwAA\\incremental-state"
[2022-08-29T15:13:51Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:13:55Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-08-29T15:13:55Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpd5AuBb#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:15:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-08-29T15:15:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-08-29T15:15:53Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpd5AuBb#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpd5AuBb\\incremental-state"
[2022-08-29T15:17:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-08-29T15:17:52Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpd5AuBb#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpd5AuBb\\incremental-state"
[2022-08-29T15:18:08Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpd5AuBb"
[2022-08-29T15:18:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-08-29T15:18:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-08-29T15:18:09Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpd5AuBb#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpd5AuBb\\incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2022-08-29T15:18:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-08-29T15:18:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2022-08-29T15:18:57Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:18:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-08-29T15:18:57Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp5lxsrR#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:19:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-08-29T15:19:07Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp5lxsrR#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmp5lxsrR\\incremental-state"
[2022-08-29T15:19:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-08-29T15:19:21Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp5lxsrR#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmp5lxsrR\\incremental-state"
[2022-08-29T15:19:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:19:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-08-29T15:19:22Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpkPJmdv#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:19:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-08-29T15:19:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-08-29T15:19:32Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpkPJmdv#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpkPJmdv\\incremental-state"
[2022-08-29T15:19:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-08-29T15:19:46Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpkPJmdv#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpkPJmdv\\incremental-state"
[2022-08-29T15:19:47Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:19:47Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-08-29T15:19:47Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp9qUThY#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:19:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-08-29T15:19:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-08-29T15:19:58Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp9qUThY#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmp9qUThY\\incremental-state"
[2022-08-29T15:20:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-08-29T15:20:11Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp9qUThY#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmp9qUThY\\incremental-state"
Executing benchmark diesel-1.4.8 (4/8)
Preparing diesel-1.4.8
[2022-08-29T15:20:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-08-29T15:20:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-08-29T15:20:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:20:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-08-29T15:20:46Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpTNlxI5#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:21:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-08-29T15:21:11Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpTNlxI5#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpTNlxI5\\incremental-state"
[2022-08-29T15:21:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-08-29T15:21:40Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpTNlxI5#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpTNlxI5\\incremental-state"
[2022-08-29T15:21:46Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpTNlxI5"
[2022-08-29T15:21:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-08-29T15:21:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-08-29T15:21:46Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpTNlxI5#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpTNlxI5\\incremental-state"
[2022-08-29T15:21:52Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:21:52Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-08-29T15:21:52Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp9f0IdJ#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:22:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-08-29T15:22:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-08-29T15:22:20Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp9f0IdJ#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmp9f0IdJ\\incremental-state"
[2022-08-29T15:22:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-08-29T15:22:56Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp9f0IdJ#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmp9f0IdJ\\incremental-state"
[2022-08-29T15:23:02Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmp9f0IdJ"
[2022-08-29T15:23:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-08-29T15:23:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-08-29T15:23:02Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp9f0IdJ#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmp9f0IdJ\\incremental-state"
[2022-08-29T15:23:09Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:23:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-08-29T15:23:10Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpBGql5N#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:23:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-08-29T15:23:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-08-29T15:23:39Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpBGql5N#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpBGql5N\\incremental-state"
[2022-08-29T15:24:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-08-29T15:24:13Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpBGql5N#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpBGql5N\\incremental-state"
[2022-08-29T15:24:19Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpBGql5N"
[2022-08-29T15:24:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-08-29T15:24:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-08-29T15:24:19Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpBGql5N#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpBGql5N\\incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2022-08-29T15:24:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-08-29T15:24:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-08-29T15:24:27Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:24:27Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-08-29T15:24:27Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpVaHhJl#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:24:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-08-29T15:24:29Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpVaHhJl#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpVaHhJl\\incremental-state"
[2022-08-29T15:24:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-08-29T15:24:30Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpVaHhJl#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpVaHhJl\\incremental-state"
[2022-08-29T15:24:31Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:24:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-08-29T15:24:32Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp8IvllW#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:24:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-08-29T15:24:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-08-29T15:24:33Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp8IvllW#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmp8IvllW\\incremental-state"
[2022-08-29T15:24:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-08-29T15:24:35Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp8IvllW#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmp8IvllW\\incremental-state"
[2022-08-29T15:24:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:24:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-08-29T15:24:36Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpiE7uHV#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:24:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-08-29T15:24:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-08-29T15:24:38Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpiE7uHV#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpiE7uHV\\incremental-state"
[2022-08-29T15:24:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-08-29T15:24:40Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpiE7uHV#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpiE7uHV\\incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2022-08-29T15:24:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-08-29T15:24:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-08-29T15:24:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:24:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-08-29T15:24:42Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp8Biff9#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:24:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-08-29T15:24:46Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp8Biff9#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmp8Biff9\\incremental-state"
[2022-08-29T15:24:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-08-29T15:24:50Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp8Biff9#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmp8Biff9\\incremental-state"
[2022-08-29T15:24:52Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:24:52Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-08-29T15:24:52Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpu1mlYv#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:24:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-08-29T15:24:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-08-29T15:24:56Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpu1mlYv#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpu1mlYv\\incremental-state"
[2022-08-29T15:25:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-08-29T15:25:00Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpu1mlYv#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpu1mlYv\\incremental-state"
[2022-08-29T15:25:02Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:25:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-08-29T15:25:03Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpClhXD8#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:25:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-08-29T15:25:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-08-29T15:25:06Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpClhXD8#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpClhXD8\\incremental-state"
[2022-08-29T15:25:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-08-29T15:25:11Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpClhXD8#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpClhXD8\\incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2022-08-29T15:25:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-08-29T15:25:13Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-08-29T15:25:14Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:25:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-08-29T15:25:14Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpzDupBx#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:25:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-08-29T15:25:15Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpzDupBx#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpzDupBx\\incremental-state"
[2022-08-29T15:25:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-08-29T15:25:16Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpzDupBx#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpzDupBx\\incremental-state"
[2022-08-29T15:25:16Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:25:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-08-29T15:25:16Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpTXt2ZK#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:25:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-08-29T15:25:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-08-29T15:25:17Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpTXt2ZK#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpTXt2ZK\\incremental-state"
[2022-08-29T15:25:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-08-29T15:25:18Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpTXt2ZK#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpTXt2ZK\\incremental-state"
[2022-08-29T15:25:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:25:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-08-29T15:25:18Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpmRelMB#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:25:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-08-29T15:25:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-08-29T15:25:19Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpmRelMB#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpmRelMB\\incremental-state"
[2022-08-29T15:25:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-08-29T15:25:20Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpmRelMB#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpmRelMB\\incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2022-08-29T15:25:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-08-29T15:25:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-08-29T15:25:21Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:25:21Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-08-29T15:25:21Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmph60rfj#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:25:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-08-29T15:25:29Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmph60rfj#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmph60rfj\\incremental-state"
[2022-08-29T15:25:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-08-29T15:25:38Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmph60rfj#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmph60rfj\\incremental-state"
[2022-08-29T15:25:40Z DEBUG collector::execute] applying new row to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmph60rfj"
[2022-08-29T15:25:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-08-29T15:25:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-08-29T15:25:40Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmph60rfj#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmph60rfj\\incremental-state"
[2022-08-29T15:25:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:25:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-08-29T15:25:49Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpZ6BJDc#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:25:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-08-29T15:25:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-08-29T15:25:57Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpZ6BJDc#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpZ6BJDc\\incremental-state"
[2022-08-29T15:26:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-08-29T15:26:06Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpZ6BJDc#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpZ6BJDc\\incremental-state"
[2022-08-29T15:26:08Z DEBUG collector::execute] applying new row to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpZ6BJDc"
[2022-08-29T15:26:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-08-29T15:26:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-08-29T15:26:09Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpZ6BJDc#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpZ6BJDc\\incremental-state"
[2022-08-29T15:26:17Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-08-29T15:26:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-08-29T15:26:17Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpTcb3NN#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-08-29T15:26:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-08-29T15:26:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-08-29T15:26:25Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpTcb3NN#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpTcb3NN\\incremental-state"
[2022-08-29T15:26:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-08-29T15:26:34Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpTcb3NN#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpTcb3NN\\incremental-state"
[2022-08-29T15:26:37Z DEBUG collector::execute] applying new row to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpTcb3NN"
[2022-08-29T15:26:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-08-29T15:26:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-08-29T15:26:37Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpTcb3NN#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpTcb3NN\\incremental-state"
+ cd /d/a/rust/rust
+ RUSTC_PROFILE_MERGED_FILE=/tmp/tmp-pgo/rustc-pgo.profdata
+ /d/a/rust/rust/build/x86_64-pc-windows-msvc/llvm/bin/llvm-profdata merge -o /tmp/tmp-pgo/rustc-pgo.profdata /tmp/tmp-pgo/rustc-pgo
+ echo 'Rustc PGO statistics'
---
[RUSTC-TIMING] arrayvec test:false 0.432
   Compiling memmap2 v0.2.1
[RUSTC-TIMING] build_script_build test:false 0.489
   Compiling cpufeatures v0.2.1
warning: rustc_graphviz.21dd1d3d-cgu.10: no profile data available for function _RNvMNtNtNtCsea2lykkmMDP_4core4iter8adapters3revINtB2_3RevINtNtNtB8_3ops5range5RangejEE3newCsdJHEIP0dEP9_14rustc_graphviz Hash = 742261418966908927 up to 0 count discarded
[RUSTC-TIMING] cpufeatures test:false 0.060
   Compiling serde v1.0.143
[RUSTC-TIMING] ppv_lite86 test:false 0.790
   Compiling unic-char-range v0.9.0
---
[RUSTC-TIMING] build_script_build test:false 0.506
   Compiling either v1.6.0
[RUSTC-TIMING] lazy_static test:false 0.098
   Compiling datafrog v2.0.1
warning: rustc_fs_util.a4f79b1e-cgu.2: no profile data available for function _RNvXs5_NtCs36ww6qJ06UC_5alloc5sliceShNtNtB7_6borrow7ToOwned8to_ownedCs439iO4ZHOIQ_13rustc_fs_util Hash = 48277136972185599 up to 0 count discarded
[RUSTC-TIMING] itoa test:false 0.208
   Compiling memchr v2.5.0
[RUSTC-TIMING] build_script_build test:false 0.504
   Compiling tinyvec v0.3.4
---
[RUSTC-TIMING] rand test:false 1.944
   Compiling petgraph v0.5.1
[RUSTC-TIMING] parking_lot test:false 1.927
   Compiling object v0.29.0
warning: rustc_serialize.1845e4f6-cgu.7: no profile data available for function _RINvNtCsea2lykkmMDP_4core3ptr13drop_in_placeRjECs2bHn5u0Xi28_15rustc_serialize Hash = 742261418966908927 up to 0 count discarded

warning: rustc_serialize.1845e4f6-cgu.7: no profile data available for function _RINvNtCsea2lykkmMDP_4core9panicking13assert_failedjjECs2bHn5u0Xi28_15rustc_serialize Hash = 742261418966908927 up to 0 count discarded

warning: rustc_serialize.1845e4f6-cgu.10: no profile data available for function _RNvXsV_NtCsea2lykkmMDP_4core3fmtRjNtB5_5Debug3fmtCs2bHn5u0Xi28_15rustc_serialize Hash = 1124680650125156080 up to 0 count discarded
[RUSTC-TIMING] termcolor test:false 1.432
   Compiling gimli v0.26.1
[RUSTC-TIMING] rustc_serialize test:false 0.971
warning: `rustc_serialize` (lib) generated 3 warnings
---
   Compiling rustc_driver v0.0.0 (D:\a\rust\rust\compiler\rustc_driver)
[RUSTC-TIMING] rustc_driver test:false 85.992
   Compiling rustc_smir v0.0.0 (D:\a\rust\rust\compiler\rustc_smir)
[RUSTC-TIMING] rustc_smir test:false 0.171
warning: rustc_main.f573a340-cgu.1: no profile data available for function _RNvCsgK9UxTXGUeW_10rustc_main4main Hash = 742261418966908927 up to 0 count discarded
warning: rustc_main.f573a340-cgu.1: no profile data available for function main Hash = 742261418966908927 up to 0 count discarded


warning: rustc_main.f573a340-cgu.2: no profile data available for function _RINvNtNtCsbDixal1W2Jh_3std10sys_common9backtrace28___rust_begin_short_backtraceFEuuECsgK9UxTXGUeW_10rustc_main Hash = 170957022131388415 up to 0 count discarded

warning: rustc_main.f573a340-cgu.0: no profile data available for function _RINvNtCsbDixal1W2Jh_3std2rt10lang_startuECsgK9UxTXGUeW_10rustc_main Hash = 742261418966908927 up to 0 count discarded

warning: rustc_main.f573a340-cgu.0: no profile data available for function _RINvNtCsea2lykkmMDP_4core3ptr13drop_in_placeNCINvNtCsbDixal1W2Jh_3std2rt10lang_startuE0ECsgK9UxTXGUeW_10rustc_main Hash = 742261418966908927 up to 0 count discarded

warning: rustc_main.f573a340-cgu.0: no profile data available for function _RNCINvNtCsbDixal1W2Jh_3std2rt10lang_startuE0CsgK9UxTXGUeW_10rustc_main Hash = 742261418966908927 up to 0 count discarded

warning: rustc_main.f573a340-cgu.0: no profile data available for function _RNSNvYNCINvNtCsbDixal1W2Jh_3std2rt10lang_startuE0INtNtNtCsea2lykkmMDP_4core3ops8function6FnOnceuE9call_once6vtableCsgK9UxTXGUeW_10rustc_main Hash = 742261418966908927 up to 0 count discarded
[RUSTC-TIMING] rustc_main test:false 0.394
warning: `rustc-main` (bin "rustc-main") generated 7 warnings
    Finished release [optimized] target(s) in 8m 20s
Copying stage1 rustc from stage1 (x86_64-pc-windows-msvc -> x86_64-pc-windows-msvc / x86_64-pc-windows-msvc)
