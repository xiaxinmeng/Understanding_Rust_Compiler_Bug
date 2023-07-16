
    pub unsafe fn validate_str_mut<'a>(str_ptr: *mut u8, _str_max: usize) -> Result<(&'a mut [u8]), ()> {
        if str_ptr.is_null() {
            return Err(());
        }

        use std::slice;
        let new_str: &mut [u8] = slice::from_raw_parts_mut(str_ptr, _str_max);

        Ok(new_str)
    }

