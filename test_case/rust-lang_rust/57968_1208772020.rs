toml
[profile.dev-opt]
inherits = "dev" # or "release", ideally everything is overridden below
opt-level = 3
debug = 1
incremental = true # <- only part relevant to this issue
