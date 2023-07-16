rust
#[macro_export]
macro_rules! print_me {
    ($p:path) => {
        {
            use $p as V;
            println!("{}", V);
        }
    }
}
