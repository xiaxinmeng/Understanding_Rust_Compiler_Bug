rust
fn main() {
    let read_compiler_error;
    let mut i = 0;
    loop {
        i += 1;
        if i == 4 {
            read_compiler_error = i;
            break;
        }
    }
    let loops = 25 % read_compiler_error;

    println!("Loops are {}", loops)
}
