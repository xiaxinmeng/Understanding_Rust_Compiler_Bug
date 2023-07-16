 rust
let mut it = s.char_indices()
              .scan(0u, |i1, (i2, c)| {
                         ....  // Same code as above
              })
              // last word
              .finally(|i1, (i2, _)|  Some(Some( s.slice(*i1, i2+1))))
              .filter_map(|x| x);
