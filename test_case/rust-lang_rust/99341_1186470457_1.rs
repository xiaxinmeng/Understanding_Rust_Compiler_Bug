rs
pub const DLL_PROCESS_ATTACH: u32 = 1;
pub const DLL_PROCESS_DETACH: u32 = 0;

const TOOLCHAIN : &str = include_str!("../rust-toolchain");

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(
    _module: *mut (),
    call_reason: u32,
    _reserved: *mut std::os::raw::c_void,
) -> i32 {
    if call_reason == DLL_PROCESS_ATTACH {
        println!("test toolchain: {}", TOOLCHAIN);

        println!("going to hang on Windows 7 in std::collections::HashSet::new()");
        let mut has_set = std::collections::HashSet::new();

        println!("Unreachable on Windows 7");
        has_set.insert(0);

        1
    } else if call_reason == DLL_PROCESS_ATTACH {
        1
    } else {
        1
    }
}

#[no_mangle]
pub extern "cdecl" fn is_dll_ok() {
    if TOOLCHAIN.contains("nightly-2022-05-19-i686-pc-windows-msvc") {
        println!("under Windows 7 we can not get here, because it hangs in DllMain");
    } else if TOOLCHAIN.contains("nightly-2022-05-18-i686-pc-windows-msvc") {
        println!("on old version the dll is OK")
    } else {
        println!("sorry did not bother to parse the toolchain version to say if this one works")
    }
}
