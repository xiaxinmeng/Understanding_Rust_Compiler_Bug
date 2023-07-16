rust
let mut buffer = vec![0; 100];
let a = match socket.nb_read(buffer.as_mut_slice()) {
    Ok(_) => // do something here
    Err(Error::TryAgain) => // do something else here
    Err(e) => println!("{}", e)
};
