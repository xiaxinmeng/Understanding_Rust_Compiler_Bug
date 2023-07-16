 rust
    fn foo1() {
        let vec = ~[Some(())];
        for vec::each(vec) |slot| {
            do slot.each |_bucket| {
                break; // error: `break` outside of loop
            }
        }
    }

    fn foo2() {
        let vec = ~[Some(())];
        for vec::each(vec) |slot| {
            match *slot {
                Some(_bucket) => { break; }
                None => { }
            }
        }
