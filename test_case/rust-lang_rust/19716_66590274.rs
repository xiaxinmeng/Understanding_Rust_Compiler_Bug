 rust
    {
        ptrref1 = &ptr;
        ptrref2 = { let tmp = ptr; &tmp as *const u8 };
    }
