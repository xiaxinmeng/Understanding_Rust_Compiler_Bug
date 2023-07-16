` rust
fn ein() { }

fn zwei() { }

fn main() {
    println!("ein == zwei: {}", ein as *mut () == zwei as *mut ());  // false
    println!("ein == zwei: {}", ein as *mut () == ein as *mut ());   // true
}
