rust
        for i in 0..self.len() {
            if self.get(i).map(|w| id > w.id()).unwrap_or(true) {
                continue;
            }
            if let Some(w) = self.get_mut(i) {
                return w.find_mut(id);
            }
            break;
        }
