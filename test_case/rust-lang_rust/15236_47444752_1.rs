 rust
fn main() {                     
    spawn(proc() {});           
    extern { fn sleep(s: i32); }
    unsafe { sleep(1) }         
}                               
