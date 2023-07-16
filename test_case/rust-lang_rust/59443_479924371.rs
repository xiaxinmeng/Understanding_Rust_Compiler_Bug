rust
const PLATFORM_NAME: &str = cfg_match! {
    #[cfg(unix)] => "*nix",
    #[cfg(windows)] => "Win",
    _ => compile_error!("Unsupported platform!"),
};
