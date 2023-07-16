rust
#[cfg_attr(all(windows, target_env="msvc"), link(name="legacy_stdio_definitions", kind="static"))]
