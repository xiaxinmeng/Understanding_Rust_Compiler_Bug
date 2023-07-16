rust
let x: !;
match x {
    _ if {
        println!("Executed match!");
        loop {}
    } => (),
}
