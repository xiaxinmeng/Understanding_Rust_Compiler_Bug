
fn find_closure<'a>(&'a self, f: |&K| -> Ordering) -> Option<&'a V>;
fn find_mut_closure<'a>(&'a mut self, f: |&K| -> Ordering) -> Option<&'a mut V>;
