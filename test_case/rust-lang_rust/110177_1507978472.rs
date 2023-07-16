plain
[2023-04-13T20:16:52Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-13T20:16:52Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-13T20:16:52Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpjqbaH9#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-13T20:16:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-04-13T20:16:54Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpjqbaH9#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpjqbaH9\\incremental-state"
[2023-04-13T20:16:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-04-13T20:16:57Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpjqbaH9#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpjqbaH9\\incremental-state"
[2023-04-13T20:16:58Z DEBUG collector::benchmark::patch] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpjqbaH9"
[2023-04-13T20:16:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-04-13T20:16:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-04-13T20:16:58Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpjqbaH9#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpjqbaH9\\incremental-state"
[2023-04-13T20:16:59Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-13T20:16:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-13T20:17:00Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpy09eSA#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-13T20:17:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2023-04-13T20:21:47Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-13T20:21:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-13T20:21:50Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmptrwYuc#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-13T20:22:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-04-13T20:22:47Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmptrwYuc#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmptrwYuc\\incremental-state"
[2023-04-13T20:23:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-04-13T20:23:55Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmptrwYuc#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmptrwYuc\\incremental-state"
[2023-04-13T20:24:07Z DEBUG collector::benchmark::patch] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmptrwYuc"
[2023-04-13T20:24:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-04-13T20:24:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-04-13T20:24:07Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmptrwYuc#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmptrwYuc\\incremental-state"
[2023-04-13T20:24:22Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-13T20:24:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-13T20:24:25Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpg3iQV2#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-13T20:25:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2023-04-13T20:28:08Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-13T20:28:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-13T20:28:08Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpKcCJRz#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-13T20:28:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-04-13T20:28:15Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpKcCJRz#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpKcCJRz\\incremental-state"
[2023-04-13T20:28:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-04-13T20:28:24Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpKcCJRz#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpKcCJRz\\incremental-state"
[2023-04-13T20:28:25Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-13T20:28:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-13T20:28:25Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpdIULqn#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-13T20:28:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-04-13T20:28:32Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-04-13T20:28:32Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpdIULqn#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpdIULqn\\incremental-state"
[2023-04-13T20:28:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-04-13T20:28:41Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpdIULqn#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpdIULqn\\incremental-state"
[2023-04-13T20:28:42Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-13T20:28:42Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-13T20:28:42Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpVZWWrz#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-13T20:28:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-04-13T20:28:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-04-13T20:28:49Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpVZWWrz#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpVZWWrz\\incremental-state"
[2023-04-13T20:28:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-04-13T20:28:58Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpVZWWrz#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpVZWWrz\\incremental-state"
Executing benchmark diesel-1.4.8 (4/8)
Preparing diesel-1.4.8
[2023-04-13T20:28:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-04-13T20:28:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2023-04-13T20:30:05Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-13T20:30:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-13T20:30:05Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpwqZ2QN#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-13T20:30:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-04-13T20:30:24Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpwqZ2QN#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpwqZ2QN\\incremental-state"
[2023-04-13T20:30:46Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-04-13T20:30:46Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpwqZ2QN#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpwqZ2QN\\incremental-state"
[2023-04-13T20:30:51Z DEBUG collector::benchmark::patch] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpwqZ2QN"
[2023-04-13T20:30:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-04-13T20:30:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-04-13T20:30:51Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpwqZ2QN#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpwqZ2QN\\incremental-state"
[2023-04-13T20:30:56Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-13T20:30:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-13T20:30:56Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpdD2K3N#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-13T20:31:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2023-04-13T20:31:51Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-13T20:31:51Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-13T20:31:51Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpfRRAUw#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-13T20:31:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-04-13T20:31:52Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpfRRAUw#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpfRRAUw\\incremental-state"
[2023-04-13T20:31:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-04-13T20:31:53Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpfRRAUw#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpfRRAUw\\incremental-state"
[2023-04-13T20:31:54Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-13T20:31:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-13T20:31:54Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpEHI7zi#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-13T20:31:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-04-13T20:31:55Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-04-13T20:31:55Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpEHI7zi#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpEHI7zi\\incremental-state"
[2023-04-13T20:31:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-04-13T20:31:56Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpEHI7zi#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpEHI7zi\\incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2023-04-13T20:31:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-04-13T20:31:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2023-04-13T20:31:58Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-13T20:31:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-13T20:31:58Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpsNItvP#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-13T20:32:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-04-13T20:32:00Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpsNItvP#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpsNItvP\\incremental-state"
[2023-04-13T20:32:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-04-13T20:32:03Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpsNItvP#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpsNItvP\\incremental-state"
[2023-04-13T20:32:05Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-13T20:32:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-13T20:32:05Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpbuMXnm#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-13T20:32:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-04-13T20:32:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-04-13T20:32:07Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpbuMXnm#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpbuMXnm\\incremental-state"
[2023-04-13T20:32:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-04-13T20:32:10Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpbuMXnm#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpbuMXnm\\incremental-state"
[2023-04-13T20:32:12Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-13T20:32:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-13T20:32:12Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpNxNZ5O#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-13T20:32:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-04-13T20:32:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-04-13T20:32:14Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpNxNZ5O#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpNxNZ5O\\incremental-state"
[2023-04-13T20:32:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-04-13T20:32:18Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpNxNZ5O#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpNxNZ5O\\incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2023-04-13T20:32:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-04-13T20:32:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2023-04-13T20:32:22Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-13T20:32:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-13T20:32:22Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpDs0IzZ#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-13T20:32:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-04-13T20:32:22Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpDs0IzZ#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpDs0IzZ\\incremental-state"
[2023-04-13T20:32:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-04-13T20:32:23Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpDs0IzZ#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpDs0IzZ\\incremental-state"
[2023-04-13T20:32:23Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-13T20:32:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-04-13T20:32:23Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpGOUJUT#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-13T20:32:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-04-13T20:32:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-04-13T20:32:24Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpGOUJUT#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpGOUJUT\\incremental-state"
[2023-04-13T20:32:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-04-13T20:32:24Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpGOUJUT#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpGOUJUT\\incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2023-04-13T20:32:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-04-13T20:32:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2023-04-13T20:32:25Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-13T20:32:25Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-04-13T20:32:25Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmppNQHSV#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-13T20:32:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-04-13T20:32:31Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmppNQHSV#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmppNQHSV\\incremental-state"
[2023-04-13T20:32:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-04-13T20:32:38Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmppNQHSV#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmppNQHSV\\incremental-state"
[2023-04-13T20:32:40Z DEBUG collector::benchmark::patch] applying new row to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmppNQHSV"
[2023-04-13T20:32:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-04-13T20:32:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-04-13T20:32:40Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmppNQHSV#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmppNQHSV\\incremental-state"
[2023-04-13T20:32:46Z DEBUG collector::benchmark] Benchmark iteration 1/1
[2023-04-13T20:32:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-04-13T20:32:46Z DEBUG collector::execute] "\\\\?\\C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpOD0EOp#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-04-13T20:32:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
Finished benchmark tuple-stress (8/8)
stage-build DEBUG: Reverting working dir to `C:\a\rust\rust`
stage-build INFO: Merging Rustc PGO profiles to C:\a\rust\rust\opt-artifacts\rustc-pgo.profdata
stage-build INFO: Executing `C:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\bin\llvm-profdata merge -o C:\a\rust\rust\opt-artifacts\rustc-pgo.profdata C:\a\rust\rust\opt-artifacts\rustc-pgo`
warning: C:\a\rust\rust\opt-artifacts\rustc-pgo\default_2593420346809951494_0.profraw: malformed instrumentation profile data: not enough space for another header
error: no profile can be merged
stage-build INFO: Section `Stage 2 (rustc PGO) > Gather profiles` ended: FAIL (1076.14s)
stage-build INFO: Section `Stage 2 (rustc PGO)` ended: FAIL (2139.93s)
stage-build ERROR: The multi-stage build has failed
------------------------------------------------
Stage 1 (LLVM PGO):            2650.18s (55.33%)
  Build rustc and LLVM:        1872.60s (39.09%)
    LLVM:                       406.27s ( 8.48%)
---
Total duration:                       1h 19m 50s
------------------------------------------------
root INFO: Free disk space: 111.43 GiB out of total 299.51 GiB (62.80% used)
Traceback (most recent call last):
  File "C:\a\rust\rust\src\ci\stage-build.py", line 839, in <module>
    raise e
  File "C:\a\rust\rust\src\ci\stage-build.py", line 836, in <module>
    execute_build_pipeline(timer, pipeline, build_args)
  File "C:\a\rust\rust\src\ci\stage-build.py", line 784, in execute_build_pipeline
    gather_rustc_profiles(pipeline)
  File "C:\a\rust\rust\src\ci\stage-build.py", line 629, in gather_rustc_profiles
    cmd([
  File "C:\a\rust\rust\src\ci\stage-build.py", line 452, in cmd
    return subprocess.run(args, env=environment, check=True)
  File "C:\hostedtoolcache\windows\Python\3.11.2\x64\Lib\subprocess.py", line 571, in run
  File "C:\hostedtoolcache\windows\Python\3.11.2\x64\Lib\subprocess.py", line 571, in run
    raise CalledProcessError(retcode, process.args,
subprocess.CalledProcessError: Command '['C:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\bin\\llvm-profdata', 'merge', '-o', 'C:\\a\\rust\\rust\\opt-artifacts\\rustc-pgo.profdata', 'C:\\a\\rust\\rust\\opt-artifacts\\rustc-pgo']' returned non-zero exit status 1.
