rust
fn get_index() -> usize {
    10
}

fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = get_index();

    let element = a[index];

    println!("The value of element is: {}", element);
}
