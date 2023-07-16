rust
impl Iterator for $Type {
    fn next(&mut self) -> Self::Item {
        // If `$condition` does not mutate `self`, then it will be the same
        // with every `next` invocation. (critical assumption)
        // `if $condition` may also be `match $condition` (more likely for enums)
        if $condition { 
            self.x.next() // delegate to an inner iterator
        } else {
            self.y.next() // delegate to a different inner iterator
        } // may be more conditions and inner iterators `else if ...`
    }
}
