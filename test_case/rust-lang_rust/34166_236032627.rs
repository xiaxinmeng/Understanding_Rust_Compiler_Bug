 Rust
pub struct Gl {
    pub data: [FnPtr; LEN]
}


pub struct FnPtr {
    pub f: *const u8,
    pub is_loaded: bool,
}

fn missing_fn_panic() -> ! {
    panic!("gl function was not loaded")
}

impl FnPtr {
    fn new(ptr: *const u8) -> FnPtr {
        if ptr.is_null() {
            FnPtr {
                f: missing_fn_panic as *const u8,
                is_loaded: false
            }
        } else {
            FnPtr { f: ptr, is_loaded: true }
        }
    }
}


pub fn load_with(loadfn: &mut FnMut(&str) -> *const u8) -> Gl {
    let mut metaloadfn = |symbol: &str, symbols: &[&str]| {
        let mut ptr = loadfn(symbol);
        if ptr.is_null() {
            for &sym in symbols {
                ptr = loadfn(sym);
                if !ptr.is_null() { break; }
            }
        }
        ptr
    };

    Gl { data: [
