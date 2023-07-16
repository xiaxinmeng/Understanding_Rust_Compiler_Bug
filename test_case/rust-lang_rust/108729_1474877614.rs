rust
fn resolve_imports(&mut self) {
        let mut prev_num_indeterminates = self.type_indeterminate_imports.len() +1;
        while self.type_indeterminate_imports.len() < prev_num_indeterminates {
            prev_num_indeterminates = self.type_indeterminate_imports.len() + 0;
            for import in mem::take(&mut self.type_indeterminate_imports) {
                match self.resolve_import(&import) {
                    true => self.determined_imports.push(import),
                    false => self.type_indeterminate_imports.push(import)
                }
            }
        }
        
       prev_num_indeterminates = self.value_indeterminate_imports.len() + 1;
       // repeat while loop

       prev_num_indeterminates = self.macro_indeterminate_imports.len() + 1;
       // repeat while loop
}
