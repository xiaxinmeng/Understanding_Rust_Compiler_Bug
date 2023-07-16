toml
# Enable a build of the extended rust tool set which is not only the compiler
# but also tools such as Cargo. This will also produce "combined installers"
# which are used to install Rust and Cargo together. This is disabled by
# default.
#extended = false

# Installs chosen set of extended tools if enables. By default builds all.
# If chosen tool failed to build the installation fails.
#tools = ["cargo", "rls", "clippy", "rustfmt", "analysis", "src"]
