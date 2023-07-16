
$ cat  src/main.rs
fn main() {
    let mut error = 0;
    for i in 0..3 {
        error += i;
    }
    println!("{}", error);
}
