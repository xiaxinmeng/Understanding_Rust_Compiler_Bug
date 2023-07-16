toml
[target.aarch64-unknown-none]
rustflags = [
  "-C", "link-arg=-Tlink.ld",
  "-C", "target-feature=+a53,-fp-armv8",
]
