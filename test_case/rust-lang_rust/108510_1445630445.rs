plain
[2023-02-27T02:35:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-27T02:35:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-02-27T02:35:08Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpLty7Db#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-27T02:35:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-02-27T02:35:11Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpLty7Db#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpLty7Db\\incremental-state"
[2023-02-27T02:35:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-02-27T02:35:15Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpLty7Db#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpLty7Db\\incremental-state"
[2023-02-27T02:35:17Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpLty7Db"
[2023-02-27T02:35:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-02-27T02:35:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-02-27T02:35:17Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpLty7Db#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpLty7Db\\incremental-state"
[2023-02-27T02:35:19Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-27T02:35:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-27T02:35:19Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpp3iQFR#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-27T02:35:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-02-27T02:35:22Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-02-27T02:35:22Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpp3iQFR#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpp3iQFR\\incremental-state"
[2023-02-27T02:35:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-02-27T02:35:27Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpp3iQFR#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpp3iQFR\\incremental-state"
[2023-02-27T02:35:29Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpp3iQFR"
[2023-02-27T02:35:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-02-27T02:35:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-02-27T02:35:29Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpp3iQFR#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpp3iQFR\\incremental-state"
[2023-02-27T02:35:31Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-27T02:35:31Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-02-27T02:35:31Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpsLPeqU#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-27T02:35:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-02-27T02:35:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-02-27T02:35:35Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpsLPeqU#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpsLPeqU\\incremental-state"
[2023-02-27T02:35:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-02-27T02:35:39Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpsLPeqU#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpsLPeqU\\incremental-state"
[2023-02-27T02:35:41Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpsLPeqU"
[2023-02-27T02:35:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-02-27T02:35:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-02-27T02:35:41Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpsLPeqU#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpsLPeqU\\incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2023-02-27T02:35:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-02-27T02:35:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2023-02-27T02:39:13Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-27T02:39:17Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-02-27T02:39:17Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpQtCgPO#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-27T02:39:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-02-27T02:39:48Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpQtCgPO#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpQtCgPO\\incremental-state"
[2023-02-27T02:40:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-02-27T02:40:26Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpQtCgPO#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpQtCgPO\\incremental-state"
[2023-02-27T02:40:33Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpQtCgPO"
[2023-02-27T02:40:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-02-27T02:40:33Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-02-27T02:40:33Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpQtCgPO#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpQtCgPO\\incremental-state"
[2023-02-27T02:40:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-27T02:40:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-27T02:40:49Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpLlcAfn#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-27T02:42:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-02-27T02:42:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-02-27T02:42:12Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpLlcAfn#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpLlcAfn\\incremental-state"
[2023-02-27T02:43:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-02-27T02:43:50Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpLlcAfn#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpLlcAfn\\incremental-state"
[2023-02-27T02:44:08Z DEBUG collector::execute] applying println to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpLlcAfn"
[2023-02-27T02:44:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-02-27T02:44:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-02-27T02:44:08Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpLlcAfn#cargo@0.60.0" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpLlcAfn\\incremental-state"
[2023-02-27T02:44:31Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-27T02:44:35Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-02-27T02:44:35Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpR7WotO#cargo@0.60.0" "--release" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-27T02:46:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2023-02-27T02:49:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-27T02:49:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-02-27T02:49:03Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpvpaswk#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-27T02:49:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-02-27T02:49:14Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpvpaswk#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpvpaswk\\incremental-state"
[2023-02-27T02:49:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-02-27T02:49:27Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpvpaswk#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpvpaswk\\incremental-state"
[2023-02-27T02:49:28Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-27T02:49:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-27T02:49:28Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpTgbHYD#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-27T02:49:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-02-27T02:49:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-02-27T02:49:38Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpTgbHYD#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpTgbHYD\\incremental-state"
[2023-02-27T02:49:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-02-27T02:49:52Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpTgbHYD#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpTgbHYD\\incremental-state"
[2023-02-27T02:49:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-27T02:49:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-02-27T02:49:53Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpKOqS9m#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-27T02:50:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2023-02-27T02:54:28Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-27T02:54:28Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-02-27T02:54:28Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmptoBFJM#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-27T02:54:29Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-02-27T02:54:29Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmptoBFJM#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmptoBFJM\\incremental-state"
[2023-02-27T02:54:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-02-27T02:54:31Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmptoBFJM#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmptoBFJM\\incremental-state"
[2023-02-27T02:54:32Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-27T02:54:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-27T02:54:32Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpAzWlDJ#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-27T02:54:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-02-27T02:54:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-02-27T02:54:34Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpAzWlDJ#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpAzWlDJ\\incremental-state"
[2023-02-27T02:54:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-02-27T02:54:35Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpAzWlDJ#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpAzWlDJ\\incremental-state"
[2023-02-27T02:54:37Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-27T02:54:37Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-02-27T02:54:37Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpqmvB3V#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-27T02:54:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-02-27T02:54:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-02-27T02:54:38Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpqmvB3V#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpqmvB3V\\incremental-state"
[2023-02-27T02:54:40Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-02-27T02:54:40Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpqmvB3V#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpqmvB3V\\incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2023-02-27T02:54:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-02-27T02:54:41Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2023-02-27T02:54:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-27T02:54:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-27T02:54:53Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpny9vEL#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-27T02:54:56Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-02-27T02:54:56Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpny9vEL#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpny9vEL\\incremental-state"
[2023-02-27T02:55:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-02-27T02:55:01Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpny9vEL#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpny9vEL\\incremental-state"
[2023-02-27T02:55:03Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-27T02:55:03Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-02-27T02:55:03Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmplynQcv#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-27T02:55:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-02-27T02:55:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-02-27T02:55:07Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmplynQcv#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmplynQcv\\incremental-state"
[2023-02-27T02:55:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-02-27T02:55:12Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmplynQcv#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmplynQcv\\incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2023-02-27T02:55:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-02-27T02:55:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2023-02-27T02:55:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-27T02:55:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-27T02:55:18Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpBZAhvX#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-27T02:55:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-02-27T02:55:18Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpBZAhvX#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpBZAhvX\\incremental-state"
[2023-02-27T02:55:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-02-27T02:55:19Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpBZAhvX#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpBZAhvX\\incremental-state"
[2023-02-27T02:55:20Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-27T02:55:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-02-27T02:55:20Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpLX8hjp#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-27T02:55:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-02-27T02:55:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-02-27T02:55:21Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpLX8hjp#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpLX8hjp\\incremental-state"
[2023-02-27T02:55:21Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-02-27T02:55:21Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpLX8hjp#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpLX8hjp\\incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2023-02-27T02:55:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-02-27T02:55:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2023-02-27T02:55:23Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-27T02:55:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-02-27T02:55:23Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpUADTDo#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-27T02:55:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-02-27T02:55:31Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpUADTDo#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpUADTDo\\incremental-state"
[2023-02-27T02:55:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-02-27T02:55:39Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpUADTDo#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpUADTDo\\incremental-state"
[2023-02-27T02:55:42Z DEBUG collector::execute] applying new row to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpUADTDo"
[2023-02-27T02:55:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-02-27T02:55:42Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-02-27T02:55:42Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpUADTDo#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpUADTDo\\incremental-state"
[2023-02-27T02:55:50Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-27T02:55:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-02-27T02:55:50Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp9oinJy#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-27T02:55:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-02-27T02:55:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-02-27T02:55:58Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp9oinJy#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmp9oinJy\\incremental-state"
[2023-02-27T02:56:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-02-27T02:56:07Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp9oinJy#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmp9oinJy\\incremental-state"
[2023-02-27T02:56:10Z DEBUG collector::execute] applying new row to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmp9oinJy"
[2023-02-27T02:56:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-02-27T02:56:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-02-27T02:56:10Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmp9oinJy#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmp9oinJy\\incremental-state"
[2023-02-27T02:56:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-02-27T02:56:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-02-27T02:56:18Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpL8XkHL#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-02-27T02:56:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-02-27T02:56:26Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-02-27T02:56:26Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpL8XkHL#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpL8XkHL\\incremental-state"
[2023-02-27T02:56:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-02-27T02:56:34Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpL8XkHL#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpL8XkHL\\incremental-state"
[2023-02-27T02:56:37Z DEBUG collector::execute] applying new row to "C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpL8XkHL"
[2023-02-27T02:56:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-02-27T02:56:37Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2023-02-27T02:56:37Z DEBUG collector::execute] "\\\\?\\D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\stage0\\bin\\cargo.exe" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///C:/Users/RUNNER~1/AppData/Local/Temp/.tmpL8XkHL#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=C:\\Users\\RUNNER~1\\AppData\\Local\\Temp\\.tmpL8XkHL\\incremental-state"
stage-build DEBUG: Reverting working dir to `D:\a\rust\rust`
stage-build INFO: Merging Rustc PGO profiles to D:\a\rust\rust\opt-artifacts\rustc-pgo.profdata
stage-build INFO: Executing `D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\bin\llvm-profdata merge -o D:\a\rust\rust\opt-artifacts\rustc-pgo.profdata D:\a\rust\rust\opt-artifacts\rustc-pgo`
stage-build INFO: Executing `D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\bin\llvm-profdata merge -o D:\a\rust\rust\opt-artifacts\rustc-pgo.profdata D:\a\rust\rust\opt-artifacts\rustc-pgo`
warning: D:\a\rust\rust\opt-artifacts\rustc-pgo\default_15195293572841935730_0.profraw: invalid instrumentation profile data (bad magic)
error: no profile can be merged
stage-build INFO: Section `Stage 2 (rustc PGO) > Gather profiles` ended: FAIL (1416.47s)
stage-build INFO: Section `Stage 2 (rustc PGO)` ended: FAIL (2508.67s)
stage-build ERROR: The multi-stage build has failed
------------------------------------------------
Stage 1 (LLVM PGO):            3013.78s (54.57%)
  Build rustc and LLVM:        2213.15s (40.08%)
    LLVM:                       333.91s ( 6.05%)
---
Total duration:                        1h 32m 2s
------------------------------------------------
root INFO: Free disk space: 35.35 GiB out of total 56.00 GiB (36.87% used)
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
  File "C:\hostedtoolcache\windows\Python\3.11.2\x64\Lib\subprocess.py", line 571, in run
  File "C:\hostedtoolcache\windows\Python\3.11.2\x64\Lib\subprocess.py", line 571, in run
    raise CalledProcessError(retcode, process.args,
subprocess.CalledProcessError: Command '['D:\\a\\rust\\rust\\build\\x86_64-pc-windows-msvc\\llvm\\bin\\llvm-profdata', 'merge', '-o', 'D:\\a\\rust\\rust\\opt-artifacts\\rustc-pgo.profdata', 'D:\\a\\rust\\rust\\opt-artifacts\\rustc-pgo']' returned non-zero exit status 1.
