plain
[2023-01-31T11:50:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-31T11:50:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-31T11:50:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphyNQI0#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-31T11:50:39Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-31T11:50:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphyNQI0#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmphyNQI0/incremental-state"
[2023-01-31T11:50:43Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-31T11:50:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphyNQI0#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmphyNQI0/incremental-state"
[2023-01-31T11:50:45Z DEBUG collector::execute] applying println to "/tmp/.tmphyNQI0"
[2023-01-31T11:50:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-31T11:50:45Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-31T11:50:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphyNQI0#bitmaps@3.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmphyNQI0/incremental-state"
[2023-01-31T11:50:46Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-31T11:50:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-31T11:50:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTrSTf9#bitmaps@3.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-31T11:50:49Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2023-01-31T11:50:56Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-31T11:50:56Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-31T11:50:56Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBfrH8G#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-31T11:50:59Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-31T11:50:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBfrH8G#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpBfrH8G/incremental-state"
[2023-01-31T11:51:04Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-31T11:51:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBfrH8G#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpBfrH8G/incremental-state"
[2023-01-31T11:51:05Z DEBUG collector::execute] applying println to "/tmp/.tmpBfrH8G"
[2023-01-31T11:51:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-31T11:51:05Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-31T11:51:05Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBfrH8G#bitmaps@3.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpBfrH8G/incremental-state"
Executing benchmark cargo-0.60.0 (2/8)
Preparing cargo-0.60.0
[2023-01-31T11:51:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-01-31T11:51:06Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
[2023-01-31T11:52:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-31T11:52:09Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-31T11:52:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpihhKMG#cargo@0.60.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark cargo-0.60.0 (2/8)
collector error: Failed to profile 'cargo-0.60.0' with Eprintln, recorded: expected success, got exit status: 101
stderr=    Checking unicode-normalization v0.1.19
    Checking form_urlencoded v1.0.1
   Compiling typenum v1.15.0
    Checking foreign-types-shared v0.1.1
    Checking foreign-types-shared v0.1.1
    Checking regex-syntax v0.6.25
    Checking openssl-probe v0.1.5
   Compiling openssl v0.10.38
   Compiling curl v0.4.42
   Compiling serde_json v1.0.79
   Compiling crc32fast v1.3.2
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "foreign_types_shared", "/cargo/registry/src/github.com-1ecc6299db9ec823/foreign-types-shared-0.1.1/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=c353154620d0db36", "-C", "extra-filename=-c353154620d0db36", "--out-dir", "/tmp/.tmpihhKMG/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpihhKMG/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "unicode_normalization", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/unicode-normalization-0.1.19/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"std\"", "-C", "metadata=54ef0a7508ee26c5", "-C", "extra-filename=-54ef0a7508ee26c5", "--out-dir", "/tmp/.tmpihhKMG/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpihhKMG/target/debug/deps", "--extern", "tinyvec=/tmp/.tmpihhKMG/target/debug/deps/libtinyvec-4f577378a76585ca.rmeta", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
    Checking anyhow v1.0.55
    Checking bstr v0.2.17
    Checking socket2 v0.4.4
exiting -- non-wrapped rustc
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "form_urlencoded", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/form_urlencoded-1.0.1/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=94e48305b76d4df7", "-C", "extra-filename=-94e48305b76d4df7", "--out-dir", "/tmp/.tmpihhKMG/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpihhKMG/target/debug/deps", "--extern", "matches=/tmp/.tmpihhKMG/target/debug/deps/libmatches-1bd008bb47338c7e.rmeta", "--extern", "percent_encoding=/tmp/.tmpihhKMG/target/debug/deps/libpercent_encoding-4c30c71be0ff39f5.rmeta", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
warning: build failed, waiting for other jobs to finish...
exiting -- non-wrapped rustc
error: could not compile `unicode-normalization`
error: could not compile `unicode-normalization`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "regex_syntax", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/regex-syntax-0.6.25/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"unicode\"", "--cfg", "feature=\"unicode-age\"", "--cfg", "feature=\"unicode-bool\"", "--cfg", "feature=\"unicode-case\"", "--cfg", "feature=\"unicode-gencat\"", "--cfg", "feature=\"unicode-perl\"", "--cfg", "feature=\"unicode-script\"", "--cfg", "feature=\"unicode-segment\"", "-C", "metadata=b65cedb3bd8cf729", "-C", "extra-filename=-b65cedb3bd8cf729", "--out-dir", "/tmp/.tmpihhKMG/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpihhKMG/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/serde_json-1.0.79/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"raw_value\"", "--cfg", "feature=\"std\"", "-C", "metadata=79a0dfd723d5c993", "-C", "extra-filename=-79a0dfd723d5c993", "--out-dir", "/tmp/.tmpihhKMG/target/debug/build/serde_json-79a0dfd723d5c993", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpihhKMG/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/openssl-0.10.38/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=896abc801f90f78a", "-C", "extra-filename=-896abc801f90f78a", "--out-dir", "/tmp/.tmpihhKMG/target/debug/build/openssl-896abc801f90f78a", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpihhKMG/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
exiting -- non-wrapped rustc
error: could not compile `regex-syntax`
error: could not compile `serde_json`
error: could not compile `serde_json`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "build_script_build", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/curl-0.4.42/build.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "bin", "--emit=dep-info,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"http2\"", "--cfg", "feature=\"openssl-probe\"", "--cfg", "feature=\"openssl-sys\"", "--cfg", "feature=\"ssl\"", "-C", "metadata=277acf8b1a3267e5", "-C", "extra-filename=-277acf8b1a3267e5", "--out-dir", "/tmp/.tmpihhKMG/target/debug/build/curl-277acf8b1a3267e5", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpihhKMG/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
error: could not compile `openssl`
exiting -- non-wrapped rustc
error: could not compile `curl`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "openssl_probe", "/cargo/registry/src/github.com-1ecc6299db9ec823/openssl-probe-0.1.5/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=74df13d75d3fe535", "-C", "extra-filename=-74df13d75d3fe535", "--out-dir", "/tmp/.tmpihhKMG/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpihhKMG/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `openssl-probe`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "crc32fast", "/cargo/registry/src/github.com-1ecc6299db9ec823/crc32fast-1.3.2/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"std\"", "-C", "metadata=b10e319b5424d686", "-C", "extra-filename=-b10e319b5424d686", "--out-dir", "/tmp/.tmpihhKMG/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpihhKMG/target/debug/deps", "--extern", "cfg_if=/tmp/.tmpihhKMG/target/debug/deps/libcfg_if-90e3626e3c41eec3.rmeta", "--cap-lints", "allow", "--cfg", "crc32fast_stdarchx86", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "aho_corasick", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/aho-corasick-0.7.18/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"std\"", "-C", "metadata=414657c627aa66fe", "-C", "extra-filename=-414657c627aa66fe", "--out-dir", "/tmp/.tmpihhKMG/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpihhKMG/target/debug/deps", "--extern", "memchr=/tmp/.tmpihhKMG/target/debug/deps/libmemchr-d022af6da608c734.rmeta", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `aho-corasick`
error: could not compile `aho-corasick`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "bstr", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/bstr-0.2.17/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"lazy_static\"", "--cfg", "feature=\"regex-automata\"", "--cfg", "feature=\"std\"", "--cfg", "feature=\"unicode\"", "-C", "metadata=f74a94f7871d3a68", "-C", "extra-filename=-f74a94f7871d3a68", "--out-dir", "/tmp/.tmpihhKMG/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpihhKMG/target/debug/deps", "--extern", "lazy_static=/tmp/.tmpihhKMG/target/debug/deps/liblazy_static-3ab14ef94a1f89cf.rmeta", "--extern", "memchr=/tmp/.tmpihhKMG/target/debug/deps/libmemchr-d022af6da608c734.rmeta", "--extern", "regex_automata=/tmp/.tmpihhKMG/target/debug/deps/libregex_automata-8df462d37214bbaf.rmeta", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `bstr`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "socket2", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/socket2-0.4.4/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "-C", "metadata=3c9f2f616207cbbb", "-C", "extra-filename=-3c9f2f616207cbbb", "--out-dir", "/tmp/.tmpihhKMG/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpihhKMG/target/debug/deps", "--extern", "libc=/tmp/.tmpihhKMG/target/debug/deps/liblibc-6a685a183885f2de.rmeta", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `socket2`
"/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" ["--crate-name", "anyhow", "--edition=2018", "/cargo/registry/src/github.com-1ecc6299db9ec823/anyhow-1.0.55/src/lib.rs", "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat", "--crate-type", "lib", "--emit=dep-info,metadata", "-C", "embed-bitcode=no", "-C", "debuginfo=2", "--cfg", "feature=\"default\"", "--cfg", "feature=\"std\"", "-C", "metadata=e21bf3d3cc947453", "-C", "extra-filename=-e21bf3d3cc947453", "--out-dir", "/tmp/.tmpihhKMG/target/debug/deps", "-C", "linker=clang", "-L", "dependency=/tmp/.tmpihhKMG/target/debug/deps", "--cap-lints", "allow", "-Adeprecated", "-Aunknown-lints", "-Zincremental-verify-ich"]
exiting -- non-wrapped rustc
error: could not compile `anyhow`

 stdout=

Executing benchmark ctfe-stress-5 (3/8)
---
[2023-01-31T11:52:10Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-31T11:52:10Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-31T11:52:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTaIHpw#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-31T11:52:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-31T11:52:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTaIHpw#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpTaIHpw/incremental-state"
[2023-01-31T11:52:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-31T11:52:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpTaIHpw#ctfe-stress-5@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpTaIHpw/incremental-state"
[2023-01-31T11:52:32Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-31T11:52:32Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-31T11:52:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcwJpW4#ctfe-stress-5@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-31T11:52:41Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2023-01-31T11:52:53Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-31T11:52:53Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-31T11:52:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppisdej#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-31T11:53:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-31T11:53:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppisdej#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppisdej/incremental-state"
[2023-01-31T11:53:13Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-31T11:53:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppisdej#ctfe-stress-5@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmppisdej/incremental-state"
Executing benchmark diesel-1.4.8 (4/8)
Preparing diesel-1.4.8
[2023-01-31T11:53:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-01-31T11:53:14Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2023-01-31T11:54:34Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-31T11:54:34Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-31T11:54:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyr9xGS#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-31T11:54:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-31T11:54:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyr9xGS#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpyr9xGS/incremental-state"
[2023-01-31T11:55:27Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-31T11:55:27Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyr9xGS#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpyr9xGS/incremental-state"
[2023-01-31T11:55:31Z DEBUG collector::execute] applying println to "/tmp/.tmpyr9xGS"
[2023-01-31T11:55:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-31T11:55:31Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-31T11:55:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyr9xGS#diesel@1.4.8" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpyr9xGS/incremental-state"
[2023-01-31T11:55:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-31T11:55:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-31T11:55:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzbTrXN#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-31T11:56:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-31T11:56:01Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-31T11:56:01Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzbTrXN#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzbTrXN/incremental-state"
[2023-01-31T11:56:30Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-31T11:56:30Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzbTrXN#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzbTrXN/incremental-state"
[2023-01-31T11:56:35Z DEBUG collector::execute] applying println to "/tmp/.tmpzbTrXN"
[2023-01-31T11:56:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-31T11:56:35Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2023-01-31T11:56:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpzbTrXN#diesel@1.4.8" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpzbTrXN/incremental-state"
Executing benchmark externs (5/8)
Preparing externs
[2023-01-31T11:56:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2023-01-31T11:56:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
---
[2023-01-31T11:56:46Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-31T11:56:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-31T11:56:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwRDFW6#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-31T11:56:47Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-31T11:56:47Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwRDFW6#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwRDFW6/incremental-state"
[2023-01-31T11:56:48Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-31T11:56:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwRDFW6#externs@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpwRDFW6/incremental-state"
Executing benchmark match-stress (6/8)
Preparing match-stress
[2023-01-31T11:56:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-01-31T11:56:49Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2023-01-31T11:56:50Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-31T11:56:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=Some(Full), patch=None
[2023-01-31T11:56:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMwVpcE#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-31T11:56:53Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrFull), patch=None
[2023-01-31T11:56:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMwVpcE#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMwVpcE/incremental-state"
[2023-01-31T11:56:57Z INFO  collector::execute] run_rustc with incremental=true, profile=Check, scenario=Some(IncrUnchanged), patch=None
[2023-01-31T11:56:57Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMwVpcE#match-stress@0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpMwVpcE/incremental-state"
[2023-01-31T11:56:59Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-31T11:56:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-31T11:56:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmp3bVXDJ#match-stress@0.1.0" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-31T11:57:02Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
---
[2023-01-31T11:57:07Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-31T11:57:07Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-31T11:57:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdF9BAr#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-31T11:57:11Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-31T11:57:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdF9BAr#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdF9BAr/incremental-state"
[2023-01-31T11:57:15Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-31T11:57:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpdF9BAr#match-stress@0.1.0" "--release" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpdF9BAr/incremental-state"
Executing benchmark token-stream-stress (7/8)
Preparing token-stream-stress
[2023-01-31T11:57:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-01-31T11:57:16Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
[2023-01-31T11:57:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-31T11:57:18Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2023-01-31T11:57:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDD6Cmm#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-31T11:57:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrFull), patch=None
[2023-01-31T11:57:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDD6Cmm#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDD6Cmm/incremental-state"
[2023-01-31T11:57:18Z INFO  collector::execute] run_rustc with incremental=true, profile=Debug, scenario=Some(IncrUnchanged), patch=None
[2023-01-31T11:57:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpDD6Cmm#token-stream-stress@0.0.0" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpDD6Cmm/incremental-state"
[2023-01-31T11:57:19Z DEBUG collector::execute] Benchmark iteration 1/1
[2023-01-31T11:57:19Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2023-01-31T11:57:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgFZBpl#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln"
[2023-01-31T11:57:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-31T11:57:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrFull), patch=None
[2023-01-31T11:57:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgFZBpl#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpgFZBpl/incremental-state"
[2023-01-31T11:57:19Z INFO  collector::execute] run_rustc with incremental=true, profile=Opt, scenario=Some(IncrUnchanged), patch=None
[2023-01-31T11:57:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpgFZBpl#token-stream-stress@0.0.0" "--release" "--bin" "token-stream-stress-bin" "--" "--wrap-rustc-with" "Eprintln" "-C" "incremental=/tmp/.tmpgFZBpl/incremental-state"
Executing benchmark tuple-stress (8/8)
Preparing tuple-stress
[2023-01-31T11:57:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Check, scenario=None, patch=None
[2023-01-31T11:57:20Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
---
Gather profiles (rustc PGO):             567.81s (16.65%)
Total duration:                         3411.15s
---------------------------------------------------------
Traceback (most recent call last):
  File "../src/ci/stage-build.py", line 659, in <module>
    raise e
  File "../src/ci/stage-build.py", line 656, in <module>
    execute_build_pipeline(timer, pipeline, build_args)
  File "../src/ci/stage-build.py", line 612, in execute_build_pipeline
    gather_rustc_profiles(pipeline)
  File "../src/ci/stage-build.py", line 504, in gather_rustc_profiles
    LLVM_PROFILE_FILE=str(pipeline.rustc_profile_template_path())
  File "../src/ci/stage-build.py", line 383, in run_compiler_benchmarks
    **env
  File "../src/ci/stage-build.py", line 333, in cmd
    return subprocess.run(args, env=environment, check=True)
  File "/usr/lib64/python3.6/subprocess.py", line 438, in run
    output=stdout, stderr=stderr)
subprocess.CalledProcessError: Command '['/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo', 'run', '-p', 'collector', '--bin', 'collector', '--', 'profile_local', 'eprintln', '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc', '--id', 'Test', '--cargo', '/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo', '--profiles', 'Check,Debug,Opt', '--scenarios', 'All', '--include', 'externs,ctfe-stress-5,cargo-0.60.0,token-stream-stress,match-stress,tuple-stress,diesel-1.4.8,bitmaps-3.1.0']' returned non-zero exit status 1.
