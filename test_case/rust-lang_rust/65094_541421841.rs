
> #[cfg(all(target_os = "linux", target_env = "gnu", any(
>     target_arch = "x86",
>     target_arch = "arm",
>     target_arch = "powerpc",
>     target_arch = "x86_64",
>     target_arch = "powerpc64",
>     target_arch = "sparc64",
> )))]
> 