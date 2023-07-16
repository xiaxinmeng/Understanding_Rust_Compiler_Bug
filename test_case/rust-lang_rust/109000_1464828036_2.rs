toml
rustflags = ["-Z emit-stack-sizes"]

[target.avr-atmega328p]
runner = "qemu-system-avr -M uno -nographic -serial tcp::5678,server=on -bios"

[build]
target = "avr-atmega328p.json"

[unstable]
build-std = ["core"]
