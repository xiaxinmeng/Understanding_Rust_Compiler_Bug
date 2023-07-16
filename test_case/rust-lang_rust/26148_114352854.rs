 rust
#[link(name = "LLVM-3.7.0svn")]
#[link(name = "pthread")]
#[link(name = "rt")]
#[link(name = "dl")]
#[link(name = "m")]
#[cfg_attr(not(target_env = "msvc"), link(name = "c++"))]
extern {}
