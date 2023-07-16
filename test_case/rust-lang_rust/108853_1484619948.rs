toml
[profile.examples-fix]
inherits = "examples"

[profile.examples-fix.package.compiler_builtins]
rustflags = ["-Zshare-generics=off"]

[unstable]
profile-rustflags = true
