toml
[build]
target = "thumbv6m-none-eabi"

[target.thumbv6m-none-eabi]
runner = ["qemu-system-arm", "-cpu", "cortex-m3", "-machine", "lm3s6965evb", "-nographic", "-semihosting-config", "enable=on,target=native", "-kernel"]
rustflags = ["-C", "link-arg=-Tlink.x"]
