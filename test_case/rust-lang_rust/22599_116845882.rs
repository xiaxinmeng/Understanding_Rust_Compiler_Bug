 rust
fn main() {
    let mut state = 0;
    println!("{:?}", state);

    let ch = 'z';

    state = match ch  {
        bar => 1,
    };
    println!("{:?}", state);
}
