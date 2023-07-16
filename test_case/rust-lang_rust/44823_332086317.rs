rust
            match item.node {
                ItemKind::Use(..) => {
                    // don't suggest placing a use before the prelude
                    // import or other generated ones
<<<<<<< HEAD
                    if item.span == DUMMY_SP {
                        let mut span = item.span;
                        span.hi = span.lo;
                        self.span = Some(span);
||||||| parent of 74748b11bb3... WIP: don't suggest placing `use` statements into expanded code
                    if item.span == DUMMY_SP {
                        self.span = Some(item.span.with_hi(item.span.lo()));
=======
                    if item.span.ctxt().outer().expn_info().is_none() {
                        self.span = Some(item.span.with_hi(item.span.lo()));
>>>>>>> 74748b11bb3... WIP: don't suggest placing `use` statements into expanded code
                        self.found_use = true;
                        return;
                    }
                },
                // don't place use before extern crate
                ItemKind::ExternCrate(_) => {}
                // but place them before the first other item
                _ => if self.span.map_or(true, |span| item.span < span ) {
<<<<<<< HEAD
                    let mut span = item.span;
                    span.hi = span.lo;
                    self.span = Some(span);
||||||| parent of 74748b11bb3... WIP: don't suggest placing `use` statements into expanded code
                    self.span = Some(item.span.with_hi(item.span.lo()));
=======
                    if item.span.ctxt().outer().expn_info().is_none() {
                        // don't insert between attributes and an item
                        if item.attrs.is_empty() {
                            self.span = Some(item.span.with_hi(item.span.lo()));
                        } else {
                            // find the first attribute on the item
                            for attr in &item.attrs {
                                if self.span.map_or(true, |span| attr.span < span) {
                                    self.span = Some(attr.span.with_hi(attr.span.lo()));
                                }
                            }
                        }
                    }
>>>>>>> 74748b11bb3... WIP: don't suggest placing `use` statements into expanded code
                },
