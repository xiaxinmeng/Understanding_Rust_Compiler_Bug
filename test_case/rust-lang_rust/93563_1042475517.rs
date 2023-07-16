rust
fn main() {
    let (tx, rx) = crossbeam_channel::unbounded();
    for _ in 0..100 {
        std::thread::Builder::new()
            .name("worker".to_string())
            .spawn({
                let tx = tx.clone();
                move || {
                    for i in 0..usize::MAX {
                        tx.send(i).unwrap();
                        std::thread::sleep(std::time::Duration::from_millis(1));
                    }
                }
            })
            .unwrap();
    }

    let mut sum = 0usize;

    // This impl uses ~26% of a core in top for crossbeam
    // and ~11% of a thread for std::sync::mpsc
    for v in rx.iter() {
        sum = sum.wrapping_add(v);
    }

    // This impl uses ~0.9% of a core in top for crossbeam
    // and ~1.8% in current std::sync::mpsc
    /*
    loop {
        match rx.try_recv() {
            Ok(v) => {
                sum = sum.wrapping_add(v);
            }
            Err(crossbeam_channel::TryRecvError::Empty) => {
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
            Err(crossbeam_channel::TryRecvError::Disconnected) => {
                break;
            }
        }
    }
    */

    println!("{}", sum);
}
