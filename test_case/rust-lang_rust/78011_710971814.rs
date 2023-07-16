
cargo-features = ["named-profiles"]

[profile.coverage]
panic = "abort"
opt-level = 0
overflow-checks = false
incremental = false
codegen-units = 1
inherits = "test"
