Rust
        if let Some(_d) = self.option_loud_drop(8)
            && let Some(_e) = self.option_loud_drop(7)
            && let Some(_e) = self.option_loud_drop(6)
            && self.option_loud_drop(1).is_some()
            && self.option_loud_drop(2).is_some()
            && self.option_loud_drop(3).is_some()
            && self.option_loud_drop(4).is_some() {
                self.print(5);
        }
