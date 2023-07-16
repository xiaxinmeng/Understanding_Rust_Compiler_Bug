rust
let something = Some(1);
    
match something {
    | None
    | Some(x) => println!("something = {}", x)
};
