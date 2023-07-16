rust
// exit.rs
fn detail_exit(message:String){
    if (test) {
        panic!(message)
    } else (not test) {
        resume_unwind(Box::new(message))
    }
}
