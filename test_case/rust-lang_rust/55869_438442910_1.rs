rust
pub fn iterate<St, F>(initial_value: St, f: F) -> Iterate<St, F>
where
    F: FnMut(&St) -> St, 
