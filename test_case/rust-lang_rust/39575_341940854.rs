rust
#![feature(getpid)]

fn hacky_ptr_write<T>(ptr: &T, value: u32) {
    std::process::Command::new("gdb").arg("-batch").arg("-quiet")
        .arg("-pid").arg(format!("{}", std::process::id())).arg("-ex")
        .arg(format!("set {{unsigned long}}{:p}={}",ptr,value))
        .output() .unwrap();
}
fn main() {
    let q = &mut Box::new(55);
    hacky_ptr_write(&q, 0);
    *q = Box::new(44); // Segmentation fault
}
