rust
fn foo() -> i32 {
    #[cfg(unix)]
    #[allow(unused)]
    {
        println!("test");
        let bar = 5;
        3
    }
}
