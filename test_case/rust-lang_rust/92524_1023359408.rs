toml
[target.'cfg(all(windows, target_env="msvc"))']
rustflags = [
    "-C", "link-arg=/stack:8388608"
]
