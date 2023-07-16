rust
fn main() {
    use std::fs::{create_dir_all, remove_dir_all};
    use std::thread::spawn;
    for _ in 0..10000 {
        let _ = remove_dir_all("rustdoc");
        let t1 = spawn(|| {
            create_dir_all("rustdoc/primitive").unwrap();
            let _ = remove_dir_all("rustdoc/primitive");
            create_dir_all("rustdoc/primitive").unwrap();
        });
        let t2 = spawn(|| {
            create_dir_all("rustdoc/primitive/no_std").unwrap();
            let _ = remove_dir_all("rustdoc/primitive/no_std");
            create_dir_all("rustdoc/primitive/no_std").unwrap();
        });
        t1.join().unwrap();
        t2.join().unwrap();
    }
}
