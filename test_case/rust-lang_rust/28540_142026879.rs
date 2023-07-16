 Rust
        SearchOp::RsQiFn( r, q, f )
            => Box::new( move | var :&LLFact | -> bool
            {
                match var.forward
                {
                    DataPoint::None => true,
                    _ => false
                }; // <- note implicit semicolon
                & & match var.reverse
                {
                    DataPoint::Key( k ) => k == r,
                    _ => false
                }
            }),
