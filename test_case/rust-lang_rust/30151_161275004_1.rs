 rust
            let result = get_memrchr(s.as_bytes());
            for i in 0..n {
                match result {
                    Some(x) => ret += x,
                    None => ()
                };
                ret += i;
            }
