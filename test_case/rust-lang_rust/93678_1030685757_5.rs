rs
fn nested() {
    let _ = || unsafe {
        let _ = || unsafe { unsf() };
    };
}
