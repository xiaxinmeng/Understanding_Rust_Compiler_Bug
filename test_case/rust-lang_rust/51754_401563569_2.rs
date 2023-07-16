rust
impl<'a> MatcherPos<'a> {                                                                           
    /// Add `m` as a named match for the `idx`-th metavar.                                          
    fn push_match(&mut self, idx: usize, m: NamedMatch) {                                           
        let matches = Rc::make_mut(&mut self.matches[idx]);                                         
        matches.push(m);                                                                            
    }                                                                                               
}
