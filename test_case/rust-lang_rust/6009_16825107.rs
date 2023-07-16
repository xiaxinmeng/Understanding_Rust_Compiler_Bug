
    mod m {
        condition! {
            sadness: int -> int;
        }

        mod n {
            use super::sadness;

            #[test]                                                                                                                                                                                                  
            fn test_conditions_are_public() {
                let mut trapped = false;
                do sadness::cond.trap(|_| {
                    0
                }).in {
                    sadness::cond.raise(0);
                }
            }
        }
    }

