
<anon>:5:5: 8:6 error: reached the recursion limit during monomorphization
<anon>:5     pub fn matches<F: Fn()>(&self, f: &F) {
<anon>:6         let &A(ref term) = self;
<anon>:7         term.matches(f);
<anon>:8     }
playpen: application terminated with error code 101
