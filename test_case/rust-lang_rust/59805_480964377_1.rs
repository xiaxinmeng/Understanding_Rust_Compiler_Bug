
#.cargo/config
[target.arm-unknown-linux-gnueabi]
linker = "arm-linux-gnueabi-gcc"
rustflags = [
  "-C", "target-cpu=cortex-a9",
  "-C", "target-feature=+neon",
]
