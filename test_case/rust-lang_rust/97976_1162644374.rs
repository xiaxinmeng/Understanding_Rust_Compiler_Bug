rust
#[cfg(any(all(windows, feature = "winapi"), doc))]
fn doc_me() {
    let text = vec![0_u16];
    let text = text.as_ptr();
    unsafe { winapi::um::debugapi::OutputDebugStringW(text) }
}
