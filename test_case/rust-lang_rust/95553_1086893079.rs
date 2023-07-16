plain
[2022-04-03T15:17:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpaBIhrb#cargo:0.29.0" "--lib" "--" "--skip-this-rustc"
Running cargo: Debug + [Full]
[2022-04-03T15:19:30Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-03T15:19:31Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-04-03T15:19:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpWxsboS#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-04-03T15:20:05Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-03T15:20:05Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-04-03T15:20:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpP6bMUJ#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln"
7 benchmarks remaining
---
6 benchmarks remaining
Preparing deeply-nested-async
[2022-04-03T15:22:12Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-04-03T15:22:12Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-04-03T15:22:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3Ul0It#deeply-nested-async:0.1.0" "--" "--skip-this-rustc"
[2022-04-03T15:22:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpLSaLJH#deeply-nested-async:0.1.0" "--release" "--" "--skip-this-rustc"
[2022-04-03T15:22:13Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-03T15:22:13Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-04-03T15:22:13Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-04-03T15:22:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpjBbgWN#deeply-nested-async:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-04-03T15:22:14Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-03T15:22:14Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-04-03T15:22:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVRS9Qw#deeply-nested-async:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
5 benchmarks remaining
---
[2022-04-03T15:25:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpc9dT6S#hyper:0.13.0-alpha.4" "--" "--wrap-rustc-with" "eprintln"
Running hyper-2: Opt + [Full]
[2022-04-03T15:25:04Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-03T15:25:04Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-04-03T15:25:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpotqven#hyper:0.13.0-alpha.4" "--release" "--" "--wrap-rustc-with" "eprintln"
collector error: Failed to profile 'hyper-2' with Eprintln, recorded: expected success, got exit status: 101

stderr=   Compiling cfg-if v0.1.10
   Compiling proc-macro2 v1.0.6
   Compiling unicode-xid v0.2.0
   Compiling syn v1.0.8
   Compiling semver-parser v0.7.0
---
   Compiling pin-utils v0.1.0-alpha.4
   Compiling futures-sink-preview v0.3.0-alpha.19
   Compiling fnv v1.0.6
   Compiling scopeguard v1.0.0
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "cfg_if", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/cfg-if-0.1.10/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "opt-level=3", "-C", "embed-bitcode=no", "-C", "metadata=ecbcf349e8423669", "-C", "extra-filename=-ecbcf349e8423669", "--out-dir", "/tmp/.tmpotqven/target/release/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpotqven/target/release/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints"]
exiting -- non-wrapped rustc
warning: build failed, waiting for other jobs to finish...
warning: build failed, waiting for other jobs to finish...
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-1.0.6/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debug-assertions=off", "--cfg", "feature=\"default\"", "--cfg", "feature=\"proc-macro\"", "-C", "metadata=b7ea3fb568429479", "-C", "extra-filename=-b7ea3fb568429479", "--out-dir", "/tmp/.tmpotqven/target/release/build/proc-macro2-b7ea3fb568429479", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpotqven/target/release/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints"]
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "/cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.65/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debug-assertions=off", "--cfg", "feature=\"default\"", "--cfg", "feature=\"std\"", "-C", "metadata=e641ca0c7414ff7e", "-C", "extra-filename=-e641ca0c7414ff7e", "--out-dir", "/tmp/.tmpotqven/target/release/build/libc-e641ca0c7414ff7e", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpotqven/target/release/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints"]
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "semver_parser", "/cargo/registry/src/github.com-1ecc6299db9ec823/semver-parser-0.7.0/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debug-assertions=off", "-C", "metadata=66c4dbff728cc71a", "-C", "extra-filename=-66c4dbff728cc71a", "--out-dir", "/tmp/.tmpotqven/target/release/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpotqven/target/release/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints"]
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "autocfg", "/cargo/registry/src/github.com-1ecc6299db9ec823/autocfg-0.1.7/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debug-assertions=off", "-C", "metadata=4102dd659450dcc2", "-C", "extra-filename=-4102dd659450dcc2", "--out-dir", "/tmp/.tmpotqven/target/release/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpotqven/target/release/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints"]
exiting -- non-wrapped rustc
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/syn-1.0.8/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debug-assertions=off", "--cfg", "feature=\"clone-impls\"", "--cfg", "feature=\"default\"", "--cfg", "feature=\"derive\"", "--cfg", "feature=\"extra-traits\"", "--cfg", "feature=\"full\"", "--cfg", "feature=\"parsing\"", "--cfg", "feature=\"printing\"", "--cfg", "feature=\"proc-macro\"", "--cfg", "feature=\"quote\"", "--cfg", "feature=\"visit\"", "--cfg", "feature=\"visit-mut\"", "-C", "metadata=abc285b68fca49b8", "-C", "extra-filename=-abc285b68fca49b8", "--out-dir", "/tmp/.tmpotqven/target/release/build/syn-abc285b68fca49b8", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpotqven/target/release/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints"]
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "spin", "/cargo/registry/src/github.com-1ecc6299db9ec823/spin-0.5.2/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "opt-level=3", "-C", "embed-bitcode=no", "-C", "metadata=3f7e83477cf040b7", "-C", "extra-filename=-3f7e83477cf040b7", "--out-dir", "/tmp/.tmpotqven/target/release/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpotqven/target/release/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints"]
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "unicode_xid", "/cargo/registry/src/github.com-1ecc6299db9ec823/unicode-xid-0.2.0/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debug-assertions=off", "--cfg", "feature=\"default\"", "-C", "metadata=683d2734d2217370", "-C", "extra-filename=-683d2734d2217370", "--out-dir", "/tmp/.tmpotqven/target/release/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpotqven/target/release/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints"]
exiting -- non-wrapped rustc
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "/cargo/registry/src/github.com-1ecc6299db9ec823/byteorder-1.3.2/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debug-assertions=off", "--cfg", "feature=\"default\"", "--cfg", "feature=\"std\"", "-C", "metadata=1af37ff1b533af48", "-C", "extra-filename=-1af37ff1b533af48", "--out-dir", "/tmp/.tmpotqven/target/release/build/byteorder-1af37ff1b533af48", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpotqven/target/release/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints"]
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "futures_core", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/futures-core-preview-0.3.0-alpha.19/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "opt-level=3", "-C", "embed-bitcode=no", "--cfg", "feature=\"alloc\"", "--cfg", "feature=\"default\"", "--cfg", "feature=\"std\"", "-C", "metadata=4d3183d0e04ff3e9", "-C", "extra-filename=-4d3183d0e04ff3e9", "--out-dir", "/tmp/.tmpotqven/target/release/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpotqven/target/release/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints"]
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "slab", "/cargo/registry/src/github.com-1ecc6299db9ec823/slab-0.4.2/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "opt-level=3", "-C", "embed-bitcode=no", "-C", "metadata=e29780aeb97a901c", "-C", "extra-filename=-e29780aeb97a901c", "--out-dir", "/tmp/.tmpotqven/target/release/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpotqven/target/release/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints"]
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "pin_utils", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/pin-utils-0.1.0-alpha.4/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "opt-level=3", "-C", "embed-bitcode=no", "-C", "metadata=30e8bd6b350312e3", "-C", "extra-filename=-30e8bd6b350312e3", "--out-dir", "/tmp/.tmpotqven/target/release/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpotqven/target/release/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints"]
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "fnv", "/cargo/registry/src/github.com-1ecc6299db9ec823/fnv-1.0.6/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "opt-level=3", "-C", "embed-bitcode=no", "-C", "metadata=9a06682178ca1444", "-C", "extra-filename=-9a06682178ca1444", "--out-dir", "/tmp/.tmpotqven/target/release/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpotqven/target/release/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints"]
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "futures_sink", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/futures-sink-preview-0.3.0-alpha.19/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "opt-level=3", "-C", "embed-bitcode=no", "--cfg", "feature=\"alloc\"", "--cfg", "feature=\"default\"", "--cfg", "feature=\"std\"", "-C", "metadata=141fe3e673eba110", "-C", "extra-filename=-141fe3e673eba110", "--out-dir", "/tmp/.tmpotqven/target/release/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpotqven/target/release/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints"]
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "scopeguard", "/cargo/registry/src/github.com-1ecc6299db9ec823/scopeguard-1.0.0/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "opt-level=3", "-C", "embed-bitcode=no", "-C", "metadata=ed006c75ad343c37", "-C", "extra-filename=-ed006c75ad343c37", "--out-dir", "/tmp/.tmpotqven/target/release/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpotqven/target/release/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints"]
exiting -- non-wrapped rustc
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "/cargo/registry/src/github.com-1ecc6299db9ec823/log-0.4.8/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debug-assertions=off", "--cfg", "feature=\"std\"", "-C", "metadata=f6b2b6f13cd0e87d", "-C", "extra-filename=-f6b2b6f13cd0e87d", "--out-dir", "/tmp/.tmpotqven/target/release/build/log-f6b2b6f13cd0e87d", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpotqven/target/release/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints"]
exiting -- non-wrapped rustc


 stdout=
4 benchmarks remaining
---
3 benchmarks remaining
Preparing ripgrep
[2022-04-03T15:25:30Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-04-03T15:25:30Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-04-03T15:25:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQ5A8nC#ripgrep:0.8.1" "--" "--skip-this-rustc"
[2022-04-03T15:25:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprsu1IU#ripgrep:0.8.1" "--release" "--" "--skip-this-rustc"
[2022-04-03T15:26:09Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-03T15:26:09Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-04-03T15:26:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpvcryD1#ripgrep:0.8.1" "--" "--wrap-rustc-with" "eprintln"
Running ripgrep: Opt + [Full]
Running ripgrep: Opt + [Full]
[2022-04-03T15:26:14Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-03T15:26:15Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-04-03T15:26:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpKZZ8yc#ripgrep:0.8.1" "--release" "--" "--wrap-rustc-with" "eprintln"
2 benchmarks remaining
Preparing serde
[2022-04-03T15:26:38Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-04-03T15:26:38Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-04-03T15:26:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "serde/Cargo.toml" "-p" "file:///tmp/.tmpTV01lW/serde#1.0.37" "--release" "--" "--skip-this-rustc"
[2022-04-03T15:26:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "serde/Cargo.toml" "-p" "file:///tmp/.tmpwxhc0N/serde#1.0.37" "--" "--skip-this-rustc"
[2022-04-03T15:26:39Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-03T15:26:39Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-04-03T15:26:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "serde/Cargo.toml" "-p" "file:///tmp/.tmpCQJARr/serde#1.0.37" "--" "--wrap-rustc-with" "eprintln"
Running serde: Opt + [Full]
Running serde: Opt + [Full]
[2022-04-03T15:26:43Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-03T15:26:43Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-04-03T15:26:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "serde/Cargo.toml" "-p" "file:///tmp/.tmpHly3KA/serde#1.0.37" "--release" "--" "--wrap-rustc-with" "eprintln"
1 benchmark remaining
Preparing syn
[2022-04-03T15:26:47Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-04-03T15:26:47Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-04-03T15:26:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppYPcgG#syn:0.11.11" "--" "--skip-this-rustc"
[2022-04-03T15:26:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmplc7QMQ#syn:0.11.11" "--release" "--" "--skip-this-rustc"
[2022-04-03T15:26:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-04-03T15:26:53Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-04-03T15:26:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRmkzt1#syn:0.11.11" "--" "--wrap-rustc-with" "eprintln"
Running syn: Opt + [Full]
