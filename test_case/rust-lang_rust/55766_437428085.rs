
    fn foo<T>(mut a: T, b: T) {
        Pin::new_unchecked(&mut a); // should mean `a` can never move again
        let a2 = mem::replace(&mut a, b);
        // the address of `a` changed to `a2`'s stack slot
    }
    