 rust
    data.y = vec![];  // is ok
    drop(data.y); // error: use of moved value: `data.y`
