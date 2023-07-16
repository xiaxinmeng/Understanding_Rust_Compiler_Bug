rust
    #[cfg_attr(prefixed, export_name = "_rjem_malloc_conf")]
    #[cfg_attr(not(prefixed), export_name = "malloc_conf")]
    pub static mut malloc_conf: *const libc::c_char 
        = &b"background_thread:true\0"[0] as *const u8 as *const _;
    