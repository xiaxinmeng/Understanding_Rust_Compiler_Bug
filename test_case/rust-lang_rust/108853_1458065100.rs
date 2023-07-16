
cargo-features = ["profile-rustflags"]

[profile.release.package.compiler_builtins]
rustflags = ["-Zshare-generics=off"]
