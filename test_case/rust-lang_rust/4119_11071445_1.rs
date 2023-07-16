
mod consts {

    #[cfg(target_os = "macos")]
    pub use macos::sysname;
    // etc.

    mod macos {
        pub fn sysname() -> ~str { ~"macos" }
        pub fn exe_suffix() -> ~str { ~"" }
        pub fn dll_suffix() -> ~str { ~".dylib" }
    }
}           
