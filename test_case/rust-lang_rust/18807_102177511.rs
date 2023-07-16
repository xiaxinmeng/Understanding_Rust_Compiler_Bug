 rust
    #[no_mangle]
    pub extern "C" fn mandel(x: f32, y: f32, dwell: i32) -> i32 {
      //your code here
    }
    #[allow(dead_code)]
    fn spare() { println!(""); } //adding this (doesn't have to be named "spare") makes the compilation work.
    // you don't even need to add this function signature where you're using these functions.
