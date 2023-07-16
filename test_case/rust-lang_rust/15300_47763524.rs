 rust
    let b3 = {
        let tmp1 = *{a = 10; &mut a};
        let tmp2 = *{a = 100; &mut a};
        my_add(&tmp1, &tmp2)
    };
