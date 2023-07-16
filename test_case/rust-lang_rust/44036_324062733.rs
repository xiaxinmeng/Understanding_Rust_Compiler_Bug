 toml
[target.thumbv7m-none-eabi]
rustflags = ["--cfg", "target_device=stm32f100"]

[build]
target = "thumbv7m-none-eabi"
