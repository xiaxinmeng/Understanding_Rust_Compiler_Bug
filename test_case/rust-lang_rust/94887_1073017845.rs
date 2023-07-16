plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
    |
9   | mod tests;
    | ---------- previous definition of the module `tests` here
...
340 | mod tests {
    | ^^^^^^^^^ `tests` redefined here
    |
    = note: `tests` must be defined only once in the type namespace of this module
For more information about this error, try `rustc --explain E0428`.
error: could not compile `std` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
