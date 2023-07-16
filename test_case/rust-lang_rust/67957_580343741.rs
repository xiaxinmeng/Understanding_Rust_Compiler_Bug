rust
        for i in 0..self.len() {
            if let Some(w) = self.get_mut(i) {
                if id > w.id() {
                    continue;
                }
                return w.find_mut(id);
            }
            break;
        }
