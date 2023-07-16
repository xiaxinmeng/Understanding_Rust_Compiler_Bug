 rust
let (printSend, printRec) = channel();

printSend.send(Some(String::from_str("Fake data"))).unwrap();

let printer = thread::spawn(move || {
    loop {
        match printRec.recv() {
            Ok(file) => {
                match file {
                    Some(f) => println!("{}", f),
                    None => {}
                }
            },
            Err(_) => break
        }
    }
});
