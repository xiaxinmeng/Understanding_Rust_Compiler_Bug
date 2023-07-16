
fn main() {
    unsafe { ::std::rt::stack::record_sp_limit(0); }
    let file = File::open(&Path::new("test.txt"));
    let mut writer = BufferedWriter::new(file);
    println!("Flush: {}", writer.flush());
}
