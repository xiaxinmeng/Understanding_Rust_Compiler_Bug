
 // Creation time doesn't work on all systems, so this needs to be fixed.
    let mut created_meta = "";

    println!("{:?}", metadata.created().unwrap());

    match metadata.created().unwrap() {
        Ok() => created_meta = "All's good",
        Err() => created_meta = "ERROR",
    };
