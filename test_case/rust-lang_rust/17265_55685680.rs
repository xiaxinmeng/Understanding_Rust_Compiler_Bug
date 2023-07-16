
                let mut bytes = [0, ..4];
                return match input.inner_read(bytes) {
                    Ok(4) => {
                        let errno = (bytes[0] << 24) as i32 |
                                    (bytes[1] << 16) as i32 |
                                    (bytes[2] <<  8) as i32 |
                                    (bytes[3] <<  0) as i32;
