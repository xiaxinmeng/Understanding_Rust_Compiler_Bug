rust
        let registrar = unsafe {
            let sym = match lib.symbol(&sym) {
                Ok(f) => f,
                Err(err) => self.sess.span_fatal(span, &err),
            };
            mem::transmute::<*mut u8, fn(&mut dyn Registry)>(sym)
        };
