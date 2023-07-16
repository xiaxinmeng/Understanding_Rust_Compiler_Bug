toml
# Cargo.toml:
[dependencies]
log = { version = "0.4.5", default-features = false }

# .cargo/config.toml:
[unstable]
build-std = ["core", "compiler_builtins", "alloc"]
build-std-features = ["compiler-builtins-mem"]
