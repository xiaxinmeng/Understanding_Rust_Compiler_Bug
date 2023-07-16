rust
#[cfg_attr(all(target_os = "windows", target_env = "msvc"), link(name = "dylib.dll"))]
#[cfg_attr(not(all(target_os = "windows", target_env = "msvc")), link(name = "dylib"))]
extern "C" { ... }
