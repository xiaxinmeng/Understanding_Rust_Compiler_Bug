plain
[2022-11-02T19:24:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-02T19:24:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-11-02T19:24:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDPqnqt#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-02T19:24:09Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-11-02T19:24:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDPqnqt#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDPqnqt/incremental-state"
[2022-11-02T19:24:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-11-02T19:24:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDPqnqt#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDPqnqt/incremental-state"
Finished benchmark bitmaps-3.1.0 (1/8)
collector error: Failed to profile 'bitmaps-3.1.0' with Eprintln, recorded: expected success, got exit status: 101

stderr=    Checking bitmaps v3.1.0 (/tmp/.tmpDPqnqt)
thread 'main' panicked at 'command did not complete successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--crate-name" "bitmaps" "--edition=2018" "src/lib.rs" "--error-format=json" "--json=diagnostic-rendered-ansi,artifacts,future-incompat" "--crate-type" "lib" "--emit=dep-info,metadata" "-C" "embed-bitcode=no" "-C" "debuginfo=2" "-C" "incremental=/tmp/.tmpDPqnqt/incremental-state" "--cfg" "feature=\"default\"" "--cfg" "feature=\"std\"" "-C" "metadata=8be54c1634dcb120" "-C" "extra-filename=-8be54c1634dcb120" "--out-dir" "/tmp/.tmpDPqnqt/target/debug/deps" "-C" "linker=clang" "-L" "dependency=/tmp/.tmpDPqnqt/target/debug/deps" "-Adeprecated" "-Aunknown-lints" "-Zincremental-verify-ich"', collector/src/rustc-fake.rs:24:5
error: could not compile `bitmaps`


 stdout=
---
[2022-11-02T19:25:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-02T19:25:08Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-11-02T19:25:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpopn2sh#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-02T19:25:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-11-02T19:25:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpopn2sh#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpopn2sh/incremental-state"
[2022-11-02T19:25:51Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-11-02T19:25:51Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpopn2sh#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpopn2sh/incremental-state"
Finished benchmark cargo-0.60.0 (2/8)
collector error: Failed to profile 'cargo-0.60.0' with Eprintln, recorded: expected success, got exit status: 101

stderr=    Checking cargo v0.60.0 (/tmp/.tmpopn2sh)
thread 'main' panicked at 'command did not complete successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--crate-name" "cargo" "--edition=2021" "src/cargo/lib.rs" "--error-format=json" "--json=diagnostic-rendered-ansi,artifacts,future-incompat" "--crate-type" "lib" "--emit=dep-info,metadata" "-C" "embed-bitcode=no" "-C" "debuginfo=2" "-C" "incremental=/tmp/.tmpopn2sh/incremental-state" "-C" "metadata=9c1977274b56fa93" "-C" "extra-filename=-9c1977274b56fa93" "--out-dir" "/tmp/.tmpopn2sh/target/debug/deps" "-C" "linker=clang" "-L" "dependency=/tmp/.tmpopn2sh/target/debug/deps" "--extern" "anyhow=/tmp/.tmpopn2sh/target/debug/deps/libanyhow-e21bf3d3cc947453.rmeta" "--extern" "atty=/tmp/.tmpopn2sh/target/debug/deps/libatty-f063ea63eeeaa7e8.rmeta" "--extern" "bytesize=/tmp/.tmpopn2sh/target/debug/deps/libbytesize-2480cd8661bb4f72.rmeta" "--extern" "cargo_platform=/tmp/.tmpopn2sh/target/debug/deps/libcargo_platform-a16051cfc5033a17.rmeta" "--extern" "cargo_util=/tmp/.tmpopn2sh/target/debug/deps/libcargo_util-f8c9cdc447b61f17.rmeta" "--extern" "clap=/tmp/.tmpopn2sh/target/debug/deps/libclap-f56ea51becf6de08.rmeta" "--extern" "crates_io=/tmp/.tmpopn2sh/target/debug/deps/libcrates_io-c9d252202a17110c.rmeta" "--extern" "crossbeam_utils=/tmp/.tmpopn2sh/target/debug/deps/libcrossbeam_utils-63e17451a3a86611.rmeta" "--extern" "curl=/tmp/.tmpopn2sh/target/debug/deps/libcurl-97c7d911f6b5c4b0.rmeta" "--extern" "curl_sys=/tmp/.tmpopn2sh/target/debug/deps/libcurl_sys-338fc316e5df7403.rmeta" "--extern" "env_logger=/tmp/.tmpopn2sh/target/debug/deps/libenv_logger-3c790e516820e1d8.rmeta" "--extern" "filetime=/tmp/.tmpopn2sh/target/debug/deps/libfiletime-b2df138ea4202c20.rmeta" "--extern" "flate2=/tmp/.tmpopn2sh/target/debug/deps/libflate2-889a4f091c92691d.rmeta" "--extern" "git2=/tmp/.tmpopn2sh/target/debug/deps/libgit2-8ecd60f2be3894d3.rmeta" "--extern" "git2_curl=/tmp/.tmpopn2sh/target/debug/deps/libgit2_curl-39fa110671e44b2c.rmeta" "--extern" "glob=/tmp/.tmpopn2sh/target/debug/deps/libglob-9e9384b5fe3c4620.rmeta" "--extern" "hex=/tmp/.tmpopn2sh/target/debug/deps/libhex-52738f6114d4bb1b.rmeta" "--extern" "home=/tmp/.tmpopn2sh/target/debug/deps/libhome-d2b7580d70f177a7.rmeta" "--extern" "humantime=/tmp/.tmpopn2sh/target/debug/deps/libhumantime-44407cfbe61bf131.rmeta" "--extern" "ignore=/tmp/.tmpopn2sh/target/debug/deps/libignore-4beb726fff805bdf.rmeta" "--extern" "im_rc=/tmp/.tmpopn2sh/target/debug/deps/libim_rc-924f3044182d5f64.rmeta" "--extern" "itertools=/tmp/.tmpopn2sh/target/debug/deps/libitertools-731c5c3c3784abe8.rmeta" "--extern" "jobserver=/tmp/.tmpopn2sh/target/debug/deps/libjobserver-82539fb8b8aaa879.rmeta" "--extern" "lazy_static=/tmp/.tmpopn2sh/target/debug/deps/liblazy_static-3ab14ef94a1f89cf.rmeta" "--extern" "lazycell=/tmp/.tmpopn2sh/target/debug/deps/liblazycell-b8d866e123fe126b.rmeta" "--extern" "libc=/tmp/.tmpopn2sh/target/debug/deps/liblibc-6a685a183885f2de.rmeta" "--extern" "libgit2_sys=/tmp/.tmpopn2sh/target/debug/deps/liblibgit2_sys-e231d7ca20a9c470.rmeta" "--extern" "log=/tmp/.tmpopn2sh/target/debug/deps/liblog-bfec198e2ff177f1.rmeta" "--extern" "memchr=/tmp/.tmpopn2sh/target/debug/deps/libmemchr-d022af6da608c734.rmeta" "--extern" "num_cpus=/tmp/.tmpopn2sh/target/debug/deps/libnum_cpus-10f73e5f44701bb0.rmeta" "--extern" "opener=/tmp/.tmpopn2sh/target/debug/deps/libopener-2dacf66eb67d055d.rmeta" "--extern" "os_info=/tmp/.tmpopn2sh/target/debug/deps/libos_info-661a1dbce6170488.rmeta" "--extern" "percent_encoding=/tmp/.tmpopn2sh/target/debug/deps/libpercent_encoding-4c30c71be0ff39f5.rmeta" "--extern" "rustc_workspace_hack=/tmp/.tmpopn2sh/target/debug/deps/librustc_workspace_hack-6f0313f2d3d480a7.rmeta" "--extern" "rustfix=/tmp/.tmpopn2sh/target/debug/deps/librustfix-4f841dc72755b8ae.rmeta" "--extern" "semver=/tmp/.tmpopn2sh/target/debug/deps/libsemver-510a5dcc364d4ce3.rmeta" "--extern" "serde=/tmp/.tmpopn2sh/target/debug/deps/libserde-4799d79b2f9e8d02.rmeta" "--extern" "serde_ignored=/tmp/.tmpopn2sh/target/debug/deps/libserde_ignored-d40bb69af05edfee.rmeta" "--extern" "serde_json=/tmp/.tmpopn2sh/target/debug/deps/libserde_json-5169325ddd6ec36d.rmeta" "--extern" "shell_escape=/tmp/.tmpopn2sh/target/debug/deps/libshell_escape-b516a9b258588e1c.rmeta" "--extern" "strip_ansi_escapes=/tmp/.tmpopn2sh/target/debug/deps/libstrip_ansi_escapes-ef0a4642cffef332.rmeta" "--extern" "tar=/tmp/.tmpopn2sh/target/debug/deps/libtar-7bee1ae4577da32c.rmeta" "--extern" "tempfile=/tmp/.tmpopn2sh/target/debug/deps/libtempfile-947b4201239315bf.rmeta" "--extern" "termcolor=/tmp/.tmpopn2sh/target/debug/deps/libtermcolor-d3ae53939c9689cf.rmeta" "--extern" "toml=/tmp/.tmpopn2sh/target/debug/deps/libtoml-4b1d6e982cd6d8ac.rmeta" "--extern" "unicode_width=/tmp/.tmpopn2sh/target/debug/deps/libunicode_width-6d24e7f7ec6d5ed2.rmeta" "--extern" "unicode_xid=/tmp/.tmpopn2sh/target/debug/deps/libunicode_xid-3d7133eb5e1c05de.rmeta" "--extern" "url=/tmp/.tmpopn2sh/target/debug/deps/liburl-0aec23243bfc28e0.rmeta" "--extern" "walkdir=/tmp/.tmpopn2sh/target/debug/deps/libwalkdir-2d36edf9becddc97.rmeta" "-L" "native=/tmp/.tmpopn2sh/target/debug/build/curl-sys-75d068e63164f049/out/build" "-L" "native=/tmp/.tmpopn2sh/target/debug/build/libnghttp2-sys-6ea4bffe1440e9e9/out/i/lib" "-L" "native=/tmp/.tmpopn2sh/target/debug/build/libgit2-sys-da3c6ff9ee6b1457/out/build" "-L" "native=/tmp/.tmpopn2sh/target/debug/build/libssh2-sys-3d6a254962dd01bf/out/build" "-Adeprecated" "-Aunknown-lints" "-Zincremental-verify-ich"', collector/src/rustc-fake.rs:24:5
error: could not compile `cargo`


 stdout=
---
[2022-11-02T19:26:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpApTKk8#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpApTKk8/incremental-state"
[2022-11-02T19:26:12Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-11-02T19:26:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpApTKk8#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpApTKk8/incremental-state"
Finished benchmark ctfe-stress-5 (3/8)
collector error: Failed to profile 'ctfe-stress-5' with Eprintln, recorded: expected success, got exit status: 101

stderr=    Checking ctfe-stress-5 v0.1.0 (/tmp/.tmpApTKk8)
thread 'main' panicked at 'command did not complete successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--crate-name" "ctfe_stress_5" "src/lib.rs" "--error-format=json" "--json=diagnostic-rendered-ansi,artifacts,future-incompat" "--crate-type" "lib" "--emit=dep-info,metadata" "-C" "embed-bitcode=no" "-C" "debuginfo=2" "-C" "incremental=/tmp/.tmpApTKk8/incremental-state" "-C" "metadata=2907f87da6f5fa96" "-C" "extra-filename=-2907f87da6f5fa96" "--out-dir" "/tmp/.tmpApTKk8/target/debug/deps" "-C" "linker=clang" "-L" "dependency=/tmp/.tmpApTKk8/target/debug/deps" "-Adeprecated" "-Aunknown-lints" "-Zincremental-verify-ich"', collector/src/rustc-fake.rs:24:5
error: could not compile `ctfe-stress-5`


 stdout=
---
[2022-11-02T19:26:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpe0MSUb#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpe0MSUb/incremental-state"
[2022-11-02T19:27:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-11-02T19:27:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpe0MSUb#diesel@1.4.8" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpe0MSUb/incremental-state"
Finished benchmark diesel-1.4.8 (4/8)
collector error: Failed to profile 'diesel-1.4.8' with Eprintln, recorded: expected success, got exit status: 101

stderr=    Checking diesel v1.4.8 (/tmp/.tmpe0MSUb)
thread 'main' panicked at 'command did not complete successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--crate-name" "diesel" "src/lib.rs" "--error-format=json" "--json=diagnostic-rendered-ansi,artifacts,future-incompat" "--crate-type" "lib" "--emit=dep-info,metadata" "-C" "embed-bitcode=no" "-C" "debuginfo=2" "-C" "incremental=/tmp/.tmpe0MSUb/incremental-state" "--cfg" "feature=\"32-column-tables\"" "--cfg" "feature=\"default\"" "--cfg" "feature=\"with-deprecated\"" "-C" "metadata=e4a4ad78620500f4" "-C" "extra-filename=-e4a4ad78620500f4" "--out-dir" "/tmp/.tmpe0MSUb/target/debug/deps" "-C" "linker=clang" "-L" "dependency=/tmp/.tmpe0MSUb/target/debug/deps" "--extern" "byteorder=/tmp/.tmpe0MSUb/target/debug/deps/libbyteorder-19cac908f6232626.rmeta" "--extern" "diesel_derives=/tmp/.tmpe0MSUb/target/debug/deps/libdiesel_derives-555e7dc0bab471fb.so" "-Adeprecated" "-Aunknown-lints" "-Zincremental-verify-ich"', collector/src/rustc-fake.rs:24:5
error: could not compile `diesel`


 stdout=
---
[2022-11-02T19:27:06Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-02T19:27:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-11-02T19:27:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFKZMXR#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-02T19:27:07Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-11-02T19:27:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFKZMXR#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFKZMXR/incremental-state"
[2022-11-02T19:27:08Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-11-02T19:27:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFKZMXR#externs@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpFKZMXR/incremental-state"
Finished benchmark externs (5/8)
collector error: Failed to profile 'externs' with Eprintln, recorded: expected success, got exit status: 101

stderr=    Checking externs v0.1.0 (/tmp/.tmpFKZMXR)
thread 'main' panicked at 'command did not complete successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--crate-name" "externs" "--edition=2018" "src/lib.rs" "--error-format=json" "--json=diagnostic-rendered-ansi,artifacts,future-incompat" "--crate-type" "lib" "--emit=dep-info,metadata" "-C" "embed-bitcode=no" "-C" "debuginfo=2" "-C" "incremental=/tmp/.tmpFKZMXR/incremental-state" "-C" "metadata=85cf8b98573b75fc" "-C" "extra-filename=-85cf8b98573b75fc" "--out-dir" "/tmp/.tmpFKZMXR/target/debug/deps" "-C" "linker=clang" "-L" "dependency=/tmp/.tmpFKZMXR/target/debug/deps" "-Adeprecated" "-Aunknown-lints" "-Zincremental-verify-ich"', collector/src/rustc-fake.rs:24:5
error: could not compile `externs`


 stdout=
---
[2022-11-02T19:27:09Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-02T19:27:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-11-02T19:27:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWMBfSa#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-02T19:27:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-11-02T19:27:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWMBfSa#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpWMBfSa/incremental-state"
[2022-11-02T19:27:14Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-11-02T19:27:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWMBfSa#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpWMBfSa/incremental-state"
Finished benchmark match-stress (6/8)
collector error: Failed to profile 'match-stress' with Eprintln, recorded: expected success, got exit status: 101

stderr=    Checking match-stress v0.1.0 (/tmp/.tmpWMBfSa)
thread 'main' panicked at 'command did not complete successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--crate-name" "match_stress" "src/lib.rs" "--error-format=json" "--json=diagnostic-rendered-ansi,artifacts,future-incompat" "--crate-type" "lib" "--emit=dep-info,metadata" "-C" "embed-bitcode=no" "-C" "debuginfo=2" "-C" "incremental=/tmp/.tmpWMBfSa/incremental-state" "-C" "metadata=d5fca03593e28fab" "-C" "extra-filename=-d5fca03593e28fab" "--out-dir" "/tmp/.tmpWMBfSa/target/debug/deps" "-C" "linker=clang" "-L" "dependency=/tmp/.tmpWMBfSa/target/debug/deps" "-Adeprecated" "-Aunknown-lints" "-Zincremental-verify-ich"', collector/src/rustc-fake.rs:24:5
error: could not compile `match-stress`


 stdout=
---
[2022-11-02T19:27:16Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-11-02T19:27:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2022-11-02T19:27:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLEixBy#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2022-11-02T19:27:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2022-11-02T19:27:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLEixBy#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpLEixBy/incremental-state"
[2022-11-02T19:27:16Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-11-02T19:27:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLEixBy#token-stream-stress@0.0.0" "--profile" "check" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpLEixBy/incremental-state"
Finished benchmark token-stream-stress (7/8)
collector error: Failed to profile 'token-stream-stress' with Eprintln, recorded: expected success, got exit status: 101

stderr=    Checking token-stream-stress v0.0.0 (/tmp/.tmpLEixBy)
thread 'main' panicked at 'command did not complete successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--crate-name" "token_stream_stress_bin" "--edition=2018" "src/main.rs" "--error-format=json" "--json=diagnostic-rendered-ansi,artifacts,future-incompat" "--crate-type" "bin" "--emit=dep-info,metadata" "-C" "embed-bitcode=no" "-C" "debuginfo=2" "-C" "incremental=/tmp/.tmpLEixBy/incremental-state" "-C" "metadata=75dff0543866470a" "-C" "extra-filename=-75dff0543866470a" "--out-dir" "/tmp/.tmpLEixBy/target/debug/deps" "-C" "linker=clang" "-L" "dependency=/tmp/.tmpLEixBy/target/debug/deps" "--extern" "token_stream_stress=/tmp/.tmpLEixBy/target/debug/deps/libtoken_stream_stress-e89ba8b8710690a7.so" "-Adeprecated" "-Aunknown-lints" "-Zincremental-verify-ich"', collector/src/rustc-fake.rs:24:5
error: could not compile `token-stream-stress`


 stdout=
---
[2022-11-02T19:27:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHv5vEE#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpHv5vEE/incremental-state"
[2022-11-02T19:27:28Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2022-11-02T19:27:28Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHv5vEE#tuple-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpHv5vEE/incremental-state"
Finished benchmark tuple-stress (8/8)
collector error: Failed to profile 'tuple-stress' with Eprintln, recorded: expected success, got exit status: 101

stderr=    Checking tuple-stress v0.1.0 (/tmp/.tmpHv5vEE)
thread 'main' panicked at 'command did not complete successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--crate-name" "tuple_stress" "src/main.rs" "--error-format=json" "--json=diagnostic-rendered-ansi,artifacts,future-incompat" "--crate-type" "bin" "--emit=dep-info,metadata" "-C" "embed-bitcode=no" "-C" "debuginfo=2" "-C" "incremental=/tmp/.tmpHv5vEE/incremental-state" "-C" "metadata=e01864c0fa98aaf5" "-C" "extra-filename=-e01864c0fa98aaf5" "--out-dir" "/tmp/.tmpHv5vEE/target/debug/deps" "-C" "linker=clang" "-L" "dependency=/tmp/.tmpHv5vEE/target/debug/deps" "-Adeprecated" "-Aunknown-lints" "-Zincremental-verify-ich"', collector/src/rustc-fake.rs:24:5
error: could not compile `tuple-stress`


 stdout=
