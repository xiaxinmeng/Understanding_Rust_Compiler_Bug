rust
#[cfg(target_os = "linux")] #[path = "proxyimpl/linux.rs"] pub mod proxyimpl;
#[cfg(not(target_os = "linux"))] #[path = "proxyimpl\\nonlinux.rs"] pub mod proxyimpl;
