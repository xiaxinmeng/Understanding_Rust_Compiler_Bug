
ET ::= ...        // as above
    | ET.m(...)   // *only* for m that takes `&self` 
                  // or `&mut self`, *not* `self`-by-value
