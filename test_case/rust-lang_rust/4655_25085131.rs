
% uname -a
Darwin fklock-Oenone.local 12.5.0 Darwin Kernel Version 12.5.0: Mon Jul 29 16:33:49 PDT 2013; root:xnu-2050.48.11~1/RELEASE_X86_64 x86_64
% rustc --version
/Users/fklock/opt/rust-dbg/bin/rustc 0.8-pre (e5fdc7d 2013-09-20 00:36:11 -0700)
host: x86_64-apple-darwin
% cat d.rs
fn main() {
    std::io::println("hello");
}
// Adding a `#[cfg(test)]` section does not make the warning go away.
% rustc --test -Z debug-info  d.rs
warning: no debug symbols in executable (-arch x86_64)
% 
