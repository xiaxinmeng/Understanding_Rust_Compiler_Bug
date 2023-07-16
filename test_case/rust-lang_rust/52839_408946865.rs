rust
#[cfg(any(all(target_os = "linux", any(target_arch = "aarch64",
                                       target_arch = "arm",
                                       target_arch = "powerpc",
                                       target_arch = "powerpc64",
                                       target_arch = "s390x")),
          all(target_os = "android", any(target_arch = "aarch64",
                                         target_arch = "arm")),
          all(target_os = "l4re", target_arch = "x86_64"),
          all(target_os = "openbsd", target_arch = "aarch64"),
          all(target_os = "fuchsia", target_arch = "aarch64")))]
pub type c_char = u8;

#[cfg(any(target_pointer_width = "32", windows))]
pub type c_long = i32;
