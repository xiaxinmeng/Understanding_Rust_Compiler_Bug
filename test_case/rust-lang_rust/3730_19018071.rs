 rust
    pub fn get_tydesc<T>() -> *TyDesc {
        unsafe {
            rusti::get_tydesc::<T>() as *TyDesc
        }
    }
