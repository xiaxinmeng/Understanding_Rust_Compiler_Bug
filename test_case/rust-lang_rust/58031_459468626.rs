
fn main() {
    let mut t = 0;

    t = 1;
    println!("1. t = {}", t);

    println!("2. t = {}", t);

    {
        let t = 2;
        println!("3. t = {}", t);
    }  // the `t` binding inside this scope goes away after the block goes away and the outer `t` is not accessible due to shadowing
    println!("4. t = {}", t); // this `t` is the one from earlier
}
