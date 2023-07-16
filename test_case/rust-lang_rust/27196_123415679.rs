 rust
let (printSend, printRec): (Sender<Option<String>>, Receiver<Option<String>>) = channel();

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
