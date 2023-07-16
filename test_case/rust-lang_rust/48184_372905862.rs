rust
impl<'input, 'path> WhereClause<'input, 'path> {
    pub fn add_trait_bound(&mut self, ty: &Ty) {
        let trait_path = self.trait_path;
        let params = self.params;
        let mut found = self.trait_output.map(|_| HashSet::new());
