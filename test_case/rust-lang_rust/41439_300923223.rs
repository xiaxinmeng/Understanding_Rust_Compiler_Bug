rust
fn foo(i: &mut Iterator<Item=i32>) {
    let d: Vec<_> = i.every_nth(2).take(2).collect(); // assuming `every_nth(0)` is `id`
}

fn main() {
    let itr = 0..10;
    foo(&mut itr);
    println!("{:?}", itr.next());
}
