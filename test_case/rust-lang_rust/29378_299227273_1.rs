rust
fn open_file_in_thread() -> thread::Result<()> {
    let child = thread::spawn(move || {
        File::open("foo.txt").unwrap();
        // etc.
    });
    child.join()
}
