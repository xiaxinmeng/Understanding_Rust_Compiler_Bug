rust
fn main() {
    let file = match File::open(&path) {
        Ok(mut file) => {
            match file.read_to_string(&mut test_time) {}
        }
        Err(_) => (),
    };
   // you bounded file with the value of match { ... } expr, but didn't use it until the end of fn main
}
