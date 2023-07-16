 rust
static mut MANUAL_CRATE_MAP: *CrateMap = 0 as *CrateMap;

#[no_mangle]
pub extern fn rust_set_crate_map(map: *CrateMap) {
    unsafe { MANUAL_CRATE_MAP = map; }
}
