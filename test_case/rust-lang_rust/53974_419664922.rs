
fn main() {
    let (sender, receiver) = channel::<i32>();
    let _ = thread::spawn(move || loop {
        for i in 1..10_000_000 {
            sender.send(i).unwrap();
        }

        thread::sleep(Duration::new(20, 0));
    });

    let t2 = thread::spawn(move || loop {
        let _ =  receiver.recv();
    });

    if t2.join().is_err() {
        println!("finished !");
    }
}
