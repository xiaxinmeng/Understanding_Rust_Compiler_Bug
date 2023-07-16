
[target.aarch64-unknown-linux-musl]
linker = "/home/fedora/aarch64--musl--bleeding-edge-2018.02-1/bin/aarch64-buildroot-linux-musl-gcc"
rustflags = [
  "-C", "link-arg=-lgcc",
  "-C", "target-feature=+crt-static",
]
