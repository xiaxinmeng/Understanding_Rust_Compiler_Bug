rust
#![cfg(windows)]

use winapi::shared::minwindef::*;
use winapi::um::processthreadsapi::{TlsAlloc, TlsFree};

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: DWORD, reserved: LPVOID) -> BOOL {

    const DLL_PROCESS_ATTACH: DWORD = 1;
    const DLL_PROCESS_DETACH: DWORD = 0;

    match call_reason {
        DLL_PROCESS_ATTACH => (), 
        DLL_PROCESS_DETACH => (), 
        _ => ()
    }   
    TRUE
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "stdcall" fn MyFunc() {
    // Get TLS index before Rust thread
    let tls_idx_start = unsafe { TlsAlloc() };

    // Let Rust leak some TLS slots
    println!("spawning rust thread");
    let th = std::thread::spawn(|| {
        println!("hello from Rust thread");
    }); 
    println!("joining rust thread: {:?}", th.thread().id());
    th.join().unwrap();

    // Get TLS index after Rust thread
    let tls_idx_end = unsafe { TlsAlloc() };

    // Free all TLS slots between tls_idx_start and tls_idx_end
    for tls_idx in tls_idx_start..tls_idx_end+1 {
        unsafe { TlsFree(tls_idx) };
    }   
}
