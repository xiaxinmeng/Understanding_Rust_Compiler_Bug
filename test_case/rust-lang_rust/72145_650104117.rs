
[build]
rustflags = [
	"-Clink-arg=-fuse-ld=lld",
    "-Ctarget-feature=+avx"
]
