
 let maybe_intern: &'self Option<&Intern> = do arc_interner.write |interner| {
                        match interner.str_to_intern.find(&burrowed.to_str()) { Some(v) => Some(*v), None => None }
                    };
