 rust
let t: Box<Thing> = match true {
    true => box Thing1 { name: "hello".to_string() } as Box<Thing>,
    _    => box Thing2 { name: "hello".to_string() } as Box<Thing>,
};
