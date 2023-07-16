toml
# Cargo.toml:
[dependencies]
uefi = "0.16.1"
uefi-services = "0.13.1"

# .cargo/config.toml:
[unstable]
build-std = ["core", "compiler_builtins", "alloc"]
build-std-features = ["compiler-builtins-mem"]
