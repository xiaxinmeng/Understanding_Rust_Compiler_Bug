rust
pub fn copy2(src: &[u8], dst: &mut [u8]) {
    if src.len() != dst.len() {
        let src = #[inline(never)] #[cold] #[track_caller] || {
            panic!(
                "source slice length ({}) does not match destination slice length ({})",
                src.len(), dst.len(),
            );
        };
        src();
    }
}
