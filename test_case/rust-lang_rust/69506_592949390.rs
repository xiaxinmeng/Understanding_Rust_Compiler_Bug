rust
macro_rules! empty { () => {} }

fn foo() -> u8 {
    { 0 }
    
    empty!();; // Works on 1.38, error on 1.39
}
