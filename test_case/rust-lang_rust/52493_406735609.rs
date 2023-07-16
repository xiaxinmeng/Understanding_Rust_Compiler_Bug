rust
                pub fn fetch_max(&self, val: $int_type, order: Ordering) -> $int_type {
                    self.fetch_update(|v| Some(v.max(val)), order, order).unwrap()
                }
