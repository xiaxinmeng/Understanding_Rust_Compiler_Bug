 rust
            for i in 0..n {
                match get_memrchr(s.as_bytes()) {
                    Some(x) => ret += x,
                    None => ()
                };
                ret += i;
            }
