 rust
    unsafe {
        let mut left = seq.as_mut_ptr();
        let mut right = left.offset(len as int);
        while left < right {
            //asm!("dec $0" : "+r"(right));
            right = right.offset(-1);
            let tmp = COMPLEMENTS[*left as uint];
            *left = COMPLEMENTS[*right as uint];
            *right = tmp;
            //asm!("inc $0" : "+r"(left));
            left = left.offset(1);
        }
    }
