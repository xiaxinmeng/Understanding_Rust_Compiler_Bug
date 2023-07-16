toml
# The publish profile, used for `cargo build --publish`.
[profile.publish]
# (...) everything else the same as profile.release except:
lto = true        # Enable full link-time optimization.
codegen-units = 1 # Use only 1 codegen-unit to enable full optimizations.
