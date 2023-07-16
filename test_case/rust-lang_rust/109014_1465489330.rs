plain
[2023-03-11T13:09:26Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-11T13:09:26Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-11T13:09:26Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpvzyGvy#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-11T13:09:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-11T13:09:29Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpvzyGvy#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpvzyGvy\\incremental-state"
[2023-03-11T13:09:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-11T13:09:34Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpvzyGvy#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpvzyGvy\\incremental-state"
[2023-03-11T13:09:36Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpvzyGvy"
[2023-03-11T13:09:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-11T13:09:36Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-11T13:09:36Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpvzyGvy#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpvzyGvy\\incremental-state"
[2023-03-11T13:09:38Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-11T13:09:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-11T13:09:38Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpmZX0i3#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-11T13:09:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2023-03-11T13:13:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-11T13:13:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-11T13:13:50Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpWFRfR1#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-11T13:14:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-11T13:14:23Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpWFRfR1#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpWFRfR1\\incremental-state"
[2023-03-11T13:15:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-11T13:15:04Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpWFRfR1#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpWFRfR1\\incremental-state"
[2023-03-11T13:15:12Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpWFRfR1"
[2023-03-11T13:15:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-11T13:15:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-11T13:15:12Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpWFRfR1#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpWFRfR1\\incremental-state"
[2023-03-11T13:15:25Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-11T13:15:30Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-11T13:15:30Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpfpmtoi#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-11T13:17:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-11T13:17:00Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-11T13:17:00Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpfpmtoi#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpfpmtoi\\incremental-state"
[2023-03-11T13:18:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-03-11T13:18:49Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpfpmtoi#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpfpmtoi\\incremental-state"
[2023-03-11T13:19:09Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpfpmtoi"
[2023-03-11T13:19:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-11T13:19:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-11T13:19:09Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpfpmtoi#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpfpmtoi\\incremental-state"
[2023-03-11T13:19:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-11T13:19:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-11T13:19:41Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpijCB9Y#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-11T13:21:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-11T13:21:44Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-11T13:21:44Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpijCB9Y#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpijCB9Y\\incremental-state"
[2023-03-11T13:23:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-03-11T13:23:53Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpijCB9Y#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpijCB9Y\\incremental-state"
[2023-03-11T13:24:12Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpijCB9Y"
[2023-03-11T13:24:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-11T13:24:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-03-11T13:24:12Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpijCB9Y#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpijCB9Y\\incremental-state"
Executing benchmark ctfe-stress-5 (3/8)
Preparing ctfe-stress-5
[2023-03-11T13:24:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-11T13:24:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2023-03-11T13:24:41Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-11T13:24:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-11T13:24:41Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpbtNTgt#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-11T13:24:54Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-11T13:24:54Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpbtNTgt#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpbtNTgt\\incremental-state"
[2023-03-11T13:25:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-11T13:25:09Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpbtNTgt#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpbtNTgt\\incremental-state"
[2023-03-11T13:25:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-11T13:25:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-11T13:25:10Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpeR0jKG#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-11T13:25:23Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2023-03-11T13:30:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-11T13:30:48Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-11T13:30:48Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpANSDVC#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-11T13:30:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-11T13:30:49Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpANSDVC#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpANSDVC\\incremental-state"
[2023-03-11T13:30:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-03-11T13:30:51Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpANSDVC#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpANSDVC\\incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2023-03-11T13:30:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-11T13:30:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2023-03-11T13:30:54Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-11T13:30:54Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-03-11T13:30:54Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpr9KrFU#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-11T13:30:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-03-11T13:30:58Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpr9KrFU#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpr9KrFU\\incremental-state"
[2023-03-11T13:31:03Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-03-11T13:31:03Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpr9KrFU#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpr9KrFU\\incremental-state"
[2023-03-11T13:31:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-11T13:31:05Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-03-11T13:31:05Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpWiydVh#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-11T13:31:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-11T13:31:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-03-11T13:31:09Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpWiydVh#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpWiydVh\\incremental-state"
[2023-03-11T13:31:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-03-11T13:31:14Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpWiydVh#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpWiydVh\\incremental-state"
[2023-03-11T13:31:17Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-11T13:31:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-11T13:31:17Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpPdSxko#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-11T13:31:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-11T13:31:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-11T13:31:21Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpPdSxko#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpPdSxko\\incremental-state"
[2023-03-11T13:31:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-03-11T13:31:27Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpPdSxko#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpPdSxko\\incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2023-03-11T13:31:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-03-11T13:31:29Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2023-03-11T13:32:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-03-11T13:32:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-03-11T13:32:39Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpsfSull#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-03-11T13:32:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-03-11T13:32:47Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpsfSull#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpsfSull\\incremental-state"
[2023-03-11T13:32:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-03-11T13:32:56Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpsfSull#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpsfSull\\incremental-state"
[2023-03-11T13:32:59Z DEBUG collector::execute] applying new row to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpsfSull"
[2023-03-11T13:32:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-03-11T13:32:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-03-11T13:32:59Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpsfSull#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpsfSull\\incremental-state"
stage-build DEBUG: Reverting working dir to `D:\a\rust\rust`
stage-build INFO: Merging Rustc PGO profiles to D:\a\rust\rust\opt-artifacts\rustc-pgo.profdata
stage-build INFO: Executing `D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\bin\llvm-profdata merge -o D:\a\rust\rust\opt-artifacts\rustc-pgo.profdata D:\a\rust\rust\opt-artifacts\rustc-pgo`
stage-build INFO: Rustc PGO statistics
---
Total duration:                        1h 40m 6s
------------------------------------------------
root INFO: Free disk space: 36.39 GiB out of total 56.00 GiB (35.02% used)
Traceback (most recent call last):
  File "D:\a\rust\rust\src\ci\stage-build.py", line 839, in <module>
    raise e
  File "D:\a\rust\rust\src\ci\stage-build.py", line 836, in <module>
    execute_build_pipeline(timer, pipeline, build_args)
  File "D:\a\rust\rust\src\ci\stage-build.py", line 787, in execute_build_pipeline
    clear_llvm_files(pipeline)
  File "D:\a\rust\rust\src\ci\stage-build.py", line 680, in clear_llvm_files
    delete_directory(pipeline.build_artifacts() / "llvm")
  File "D:\a\rust\rust\src\ci\stage-build.py", line 399, in delete_directory
    shutil.rmtree(path)
  File "C:\hostedtoolcache\windows\Python\3.11.2\x64\Lib\shutil.py", line 759, in rmtree
    return _rmtree_unsafe(path, onerror)
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "C:\hostedtoolcache\windows\Python\3.11.2\x64\Lib\shutil.py", line 617, in _rmtree_unsafe
    _rmtree_unsafe(fullname, onerror)
  File "C:\hostedtoolcache\windows\Python\3.11.2\x64\Lib\shutil.py", line 626, in _rmtree_unsafe
    onerror(os.rmdir, path, sys.exc_info())
  File "C:\hostedtoolcache\windows\Python\3.11.2\x64\Lib\shutil.py", line 624, in _rmtree_unsafe
    os.rmdir(path)
OSError: [WinError 145] The directory is not empty: 'D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\bin'
