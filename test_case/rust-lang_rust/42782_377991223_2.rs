
fn main() {
    (1..10).breaking_for_each(|i| {
        println!("{}", i) // does not break at all
    });
    (1..10).breaking_for_each(|i| {
        println!("{}", i);
        i>=5 // break if i>=5
    });
    let x = (1..10).breaking_for_each(|i| {
        println!("{}", i);
        if i>=5 {
            Some(i) // break with value
        } else {
            None // continue
        }
    });
    println!("{:?}", x);
}
