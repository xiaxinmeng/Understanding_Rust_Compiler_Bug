bash
ana in gizmo in flooper on  master [?] is 📦 v0.1.0 via 🦀 v1.52.1
❯ cargo run --release
   Compiling flooper v0.1.0 (/home/ana/git/tremor-rs/flooper)
    Finished release [optimized] target(s) in 0.53s
     Running `target/release/flooper`

ana in gizmo in flooper on  master [?] is 📦 v0.1.0 via 🦀 v1.52.1
❯ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/flooper`

ana in gizmo in flooper on  master [?] is 📦 v0.1.0 via 🦀 v1.52.1
❯ ./target/debug/
build/        deps/         examples/     .fingerprint/ flooper       incremental/

ana in gizmo in flooper on  master [?] is 📦 v0.1.0 via 🦀 v1.52.1
❯ ./target/debug/flooper

ana in gizmo in flooper on  master [?] is 📦 v0.1.0 via 🦀 v1.52.1
❯ ./target/release/flooper

ana in gizmo in flooper on  master [?] is 📦 v0.1.0 via 🦀 v1.52.1
❯ rustc --version --verbose
rustc 1.52.1
binary: rustc
commit-hash: unknown
commit-date: unknown
host: aarch64-unknown-linux-gnu
release: 1.52.1
LLVM version: 11.1.0

ana in gizmo in flooper on  master [?] is 📦 v0.1.0 via 🦀 v1.52.1
❯ lscpu
Architecture:                    aarch64
CPU op-mode(s):                  32-bit, 64-bit
Byte Order:                      Little Endian
CPU(s):                          16
On-line CPU(s) list:             0-15
Thread(s) per core:              1
Core(s) per socket:              16
Socket(s):                       1
NUMA node(s):                    1
Vendor ID:                       ARM
Model:                           3
Model name:                      Cortex-A72
Stepping:                        r0p3
BogoMIPS:                        50.00
L1d cache:                       512 KiB
L1i cache:                       768 KiB
L2 cache:                        8 MiB
L3 cache:                        8 MiB
NUMA node0 CPU(s):               0-15
Vulnerability Itlb multihit:     Not affected
Vulnerability L1tf:              Not affected
Vulnerability Mds:               Not affected
Vulnerability Meltdown:          Not affected
Vulnerability Spec store bypass: Not affected
Vulnerability Spectre v1:        Mitigation; __user pointer sanitization
Vulnerability Spectre v2:        Mitigation; Branch predictor hardening
Vulnerability Srbds:             Not affected
Vulnerability Tsx async abort:   Not affected
Flags:                           fp asimd evtstrm aes pmull sha1 sha2 crc32 cpuid
