rust
fn hello_world<'r, R: 'r, N>(this: N)
where
    N: Fn(&'r R) -> &'r Self,
{
    println!("Hello, World!");
}
