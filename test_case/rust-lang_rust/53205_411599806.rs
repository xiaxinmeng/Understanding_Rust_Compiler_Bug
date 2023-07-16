rust
macro_rules! my_include {() => {
    macro m() {} // 1
    
    {
        macro m() {} // 2
        
        m!(); // ERROR `m` is ambiguous (1 or 2)
    }
}}

fn main() {
    my_include!();
}
