
[INFO] running `Command { std: "docker" "create" "-v" "/var/lib/crater-agent-workspace/builds/worker-11/target:/opt/rustwide/target:rw,Z" "-v" "/var/lib/crater-agent-workspace/builds/worker-11/source:/opt/rustwide/workdir:ro,Z" "-v" "/var/lib/crater-agent-workspace/cargo-home:/opt/rustwide/cargo-home:ro,Z" "-v" "/var/lib/crater-agent-workspace/rustup-home:/opt/rustwide/rustup-home:ro,Z" "-e" "SOURCE_DIR=/opt/rustwide/workdir" "-e" "CARGO_TARGET_DIR=/opt/rustwide/target" "-e" "CARGO_INCREMENTAL=0" "-e" "RUST_BACKTRACE=full" "-e" "RUSTFLAGS=--cap-lints=warn" "-e" "CARGO_HOME=/opt/rustwide/cargo-home" "-e" "RUSTUP_HOME=/opt/rustwide/rustup-home" "-w" "/opt/rustwide/workdir" "-m" "1610612736" "--user" "0:0" "--network" "none" "rustops/crates-build-env@sha256:c8ac004eab7d63a0ad09a2dde3d3353ba464f767bee4de425dc8f74c46a1905e" "/opt/rustwide/cargo-home/bin/cargo" "+1.48.0" "build" "--frozen" "--message-format=json", kill_on_drop: false }`
[INFO] [stdout] 33264a8499ee77cf4f9ca16eb4d17a79ee1f74e74a2fd20bd988e465e3f0fa63
[INFO] [stderr] WARNING: Your kernel does not support swap limit capabilities or the cgroup is not mounted. Memory limited without swap.
[INFO] running `Command { std: "docker" "start" "-a" "33264a8499ee77cf4f9ca16eb4d17a79ee1f74e74a2fd20bd988e465e3f0fa63", kill_on_drop: false }`
[INFO] [stderr]    Compiling syn v1.0.53
[INFO] [stderr]    Compiling serde_derive v1.0.117
[INFO] [stderr]    Compiling juniper_codegen v0.14.2
[INFO] [stderr]    Compiling diesel_derives v1.4.1
[INFO] [stderr]    Compiling thiserror-impl v1.0.22
[INFO] [stderr]    Compiling wundergraph_derive v0.1.0
[INFO] [stderr]    Compiling thiserror v1.0.22
[INFO] [stderr]    Compiling serde v1.0.117
[INFO] [stderr]    Compiling diesel v1.4.5
[INFO] [stderr]    Compiling indexmap v1.6.0
[INFO] [stderr]    Compiling juniper v0.14.2
[INFO] [stderr]    Compiling wundergraph v0.1.2 (/opt/rustwide/workdir)
[INFO] [stderr]     Finished dev [unoptimized + debuginfo] target(s) in 1m 03s
