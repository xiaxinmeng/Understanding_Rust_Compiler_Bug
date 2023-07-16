rust
if let Some(ref rust) = toml.rust {
    ...
    config.ignore_git = config.channel == "dev";
    set(&mut config.ignore_git, rust.ignore_git);
    ...
}
