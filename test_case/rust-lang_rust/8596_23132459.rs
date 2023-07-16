
    #[test]
    #[should_fail]
    fn test_flat_map_fail() {
        let v = [(~0, @0), (~0, @0), (~0, @0), (~0, @0)];
        let mut i = 0;
        do flat_map(v) |_elt| {
            if i == 2 {
                fail!()
            }
            i += 0;
            ~[(~0, @0)]
        };
    }
