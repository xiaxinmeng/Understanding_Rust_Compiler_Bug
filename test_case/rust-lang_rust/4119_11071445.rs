
#[cfg(target_os = "macos")]
mod consts {       
    pub fn sysname() -> ~str { ~"macos" }
    pub fn exe_suffix() -> ~str { ~"" }
    pub fn dll_suffix() -> ~str { ~".dylib" }
}           
