toml
[target.thumbv7m-none-eabi]
rustflags = [
  "-C", "relocation-model=rwpi"
]

[build]
target = "thumbv7m-none-eabi"
