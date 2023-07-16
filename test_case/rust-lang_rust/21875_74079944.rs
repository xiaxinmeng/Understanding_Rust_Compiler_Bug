 rust
fn main() {
    let foo;
    {
        let bar = 1337;
        {
            foo = &bar;
        }
    }
}
