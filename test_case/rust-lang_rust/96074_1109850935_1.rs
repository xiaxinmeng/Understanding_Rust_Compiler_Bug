rust
pub fn constraints<C>(mut self, constraints: C) -> Layout
where
    C: Into<Vec<Constraint>>,
