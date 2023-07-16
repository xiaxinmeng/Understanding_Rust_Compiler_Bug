
[build]
[target.x86_64-fortanix-unknown-sgx]
rustflags = ["-Ctarget-feature=-lvi-cfi,-lvi-load-hardening"]
