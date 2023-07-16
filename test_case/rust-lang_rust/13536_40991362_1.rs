
#[cfg(target_os = "linux")]
mod consts {
    static FAMILY: &'static str = "unix";
    static SYSNAME: &'static str = "linux";
    ...
}
