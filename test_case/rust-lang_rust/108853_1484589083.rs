toml
[unstable]
profile-rustflags = true

[profile.release.package.compiler_builtins]
rustflags = ["-Zshare-generics=off"]
