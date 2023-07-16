
        SearchOp::RsQiFn( r, q, f )
            => Box::new( move | var :&LLFact | -> bool
            {
                true
                &&
                match var.forward
                {
                    DataPoint::None => true,
                    _ => false
                }
                &&
                match var.reverse
                {
                    DataPoint::Key( k ) => k == r,
                    _ => false
                }
            }),
