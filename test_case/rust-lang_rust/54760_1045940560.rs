`rust
#[cfg(target_os = "windows")]
/// Disables inheritance on the inout pipe handles.
fn disable_handle_inheritance() {
    use windows::Win32::Foundation::{SetHandleInformation, HANDLE_FLAGS, HANDLE_FLAG_INHERIT};
    use windows::Win32::System::Console::{
        GetStdHandle, STD_ERROR_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE,
    };

    unsafe {
        let std_err = GetStdHandle(STD_ERROR_HANDLE);
        let std_in = GetStdHandle(STD_INPUT_HANDLE);
        let std_out = GetStdHandle(STD_OUTPUT_HANDLE);

        for handle in [std_err, std_in, std_out] {
            SetHandleInformation(handle, HANDLE_FLAG_INHERIT.0, HANDLE_FLAGS(0));
        }
    }
}
