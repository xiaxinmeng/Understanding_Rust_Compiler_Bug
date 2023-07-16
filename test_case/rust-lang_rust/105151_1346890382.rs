plain
[2022-12-11T22:42:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-11T22:42:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-11T22:42:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxawbrl#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-11T22:42:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-12-11T22:42:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxawbrl#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxawbrl/incremental-state"
[2022-12-11T22:42:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-12-11T22:42:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxawbrl#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpxawbrl/incremental-state"
[2022-12-11T22:42:58Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-11T22:42:58Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-12-11T22:42:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPb63hu#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-11T22:43:06Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-12-11T22:45:14Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-11T22:45:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-12-11T22:45:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpH2Odwv#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-11T22:45:34Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-12-11T22:45:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpH2Odwv#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpH2Odwv/incremental-state"
[2022-12-11T22:45:58Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-12-11T22:45:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpH2Odwv#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpH2Odwv/incremental-state"
[2022-12-11T22:46:02Z DEBUG collector::execute] applying println to "/tmp/.tmpH2Odwv"
[2022-12-11T22:46:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-11T22:46:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-12-11T22:46:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpH2Odwv#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpH2Odwv/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2022-12-11T22:46:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-12-11T22:46:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-12-11T22:46:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-11T22:46:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-12-11T22:46:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyjGa8o#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-11T22:46:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-12-11T22:46:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyjGa8o#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpyjGa8o/incremental-state"
[2022-12-11T22:46:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-12-11T22:46:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyjGa8o#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpyjGa8o/incremental-state"
[2022-12-11T22:46:09Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-11T22:46:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-11T22:46:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp2W2zux#externs@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-11T22:46:10Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-12-11T22:46:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-11T22:46:11Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-12-11T22:46:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphHkoJT#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-11T22:46:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-12-11T22:46:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphHkoJT#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmphHkoJT/incremental-state"
[2022-12-11T22:46:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-12-11T22:46:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphHkoJT#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmphHkoJT/incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2022-12-11T22:46:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2022-12-11T22:46:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2022-12-11T22:46:14Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-11T22:46:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-12-11T22:46:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSVSMqw#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-11T22:46:17Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-12-11T22:46:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSVSMqw#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpSVSMqw/incremental-state"
[2022-12-11T22:46:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-12-11T22:46:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSVSMqw#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpSVSMqw/incremental-state"
[2022-12-11T22:46:22Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-11T22:46:22Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-11T22:46:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpp549GM#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-11T22:46:24Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-12-11T22:46:38Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-11T22:46:38Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-11T22:46:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwZ6tQG#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-11T22:46:38Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2022-12-11T22:46:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwZ6tQG#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwZ6tQG/incremental-state"
[2022-12-11T22:46:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2022-12-11T22:46:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwZ6tQG#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwZ6tQG/incremental-state"
[2022-12-11T22:46:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-11T22:46:39Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-12-11T22:46:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqrqRRp#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-11T22:46:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
---
[2022-12-11T22:46:40Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-11T22:46:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-12-11T22:46:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBveJm3#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-11T22:46:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-12-11T22:46:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBveJm3#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpBveJm3/incremental-state"
[2022-12-11T22:46:50Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-12-11T22:46:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBveJm3#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpBveJm3/incremental-state"
[2022-12-11T22:46:52Z DEBUG collector::execute] applying new row to "/tmp/.tmpBveJm3"
[2022-12-11T22:46:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-12-11T22:46:52Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-12-11T22:46:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBveJm3#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpBveJm3/incremental-state"
[2022-12-11T22:46:57Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-11T22:46:57Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-12-11T22:46:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp1CAkcT#tuple-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-11T22:47:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2022-12-11T22:47:15Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-12-11T22:47:15Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-12-11T22:47:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpL1tUhy#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2022-12-11T22:47:20Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2022-12-11T22:47:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpL1tUhy#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpL1tUhy/incremental-state"
[2022-12-11T22:47:25Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2022-12-11T22:47:25Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpL1tUhy#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpL1tUhy/incremental-state"
[2022-12-11T22:47:27Z DEBUG collector::execute] applying new row to "/tmp/.tmpL1tUhy"
[2022-12-11T22:47:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-12-11T22:47:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("new row"), path: "0-new-row.patch" })
[2022-12-11T22:47:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpL1tUhy#tuple-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpL1tUhy/incremental-state"
+ cd /checkout/obj
+ RUSTC_PROFILE_MERGED_FILE=/tmp/tmp-pgo/rustc-pgo.profdata
+ /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/tmp-pgo/rustc-pgo.profdata /tmp/tmp-pgo/rustc-pgo
+ echo 'Rustc PGO statistics'
---
[RUSTC-TIMING] git2_curl test:false 0.555
[RUSTC-TIMING] git2 test:false 4.923
[RUSTC-TIMING] toml_edit test:false 37.081
[RUSTC-TIMING] cargo test:false 48.007
error: internal compiler error: compiler/rustc_middle/src/ty/instance.rs:396:18: failed to resolve instance for <FlatMap<std::iter::Map<Flatten<std::option::IntoIter<ValuesRef<'_, std::string::String>>>, for<'a> fn(&'a std::string::String) -> &'a str {std::string::String::as_str}>, std::iter::Filter<FlatMap<SplitWhitespace<'_>, std::str::Split<'_, char>, [closure@src/tools/cargo/src/bin/cargo/commands/add.rs:360:19: 360:22]>, [closure@src/tools/cargo/src/bin/cargo/commands/add.rs:361:17: 361:20]>, for<'a> fn(&'a str) -> impl Iterator<Item = &'a str> {parse_feature}> as Iterator>::next

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/85ab8d000e0073f597aad7bb1c19e9dfc8cbbe17/compiler/rustc_errors/src/lib.rs:1576:9
stack backtrace:
   0:     0x7f0bd6b5f300 - std::backtrace_rs::backtrace::libunwind::trace::h2540c8c07dbc0a17
                               at /rustc/85ab8d000e0073f597aad7bb1c19e9dfc8cbbe17/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f0bd6b5f300 - std::backtrace_rs::backtrace::trace_unsynchronized::h64e0ad4d10fb2017
                               at /rustc/85ab8d000e0073f597aad7bb1c19e9dfc8cbbe17/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f0bd6b5f300 - std::sys_common::backtrace::_print_fmt::h6451477ed0f776b0
                               at /rustc/85ab8d000e0073f597aad7bb1c19e9dfc8cbbe17/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f0bd6b5f300 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h576e21ae7c0954a4
                               at /rustc/85ab8d000e0073f597aad7bb1c19e9dfc8cbbe17/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f0bd6bc17fe - core::fmt::write::hcb131ddd0c15f718
                               at /rustc/85ab8d000e0073f597aad7bb1c19e9dfc8cbbe17/library/core/src/fmt/mod.rs:1208:17
   5:     0x7f0bd6b4f765 - std::io::Write::write_fmt::hec164e5640917b03
                               at /rustc/85ab8d000e0073f597aad7bb1c19e9dfc8cbbe17/library/std/src/io/mod.rs:1682:15
   6:     0x7f0bd6b5f0c5 - std::sys_common::backtrace::_print::h6a73fa5166776a40
                               at /rustc/85ab8d000e0073f597aad7bb1c19e9dfc8cbbe17/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f0bd6b5f0c5 - std::sys_common::backtrace::print::h57ad279d5588234c
                               at /rustc/85ab8d000e0073f597aad7bb1c19e9dfc8cbbe17/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f0bd6b61e1f - std::panicking::default_hook::{{closure}}::hc4b6fa6506eb52c2
                               at /rustc/85ab8d000e0073f597aad7bb1c19e9dfc8cbbe17/library/std/src/panicking.rs:267:22
   9:     0x7f0bd6b61b5a - std::panicking::default_hook::h26b88df41b3e344d
                               at /rustc/85ab8d000e0073f597aad7bb1c19e9dfc8cbbe17/library/std/src/panicking.rs:286:9
  10:     0x7f0bd3d83406 - <rustc_driver[f3569c13e1df7507]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[ae65f184c67bc919]::ops::function::FnOnce<(&core[ae65f184c67bc919]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7f0bd6b62649 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h29104a2a0427fe95
                               at /rustc/85ab8d000e0073f597aad7bb1c19e9dfc8cbbe17/library/alloc/src/boxed.rs:2024:9
  12:     0x7f0bd6b62649 - std::panicking::rust_panic_with_hook::h81de260a74888be4
                               at /rustc/85ab8d000e0073f597aad7bb1c19e9dfc8cbbe17/library/std/src/panicking.rs:692:13
  13:     0x7f0bd632c013 - std[c962dfd4bdbe76a2]::panicking::begin_panic::<rustc_errors[b92b891ac752ae31]::ExplicitBug>::{closure#0}
  14:     0x7f0bd6323986 - std[c962dfd4bdbe76a2]::sys_common::backtrace::__rust_end_short_backtrace::<std[c962dfd4bdbe76a2]::panicking::begin_panic<rustc_errors[b92b891ac752ae31]::ExplicitBug>::{closure#0}, !>
  15:     0x7f0bd62c65e6 - std[c962dfd4bdbe76a2]::panicking::begin_panic::<rustc_errors[b92b891ac752ae31]::ExplicitBug>
  16:     0x7f0bd62fca16 - std[c962dfd4bdbe76a2]::panic::panic_any::<rustc_errors[b92b891ac752ae31]::ExplicitBug>
  17:     0x7f0bd62eab67 - <rustc_errors[b92b891ac752ae31]::HandlerInner>::bug::<&alloc[3a9675dc12b44c41]::string::String>
  18:     0x7f0bd62ea690 - <rustc_errors[b92b891ac752ae31]::Handler>::bug::<&alloc[3a9675dc12b44c41]::string::String>
  19:     0x7f0bd639eb00 - rustc_middle[d8bcd2f6073ec0f4]::ty::context::tls::with_context_opt::<rustc_middle[d8bcd2f6073ec0f4]::ty::context::tls::with_opt<rustc_middle[d8bcd2f6073ec0f4]::util::bug::opt_span_bug_fmt<rustc_span[b384040717ceefb]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  20:     0x7f0bd63a5f89 - rustc_middle[d8bcd2f6073ec0f4]::util::bug::opt_span_bug_fmt::<rustc_span[b384040717ceefb]::span_encoding::Span>
  21:     0x7f0bd63a5f05 - rustc_middle[d8bcd2f6073ec0f4]::util::bug::bug_fmt
  22:     0x7f0bd6319876 - <rustc_middle[d8bcd2f6073ec0f4]::ty::instance::Instance>::expect_resolve
  23:     0x7f0bd44f8ead - <rustc_monomorphize[fe4fa0512615eb28]::collector::MirNeighborCollector as rustc_middle[d8bcd2f6073ec0f4]::mir::visit::Visitor>::visit_terminator
  24:     0x7f0bd44ff443 - rustc_monomorphize[fe4fa0512615eb28]::collector::collect_neighbours
  25:     0x7f0bd44fd107 - rustc_monomorphize[fe4fa0512615eb28]::collector::collect_items_rec
  26:     0x7f0bd44fddc7 - rustc_monomorphize[fe4fa0512615eb28]::collector::collect_items_rec
  27:     0x7f0bd44fddc7 - rustc_monomorphize[fe4fa0512615eb28]::collector::collect_items_rec
  28:     0x7f0bd44fddc7 - rustc_monomorphize[fe4fa0512615eb28]::collector::collect_items_rec
  29:     0x7f0bd44fddc7 - rustc_monomorphize[fe4fa0512615eb28]::collector::collect_items_rec
  30:     0x7f0bd44fddc7 - rustc_monomorphize[fe4fa0512615eb28]::collector::collect_items_rec
  31:     0x7f0bd45007f0 - std[c962dfd4bdbe76a2]::panicking::try::<(), core[ae65f184c67bc919]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d7d58f1eb8f64edc]::sync::par_for_each_in<alloc[3a9675dc12b44c41]::vec::Vec<rustc_middle[d8bcd2f6073ec0f4]::mir::mono::MonoItem>, rustc_monomorphize[fe4fa0512615eb28]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  32:     0x7f0bd452578f - <rustc_session[31f131ebab3b1e65]::session::Session>::time::<(), rustc_monomorphize[fe4fa0512615eb28]::collector::collect_crate_mono_items::{closure#1}>
  33:     0x7f0bd44fb05d - rustc_monomorphize[fe4fa0512615eb28]::collector::collect_crate_mono_items
  34:     0x7f0bd450ad8a - rustc_monomorphize[fe4fa0512615eb28]::partitioning::collect_and_partition_mono_items
  35:     0x7f0bd536b2cb - rustc_query_system[ed2bae7141c001de]::query::plumbing::try_execute_query::<rustc_query_impl[ad3df31a26a09c18]::plumbing::QueryCtxt, rustc_query_system[ed2bae7141c001de]::query::caches::DefaultCache<(), (&std[c962dfd4bdbe76a2]::collections::hash::set::HashSet<rustc_span[b384040717ceefb]::def_id::DefId, core[ae65f184c67bc919]::hash::BuildHasherDefault<rustc_hash[dbc584c8a978da26]::FxHasher>>, &[rustc_middle[d8bcd2f6073ec0f4]::mir::mono::CodegenUnit])>>
  36:     0x7f0bd541ddf7 - rustc_query_system[ed2bae7141c001de]::query::plumbing::get_query::<rustc_query_impl[ad3df31a26a09c18]::queries::collect_and_partition_mono_items, rustc_query_impl[ad3df31a26a09c18]::plumbing::QueryCtxt>
  37:     0x7f0bd524aa0b - <rustc_query_impl[ad3df31a26a09c18]::Queries as rustc_middle[d8bcd2f6073ec0f4]::ty::query::QueryEngine>::collect_and_partition_mono_items
  38:     0x7f0bd3f33c07 - rustc_codegen_ssa[3a8307e05d8efa8c]::base::codegen_crate::<rustc_codegen_llvm[88006e5835f43fce]::LlvmCodegenBackend>
  39:     0x7f0bd3fea618 - <rustc_codegen_llvm[88006e5835f43fce]::LlvmCodegenBackend as rustc_codegen_ssa[3a8307e05d8efa8c]::traits::backend::CodegenBackend>::codegen_crate
  40:     0x7f0bd3e798ef - <rustc_session[31f131ebab3b1e65]::session::Session>::time::<alloc[3a9675dc12b44c41]::boxed::Box<dyn core[ae65f184c67bc919]::any::Any>, rustc_interface[f6df9d27a16f13b7]::passes::start_codegen::{closure#0}>
  41:     0x7f0bd3ea3b93 - rustc_interface[f6df9d27a16f13b7]::passes::start_codegen
  42:     0x7f0bd3ea2bac - <rustc_interface[f6df9d27a16f13b7]::passes::QueryContext>::enter::<<rustc_interface[f6df9d27a16f13b7]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[ae65f184c67bc919]::result::Result<alloc[3a9675dc12b44c41]::boxed::Box<dyn core[ae65f184c67bc919]::any::Any>, rustc_errors[b92b891ac752ae31]::ErrorGuaranteed>>
  43:     0x7f0bd3e1a41d - <rustc_interface[f6df9d27a16f13b7]::queries::Queries>::ongoing_codegen
  44:     0x7f0bd3d64bf1 - <rustc_interface[f6df9d27a16f13b7]::interface::Compiler>::enter::<rustc_driver[f3569c13e1df7507]::run_compiler::{closure#1}::{closure#2}, core[ae65f184c67bc919]::result::Result<core[ae65f184c67bc919]::option::Option<rustc_interface[f6df9d27a16f13b7]::queries::Linker>, rustc_errors[b92b891ac752ae31]::ErrorGuaranteed>>
  45:     0x7f0bd3cf9e46 - rustc_span[b384040717ceefb]::with_source_map::<core[ae65f184c67bc919]::result::Result<(), rustc_errors[b92b891ac752ae31]::ErrorGuaranteed>, rustc_interface[f6df9d27a16f13b7]::interface::run_compiler<core[ae65f184c67bc919]::result::Result<(), rustc_errors[b92b891ac752ae31]::ErrorGuaranteed>, rustc_driver[f3569c13e1df7507]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  46:     0x7f0bd3d58778 - <scoped_tls[27cd29d9afbae785]::ScopedKey<rustc_span[b384040717ceefb]::SessionGlobals>>::set::<rustc_interface[f6df9d27a16f13b7]::interface::run_compiler<core[ae65f184c67bc919]::result::Result<(), rustc_errors[b92b891ac752ae31]::ErrorGuaranteed>, rustc_driver[f3569c13e1df7507]::run_compiler::{closure#1}>::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[b92b891ac752ae31]::ErrorGuaranteed>>
  47:     0x7f0bd3d20230 - std[c962dfd4bdbe76a2]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f6df9d27a16f13b7]::util::run_in_thread_pool_with_globals<rustc_interface[f6df9d27a16f13b7]::interface::run_compiler<core[ae65f184c67bc919]::result::Result<(), rustc_errors[b92b891ac752ae31]::ErrorGuaranteed>, rustc_driver[f3569c13e1df7507]::run_compiler::{closure#1}>::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[b92b891ac752ae31]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[b92b891ac752ae31]::ErrorGuaranteed>>
  48:     0x7f0bd3d04554 - <<std[c962dfd4bdbe76a2]::thread::Builder>::spawn_unchecked_<rustc_interface[f6df9d27a16f13b7]::util::run_in_thread_pool_with_globals<rustc_interface[f6df9d27a16f13b7]::interface::run_compiler<core[ae65f184c67bc919]::result::Result<(), rustc_errors[b92b891ac752ae31]::ErrorGuaranteed>, rustc_driver[f3569c13e1df7507]::run_compiler::{closure#1}>::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[b92b891ac752ae31]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[ae65f184c67bc919]::result::Result<(), rustc_errors[b92b891ac752ae31]::ErrorGuaranteed>>::{closure#1} as core[ae65f184c67bc919]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  49:     0x7f0bd6b6c493 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h85a72ba036d4c02b
                               at /rustc/85ab8d000e0073f597aad7bb1c19e9dfc8cbbe17/library/alloc/src/boxed.rs:1990:9
  50:     0x7f0bd6b6c493 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h6b17c7b56a9554b9
                               at /rustc/85ab8d000e0073f597aad7bb1c19e9dfc8cbbe17/library/alloc/src/boxed.rs:1990:9
  51:     0x7f0bd6b6c493 - std::sys::unix::thread::Thread::new::thread_start::h4e013c1a945d5337
                               at /rustc/85ab8d000e0073f597aad7bb1c19e9dfc8cbbe17/library/std/src/sys/unix/thread.rs:108:17
  52:     0x7f0bd25e1ea5 - start_thread
  54:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.0-nightly (85ab8d000 2022-12-11) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C linker=clang -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C link-args=-fuse-ld=lld -C split-debuginfo=off -Z binary-dep-depinfo -Z tls-model=initial-exec
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [collect_and_partition_mono_items] collect_and_partition_mono_items
[RUSTC-TIMING] cargo test:false 0.713
error: could not compile `cargo`
Build completed unsuccessfully in 0:36:59
