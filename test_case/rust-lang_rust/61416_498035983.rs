toml
# Debuginfo level for most of Rust code, corresponds to the `-C debuginfo=N` option of `rustc`.
# `0` - no debug info
# `1` - line tables only
# `2` - full debug info with variable and type information
# Can be overriden for specific subsets of Rust code (rustc, std or tools).
# Debuginfo for tests run with compiletest is not controlled by this option
# and needs to be enabled separately with `debuginfo-level-tests`.
debuginfo-level = 2

# Debuginfo level for the compiler.
debuginfo-level-rustc = 2

# Debuginfo level for the standard library.
debuginfo-level-std = 2

# Debuginfo level for the tools.
debuginfo-level-tools = 2

