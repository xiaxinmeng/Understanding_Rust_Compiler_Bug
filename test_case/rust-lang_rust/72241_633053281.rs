rust
#[link(name = "gcc_s", cfg(all(rustc_detect_dynamic, gcc_toolchain)))]
#[link(name = "gcc_eh", cfg(all(not(rustc_detect_dynamic), gcc_toolchain)))]
#[link(name = "unwind", cfg(not(gcc_toolchain)))]
extern {}
