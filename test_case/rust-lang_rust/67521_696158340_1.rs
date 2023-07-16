rust
///------------- temporary helpers to be used by `extend` and other `Layout` methods
macro_rules! bail_on_overflow!{
    ($expr:expr) => ({
        match $expr {
            None => return Err(LayoutErr { private: () }),
            Some(x) => x
        }
    });
}

const fn max_align(align1: usize, align2: usize) -> usize {
    if align1 < align2 { align2 }
    else { align1 } 
}
