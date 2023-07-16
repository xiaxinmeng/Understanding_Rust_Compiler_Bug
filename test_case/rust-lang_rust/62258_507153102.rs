rust
    macro_rules! foo { () => (0) }
    
    use foo as macro_rules;
    
    fn main() {
        println!("{}", macro_rules!());
    }
    