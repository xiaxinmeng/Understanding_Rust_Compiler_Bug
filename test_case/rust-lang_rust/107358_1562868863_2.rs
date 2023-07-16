bash
$ rustc --version
rustc 1.69.0 (84c898d65 2023-04-16)

$ cargo --version
cargo 1.69.0 (6e9a83356 2023-04-12)

$ uname -a
Linux archlinux 6.2.12-arch1-1 #1 SMP PREEMPT_DYNAMIC Thu, 20 Apr 2023 16:11:55 +0000 x86_64 GNU/Linux

$ cat im-debug/Cargo.toml | grep -E '(tracing|jaeger|tokio)'
opentelemetry = { version = "0.18.0", features = ["rt-tokio"] }
opentelemetry-jaeger = { version = "0.17.0", features = ["rt-tokio"] }
tracing = {version="0.1.36", features = ["max_level_debug", "release_max_level_debug"] }
tracing-opentelemetry = "0.18.0"
tracing-subscriber = "0.3.15"

$ cat Cargo.toml | grep -E '(tracing|jaeger|tokio)'
tokio = { version = "1.21.2", features = ["full"] }
ntex = { version = "0.6.4", features=["tokio", "rustls"] }
