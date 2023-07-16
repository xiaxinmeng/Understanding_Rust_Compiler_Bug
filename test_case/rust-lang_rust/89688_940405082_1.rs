rust
let dummy = value;
if let Some(inner) = dummy {
    println!("inner: {:?}", inner);
} else {
    // value has moved into _dummy, above
    println!("other: {:?}", value);
    return
}
