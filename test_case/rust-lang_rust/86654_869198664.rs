rust
pub fn bar(_: &fn()) {}

pub fn baz() {
    fn good() {}
    
    // All good:
    bar(&(good as fn()));
        
    // No good:
    bar(&good);
}
