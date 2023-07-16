rust
        let mut x = E([0; 32]);
        write_volatile(&mut x, E([1; 32]));
        assert_eq!(read_volatile(&x), E([1; 32])); // line 61
        assert_eq!(x, E([1; 32]));                 // line 62
