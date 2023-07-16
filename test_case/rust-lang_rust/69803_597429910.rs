
            Ok(n) => {
                g.len += n;
                if g.buf.len() > g.len {
                    ret = Ok(g.len - start_len);
                    break;
                }
            }
