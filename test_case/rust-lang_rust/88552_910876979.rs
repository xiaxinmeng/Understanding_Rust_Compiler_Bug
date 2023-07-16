plain
Preparing cargo
[2021-09-01T23:04:14Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-09-01T23:04:14Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-09-01T23:04:14Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-09-01T23:04:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpATHXAf#cargo:0.29.0" "--profile" "check" "--lib" "--" "--skip-this-rustc"
[2021-09-01T23:04:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEEICRx#cargo:0.29.0" "--release" "--lib" "--" "--skip-this-rustc"
[2021-09-01T23:04:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpoATOHj#cargo:0.29.0" "--lib" "--" "--skip-this-rustc"
[2021-09-01T23:06:28Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-01T23:06:28Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-09-01T23:06:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbtjm0e#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2021-09-01T23:06:40Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-09-01T23:06:40Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-09-01T23:06:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbtjm0e#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpbtjm0e/incremental-state"
collector error: Failed to profile 'cargo' with Eprintln, recorded: expected success, got exit status: 101
stderr=LLVM Profile Error: Runtime and instrumentation version mismatch : expected 5, but get 7
LLVM Profile Error: Runtime and instrumentation version mismatch : expected 5, but get 7
LLVM Profile Error: Runtime and instrumentation version mismatch : expected 5, but get 7
LLVM Profile Error: Runtime and instrumentation version mismatch : expected 5, but get 7
---
LLVM Profile Error: Runtime and instrumentation version mismatch : expected 5, but get 7
warning: trait objects without an explicit `dyn` are deprecated
   --> src/crates-io/lib.rs:292:41
    |
292 | fn handle(handle: &mut Easy, read: &mut FnMut(&mut [u8]) -> usize) -> Result<String> {
    |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn FnMut(&mut [u8]) -> usize`
    = note: `#[warn(bare_trait_objects)]` on by default
    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>


LLVM Profile Error: Runtime and instrumentation version mismatch : expected 5, but get 7
warning: `crates-io` (lib) generated 1 warning
    Checking cargo v0.29.0 (/tmp/.tmpbtjm0e)
    Checking cargo v0.29.0 (/tmp/.tmpbtjm0e)
thread 'main' panicked at 'assertion failed: cmd.status().expect(\"failed to spawn\").success()', collector/src/rustc-fake.rs:247:17
error: could not compile `cargo`


 stdout=
 stdout=
5 benchmarks remaining
Preparing ctfe-stress-4
[2021-09-01T23:06:47Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-09-01T23:06:47Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-09-01T23:06:47Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-09-01T23:06:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRo5RWn#ctfe-stress-4:0.1.0" "--" "--skip-this-rustc"
[2021-09-01T23:06:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp25ezzr#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2021-09-01T23:06:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqzou05#ctfe-stress-4:0.1.0" "--release" "--" "--skip-this-rustc"
[2021-09-01T23:06:47Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-01T23:06:47Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-09-01T23:06:47Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-09-01T23:06:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSkDJWh#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2021-09-01T23:07:05Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-09-01T23:07:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSkDJWh#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpSkDJWh/incremental-state"
[2021-09-01T23:07:31Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-09-01T23:07:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSkDJWh#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpSkDJWh/incremental-state"
[2021-09-01T23:07:32Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-01T23:07:32Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-09-01T23:07:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvQEso6#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2021-09-01T23:07:50Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-09-01T23:07:50Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-09-01T23:07:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvQEso6#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpvQEso6/incremental-state"
[2021-09-01T23:08:16Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-09-01T23:08:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvQEso6#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpvQEso6/incremental-state"
[2021-09-01T23:08:17Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-01T23:08:17Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-09-01T23:08:17Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-09-01T23:08:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAJBLeU#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2021-09-01T23:08:34Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-09-01T23:08:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAJBLeU#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpAJBLeU/incremental-state"
[2021-09-01T23:08:55Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-09-01T23:08:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAJBLeU#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpAJBLeU/incremental-state"
Preparing externs
[2021-09-01T23:08:56Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-09-01T23:08:56Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-09-01T23:08:56Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-09-01T23:08:56Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-09-01T23:08:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppLyRoe#externs:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2021-09-01T23:08:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDob13X#externs:0.1.0" "--release" "--" "--skip-this-rustc"
[2021-09-01T23:08:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpA2ocxA#externs:0.1.0" "--" "--skip-this-rustc"
Running externs: Check + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2021-09-01T23:08:57Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-01T23:08:57Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-09-01T23:08:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHeir5d#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2021-09-01T23:08:57Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-09-01T23:08:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHeir5d#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpHeir5d/incremental-state"
[2021-09-01T23:08:58Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-09-01T23:08:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHeir5d#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpHeir5d/incremental-state"
[2021-09-01T23:08:59Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-01T23:08:59Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-09-01T23:08:59Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-09-01T23:08:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3F2oBO#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2021-09-01T23:09:00Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-09-01T23:09:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3F2oBO#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp3F2oBO/incremental-state"
[2021-09-01T23:09:01Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-09-01T23:09:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3F2oBO#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmp3F2oBO/incremental-state"
[2021-09-01T23:09:02Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-01T23:09:02Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-09-01T23:09:02Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-09-01T23:09:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptfL9B3#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2021-09-01T23:09:03Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-09-01T23:09:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptfL9B3#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmptfL9B3/incremental-state"
[2021-09-01T23:09:04Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-09-01T23:09:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmptfL9B3#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmptfL9B3/incremental-state"
Preparing inflate
[2021-09-01T23:09:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-09-01T23:09:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-09-01T23:09:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-09-01T23:09:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-09-01T23:09:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpngYJq7#inflate:0.1.0" "--release" "--" "--skip-this-rustc"
[2021-09-01T23:09:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpOnnacf#inflate:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2021-09-01T23:09:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpARJ1Da#inflate:0.1.0" "--" "--skip-this-rustc"
[2021-09-01T23:09:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-01T23:09:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-09-01T23:09:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-09-01T23:09:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpodchGj#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2021-09-01T23:09:07Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-09-01T23:09:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpodchGj#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpodchGj/incremental-state"
[2021-09-01T23:09:10Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-09-01T23:09:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpodchGj#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpodchGj/incremental-state"
[2021-09-01T23:09:10Z DEBUG collector::execute] applying println to "/tmp/.tmpodchGj"
[2021-09-01T23:09:10Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-09-01T23:09:10Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-09-01T23:09:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpodchGj#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpodchGj/incremental-state"
[2021-09-01T23:09:12Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-01T23:09:12Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-09-01T23:09:12Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-09-01T23:09:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEN1VvF#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2021-09-01T23:09:15Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-09-01T23:09:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEN1VvF#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpEN1VvF/incremental-state"
[2021-09-01T23:09:19Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-09-01T23:09:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEN1VvF#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpEN1VvF/incremental-state"
[2021-09-01T23:09:19Z DEBUG collector::execute] applying println to "/tmp/.tmpEN1VvF"
[2021-09-01T23:09:19Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-09-01T23:09:19Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-09-01T23:09:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpEN1VvF#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpEN1VvF/incremental-state"
[2021-09-01T23:09:23Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-01T23:09:23Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-09-01T23:09:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpV0VMJG#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2021-09-01T23:09:32Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-09-01T23:09:32Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-09-01T23:09:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpV0VMJG#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpV0VMJG/incremental-state"
[2021-09-01T23:09:41Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-09-01T23:09:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpV0VMJG#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpV0VMJG/incremental-state"
[2021-09-01T23:09:42Z DEBUG collector::execute] applying println to "/tmp/.tmpV0VMJG"
[2021-09-01T23:09:42Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-09-01T23:09:42Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2021-09-01T23:09:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpV0VMJG#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpV0VMJG/incremental-state"
Preparing match-stress-enum
[2021-09-01T23:09:51Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-09-01T23:09:51Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-09-01T23:09:51Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-09-01T23:09:51Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-09-01T23:09:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcbpoyX#match-stress-enum:0.1.0" "--release" "--" "--skip-this-rustc"
[2021-09-01T23:09:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphn6RKl#match-stress-enum:0.1.0" "--" "--skip-this-rustc"
[2021-09-01T23:09:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpo0Ve0W#match-stress-enum:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2021-09-01T23:09:51Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-01T23:09:51Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-09-01T23:09:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHcs98q#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2021-09-01T23:09:54Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-09-01T23:09:54Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-09-01T23:09:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHcs98q#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpHcs98q/incremental-state"
[2021-09-01T23:09:55Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-09-01T23:09:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHcs98q#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpHcs98q/incremental-state"
[2021-09-01T23:09:56Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-01T23:09:56Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-09-01T23:09:56Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-09-01T23:09:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqNjwWY#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2021-09-01T23:09:58Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-09-01T23:09:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqNjwWY#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpqNjwWY/incremental-state"
[2021-09-01T23:10:01Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-09-01T23:10:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpqNjwWY#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpqNjwWY/incremental-state"
[2021-09-01T23:10:01Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-01T23:10:01Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-09-01T23:10:01Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-09-01T23:10:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXXeUxM#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2021-09-01T23:10:03Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-09-01T23:10:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXXeUxM#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpXXeUxM/incremental-state"
[2021-09-01T23:10:06Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-09-01T23:10:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXXeUxM#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpXXeUxM/incremental-state"
Preparing token-stream-stress
[2021-09-01T23:10:06Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2021-09-01T23:10:06Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2021-09-01T23:10:06Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-09-01T23:10:06Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2021-09-01T23:10:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKVZ3Ha#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--skip-this-rustc"
[2021-09-01T23:10:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpiZbAGC#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--skip-this-rustc"
[2021-09-01T23:10:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLLuagG#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--skip-this-rustc"
[2021-09-01T23:10:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-01T23:10:07Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-09-01T23:10:07Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2021-09-01T23:10:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpj6HLpZ#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2021-09-01T23:10:07Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2021-09-01T23:10:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpj6HLpZ#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpj6HLpZ/incremental-state"
[2021-09-01T23:10:08Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2021-09-01T23:10:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpj6HLpZ#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpj6HLpZ/incremental-state"
[2021-09-01T23:10:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-01T23:10:08Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-09-01T23:10:08Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2021-09-01T23:10:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBsaG8K#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2021-09-01T23:10:08Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2021-09-01T23:10:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBsaG8K#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpBsaG8K/incremental-state"
[2021-09-01T23:10:09Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2021-09-01T23:10:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBsaG8K#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpBsaG8K/incremental-state"
[2021-09-01T23:10:09Z DEBUG collector::execute] Benchmark iteration 1/1
[2021-09-01T23:10:09Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2021-09-01T23:10:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHH37lV#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2021-09-01T23:10:09Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-09-01T23:10:09Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2021-09-01T23:10:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHH37lV#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpHH37lV/incremental-state"
[2021-09-01T23:10:10Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2021-09-01T23:10:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHH37lV#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpHH37lV/incremental-state"
collector error: 1 benchmarks failed
