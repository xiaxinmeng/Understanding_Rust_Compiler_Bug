rust
use std::thread;

const KILO: usize = 1024;
const MEGA: usize = 1024 * KILO;
const GIGA: usize = 1024 * MEGA;

// max failing
//const BUFFER_SIZE: usize = 4 * GIGA + 16;
const BUFFER_SIZE: usize = 4 * GIGA;
const REQUIRED_STACK_SIZE: usize = 512 * MEGA + BUFFER_SIZE;

fn main() {
    thread::Builder::new()
        .stack_size(REQUIRED_STACK_SIZE)
        .spawn(perform_double_free)
        .unwrap()
        .join()
        .unwrap();
}

fn perform_double_free() {
    let v1 = vec![0];
    let v2 = vec![0];

    verbose_drop(v2);
    verbose_drop(v1);

    println!("never reached");

    let buffer = [0u8; BUFFER_SIZE];
    mark_buffer_used(&buffer);
}

#[inline(never)]
fn verbose_drop(x: Vec<i32>) {
    println!("dropping vec at {:?}", x.as_ptr());
}

fn mark_buffer_used(buffer: &[u8]) {
    println!("{}", buffer[0]);
}
