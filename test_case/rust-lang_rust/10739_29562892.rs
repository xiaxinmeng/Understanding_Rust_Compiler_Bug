 rust
            Some(idx) => {
                let mut tmp = &mut [];
                std::util::swap(&mut tmp, &mut self.v);
                let (h, t) = tmp.mut_split(idx);
                self.v = t.mut_slice_from(1);
                Some(h)
            }
