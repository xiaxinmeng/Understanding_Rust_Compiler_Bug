toml
rustflags = [
    # defers LTO optimizations to the linker
    "-C",
    "linker-plugin-lto",
    # static linking
    "-C",
    "prefer-dynamic=off",
]
