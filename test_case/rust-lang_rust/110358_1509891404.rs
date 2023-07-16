
@nix { "action": "setPhase", "phase": "unpackPhase" }
unpacking sources
unpacking source archive /nix/store/w4vbp75lxgq5x2w7dy94f6d6d8p81rwh-source
source root is source
Executing cargoSetupPostUnpackHook
Finished cargoSetupPostUnpackHook
@nix { "action": "setPhase", "phase": "patchPhase" }
patching sources
substituteStream(): WARNING: pattern '"transforms-geoip",' doesn't match anything in file './Cargo.toml'
Executing cargoSetupPostPatchHook
Validating consistency between /build/source/Cargo.lock and /build/cargo-vendor-dir/Cargo.lock
Finished cargoSetupPostPatchHook
@nix { "action": "setPhase", "phase": "configurePhase" }
configuring
@nix { "action": "setPhase", "phase": "buildPhase" }
building
Executing cargoBuildHook
++ env CC_x86_64-unknown-linux-gnu=/nix/store/nlgyw2fv0cm8rkz8qm1jyw78vyif1bl9-gcc-wrapper-12.2.0/bin/cc CXX_x86_64-unknown-linux-gnu=/nix/store/nlgyw2fv0cm8rkz8qm1jyw78vyif1bl9-gcc-wrapper-12.2.0/bin/c++ CC_x86_64-unknown-linux-gnu=/nix/store/nlgyw2fv0cm8rkz8qm1jyw78vyif1bl9-gcc-wrapper-12.2.0/bin/cc CXX_x86_64-unknown-linux-gnu=/nix/store/nlgyw2fv0cm8rkz8qm1jyw78vyif1bl9-gcc-wrapper-12.2.0/bin/c++ cargo build -j 8 --target x86_64-unknown-linux-gnu --frozen --release --no-default-features --features=api,api-client,enrichment-tables,sinks,sources,transforms,vrl-cli,unix
[0m[0m[1m[32m   Compiling[0m proc-macro2 v1.0.55
[0m[0m[1m[32m   Compiling[0m quote v1.0.26
[0m[0m[1m[32m   Compiling[0m unicode-ident v1.0.5
[0m[0m[1m[32m   Compiling[0m libc v0.2.140
[0m[0m[1m[32m   Compiling[0m autocfg v1.1.0
[0m[0m[1m[32m   Compiling[0m serde_derive v1.0.159
[0m[0m[1m[32m   Compiling[0m cfg-if v1.0.0
[0m[0m[1m[32m   Compiling[0m syn v1.0.109
[0m[0m[1m[32m   Compiling[0m serde v1.0.159
[0m[0m[1m[32m   Compiling[0m once_cell v1.17.1
[0m[0m[1m[32m   Compiling[0m version_check v0.9.4
[0m[0m[1m[32m   Compiling[0m memchr v2.5.0
[0m[0m[1m[32m   Compiling[0m log v0.4.17
[0m[0m[1m[32m   Compiling[0m pin-project-lite v0.2.9
[0m[0m[1m[32m   Compiling[0m futures-core v0.3.28
[0m[0m[1m[32m   Compiling[0m lock_api v0.4.9
[0m[0m[1m[32m   Compiling[0m itoa v1.0.4
[0m[0m[1m[32m   Compiling[0m parking_lot_core v0.9.4
[0m[0m[1m[32m   Compiling[0m futures-io v0.3.28
[0m[0m[1m[32m   Compiling[0m ahash v0.7.6
[0m[0m[1m[32m   Compiling[0m byteorder v1.4.3
[0m[0m[1m[32m   Compiling[0m slab v0.4.7
[0m[0m[1m[32m   Compiling[0m scopeguard v1.1.0
[0m[0m[1m[32m   Compiling[0m syn v2.0.10
[0m[0m[1m[32m   Compiling[0m indexmap v1.9.3
[0m[0m[1m[32m   Compiling[0m futures-sink v0.3.28
[0m[0m[1m[32m   Compiling[0m libm v0.2.6
[0m[0m[1m[32m   Compiling[0m num-traits v0.2.15
[0m[0m[1m[32m   Compiling[0m futures-task v0.3.28
[0m[0m[1m[32m   Compiling[0m getrandom v0.2.8
[0m[0m[1m[32m   Compiling[0m jobserver v0.1.25
[0m[0m[1m[32m   Compiling[0m cc v1.0.77
[0m[0m[1m[32m   Compiling[0m iovec v0.1.4
[0m[0m[1m[32m   Compiling[0m hashbrown v0.12.3
[0m[0m[1m[32m   Compiling[0m futures-channel v0.3.28
[0m[0m[1m[32m   Compiling[0m bytes v0.4.12
[0m[0m[1m[32m   Compiling[0m socket2 v0.4.7
[0m[0m[1m[32m   Compiling[0m futures-util v0.3.28
[0m[0m[1m[32m   Compiling[0m futures v0.1.31
[0m[0m[1m[32m   Compiling[0m ryu v1.0.13
[0m[0m[1m[32m   Compiling[0m pin-utils v0.1.0
[0m[0m[1m[32m   Compiling[0m signal-hook-registry v1.4.0
[0m[0m[1m[32m   Compiling[0m num-integer v0.1.45
[0m[0m[1m[32m   Compiling[0m tracing-core v0.1.30
[0m[0m[1m[32m   Compiling[0m mio v0.8.5
[0m[0m[1m[32m   Compiling[0m bitflags v1.3.2
[0m[0m[1m[32m   Compiling[0m pkg-config v0.3.26
[0m[0m[1m[32m   Compiling[0m tokio-io v0.1.13
[0m[0m[1m[32m   Compiling[0m ppv-lite86 v0.2.17
[0m[0m[1m[32m   Compiling[0m num_cpus v1.14.0
[0m[0m[1m[32m   Compiling[0m fnv v1.0.7
[0m[0m[1m[32m   Compiling[0m rand_core v0.6.4
[0m[0m[1m[32m   Compiling[0m tokio v1.26.0
[0m[0m[1m[32m   Compiling[0m percent-encoding v2.2.0
[0m[0m[1m[32m   Compiling[0m spin v0.5.2
[0m[0m[1m[32m   Compiling[0m httparse v1.8.0
[0m[0m[1m[32m   Compiling[0m rand_chacha v0.3.1
[0m[0m[1m[32m   Compiling[0m thiserror v1.0.40
[0m[0m[1m[32m   Compiling[0m base64 v0.13.1
[0m[0m[1m[32m   Compiling[0m rand v0.8.5
[0m[0m[1m[32m   Compiling[0m form_urlencoded v1.1.0
[0m[0m[1m[32m   Compiling[0m futures-macro v0.3.28
[0m[0m[1m[32m   Compiling[0m thiserror-impl v1.0.40
[0m[0m[1m[32m   Compiling[0m typenum v1.15.0
[0m[0m[1m[32m   Compiling[0m serde_json v1.0.95
[0m[0m[1m[32m   Compiling[0m unicode-xid v0.2.4
[0m[0m[1m[32m   Compiling[0m openssl-src v111.25.0+1.1.1t
[0m[0m[1m[32m   Compiling[0m generic-array v0.14.6
[0m[0m[1m[32m   Compiling[0m either v1.8.0
[0m[0m[1m[32m   Compiling[0m openssl-sys v0.9.83
[0m[0m[1m[32m   Compiling[0m tinyvec_macros v0.1.0
[0m[0m[1m[32m   Compiling[0m tower-service v0.3.2
[0m[0m[1m[32m   Compiling[0m tinyvec v1.6.0
[0m[0m[1m[32m   Compiling[0m aho-corasick v0.7.20
[0m[0m[1m[32m   Compiling[0m tracing-attributes v0.1.23
[0m[0m[1m[32m   Compiling[0m tokio-macros v1.8.0
[0m[0m[1m[32m   Compiling[0m pin-project-internal v1.0.12
[0m[0m[1m[32m   Compiling[0m unicode-normalization v0.1.22
[0m[0m[1m[32m   Compiling[0m regex-syntax v0.6.29
[0m[0m[1m[32m   Compiling[0m unicode-bidi v0.3.8
[0m[0m[1m[32m   Compiling[0m tracing v0.1.37
[0m[0m[1m[32m   Compiling[0m pin-project v1.0.12
[0m[0m[1m[32m   Compiling[0m regex v1.7.3
[0m[0m[1m[32m   Compiling[0m httpdate v1.0.2
[0m[0m[1m[32m   Compiling[0m try-lock v0.2.3
[0m[0m[1m[32m   Compiling[0m want v0.3.0
[0m[0m[1m[32m   Compiling[0m itertools v0.10.5
[0m[0m[1m[32m   Compiling[0m heck v0.4.0
[0m[0m[1m[32m   Compiling[0m foreign-types-shared v0.1.1
[0m[0m[1m[32m   Compiling[0m openssl v0.10.48
[0m[0m[1m[32m   Compiling[0m foreign-types v0.3.2
[0m[0m[1m[32m   Compiling[0m idna v0.3.0
[0m[0m[1m[32m   Compiling[0m openssl-macros v0.1.0
[0m[0m[1m[32m   Compiling[0m bytes v1.4.0
[0m[0m[1m[32m   Compiling[0m smallvec v1.10.0
[0m[0m[1m[32m   Compiling[0m ring v0.16.20
[0m[0m[1m[32m   Compiling[0m parking_lot v0.12.1
[0m[0m[1m[32m   Compiling[0m http v0.2.9
[0m[0m[1m[32m   Compiling[0m http-body v0.4.5
[0m[0m[1m[32m   Compiling[0m openssl-probe v0.1.5
[0m[0m[1m[32m   Compiling[0m url v2.3.1
[0m[0m[1m[32m   Compiling[0m lazy_static v1.4.0
[0m[0m[1m[32m   Compiling[0m crypto-common v0.1.6
[0m[0m[1m[32m   Compiling[0m untrusted v0.7.1
[0m[0m[1m[32m   Compiling[0m time-core v0.1.0
[0m[0m[1m[32m   Compiling[0m fixedbitset v0.4.2
[0m[0m[1m[32m   Compiling[0m time-macros v0.2.6
[0m[0m[1m[32m   Compiling[0m petgraph v0.6.2
[0m[0m[1m[32m   Compiling[0m tokio-util v0.7.4 (https://github.com/vectordotdev/tokio?branch=tokio-util-0.7.4-framed-read-continue-on-error#53a17f25)
[0m[0m[1m[32m   Compiling[0m async-trait v0.1.68
[0m[0m[1m[32m   Compiling[0m h2 v0.3.16
[0m[0m[1m[32m   Compiling[0m subtle v2.4.1
[0m[0m[1m[32m   Compiling[0m num_threads v0.1.6
[0m[0m[1m[32m   Compiling[0m const-oid v0.9.1
[0m[0m[1m[32m   Compiling[0m time v0.3.17
[0m[0m[1m[32m   Compiling[0m fastrand v1.8.0
[0m[0m[1m[32m   Compiling[0m iana-time-zone v0.1.53
[0m[0m[1m[32m   Compiling[0m block-buffer v0.10.3
[0m[0m[1m[32m   Compiling[0m chrono v0.4.24 (https://github.com/vectordotdev/chrono.git?tag=v0.4.24-no-default-time-1#7ec1ad93)
[0m[0m[1m[32m   Compiling[0m mime v0.3.16
[0m[0m[1m[32m   Compiling[0m native-tls v0.2.11
[0m[0m[1m[32m   Compiling[0m digest v0.10.6
[0m[0m[1m[32m   Compiling[0m ident_case v1.0.1
[0m[0m[1m[32m   Compiling[0m strsim v0.10.0
[0m[0m[1m[32m   Compiling[0m anyhow v1.0.70
[0m[0m[1m[32m   Compiling[0m futures-executor v0.3.28
[0m[0m[1m[32m   Compiling[0m cpufeatures v0.2.5
[0m[0m[1m[32m   Compiling[0m siphasher v0.3.10
[0m[0m[1m[32m   Compiling[0m hyper v0.14.25
[0m[0m[1m[32m   Compiling[0m futures v0.3.28
[0m[0m[1m[32m   Compiling[0m crc32fast v1.3.2
[0m[0m[1m[32m   Compiling[0m serde_urlencoded v0.7.1
[0m[0m[1m[32m   Compiling[0m crossbeam-utils v0.8.15
[0m[0m[1m[32m   Compiling[0m encoding_rs v0.8.32
[0m[0m[1m[32m   Compiling[0m synstructure v0.12.6
[0m[0m[1m[32m   Compiling[0m zeroize_derive v1.3.2
[0m[0m[1m[32m   Compiling[0m zeroize v1.5.7
[0m[0m[1m[32m   Compiling[0m dirs-sys-next v0.1.2
[0m[0m[1m[32m   Compiling[0m crunchy v0.2.2
[0m[0m[1m[32m   Compiling[0m dirs-next v2.0.0
[0m[0m[1m[32m   Compiling[0m tower-layer v0.3.2
[0m[0m[1m[32m   Compiling[0m tiny-keccak v2.0.2
[0m[0m[1m[32m   Compiling[0m term v0.7.0
[0m[0m[1m[32m   Compiling[0m phf_shared v0.10.0
[0m[0m[1m[32m   Compiling[0m bit-vec v0.6.3
[0m[0m[1m[32m   Compiling[0m semver v1.0.17
[0m[0m[1m[32m   Compiling[0m precomputed-hash v0.1.1
[0m[0m[1m[32m   Compiling[0m new_debug_unreachable v1.0.4
[0m[0m[1m[32m   Compiling[0m ucd-trie v0.1.5
[0m[0m[1m[32m   Compiling[0m string_cache v0.8.4
[0m[0m[1m[32m   Compiling[0m pest v2.5.6
[0m[0m[1m[32m   Compiling[0m bit-set v0.5.3
[0m[0m[1m[32m   Compiling[0m ascii-canvas v3.0.0
[0m[0m[1m[32m   Compiling[0m darling_core v0.14.2
[0m[0m[1m[32m   Compiling[0m ena v0.14.0
[0m[0m[1m[32m   Compiling[0m lalrpop-util v0.19.9
[0m[0m[1m[32m   Compiling[0m atty v0.2.14
[0m[0m[1m[32m   Compiling[0m diff v0.1.13
[0m[0m[1m[32m   Compiling[0m pico-args v0.4.2
[0m[0m[1m[32m   Compiling[0m lalrpop v0.19.8
[0m[0m[1m[32m   Compiling[0m toml v0.5.11
[0m[0m[1m[32m   Compiling[0m parking v2.0.0
[0m[0m[1m[32m   Compiling[0m waker-fn v1.1.0
[0m[0m[1m[32m   Compiling[0m event-listener v2.5.3
[0m[0m[1m[32m   Compiling[0m paste v1.0.12
[0m[0m[1m[32m   Compiling[0m proc-macro-crate v1.2.1
[0m[0m[1m[32m   Compiling[0m darling_macro v0.14.2
[0m[0m[1m[32m   Compiling[0m futures-lite v1.12.0
[0m[0m[1m[32m   Compiling[0m rustc_version v0.4.0
[0m[0m[1m[32m   Compiling[0m prost-derive v0.11.8
[0m[0m[1m[32m   Compiling[0m tower v0.4.13
[0m[0m[1m[32m   Compiling[0m rustls-pemfile v1.0.1
[0m[0m[1m[32m   Compiling[0m darling v0.14.2
[0m[0m[1m[32m   Compiling[0m minimal-lexical v0.2.1
[0m[0m[1m[32m   Compiling[0m doc-comment v0.3.3
[0m[0m[1m[32m   Compiling[0m nom v7.1.3
[0m[0m[1m[32m   Compiling[0m phf_shared v0.11.1
[0m[0m[1m[32m   Compiling[0m webpki v0.22.0
[0m[0m[1m[32m   Compiling[0m spin v0.9.4
[0m[0m[1m[32m   Compiling[0m num-bigint v0.4.3
[0m[0m[1m[32m   Compiling[0m rustls v0.20.7
[0m[0m[1m[32m   Compiling[0m io-lifetimes v1.0.3
[0m[0m[1m[32m   Compiling[0m sct v0.7.0
[0m[0m[1m[32m   Compiling[0m lua-src v544.0.1
[0m[0m[1m[32m   Compiling[0m luajit-src v210.4.3+resty8384278
[0m[0m[1m[32m   Compiling[0m unicode-width v0.1.10
[0m[0m[1m[32m   Compiling[0m static_assertions v1.1.0
[0m[0m[1m[32m   Compiling[0m rustix v0.37.5
[0m[0m[1m[32m   Compiling[0m termcolor v1.2.0
[0m[0m[1m[32m   Compiling[0m mlua v0.8.8
[0m[0m[1m[32m   Compiling[0m phf_generator v0.11.1
[0m[0m[1m[32m   Compiling[0m async-graphql-value v5.0.7
[0m[0m[1m[32m   Compiling[0m bytes-utils v0.1.3
[0m[0m[1m[32m   Compiling[0m tokio-stream v0.1.12
[0m[0m[1m[32m   Compiling[0m async-stream-impl v0.3.4
[0m[0m[1m[32m   Compiling[0m multer v2.0.4
[0m[0m[1m[32m   Compiling[0m prettyplease v0.1.21
[0m[0m[1m[32m   Compiling[0m hex v0.4.3
[0m[0m[1m[32m   Compiling[0m linux-raw-sys v0.3.0
[0m[0m[1m[32m   Compiling[0m cache-padded v1.2.0
[0m[0m[1m[32m   Compiling[0m concurrent-queue v1.2.4
[0m[0m[1m[32m   Compiling[0m async-stream v0.3.4
[0m[0m[1m[32m   Compiling[0m async-graphql-parser v5.0.7
[0m[0m[1m[32m   Compiling[0m phf_codegen v0.11.1
[0m[0m[1m[32m   Compiling[0m prost v0.11.8
[0m[0m[1m[32m   Compiling[0m phf v0.11.1
[0m[0m[1m[32m   Compiling[0m aws-smithy-types v0.51.0
[0m[0m[1m[32m   Compiling[0m Inflector v0.11.4
[0m[0m[1m[32m   Compiling[0m serde_spanned v0.6.1
[0m[0m[1m[32m   Compiling[0m toml_datetime v0.6.1
[0m[0m[1m[32m   Compiling[0m snafu-derive v0.7.4
[0m[0m[1m[32m   Compiling[0m env_logger v0.8.4
[0m[0m[1m[32m   Compiling[0m parse-zoneinfo v0.3.0
[0m[0m[1m[32m   Compiling[0m winnow v0.3.5
[0m[0m[1m[32m   Compiling[0m dyn-clone v1.0.11
[0m[0m[1m[32m   Compiling[0m tempfile v3.5.0
[0m[0m[1m[32m   Compiling[0m chrono-tz-build v0.1.0
[0m[0m[1m[32m   Compiling[0m quickcheck v1.0.3
[0m[0m[1m[32m   Compiling[0m async-graphql-derive v5.0.7
[0m[0m[1m[32m   Compiling[0m snafu v0.7.4
[0m[0m[1m[32m   Compiling[0m toml_edit v0.19.6
[0m[0m[1m[32m   Compiling[0m prost-types v0.11.8
[0m[0m[1m[32m   Compiling[0m lookup v0.1.0 (https://github.com/vectordotdev/vrl?rev=v0.2.0#258cc611)
[0m[0m[1m[32m   Compiling[0m async-channel v1.7.1
[0m[0m[1m[32m   Compiling[0m async-lock v2.6.0
[0m[0m[1m[32m   Compiling[0m derivative v2.2.0
[0m[0m[1m[32m   Compiling[0m inherent v1.0.3
[0m[0m[1m[32m   Compiling[0m which v4.3.0
[0m[0m[1m[32m   Compiling[0m unicase v2.6.0
[0m[0m[1m[32m   Compiling[0m multimap v0.8.3
[0m[0m[1m[32m   Compiling[0m prost-build v0.11.8
[0m[0m[1m[32m   Compiling[0m toml v0.7.3
[0m[0m[1m[32m   Compiling[0m chrono-tz v0.8.1
[0m[0m[1m[32m   Compiling[0m codespan-reporting v0.11.1
[0m[0m[1m[32m   Compiling[0m async-graphql v5.0.7
[0m[0m[1m[32m   Compiling[0m sha2 v0.10.6
[0m[0m[1m[32m   Compiling[0m darling_core v0.13.4
[0m[0m[1m[32m   Compiling[0m ordered-float v3.6.0
[0m[0m[1m[32m   Compiling[0m bstr v0.2.17
[0m[0m[1m[32m   Compiling[0m rustc-hash v1.1.0
[0m[0m[1m[32m   Compiling[0m vrl-diagnostic v0.1.0 (https://github.com/vectordotdev/vrl?rev=v0.2.0#258cc611)
[0m[0m[1m[32m   Compiling[0m vrl-parser v0.1.0 (https://github.com/vectordotdev/vrl?rev=v0.2.0#258cc611)
[0m[0m[1m[32m   Compiling[0m aws-smithy-eventstream v0.51.0
[0m[0m[1m[32m   Compiling[0m tokio-rustls v0.23.4
[0m[0m[1m[32m   Compiling[0m aws-smithy-http v0.51.0
[0m[0m[1m[32m   Compiling[0m darling_macro v0.13.4
[0m[0m[1m[32m   Compiling[0m rustls-native-certs v0.6.2
[0m[0m[1m[32m   Compiling[0m polling v2.5.0
[0m[0m[1m[32m   Compiling[0m concurrent-queue v2.0.0
[0m[0m[1m[32m   Compiling[0m zstd-sys v2.0.4+zstd.1.5.2
[0m[0m[1m[32m   Compiling[0m async-io v1.12.0
[0m[0m[1m[32m   Compiling[0m async-task v4.3.0
[0m[0m[1m[32m   Compiling[0m getrandom v0.1.16
[0m[0m[1m[32m   Compiling[0m anymap v1.0.0-beta.2
[0m[0m[1m[32m   Compiling[0m mime_guess v2.0.4
[0m[0m[1m[32m   Compiling[0m darling v0.13.4
[0m[0m[1m[32m   Compiling[0m signal-hook v0.3.14
[0m[0m[1m[32m   Compiling[0m base64 v0.21.0
[0m[0m[1m[32m   Compiling[0m atomic-waker v1.0.0
[0m[0m[1m[32m   Compiling[0m blocking v1.3.0
[0m[0m[1m[32m   Compiling[0m aws-smithy-http-tower v0.51.0
[0m[0m[1m[32m   Compiling[0m aws-smithy-async v0.51.0
[0m[0m[1m[32m   Compiling[0m aws-types v0.51.0
[0m[0m[1m[32m   Compiling[0m sha1 v0.10.5
[0m[0m[1m[32m   Compiling[0m debug-helper v0.3.13
[0m[0m[1m[32m   Compiling[0m rustversion v1.0.9
[0m[0m[1m[32m   Compiling[0m ipnet v2.5.1
[0m[0m[1m[32m   Compiling[0m cidr-utils v0.5.10
[0m[0m[1m[32m   Compiling[0m rand_core v0.5.1
[0m[0m[1m[32m   Compiling[0m webpki-roots v0.22.5
[0m[0m[1m[32m   Compiling[0m serde_with_macros v2.3.1
[0m[0m[1m[32m   Compiling[0m uuid v1.3.0
[0m[0m[1m[32m   Compiling[0m ordered-float v2.10.0
[0m[0m[1m[32m   Compiling[0m base64ct v1.5.3
[0m[0m[1m[32m   Compiling[0m linked-hash-map v0.5.6
[0m[0m[1m[32m   Compiling[0m serde_with v2.3.1
[0m[0m[1m[32m   Compiling[0m hyper-rustls v0.23.1
[0m[0m[1m[32m   Compiling[0m hmac v0.12.1
[0m[0m[1m[32m   Compiling[0m ghost v0.1.6
[0m[0m[1m[32m   Compiling[0m ctor v0.2.0
[0m[0m[1m[32m   Compiling[0m portable-atomic v0.3.15
[0m[0m[1m[32m   Compiling[0m adler v1.0.2
[0m[0m[1m[32m   Compiling[0m inventory v0.3.5
[0m[0m[1m[32m   Compiling[0m miniz_oxide v0.6.2
[0m[0m[1m[32m   Compiling[0m vector-config-common v0.1.0 (/build/source/lib/vector-config-common)
[0m[0m[1m[32m   Compiling[0m async-executor v1.5.0
[0m[0m[1m[32m   Compiling[0m serde_derive_internals v0.26.0
[0m[0m[1m[32m   Compiling[0m memoffset v0.7.1
[0m[0m[1m[32m   Compiling[0m metrics v0.20.1
[0m[0m[1m[32m   Compiling[0m flate2 v1.0.25
[0m[0m[1m[32m   Compiling[0m vector-config-macros v0.1.0 (/build/source/lib/vector-config-macros)
[0m[0m[1m[32m   Compiling[0m no-proxy v0.3.2
[0m[0m[1m[32m   Compiling[0m aws-sigv4 v0.51.0
[0m[0m[1m[32m   Compiling[0m md-5 v0.10.5
[0m[0m[1m[32m   Compiling[0m metrics-macros v0.6.0
[0m[0m[1m[32m   Compiling[0m async-net v1.7.0
[0m[0m[1m[32m   Compiling[0m num-rational v0.3.2
[0m[0m[1m[32m   Compiling[0m async-fs v1.6.0
[0m[0m[1m[32m   Compiling[0m async-process v1.6.0
[0m[0m[1m[32m   Compiling[0m memoffset v0.6.5
[0m[0m[1m[32m   Compiling[0m http-range-header v0.3.0
[0m[0m[1m[32m   Compiling[0m block-padding v0.3.2
[0m[0m[1m[32m   Compiling[0m futures-timer v3.0.2
[0m[0m[1m[32m   Compiling[0m indoc v2.0.1
[0m[0m[1m[32m   Compiling[0m xmlparser v0.13.5
[0m[0m[1m[32m   Compiling[0m inout v0.1.3
[0m[0m[1m[32m   Compiling[0m tower-http v0.3.5
[0m[0m[1m[32m   Compiling[0m rand_chacha v0.2.2
[0m[0m[1m[32m   Compiling[0m tokio-io-timeout v1.2.0
[0m[0m[1m[32m   Compiling[0m ptr_meta_derive v0.1.4
[0m[0m[1m[32m   Compiling[0m bytecheck v0.6.9
[0m[0m[1m[32m   Compiling[0m heim-common v0.1.0-rc.1 (https://github.com/vectordotdev/heim.git?branch=update-nix#76fa765c)
[0m[0m[1m[32m   Compiling[0m tokio-openssl v0.6.3
[0m[0m[1m[32m   Compiling[0m io-lifetimes v0.7.5
[0m[0m[1m[32m   Compiling[0m rustix v0.36.4
[0m[0m[1m[32m   Compiling[0m ptr_meta v0.1.4
[0m[0m[1m[32m   Compiling[0m hyper-timeout v0.4.1
[0m[0m[1m[32m   Compiling[0m rand v0.7.3
[0m[0m[1m[32m   Compiling[0m nix v0.23.1
[0m[0m[1m[32m   Compiling[0m smol v1.2.5
[0m[0m[1m[32m   Compiling[0m uom v0.31.1
[0m[0m[1m[32m   Compiling[0m cipher v0.4.3
[0m[0m[1m[32m   Compiling[0m axum-core v0.3.2
[0m[0m[1m[32m   Compiling[0m yaml-rust v0.4.5
[0m[0m[1m[32m   Compiling[0m pem-rfc7468 v0.6.0
[0m[0m[1m[32m   Compiling[0m serde-value v0.7.0
[0m[0m[1m[32m   Compiling[0m stream-cancel v0.8.1
[0m[0m[1m[32m   Compiling[0m headers-core v0.2.0
[0m[0m[1m[32m   Compiling[0m regex-automata v0.1.10
[0m[0m[1m[32m   Compiling[0m bytecheck_derive v0.6.9
[0m[0m[1m[32m   Compiling[0m pem v1.1.0
[0m[0m[1m[32m   Compiling[0m onig_sys v69.8.1
[0m[0m[1m[32m   Compiling[0m raw-cpuid v10.6.0
[0m[0m[1m[32m   Compiling[0m crossbeam-epoch v0.9.13
[0m[0m[1m[32m   Compiling[0m overload v0.1.1
[0m[0m[1m[32m   Compiling[0m linux-raw-sys v0.1.3
[0m[0m[1m[32m   Compiling[0m rend v0.4.0
[0m[0m[1m[32m   Compiling[0m data-encoding v2.3.3
[0m[0m[1m[32m   Compiling[0m k8s-openapi v0.16.0
[0m[0m[1m[32m   Compiling[0m outref v0.5.1
[0m[0m[1m[32m   Compiling[0m match_cfg v0.1.0
[0m[0m[1m[32m   Compiling[0m rustix v0.35.13
[0m[0m[1m[32m   Compiling[0m vsimd v0.8.0
[0m[0m[1m[32m   Compiling[0m seahash v4.1.0
[0m[0m[1m[32m   Compiling[0m void v1.0.2
[0m[0m[1m[32m   Compiling[0m unreachable v1.0.0
[0m[0m[1m[32m   Compiling[0m hostname v0.3.1
[0m[0m[1m[32m   Compiling[0m base64-simd v0.8.0
[0m[0m[1m[32m   Compiling[0m nu-ansi-term v0.46.0
[0m[0m[1m[32m   Compiling[0m matchers v0.1.0
[0m[0m[1m[32m   Compiling[0m headers v0.3.8
[0m[0m[1m[32m   Compiling[0m der v0.6.1
[0m[0m[1m[32m   Compiling[0m serde_yaml v0.8.26
[0m[0m[1m[32m   Compiling[0m heim-runtime v0.1.0-rc.1 (https://github.com/vectordotdev/heim.git?branch=update-nix#76fa765c)
[0m[0m[1m[32m   Compiling[0m axum v0.6.7
[0m[0m[1m[32m   Compiling[0m pest_meta v2.5.6
[0m[0m[1m[32m   Compiling[0m tokio-native-tls v0.3.0
[0m[0m[1m[32m   Compiling[0m hyper-tls v0.5.0
[0m[0m[1m[32m   Compiling[0m tracing-log v0.1.3
[0m[0m[1m[32m   Compiling[0m aws-smithy-client v0.51.0
[0m[0m[1m[32m   Compiling[0m reqwest v0.11.16
[0m[0m[1m[32m   Compiling[0m aws-http v0.51.0
[0m[0m[1m[32m   Compiling[0m aws-sig-auth v0.51.0
[0m[0m[1m[32m   Compiling[0m aws-endpoint v0.51.0
[0m[0m[1m[32m   Compiling[0m sharded-slab v0.1.4
[0m[0m[1m[32m   Compiling[0m nibble_vec v0.1.0
[0m[0m[1m[32m   Compiling[0m tracing-serde v0.1.3
[0m[0m[1m[32m   Compiling[0m num-iter v0.1.43
[0m[0m[1m[32m   Compiling[0m thread_local v1.1.4
[0m[0m[1m[32m   Compiling[0m semver-parser v0.7.0
[0m[0m[1m[32m   Compiling[0m erased-serde v0.3.23
[0m[0m[1m[32m   Compiling[0m linux-raw-sys v0.0.46
[0m[0m[1m[32m   Compiling[0m rkyv v0.7.40
[0m[0m[1m[32m   Compiling[0m zstd-safe v5.0.2+zstd.1.5.2
[0m[0m[1m[32m   Compiling[0m ascii v0.9.3
[0m[0m[1m[32m   Compiling[0m const-oid v0.6.2
[0m[0m[1m[32m   Compiling[0m arc-swap v1.6.0
[0m[0m[1m[32m   Compiling[0m endian-type v0.1.2
[0m[0m[1m[32m   Compiling[0m crossbeam-queue v0.3.8
[0m[0m[1m[32m   Compiling[0m http-types v2.12.0
[0m[0m[1m[32m   Compiling[0m radix_trie v0.2.1
[0m[0m[1m[32m   Compiling[0m der v0.4.5
[0m[0m[1m[32m   Compiling[0m combine v3.8.1
[0m[0m[1m[32m   Compiling[0m tracing-subscriber v0.3.16
[0m[0m[1m[32m   Compiling[0m semver v0.9.0
[0m[0m[1m[32m   Compiling[0m pest_generator v2.5.6
[0m[0m[1m[32m   Compiling[0m spki v0.6.0
[0m[0m[1m[32m   Compiling[0m is-terminal v0.4.1
[0m[0m[1m[32m   Compiling[0m quanta v0.10.1
[0m[0m[1m[32m   Compiling[0m aws-smithy-xml v0.51.0
[0m[0m[1m[32m   Compiling[0m tonic-build v0.8.4
[0m[0m[1m[32m   Compiling[0m aws-smithy-json v0.51.0
[0m[0m[1m[32m   Compiling[0m azure_core v0.5.0 (https://github.com/Azure/azure-sdk-for-rust.git?rev=b4544d4920fa3064eb921340054cd9cc130b7664#b4544d49)
[0m[0m[1m[32m   Compiling[0m treediff v3.0.2
[0m[0m[1m[32m   Compiling[0m serde_qs v0.8.5
[0m[0m[1m[32m   Compiling[0m stringprep v0.1.2
[0m[0m[1m[32m   Compiling[0m digest v0.9.0
[0m[0m[1m[32m   Compiling[0m rkyv_derive v0.7.40
[0m[0m[1m[32m   Compiling[0m libz-sys v1.1.8
[0m[0m[1m[32m   Compiling[0m dirs-sys v0.3.7
[0m[0m[1m[32m   Compiling[0m ahash v0.8.2
[0m[0m[1m[32m   Compiling[0m sketches-ddsketch v0.2.0
[0m[0m[1m[32m   Compiling[0m num-bigint-dig v0.8.2
[0m[0m[1m[32m   Compiling[0m convert_case v0.4.0
[0m[0m[1m[32m   Compiling[0m utf-8 v0.7.6
[0m[0m[1m[32m   Compiling[0m matchit v0.7.0
[0m[0m[1m[32m   Compiling[0m cookie-factory v0.3.2
[0m[0m[1m[32m   Compiling[0m infer v0.2.3
[0m[0m[1m[32m   Compiling[0m signature v1.6.4
[0m[0m[1m[32m   Compiling[0m utf8parse v0.2.0
[0m[0m[1m[32m   Compiling[0m sync_wrapper v0.1.1
[0m[0m[1m[32m   Compiling[0m xml-rs v0.8.4
[0m[0m[1m[32m   Compiling[0m zstd-safe v6.0.3+zstd.1.5.2
[0m[0m[1m[32m   Compiling[0m glob v0.3.1
[0m[0m[1m[32m   Compiling[0m urlencoding v2.1.2
[0m[0m[1m[32m   Compiling[0m aws-smithy-query v0.51.0
[0m[0m[1m[32m   Compiling[0m amq-protocol-types v7.0.1
[0m[0m[1m[32m   Compiling[0m grok v2.0.0
[0m[0m[1m[32m   Compiling[0m serde-xml-rs v0.6.0
[0m[0m[1m[32m   Compiling[0m derive_more v0.99.17
[0m[0m[1m[32m   Compiling[0m tungstenite v0.18.0
[0m[0m[1m[32m   Compiling[0m metrics-util v0.14.0
[0m[0m[1m[32m   Compiling[0m dirs v4.0.0
[0m[0m[1m[32m   Compiling[0m json-patch v0.2.6
[0m[0m[1m[32m   Compiling[0m graphql-parser v0.4.0
[0m[0m[1m[32m   Compiling[0m onig v6.4.0
[0m[0m[1m[32m   Compiling[0m pest_derive v2.5.6
[0m[0m[1m[32m   Compiling[0m terminal_size v0.2.2
[0m[0m[1m[32m   Compiling[0m pkcs8 v0.9.0
[0m[0m[1m[32m   Compiling[0m rustc_version v0.2.3
[0m[0m[1m[32m   Compiling[0m spki v0.4.1
[0m[0m[1m[32m   Compiling[0m pem-rfc7468 v0.2.3
[0m[0m[1m[32m   Compiling[0m vector-core v0.1.0 (/build/source/lib/vector-core)
[0m[0m[1m[32m   Compiling[0m datadog-grok v0.1.0 (https://github.com/vectordotdev/vrl?rev=v0.2.0#258cc611)
[0m[0m[1m[32m   Compiling[0m crc32c v0.6.3
[0m[0m[1m[32m   Compiling[0m num_enum_derive v0.5.11
[0m[0m[1m[32m   Compiling[0m tracing-futures v0.2.5
[0m[0m[1m[32m   Compiling[0m serde_path_to_error v0.1.8
[0m[0m[1m[32m   Compiling[0m graphql-introspection-query v0.2.0
[0m[0m[1m[32m   Compiling[0m block-buffer v0.9.0
[0m[0m[1m[32m   Compiling[0m enumflags2_derive v0.7.4
[0m[0m[1m[32m   Compiling[0m async-recursion v1.0.4
[0m[0m[1m[32m   Compiling[0m typetag-impl v0.2.7
[0m[0m[1m[32m   Compiling[0m lz4-sys v1.9.4
[0m[0m[1m[32m   Compiling[0m memmap2 v0.5.10
[0m[0m[1m[32m   Compiling[0m fslock v0.2.1
[0m[0m[1m[32m   Compiling[0m vte_generate_state_changes v0.1.1
[0m[0m[1m[32m   Compiling[0m csv-core v0.1.10
[0m[0m[1m[32m   Compiling[0m woothee v0.13.0
[0m[0m[1m[32m   Compiling[0m clap_lex v0.4.1
[0m[0m[1m[32m   Compiling[0m opaque-debug v0.3.0
[0m[0m[1m[32m   Compiling[0m matches v0.1.9
[0m[0m[1m[32m   Compiling[0m rust_decimal v1.29.1
[0m[0m[1m[32m   Compiling[0m fs_extra v1.2.0
[0m[0m[1m[32m   Compiling[0m proc-macro-hack v0.5.19
[0m[0m[1m[32m   Compiling[0m rle-decode-fast v1.0.3
[0m[0m[1m[32m   Compiling[0m snap v1.1.0
[0m[0m[1m[32m   Compiling[0m platforms v1.1.0
[0m[0m[1m[32m   Compiling[0m arrayvec v0.5.2
[0m[0m[1m[32m   Compiling[0m fallible-iterator v0.2.0
[0m[0m[1m[32m   Compiling[0m lockfree-object-pool v0.1.3
[0m[0m[1m[32m   Compiling[0m dlv-list v0.3.0
[0m[0m[1m[32m   Compiling[0m arrayvec v0.7.2
[0m[0m[1m[32m   Compiling[0m vte v0.10.1
[0m[0m[1m[32m   Compiling[0m metrics-tracing-context v0.12.0
[0m[0m[1m[32m   Compiling[0m postgres-protocol v0.6.4
[0m[0m[1m[32m   Compiling[0m ordered-multimap v0.4.3
[0m[0m[1m[32m   Compiling[0m tikv-jemalloc-sys v0.5.2+5.3.0-patched
[0m[0m[1m[32m   Compiling[0m libflate_lz77 v1.1.0
[0m[0m[1m[32m   Compiling[0m idna v0.2.3
[0m[0m[1m[32m   Compiling[0m sha2 v0.9.9
[0m[0m[1m[32m   Compiling[0m typetag v0.2.7
[0m[0m[1m[32m   Compiling[0m clap_builder v4.1.14
[0m[0m[1m[32m   Compiling[0m csv v1.2.1
[0m[0m[1m[32m   Compiling[0m enumflags2 v0.7.5
[0m[0m[1m[32m   Compiling[0m num_enum v0.5.11
[0m[0m[1m[32m   Compiling[0m graphql_client_codegen v0.12.0
[0m[0m[1m[32m   Compiling[0m oauth2 v4.3.0
[0m[0m[1m[32m   Compiling[0m tonic v0.8.3
[0m[0m[1m[32m   Compiling[0m pkcs8 v0.7.6
[0m[0m[1m[32m   Compiling[0m rustc_version_runtime v0.2.1
[0m[0m[1m[32m   Compiling[0m pkcs1 v0.4.1
[0m[0m[1m[32m   Compiling[0m datadog-search-syntax v0.1.0 (https://github.com/vectordotdev/vrl?rev=v0.2.0#258cc611)
[0m[0m[1m[32m   Compiling[0m tokio-tungstenite v0.18.0
[0m[0m[1m[32m   Compiling[0m amq-protocol-uri v7.0.1
[0m[0m[1m[32m   Compiling[0m ed25519 v1.5.3
[0m[0m[1m[32m   Compiling[0m curve25519-dalek v3.2.0
[0m[0m[1m[32m   Compiling[0m hyper-proxy v0.9.1
[0m[0m[1m[32m   Compiling[0m tcp-stream v0.24.4
[0m[0m[1m[32m   Compiling[0m quanta v0.11.0
[0m[0m[1m[32m   Compiling[0m kube-core v0.75.0
[0m[0m[1m[32m   Compiling[0m signal-hook-mio v0.2.3
[0m[0m[1m[32m   Compiling[0m simple_asn1 v0.6.2
[0m[0m[1m[32m   Compiling[0m twox-hash v1.6.3
[0m[0m[1m[32m   Compiling[0m syslog_loose v0.18.0
[0m[0m[1m[32m   Compiling[0m secrecy v0.8.0
[0m[0m[1m[32m   Compiling[0m sha-1 v0.10.1
[0m[0m[1m[32m   Compiling[0m signature v2.0.0
[0m[0m[1m[32m   Compiling[0m webpki v0.21.4
[0m[0m[1m[32m   Compiling[0m sct v0.6.1
[0m[0m[1m[32m   Compiling[0m jsonpath_lib v0.3.0
[0m[0m[1m[32m   Compiling[0m tracing-core v0.2.0 (https://github.com/tokio-rs/tracing?rev=e0642d949891546a3bb7e47080365ee7274f05cd#e0642d94)
[0m[0m[1m[32m   Compiling[0m dashmap v5.4.0
[0m[0m[1m[32m   Compiling[0m serde_bytes v0.11.9
[0m[0m[1m[32m   Compiling[0m clap_derive v4.1.14
[0m[0m[1m[32m   Compiling[0m enum-as-inner v0.4.0
[0m[0m[1m[32m   Compiling[0m typed-builder v0.10.0
[0m[0m[1m[32m   Compiling[0m zerocopy-derive v0.3.2
[0m[0m[1m[32m   Compiling[0m bitmask-enum v2.1.0
[0m[0m[1m[32m   Compiling[0m rdkafka-sys v4.3.0+1.9.2
[0m[0m[1m[32m   Compiling[0m error-chain v0.12.4
[0m[0m[1m[32m   Compiling[0m instant v0.1.12
[0m[0m[1m[32m   Compiling[0m proc-macro-nested v0.1.7
[0m[0m[1m[32m   Compiling[0m peeking_take_while v1.0.0
[0m[0m[1m[32m   Compiling[0m adler32 v1.2.0
[0m[0m[1m[32m   Compiling[0m unicode-segmentation v1.10.0
[0m[0m[1m[32m   Compiling[0m quick-error v1.2.3
[0m[0m[1m[32m   Compiling[0m crc-catalog v2.1.0
[0m[0m[1m[32m   Compiling[0m keccak v0.1.3
[0m[0m[1m[32m   Compiling[0m heim-host v0.1.0-rc.1 (https://github.com/vectordotdev/heim.git?branch=update-nix#76fa765c)
[0m[0m[1m[32m   Compiling[0m amq-protocol v7.0.1
[0m[0m[1m[32m   Compiling[0m float_eq v1.0.1
[0m[0m[1m[32m   Compiling[0m clap v4.1.14
[0m[0m[1m[32m   Compiling[0m sha3 v0.10.6
[0m[0m[1m[32m   Compiling[0m crc v3.0.1
[0m[0m[1m[32m   Compiling[0m resolv-conf v0.7.0
[0m[0m[1m[32m   Compiling[0m libflate v1.2.0
[0m[0m[1m[32m   Compiling[0m backoff v0.4.0
[0m[0m[1m[32m   Compiling[0m zerocopy v0.6.1
[0m[0m[1m[32m   Compiling[0m trust-dns-proto v0.21.2
[0m[0m[1m[32m   Compiling[0m tracing v0.2.0 (https://github.com/tokio-rs/tracing?rev=e0642d949891546a3bb7e47080365ee7274f05cd#e0642d94)
[0m[0m[1m[32m   Compiling[0m rustls v0.19.1
[0m[0m[1m[32m   Compiling[0m rsa v0.8.1
[0m[0m[1m[32m   Compiling[0m jsonwebtoken v8.2.0
[0m[0m[1m[32m   Compiling[0m amq-protocol-tcp v7.0.1
[0m[0m[1m[32m   Compiling[0m ed25519-dalek v1.0.1
[0m[0m[1m[32m   Compiling[0m kube-client v0.75.0
[0m[0m[1m[32m   Compiling[0m graphql_query_derive v0.12.0
[0m[0m[1m[32m   Compiling[0m datadog-filter v0.1.0 (https://github.com/vectordotdev/vrl?rev=v0.2.0#258cc611)
[0m[0m[1m[32m   Compiling[0m signatory v0.23.2
[0m[0m[1m[32m   Compiling[0m strip-ansi-escapes v0.1.1
[0m[0m[1m[32m   Compiling[0m postgres-types v0.2.4
[0m[0m[1m[32m   Compiling[0m rust-ini v0.18.0
[0m[0m[1m[32m   Compiling[0m uaparser v0.6.0
[0m[0m[1m[32m   Compiling[0m opentelemetry-proto v0.1.0 (/build/source/lib/opentelemetry-proto)
[0m[0m[1m[32m   Compiling[0m ureq v2.6.2
[0m[0m[1m[32m   Compiling[0m aws-smithy-types v0.55.0
[0m[0m[1m[32m   Compiling[0m aws-smithy-types v0.54.4
[0m[0m[1m[32m   Compiling[0m ctr v0.9.2
[0m[0m[1m[32m   Compiling[0m cfb-mode v0.8.2
[0m[0m[1m[32m   Compiling[0m ofb v0.6.1
[0m[0m[1m[32m   Compiling[0m cbc v0.1.2
[0m[0m[1m[32m   Compiling[0m aes v0.8.2
[0m[0m[1m[32m   Compiling[0m roxmltree v0.18.0
[0m[0m[1m[32m   Compiling[0m nix v0.26.2 (https://github.com/vectordotdev/nix.git?branch=memfd/gnu/musl#6c53a918)
[0m[0m[1m[32m   Compiling[0m async-global-executor v2.3.1
[0m[0m[1m[32m   Compiling[0m strum_macros v0.24.3
[0m[0m[1m[32m   Compiling[0m lru-cache v0.1.2
[0m[0m[1m[32m   Compiling[0m serde_with_macros v1.5.2
[0m[0m[1m[32m   Compiling[0m loki-logproto v0.1.0 (/build/source/lib/loki-logproto)
[0m[0m[1m[32m   Compiling[0m prometheus-parser v0.1.0 (/build/source/lib/prometheus-parser)
[0m[0m[1m[32m   Compiling[0m pulsar v5.1.0
[0m[0m[1m[32m   Compiling[0m flume v0.10.14
[0m[0m[1m[32m   Compiling[0m rmp v0.8.11
[0m[0m[1m[32m   Compiling[0m charset v0.1.3
[0m[0m[1m[32m   Compiling[0m backon v0.4.0
[0m[0m[1m[32m   Compiling[0m executor-trait v2.1.0
[0m[0m[1m[32m   Compiling[0m reactor-trait v1.1.0
[0m[0m[1m[32m   Compiling[0m quick-xml v0.28.1
[0m[0m[1m[32m   Compiling[0m serde_repr v0.1.9
[0m[0m[1m[32m   Compiling[0m fix-hidden-lifetime-bug-proc_macros v0.2.5
[0m[0m[1m[32m   Compiling[0m dns-lookup v1.0.8
[0m[0m[1m[32m   Compiling[0m inotify-sys v0.1.5
[0m[0m[1m[32m   Compiling[0m utf8-width v0.1.6
[0m[0m[1m[32m   Compiling[0m RustyXML v0.3.0
[0m[0m[1m[32m   Compiling[0m simpl v0.1.0
[0m[0m[1m[32m   Compiling[0m same-file v1.0.6
[0m[0m[1m[32m   Compiling[0m hyper-openssl v0.9.2
[0m[0m[1m[32m   Compiling[0m quoted_printable v0.4.7
[0m[0m[1m[32m   Compiling[0m macaddr v1.0.1
[0m[0m[1m[32m   Compiling[0m lapin v2.1.1
[0m[0m[1m[32m   Compiling[0m encode_unicode v1.0.0
[0m[0m[1m[32m   Compiling[0m scoped-tls v1.0.1
[0m[0m[1m[32m   Compiling[0m strum v0.24.1
[0m[0m[1m[32m   Compiling[0m base16 v0.2.1
[0m[0m[1m[32m   Compiling[0m prettytable-rs v0.10.0
[0m[0m[1m[32m   Compiling[0m apache-avro v0.14.0
[0m[0m[1m[32m   Compiling[0m warp v0.3.4
[0m[0m[1m[32m   Compiling[0m fix-hidden-lifetime-bug v0.2.5
[0m[0m[1m[32m   Compiling[0m reqsign v0.8.5
[0m[0m[1m[32m   Compiling[0m heim-net v0.1.0-rc.1 (https://github.com/vectordotdev/heim.git?branch=update-nix#76fa765c)
[0m[0m[1m[32m   Compiling[0m walkdir v2.3.2
[0m[0m[1m[32m   Compiling[0m smpl_jwt v0.7.1
[0m[0m[1m[32m   Compiling[0m azure_storage v0.6.0 (https://github.com/Azure/azure-sdk-for-rust.git?rev=b4544d4920fa3064eb921340054cd9cc130b7664#b4544d49)
[0m[0m[1m[32m   Compiling[0m inotify v0.9.6
[0m[0m[1m[32m   Compiling[0m async-reactor-trait v1.1.0
[0m[0m[1m[32m   Compiling[0m async-global-executor-trait v2.1.0
[0m[0m[1m[32m   Compiling[0m pinky-swear v6.1.0
[0m[0m[1m[32m   Compiling[0m serde_with v1.14.0
[0m[0m[1m[32m   Compiling[0m trust-dns-resolver v0.21.2
[0m[0m[1m[32m   Compiling[0m rustyline v11.0.0
[0m[0m[1m[32m   Compiling[0m aws-smithy-http v0.54.4
[0m[0m[1m[32m   Compiling[0m aws-smithy-http v0.55.0
[0m[0m[1m[32m   Compiling[0m arr_macro_impl v0.2.1
[0m[0m[1m[32m   Compiling[0m tokio-postgres v0.7.7
[0m[0m[1m[32m   Compiling[0m kube-runtime v0.75.0
[0m[0m[1m[32m   Compiling[0m aws-smithy-checksums v0.51.0
[0m[0m[1m[32m   Compiling[0m nkeys v0.2.0
[0m[0m[1m[32m   Compiling[0m graphql_client v0.12.0
[0m[0m[1m[32m   Compiling[0m rustls-native-certs v0.5.0
[0m[0m[1m[32m   Compiling[0m tracing-futures v0.3.0 (https://github.com/tokio-rs/tracing?rev=e0642d949891546a3bb7e47080365ee7274f05cd#e0642d94)
[0m[0m[1m[32m   Compiling[0m heim-cpu v0.1.0-rc.1 (https://github.com/vectordotdev/heim.git?branch=update-nix#76fa765c)
[0m[0m[1m[32m   Compiling[0m bson v2.5.0
[0m[0m[1m[32m   Compiling[0m crossterm v0.25.0
[0m[0m[1m[32m   Compiling[0m openidconnect v2.4.0
[0m[0m[1m[32m   Compiling[0m webbrowser v0.8.8
[0m[0m[1m[32m   Compiling[0m aws-sdk-sts v0.21.0
[0m[0m[1m[32m   Compiling[0m vector v0.29.0 (/build/source)
[0m[0m[1m[32m   Compiling[0m aws-sdk-sso v0.21.0
[0m[0m[1m[32m   Compiling[0m heim-memory v0.1.0-rc.1 (https://github.com/vectordotdev/heim.git?branch=update-nix#76fa765c)
[0m[0m[1m[32m   Compiling[0m heim-disk v0.1.0-rc.1 (https://github.com/vectordotdev/heim.git?branch=update-nix#76fa765c)
[0m[0m[1m[32m   Compiling[0m fakedata_generator v0.2.4
[0m[0m[1m[32m   Compiling[0m bollard-stubs v1.42.0-rc.7
[0m[0m[1m[32m   Compiling[0m linked_hash_set v0.1.4
[0m[0m[1m[32m   Compiling[0m hyperlocal v0.8.0
[0m[0m[1m[32m   Compiling[0m crossbeam-channel v0.5.6
[0m[0m[1m[32m   Compiling[0m pbkdf2 v0.11.0
[0m[0m[1m[32m   Compiling[0m combine v4.6.6
[0m[0m[1m[32m   Compiling[0m async-compat v0.2.1
[0m[0m[1m[32m   Compiling[0m nuid v0.3.2
[0m[0m[1m[32m   Compiling[0m ipnetwork v0.18.0
[0m[0m[1m[32m   Compiling[0m serde_nanos v0.1.2
[0m[0m[1m[32m   Compiling[0m quick-xml v0.27.1
[0m[0m[1m[32m   Compiling[0m scan_fmt v0.2.6
[0m[0m[1m[32m   Compiling[0m base64-url v1.4.13
[0m[0m[1m[32m   Compiling[0m rustls-pemfile v0.2.1
[0m[0m[1m[32m   Compiling[0m filetime v0.2.18
[0m[0m[1m[32m   Compiling[0m bstr v1.4.0
[0m[0m[1m[32m   Compiling[0m md5 v0.7.0
[0m[0m[1m[32m   Compiling[0m take_mut v0.2.2
[0m[0m[1m[32m   Compiling[0m no-std-compat v0.4.1
[0m[0m[1m[32m   Compiling[0m unsafe-libyaml v0.2.7
[0m[0m[1m[32m   Compiling[0m cassowary v0.3.0
[0m[0m[1m[32m   Compiling[0m nonzero_ext v0.3.0
[0m[0m[1m[32m   Compiling[0m bytemuck v1.12.3
[0m[0m[1m[32m   Compiling[0m exitcode v1.1.2
[0m[0m[1m[32m   Compiling[0m data-url v0.2.0
[0m[0m[1m[32m   Compiling[0m bit-vec v0.4.4
[0m[0m[1m[32m   Compiling[0m retain_mut v0.1.7
[0m[0m[1m[32m   Compiling[0m flagset v0.4.3
[0m[0m[1m[32m   Compiling[0m json v0.12.4
[0m[0m[1m[32m   Compiling[0m nats v0.24.0
[0m[0m[1m[32m   Compiling[0m opendal v0.30.5
[0m[0m[1m[32m   Compiling[0m redis v0.23.0
[0m[0m[1m[32m   Compiling[0m roaring v0.10.1
[0m[0m[1m[32m   Compiling[0m bloom v0.3.2
[0m[0m[1m[32m   Compiling[0m serde_yaml v0.9.19
[0m[0m[1m[32m   Compiling[0m bollard v0.14.0
[0m[0m[1m[32m   Compiling[0m tui v0.19.0
[0m[0m[1m[32m   Compiling[0m governor v0.5.1
[0m[0m[1m[32m   Compiling[0m mongodb v2.4.0
[0m[0m[1m[32m   Compiling[0m azure_storage_blobs v0.6.0 (https://github.com/Azure/azure-sdk-for-rust.git?rev=b4544d4920fa3064eb921340054cd9cc130b7664#b4544d49)
[0m[0m[1m[32m   Compiling[0m colored v2.0.0
[0m[0m[1m[32m   Compiling[0m notify v5.1.0
[0m[0m[1m[32m   Compiling[0m maxminddb v0.23.0
[0m[0m[1m[32m   Compiling[0m fakedata v0.1.0 (/build/source/lib/fakedata)
[0m[0m[1m[32m   Compiling[0m heim v0.1.0-rc.1 (https://github.com/vectordotdev/heim.git?branch=update-nix#76fa765c)
[0m[0m[1m[32m   Compiling[0m aws-config v0.51.0
[0m[0m[1m[32m   Compiling[0m arr_macro v0.2.1
[0m[0m[1m[32m   Compiling[0m syslog v6.0.1
[0m[0m[1m[32m   Compiling[0m postgres-openssl v0.5.0
[0m[0m[1m[32m   Compiling[0m tracing-tower v0.1.0 (https://github.com/tokio-rs/tracing?rev=e0642d949891546a3bb7e47080365ee7274f05cd#e0642d94)
[0m[0m[1m[32m   Compiling[0m kube v0.75.0
[0m[0m[1m[32m   Compiling[0m vector-api-client v0.1.2 (/build/source/lib/vector-api-client)
[0m[0m[1m[32m   Compiling[0m aws-sdk-s3 v0.21.0
[0m[0m[1m[32m   Compiling[0m aws-sigv4 v0.55.0
[0m[0m[1m[32m   Compiling[0m aws-smithy-http-tower v0.54.4
[0m[0m[1m[32m   Compiling[0m tikv-jemallocator v0.5.0
[0m[0m[1m[32m   Compiling[0m async-graphql-warp v5.0.7
[0m[0m[1m[32m   Compiling[0m goauth v0.13.1
[0m[0m[1m[32m   Compiling[0m azure_identity v0.6.0 (https://github.com/Azure/azure-sdk-for-rust.git?rev=b4544d4920fa3064eb921340054cd9cc130b7664#b4544d49)
[0m[0m[1m[32m   Compiling[0m rmpv v1.0.0
[0m[0m[1m[32m   Compiling[0m rmp-serde v1.1.1
[0m[0m[1m[32m   Compiling[0m tracing-limit v0.1.0 (/build/source/lib/tracing-limit)
[0m[0m[1m[32m   Compiling[0m crossterm v0.26.1
[0m[0m[1m[32m   Compiling[0m hashbrown v0.13.2
[0m[0m[1m[32m   Compiling[0m num-format v0.4.4
[0m[0m[1m[32m   Compiling[0m aws-sdk-cloudwatch v0.21.0
[0m[0m[1m[32m   Compiling[0m aws-sdk-sqs v0.21.0
[0m[0m[1m[32m   Compiling[0m aws-sdk-cloudwatchlogs v0.21.0
[0m[0m[1m[32m   Compiling[0m aws-sdk-firehose v0.21.0
[0m[0m[1m[32m   Compiling[0m aws-sdk-kinesis v0.21.0
[0m[0m[1m[32m   Compiling[0m serde-toml-merge v0.3.1
[0m[0m[1m[32m   Compiling[0m enum_dispatch v0.3.11
[0m[0m[1m[32m   Compiling[0m portpicker v1.0.0 (/build/source/lib/portpicker)
[0m[0m[1m[32m   Compiling[0m rand_distr v0.4.3
[0m[0m[1m[32m   Compiling[0m listenfd v1.0.1
[0m[0m[1m[32m   Compiling[0m bytesize v1.2.0
[0m[0m[1m[32m   Compiling[0m infer v0.13.0
[0m[0m[1m[32m   Compiling[0m number_prefix v0.4.0
[0m[0m[1m[32m   Compiling[0m lru v0.10.0
[0m[0m[1m[32m   Compiling[0m hash_hasher v2.0.3
[0m[0m[1m[32m   Compiling[0m lz4 v1.24.0
[0m[0m[1m[32m   Compiling[0m value v0.1.0 (https://github.com/vectordotdev/vrl?rev=v0.2.0#258cc611)
[0m[0m[1m[32m   Compiling[0m vrl-core v0.1.0 (https://github.com/vectordotdev/vrl?rev=v0.2.0#258cc611)
[0m[0m[1m[32m   Compiling[0m zstd v0.12.3+zstd.1.5.2
[0m[0m[1m[32m   Compiling[0m zstd v0.11.2+zstd.1.5.2
[0m[0m[1m[32m   Compiling[0m rdkafka v0.29.0
[0m[0m[1m[32m   Compiling[0m async-compression v0.3.15
[0m[0m[1m[32m   Compiling[0m vrl-compiler v0.1.0 (https://github.com/vectordotdev/vrl?rev=v0.2.0#258cc611)
[0m[0m[1m[32m   Compiling[0m tower-http v0.4.0
[0m[0m[1m[32m   Compiling[0m vector-config v0.1.0 (/build/source/lib/vector-config)
[0m[0m[1m[32m   Compiling[0m vrl v0.1.0 (https://github.com/vectordotdev/vrl?rev=v0.2.0#258cc611)
[0m[0m[1m[32m   Compiling[0m vector-common v0.1.0 (/build/source/lib/vector-common)
[0m[0m[1m[32m   Compiling[0m vector-lookup v0.1.0 (/build/source/lib/vector-lookup)
[0m[0m[1m[32m   Compiling[0m enrichment v0.1.0 (/build/source/lib/enrichment)
[0m[0m[1m[32m   Compiling[0m vector-buffers v0.1.0 (/build/source/lib/vector-buffers)
[0m[0m[1m[32m   Compiling[0m vrl-stdlib v0.1.0 (https://github.com/vectordotdev/vrl?rev=v0.2.0#258cc611)
[0m[0m[1m[32m   Compiling[0m vrl-cli v0.1.0 (https://github.com/vectordotdev/vrl?rev=v0.2.0#258cc611)
[0m[0m[1m[32m   Compiling[0m codecs v0.1.0 (/build/source/lib/codecs)
[0m[1m[38;5;9merror: internal compiler error[0m[0m[1m: no errors encountered even though `delay_span_bug` issued[0m

[0m[1m[38;5;9merror: internal compiler error[0m[0m[1m: broken MIR in Item(WithOptConstParam { did: DefId(0:2569 ~ vector_buffers[7038]::variants::disk_v2::record::_::{impl#0}::resolve), const_param_did: None }) (after phase change to runtime-optimized) at bb4[53]:[0m
[0m[1m                                Field projection `(*_4).field[3]` specified type `rkyv::boxed::ArchivedBox<[u8]>`, but actual type is `<rkyv::with::With<rkyv::with::With<&[u8], rkyv::with::RefAsBox>, rkyv::with::CopyOptimize> as rkyv::Archive>::Archived`[0m
[0m  [0m[0m[1m[38;5;12m--> [0m[0mlib/vector-buffers/src/variants/disk_v2/record.rs:46:10[0m
[0m   [0m[0m[1m[38;5;12m|[0m
[0m[1m[38;5;12m46[0m[0m [0m[0m[1m[38;5;12m|[0m[0m [0m[0m#[derive(Archive, Serialize, Debug)][0m
[0m   [0m[0m[1m[38;5;12m| [0m[0m         [0m[0m[1m[38;5;9m^^^^^^^[0m
[0m   [0m[0m[1m[38;5;12m|[0m
[0m   [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic[0m
[0m              1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, &alloc::string::String>[0m
[0m              2: <rustc_const_eval::transform::validate::TypeChecker>::fail::<alloc::string::String>[0m
[0m              3: <rustc_const_eval::transform::validate::TypeChecker as rustc_middle::mir::visit::Visitor>::visit_projection_elem[0m
[0m              4: <rustc_const_eval::transform::validate::TypeChecker as rustc_middle::mir::visit::Visitor>::visit_place[0m
[0m              5: <rustc_const_eval::transform::validate::TypeChecker as rustc_middle::mir::visit::Visitor>::visit_rvalue[0m
[0m              6: <rustc_const_eval::transform::validate::TypeChecker as rustc_middle::mir::visit::Visitor>::visit_statement[0m
[0m              7: <rustc_const_eval::transform::validate::Validator as rustc_middle::mir::MirPass>::run_pass[0m
[0m              8: rustc_mir_transform::pass_manager::run_passes_inner[0m
[0m              9: rustc_mir_transform::optimized_mir[0m
[0m             10: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::optimized_mir, rustc_query_impl::plumbing::QueryCtxt>[0m
[0m             11: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::optimized_mir[0m
[0m             12: <rustc_metadata::rmeta::encoder::EncodeContext>::encode_crate_root[0m
[0m             13: rustc_metadata::rmeta::encoder::encode_metadata_impl[0m
[0m             14: rustc_data_structures::sync::join::<rustc_metadata::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata::rmeta::encoder::encode_metadata::{closure#1}, (), ()>[0m
[0m             15: rustc_metadata::rmeta::encoder::encode_metadata[0m
[0m             16: rustc_metadata::fs::encode_and_write_metadata[0m
[0m             17: rustc_interface::passes::start_codegen[0m
[0m             18: <rustc_interface::queries::Queries>::ongoing_codegen[0m
[0m             19: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>[0m
[0m             20: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>[0m
[0m             21: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}[0m
[0m             22: std::sys::unix::thread::Thread::new::thread_start[0m
[0m             23: start_thread[0m
[0m             24: __clone3[0m
[0m           [0m
[0m   [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: this error: internal compiler error originates in the macro `::core::ptr::addr_of_mut` which comes from the expansion of the derive macro `Archive` (in Nightly builds, run with -Z macro-backtrace for more info)[0m

[0m[1m[38;5;9merror: internal compiler error[0m[0m[1m: broken MIR in Item(WithOptConstParam { did: DefId(0:2569 ~ vector_buffers[7038]::variants::disk_v2::record::_::{impl#0}::resolve), const_param_did: None }) (after phase change to runtime-optimized) at bb7[12]:[0m
[0m[1m                                Field projection `_3.field[3]` specified type `rkyv::boxed::BoxResolver<()>`, but actual type is `<rkyv::with::With<rkyv::with::With<&[u8], rkyv::with::RefAsBox>, rkyv::with::CopyOptimize> as rkyv::Archive>::Resolver`[0m
[0m  [0m[0m[1m[38;5;12m--> [0m[0mlib/vector-buffers/src/variants/disk_v2/record.rs:46:10[0m
[0m   [0m[0m[1m[38;5;12m|[0m
[0m[1m[38;5;12m46[0m[0m [0m[0m[1m[38;5;12m|[0m[0m [0m[0m#[derive(Archive, Serialize, Debug)][0m
[0m   [0m[0m[1m[38;5;12m| [0m[0m         [0m[0m[1m[38;5;9m^^^^^^^[0m
[0m   [0m[0m[1m[38;5;12m|[0m
[0m   [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic[0m
[0m              1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, &alloc::string::String>[0m
[0m              2: <rustc_const_eval::transform::validate::TypeChecker>::fail::<alloc::string::String>[0m
[0m              3: <rustc_const_eval::transform::validate::TypeChecker as rustc_middle::mir::visit::Visitor>::visit_projection_elem[0m
[0m              4: <rustc_const_eval::transform::validate::TypeChecker as rustc_middle::mir::visit::Visitor>::visit_place[0m
[0m              5: <rustc_const_eval::transform::validate::TypeChecker as rustc_middle::mir::visit::Visitor>::visit_rvalue[0m
[0m              6: <rustc_const_eval::transform::validate::TypeChecker as rustc_middle::mir::visit::Visitor>::visit_statement[0m
[0m              7: <rustc_const_eval::transform::validate::Validator as rustc_middle::mir::MirPass>::run_pass[0m
[0m              8: rustc_mir_transform::pass_manager::run_passes_inner[0m
[0m              9: rustc_mir_transform::optimized_mir[0m
[0m             10: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::optimized_mir, rustc_query_impl::plumbing::QueryCtxt>[0m
[0m             11: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::optimized_mir[0m
[0m             12: <rustc_metadata::rmeta::encoder::EncodeContext>::encode_crate_root[0m
[0m             13: rustc_metadata::rmeta::encoder::encode_metadata_impl[0m
[0m             14: rustc_data_structures::sync::join::<rustc_metadata::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata::rmeta::encoder::encode_metadata::{closure#1}, (), ()>[0m
[0m             15: rustc_metadata::rmeta::encoder::encode_metadata[0m
[0m             16: rustc_metadata::fs::encode_and_write_metadata[0m
[0m             17: rustc_interface::passes::start_codegen[0m
[0m             18: <rustc_interface::queries::Queries>::ongoing_codegen[0m
[0m             19: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>[0m
[0m             20: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>[0m
[0m             21: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}[0m
[0m             22: std::sys::unix::thread::Thread::new::thread_start[0m
[0m             23: start_thread[0m
[0m             24: __clone3[0m
[0m           [0m
[0m   [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: this error: internal compiler error originates in the derive macro `Archive` (in Nightly builds, run with -Z macro-backtrace for more info)[0m

[0m[1m[38;5;9merror: internal compiler error[0m[0m[1m: broken MIR in Item(WithOptConstParam { did: DefId(0:2578 ~ vector_buffers[7038]::variants::disk_v2::record::_#1::{impl#0}::serialize), const_param_did: None }) (after phase change to runtime-optimized) at bb18[5]:[0m
[0m[1m                                Field projection `_3.field[3]` specified type `rkyv::boxed::BoxResolver<()>`, but actual type is `<rkyv::with::With<rkyv::with::With<&[u8], rkyv::with::RefAsBox>, rkyv::with::CopyOptimize> as rkyv::Archive>::Resolver`[0m
[0m  [0m[0m[1m[38;5;12m--> [0m[0mlib/vector-buffers/src/variants/disk_v2/record.rs:46:19[0m
[0m   [0m[0m[1m[38;5;12m|[0m
[0m[1m[38;5;12m46[0m[0m [0m[0m[1m[38;5;12m|[0m[0m [0m[0m#[derive(Archive, Serialize, Debug)][0m
[0m   [0m[0m[1m[38;5;12m| [0m[0m                  [0m[0m[1m[38;5;9m^^^^^^^^^[0m
[0m   [0m[0m[1m[38;5;12m|[0m
[0m   [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: delayed at    0: <rustc_errors::HandlerInner>::emit_diagnostic[0m
[0m              1: <rustc_errors::Handler>::delay_span_bug::<rustc_span::span_encoding::Span, &alloc::string::String>[0m
[0m              2: <rustc_const_eval::transform::validate::TypeChecker>::fail::<alloc::string::String>[0m
[0m              3: <rustc_const_eval::transform::validate::TypeChecker as rustc_middle::mir::visit::Visitor>::visit_projection_elem[0m
[0m              4: <rustc_const_eval::transform::validate::TypeChecker as rustc_middle::mir::visit::Visitor>::visit_place[0m
[0m              5: <rustc_const_eval::transform::validate::TypeChecker as rustc_middle::mir::visit::Visitor>::visit_statement[0m
[0m              6: <rustc_const_eval::transform::validate::Validator as rustc_middle::mir::MirPass>::run_pass[0m
[0m              7: rustc_mir_transform::pass_manager::run_passes_inner[0m
[0m              8: rustc_mir_transform::optimized_mir[0m
[0m              9: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::queries::optimized_mir, rustc_query_impl::plumbing::QueryCtxt>[0m
[0m             10: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::optimized_mir[0m
[0m             11: <rustc_metadata::rmeta::encoder::EncodeContext>::encode_crate_root[0m
[0m             12: rustc_metadata::rmeta::encoder::encode_metadata_impl[0m
[0m             13: rustc_data_structures::sync::join::<rustc_metadata::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata::rmeta::encoder::encode_metadata::{closure#1}, (), ()>[0m
[0m             14: rustc_metadata::rmeta::encoder::encode_metadata[0m
[0m             15: rustc_metadata::fs::encode_and_write_metadata[0m
[0m             16: rustc_interface::passes::start_codegen[0m
[0m             17: <rustc_interface::queries::Queries>::ongoing_codegen[0m
[0m             18: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>[0m
[0m             19: std::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>[0m
[0m             20: <<std::thread::Builder>::spawn_unchecked_<rustc_interface::util::run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>::{closure#1} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}[0m
[0m             21: std::sys::unix::thread::Thread::new::thread_start[0m
[0m             22: start_thread[0m
[0m             23: __clone3[0m
[0m           [0m
[0m   [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: this error: internal compiler error originates in the derive macro `Serialize` (in Nightly builds, run with -Z macro-backtrace for more info)[0m

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.68.2 (9eb3afe9e 2023-03-27) (built from a source tarball) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C linker=/nix/store/nlgyw2fv0cm8rkz8qm1jyw78vyif1bl9-gcc-wrapper-12.2.0/bin/cc -C link-args=-rdynamic -C target-feature=-crt-static

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
[0m[0m[1m[31merror[0m[1m:[0m could not compile `vector-buffers`
[0m[0m[1m[33mwarning[0m[1m:[0m build failed, waiting for other jobs to finish...

