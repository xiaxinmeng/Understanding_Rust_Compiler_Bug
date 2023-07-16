plain
7 benchmarks remaining
Preparing clap-rs
[2022-03-18T11:10:44Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-18T11:10:44Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-18T11:10:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQeXwi8#clap:2.29.0" "--release" "--" "--skip-this-rustc"
[2022-03-18T11:10:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprrDDNG#clap:2.29.0" "--" "--skip-this-rustc"
[2022-03-18T11:10:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:10:45Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-18T11:10:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppawgi2#clap:2.29.0" "--" "--wrap-rustc-with" "eprintln"
Running clap-rs: Opt + [Full]
Running clap-rs: Opt + [Full]
[2022-03-18T11:10:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:10:53Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-18T11:10:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpr1d2kL#clap:2.29.0" "--release" "--" "--wrap-rustc-with" "eprintln"
6 benchmarks remaining
Preparing deeply-nested-async
[2022-03-18T11:11:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-18T11:11:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-18T11:11:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp5ePzD5#deeply-nested-async:0.1.0" "--" "--skip-this-rustc"
[2022-03-18T11:11:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpedA7bI#deeply-nested-async:0.1.0" "--release" "--" "--skip-this-rustc"
[2022-03-18T11:11:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:11:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-18T11:11:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-18T11:11:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXDmUol#deeply-nested-async:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-03-18T11:11:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:11:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-18T11:11:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp0cXP6d#deeply-nested-async:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
5 benchmarks remaining
---
[2022-03-18T11:11:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpm4Hh9k#hyper:0.13.0-alpha.4" "--" "--skip-this-rustc"
Running hyper-2: Debug + [Full]
[2022-03-18T11:11:28Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:11:29Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-18T11:11:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSKEaZf#hyper:0.13.0-alpha.4" "--" "--wrap-rustc-with" "eprintln"
[2022-03-18T11:11:32Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:11:32Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-18T11:11:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTel0TM#hyper:0.13.0-alpha.4" "--release" "--" "--wrap-rustc-with" "eprintln"
4 benchmarks remaining
4 benchmarks remaining
Preparing regex
[2022-03-18T11:11:38Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-18T11:11:38Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-18T11:11:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpquqE6Q#regex:0.1.80" "--" "--skip-this-rustc"
[2022-03-18T11:11:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpW6hXr2#regex:0.1.80" "--release" "--" "--skip-this-rustc"
[2022-03-18T11:11:43Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:11:43Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-18T11:11:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpfsg4iC#regex:0.1.80" "--" "--wrap-rustc-with" "eprintln"
Running regex: Opt + [Full]
---
[2022-03-18T11:11:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpw66k5J#ripgrep:0.8.1" "--release" "--" "--skip-this-rustc"
Running ripgrep: Debug + [Full]
[2022-03-18T11:12:20Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:12:20Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-18T11:12:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzF8Nvg#ripgrep:0.8.1" "--" "--wrap-rustc-with" "eprintln"
[2022-03-18T11:12:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:12:25Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-18T11:12:25Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-18T11:12:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPWH7vO#ripgrep:0.8.1" "--release" "--" "--wrap-rustc-with" "eprintln"
Preparing serde
[2022-03-18T11:12:41Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-18T11:12:41Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-18T11:12:41Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-18T11:12:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "serde/Cargo.toml" "-p" "file:///tmp/.tmpsn7rqR/serde#1.0.37" "--" "--skip-this-rustc"
[2022-03-18T11:12:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "serde/Cargo.toml" "-p" "file:///tmp/.tmpv1mNCj/serde#1.0.37" "--release" "--" "--skip-this-rustc"
[2022-03-18T11:12:41Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:12:41Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-18T11:12:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "serde/Cargo.toml" "-p" "file:///tmp/.tmpNxtu0u/serde#1.0.37" "--" "--wrap-rustc-with" "eprintln"
Running serde: Opt + [Full]
Running serde: Opt + [Full]
[2022-03-18T11:12:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:12:45Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-18T11:12:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "serde/Cargo.toml" "-p" "file:///tmp/.tmp8mJEbI/serde#1.0.37" "--release" "--" "--wrap-rustc-with" "eprintln"
1 benchmark remaining
Preparing syn
[2022-03-18T11:12:48Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-18T11:12:48Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-18T11:12:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptigDpy#syn:0.11.11" "--" "--skip-this-rustc"
[2022-03-18T11:12:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgwybQI#syn:0.11.11" "--release" "--" "--skip-this-rustc"
[2022-03-18T11:12:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:12:53Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-18T11:12:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpYHITg3#syn:0.11.11" "--" "--wrap-rustc-with" "eprintln"
Running syn: Opt + [Full]
Running syn: Opt + [Full]
[2022-03-18T11:12:54Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:12:54Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-18T11:12:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHzIkoa#syn:0.11.11" "--release" "--" "--wrap-rustc-with" "eprintln"
+ /rustroot/bin/llvm-profdata merge -o /tmp/llvm-pgo.profdata ./build/x86_64-unknown-linux-gnu/llvm/build/profiles
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py build --target=x86_64-unknown-linux-gnu --host=x86_64-unknown-linux-gnu --stage 2 library/std --rust-profile-generate=/tmp/rustc-pgo
warning: sqlite3 not available in python, skipping build directory lock
---
[2022-03-18T11:26:04Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:26:04Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-03-18T11:26:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaisRch#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-03-18T11:26:17Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-03-18T11:26:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaisRch#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpaisRch/incremental-state"
[2022-03-18T11:26:31Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-03-18T11:26:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaisRch#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpaisRch/incremental-state"
[2022-03-18T11:26:34Z DEBUG collector::execute] applying println to "/tmp/.tmpaisRch"
[2022-03-18T11:26:34Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-18T11:26:34Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-18T11:26:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaisRch#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpaisRch/incremental-state"
[2022-03-18T11:26:41Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:26:41Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-18T11:26:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpl3lrFD#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-03-18T11:27:05Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
---
[2022-03-18T11:29:33Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:29:33Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-03-18T11:29:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLAjJKs#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-03-18T11:29:56Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-03-18T11:29:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLAjJKs#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpLAjJKs/incremental-state"
[2022-03-18T11:30:23Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-03-18T11:30:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLAjJKs#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpLAjJKs/incremental-state"
[2022-03-18T11:30:23Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:30:23Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-18T11:30:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMJCIDH#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-03-18T11:30:43Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-18T11:30:43Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-18T11:30:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMJCIDH#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpMJCIDH/incremental-state"
[2022-03-18T11:31:06Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-03-18T11:31:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMJCIDH#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpMJCIDH/incremental-state"
[2022-03-18T11:31:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:31:07Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-18T11:31:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp9j5MJ2#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-03-18T11:31:26Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
---
[2022-03-18T11:31:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:31:49Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-03-18T11:31:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBaaUP8#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-03-18T11:31:50Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-03-18T11:31:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBaaUP8#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpBaaUP8/incremental-state"
[2022-03-18T11:31:50Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-03-18T11:31:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBaaUP8#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpBaaUP8/incremental-state"
[2022-03-18T11:31:51Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:31:51Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-18T11:31:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpayqPeJ#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-03-18T11:31:52Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-18T11:31:52Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-18T11:31:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpayqPeJ#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpayqPeJ/incremental-state"
[2022-03-18T11:31:53Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-03-18T11:31:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpayqPeJ#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpayqPeJ/incremental-state"
[2022-03-18T11:31:54Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:31:54Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-18T11:31:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpB3eUnh#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-03-18T11:31:55Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-18T11:31:55Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-18T11:31:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpB3eUnh#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpB3eUnh/incremental-state"
[2022-03-18T11:31:56Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-03-18T11:31:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpB3eUnh#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpB3eUnh/incremental-state"
Preparing inflate
[2022-03-18T11:31:57Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-18T11:31:57Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-18T11:31:57Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
---
[2022-03-18T11:31:57Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:31:57Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-03-18T11:31:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8cSFXK#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-03-18T11:32:00Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-03-18T11:32:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8cSFXK#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp8cSFXK/incremental-state"
[2022-03-18T11:32:02Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-03-18T11:32:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8cSFXK#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp8cSFXK/incremental-state"
[2022-03-18T11:32:03Z DEBUG collector::execute] applying println to "/tmp/.tmp8cSFXK"
[2022-03-18T11:32:03Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-18T11:32:03Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-18T11:32:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8cSFXK#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp8cSFXK/incremental-state"
[2022-03-18T11:32:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:32:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-18T11:32:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp8doUy9#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-03-18T11:32:08Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
---
[2022-03-18T11:32:15Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:32:15Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-18T11:32:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7nJppQ#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-03-18T11:32:20Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-18T11:32:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7nJppQ#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp7nJppQ/incremental-state"
[2022-03-18T11:32:26Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-03-18T11:32:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7nJppQ#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp7nJppQ/incremental-state"
[2022-03-18T11:32:26Z DEBUG collector::execute] applying println to "/tmp/.tmp7nJppQ"
[2022-03-18T11:32:26Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-18T11:32:26Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-18T11:32:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp7nJppQ#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp7nJppQ/incremental-state"
Preparing match-stress-enum
[2022-03-18T11:32:32Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-03-18T11:32:32Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-18T11:32:32Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
---
[2022-03-18T11:32:33Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:32:33Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-03-18T11:32:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzqspSv#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-03-18T11:32:35Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-03-18T11:32:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzqspSv#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpzqspSv/incremental-state"
[2022-03-18T11:32:37Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-03-18T11:32:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzqspSv#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpzqspSv/incremental-state"
[2022-03-18T11:32:37Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:32:37Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-18T11:32:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppeUSED#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-03-18T11:32:39Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-18T11:32:39Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-18T11:32:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppeUSED#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmppeUSED/incremental-state"
[2022-03-18T11:32:41Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-03-18T11:32:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppeUSED#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmppeUSED/incremental-state"
[2022-03-18T11:32:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:32:42Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-18T11:32:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpf5FbhA#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-03-18T11:32:44Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-18T11:32:44Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-18T11:32:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpf5FbhA#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpf5FbhA/incremental-state"
[2022-03-18T11:32:46Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-03-18T11:32:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpf5FbhA#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpf5FbhA/incremental-state"
Preparing token-stream-stress
[2022-03-18T11:32:46Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-03-18T11:32:46Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-18T11:32:46Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
---
[2022-03-18T11:32:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:32:48Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-03-18T11:32:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFSqoVf#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-03-18T11:32:48Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-03-18T11:32:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFSqoVf#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpFSqoVf/incremental-state"
[2022-03-18T11:32:48Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-03-18T11:32:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFSqoVf#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpFSqoVf/incremental-state"
[2022-03-18T11:32:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:32:48Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-18T11:32:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpemJR5m#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-03-18T11:32:49Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-18T11:32:49Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-18T11:32:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpemJR5m#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpemJR5m/incremental-state"
[2022-03-18T11:32:49Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-03-18T11:32:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpemJR5m#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpemJR5m/incremental-state"
[2022-03-18T11:32:49Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-18T11:32:49Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-18T11:32:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRGDjH7#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-03-18T11:32:50Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-18T11:32:50Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-18T11:32:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRGDjH7#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpRGDjH7/incremental-state"
[2022-03-18T11:32:50Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-03-18T11:32:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRGDjH7#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpRGDjH7/incremental-state"
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu --include-default-paths build-manifest --rust-profile-use=/tmp/rustc-pgo.profdata --llvm-profile-use=/tmp/llvm-pgo.profdata
warning: sqlite3 not available in python, skipping build directory lock
---
   Compiling clippy_lints v0.1.61 (/checkout/src/tools/clippy/clippy_lints)
[RUSTC-TIMING] pulldown_cmark test:false 2.288
[RUSTC-TIMING] toml test:false 2.784
[RUSTC-TIMING] cargo_metadata test:false 4.348
error[E0599]: no variant or associated item named `I8` found for enum `rustc_hir::LangItem` in the current scope
   |
86 |                     LangItem::I8,
   |                               ^^ variant or associated item not found in `rustc_hir::LangItem`


error[E0599]: no variant or associated item named `I16` found for enum `rustc_hir::LangItem` in the current scope
   |
87 |                     LangItem::I16,
   |                               ^^^ variant or associated item not found in `rustc_hir::LangItem`


error[E0599]: no variant or associated item named `I32` found for enum `rustc_hir::LangItem` in the current scope
   |
88 |                     LangItem::I32,
   |                               ^^^ variant or associated item not found in `rustc_hir::LangItem`


error[E0599]: no variant or associated item named `I64` found for enum `rustc_hir::LangItem` in the current scope
   |
89 |                     LangItem::I64,
   |                               ^^^ variant or associated item not found in `rustc_hir::LangItem`


error[E0599]: no variant or associated item named `Isize` found for enum `rustc_hir::LangItem` in the current scope
   |
90 |                     LangItem::Isize
   |                               ^^^^^ variant or associated item not found in `rustc_hir::LangItem`


error[E0599]: no method named `slice_impl` found for reference `&LanguageItems` in the current scope
    |
427 | ...                   cx.tcx.lang_items().slice_impl()
427 | ...                   cx.tcx.lang_items().slice_impl()
    |                                           ^^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `str_impl` found for reference `&LanguageItems` in the current scope
    |
    |
432 | ...                   cx.tcx.lang_items().str_impl()
    |                                           ^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `slice_alloc_impl` found for reference `&LanguageItems` in the current scope
   |
   |
54 |                 .map(|impl_did| Some(impl_did) == cx.tcx.lang_items().slice_alloc_impl())
   |                                                                       ^^^^^^^^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `slice_impl` found for reference `&LanguageItems` in the current scope
  --> src/tools/clippy/clippy_lints/src/methods/suspicious_splitn.rs:16:23
   |
16 |         if lang_items.slice_impl() == Some(impl_id) || lang_items.str_impl() == Some(impl_id);
   |                       ^^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `str_impl` found for reference `&LanguageItems` in the current scope
  --> src/tools/clippy/clippy_lints/src/methods/suspicious_splitn.rs:16:67
   |
16 |         if lang_items.slice_impl() == Some(impl_id) || lang_items.str_impl() == Some(impl_id);
   |                                                                   ^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `slice_impl` found for reference `&LanguageItems` in the current scope
  --> src/tools/clippy/clippy_lints/src/methods/suspicious_splitn.rs:31:31
   |
31 |                 if lang_items.slice_impl() == Some(impl_id) {
   |                               ^^^^^^^^^^ method not found in `&LanguageItems`
error[E0277]: the size for values of type `str` cannot be known at compilation time
  --> src/tools/clippy/clippy_lints/src/methods/suspicious_splitn.rs:26:18
   |
   |
26 |             let (msg, note_msg) = if count == 0 {
   |
   = help: the trait `std::marker::Sized` is not implemented for `str`
   = note: all local variables must have a statically known size
   = help: unsized locals are gated as an unstable feature
---
[RUSTC-TIMING] serde test:false 4.039
   Compiling toml v0.5.7
[RUSTC-TIMING] tokio_util test:false 1.333
   Compiling futures-executor v0.3.19
error[E0599]: no variant or associated item named `I8` found for enum `rustc_hir::LangItem` in the current scope
   |
86 |                     LangItem::I8,
   |                               ^^ variant or associated item not found in `rustc_hir::LangItem`


error[E0599]: no variant or associated item named `I16` found for enum `rustc_hir::LangItem` in the current scope
   |
87 |                     LangItem::I16,
   |                               ^^^ variant or associated item not found in `rustc_hir::LangItem`


error[E0599]: no variant or associated item named `I32` found for enum `rustc_hir::LangItem` in the current scope
   |
88 |                     LangItem::I32,
   |                               ^^^ variant or associated item not found in `rustc_hir::LangItem`


error[E0599]: no variant or associated item named `I64` found for enum `rustc_hir::LangItem` in the current scope
   |
89 |                     LangItem::I64,
   |                               ^^^ variant or associated item not found in `rustc_hir::LangItem`


error[E0599]: no variant or associated item named `Isize` found for enum `rustc_hir::LangItem` in the current scope
   |
90 |                     LangItem::Isize
   |                               ^^^^^ variant or associated item not found in `rustc_hir::LangItem`


error[E0599]: no method named `slice_impl` found for reference `&LanguageItems` in the current scope
    |
427 | ...                   cx.tcx.lang_items().slice_impl()
427 | ...                   cx.tcx.lang_items().slice_impl()
    |                                           ^^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `str_impl` found for reference `&LanguageItems` in the current scope
    |
    |
432 | ...                   cx.tcx.lang_items().str_impl()
    |                                           ^^^^^^^^ method not found in `&LanguageItems`
[RUSTC-TIMING] rayon test:false 3.145
   Compiling futures v0.3.19
[RUSTC-TIMING] futures test:false 0.065
   Compiling jsonrpc-core v18.0.0
---
[RUSTC-TIMING] parity_tokio_ipc test:false 0.740
   Compiling jsonrpc-pubsub v18.0.0
[RUSTC-TIMING] futures_executor test:false 1.751
   Compiling jsonrpc-ipc-server v18.0.0
error[E0599]: no method named `slice_alloc_impl` found for reference `&LanguageItems` in the current scope
   |
   |
54 |                 .map(|impl_did| Some(impl_did) == cx.tcx.lang_items().slice_alloc_impl())
   |                                                                       ^^^^^^^^^^^^^^^^ method not found in `&LanguageItems`
[RUSTC-TIMING] toml test:false 2.238
   Compiling jsonrpc-client-transports v18.0.0
   Compiling jsonrpc-client-transports v18.0.0
error[E0599]: no method named `slice_impl` found for reference `&LanguageItems` in the current scope
  --> src/tools/clippy/clippy_lints/src/methods/suspicious_splitn.rs:16:23
   |
16 |         if lang_items.slice_impl() == Some(impl_id) || lang_items.str_impl() == Some(impl_id);
   |                       ^^^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `str_impl` found for reference `&LanguageItems` in the current scope
  --> src/tools/clippy/clippy_lints/src/methods/suspicious_splitn.rs:16:67
   |
16 |         if lang_items.slice_impl() == Some(impl_id) || lang_items.str_impl() == Some(impl_id);
   |                                                                   ^^^^^^^^ method not found in `&LanguageItems`

error[E0599]: no method named `slice_impl` found for reference `&LanguageItems` in the current scope
  --> src/tools/clippy/clippy_lints/src/methods/suspicious_splitn.rs:31:31
   |
31 |                 if lang_items.slice_impl() == Some(impl_id) {
   |                               ^^^^^^^^^^ method not found in `&LanguageItems`
error[E0277]: the size for values of type `str` cannot be known at compilation time
  --> src/tools/clippy/clippy_lints/src/methods/suspicious_splitn.rs:26:18
   |
   |
26 |             let (msg, note_msg) = if count == 0 {
   |
   = help: the trait `std::marker::Sized` is not implemented for `str`
   = note: all local variables must have a statically known size
   = help: unsized locals are gated as an unstable feature
---
[TIMING] RustDemangler { compiler: Compiler { stage: 1, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None }, extra_features: [] } -- 0.000
Dist rust-demangler-nightly-x86_64-unknown-linux-gnu
 finished in 1.746 seconds
[TIMING] RustDemangler { compiler: Compiler { stage: 1, host: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } }, target: TargetSelection { triple: "x86_64-unknown-linux-gnu", file: None } } -- 1.753
thread 'main' panicked at 'clippy expected to build - essential tool', src/bootstrap/dist.rs:1126:14
Build completed unsuccessfully in 0:29:58
