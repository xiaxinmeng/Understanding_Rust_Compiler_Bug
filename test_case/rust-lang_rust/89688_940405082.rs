rust
let Some(inner) = value else {
    println!("other: {:?}", value);
    return
};
println!("inner: {:?}", inner);
