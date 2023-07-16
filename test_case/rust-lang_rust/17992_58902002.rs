
fn borrow_mut(x: &mut int) {
    *x += 1;
    second_hand_mut(x);
}

fn second_hand_mut(x: &mut int) {
    *x += 2;
}

fn main() {
    let mut x = 5i;
    borrow_mut(&mut x);
    println!("{}", x);      // Prints 8
}
