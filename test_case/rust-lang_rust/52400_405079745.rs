rust
    let _: [u8; 0] = [4; {
        match &1 as *const i32 as usize {
            0 => 42, // should not have "a raw memory access tried to access part of a pointer value as raw bytes" here, any other error is expected and fine
            n => n,
        } 
    }];
