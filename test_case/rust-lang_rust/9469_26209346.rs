
    unsafe "int_overflow" {
        // integer overflows don't fail, others unsafe operations are still forbidden
    }
    unsafe "ffi", "raw_ptr"{
        // ffi and dereferencing raw pointers allowed, others unsafe operations are still forbidden
    }
