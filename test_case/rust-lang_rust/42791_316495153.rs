rust
#[cfg(windows)]
mod execs {
    pub const NPM: &'static str = "npm.cmd";
    pub const STYLUS: &'static str = "stylus.cmd";
}
#[cfg(not(windows))]
mod execs {
    pub const NPM: &'static str = "npm";
    pub const STYLUS: &'static str = "stylus";
}

...

if !Command::new(execs::STYLUS)
        .arg(stylus_dir)
        .arg("--out")
        .arg(theme_dir)
        .arg("--use")
        .arg("nib")
        .status()?
        .success() {
    bail!("Stylus encoutered an error");
}
