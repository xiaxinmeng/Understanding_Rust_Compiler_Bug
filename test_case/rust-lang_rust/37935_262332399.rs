rust
        for _ in 0..1 {
            match x {
                Foo::A => true,
                Foo::B => false,
                _ => false,
            }
        }
