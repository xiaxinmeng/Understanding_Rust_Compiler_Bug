toml
# The development profile, used for `cargo build`.
[profile.dev]
opt-level = 0      # controls the `--opt-level` the compiler builds with.
                   # 0-1 is good for debugging. 2 is well-optimized. Max is 3.
                   # 's' attempts to reduce size, 'z' reduces size even more.
debug-assertions = true # controls whether debug assertions are enabled
                   # (e.g. debug_assert!() and arithmetic overflow checks)
