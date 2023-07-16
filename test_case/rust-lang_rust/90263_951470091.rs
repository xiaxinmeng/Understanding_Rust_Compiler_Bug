rust
// init_rtld_audit_interface
#[used]
#[allow(non_upper_case_globals)]
#[link_section = ".init_array"]
static init_rtld_audit_interface: unsafe extern "C" fn(i32, *const *const i8, *const *const i8) = {
    #[link_section = ".text.startup"]
    unsafe extern "C" fn init_rtld_audit_interface(
        _argc: i32,
        _argv: *const *const i8,
        _envp: *const *const i8,
    ) {
        let _ = std::fs::metadata("");
    }
    init_rtld_audit_interface
};

// la_version
#[no_mangle]
unsafe extern "C" fn la_version(version: u32) -> u32 {
    version
}
