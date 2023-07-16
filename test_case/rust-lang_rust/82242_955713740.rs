toml
# Cargo.toml

[profile.release]
panic = "abort"
codegen-units = 1
opt-level = "z"
lto = false # or "thin"
