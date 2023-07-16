rust
macro_rules! empty { () => {} }

fn foo() -> u8 {
    { 0 }
    
    empty!();
}
