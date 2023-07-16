toml
[target.aarch64-apple-ios-macabi]
linker = "/usr/bin/clang"
rustflags = ["-C", "link-arg=--ld-path=/opt/homebrew/bin/mold"]
