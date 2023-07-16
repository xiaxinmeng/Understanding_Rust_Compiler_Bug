rust
...
socket
    .for_each(|message| {
        println!("recieved: {:?}", message) // error: mismatched types: expected `IntoFuture` but found `()`
    })
    .map_err(|e| println!("read error: {:?}", e))
...
