rust
#[cfg(all(target_env = "msvc", debug))] // " if " -- appease style checker
#[link(name = "msvcrtd")]
extern {}

#[cfg(all(target_env = "msvc", not(debug_assertions)))] // " if " -- appease style checker
#[link(name = "msvcrt")]
extern {}
