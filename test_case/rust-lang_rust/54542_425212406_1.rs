rust
    (arr.as_mut_ptr() as *mut Vec<u8>).offset(i).write(vec![]);
