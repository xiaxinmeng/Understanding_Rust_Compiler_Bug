 rust
fn main() {
    let mut r = vec![1, 2].into_iter();
    let mut flag = true;
    'out: while flag {
        flag = false;
        if let Some(x) = r.next() {
            flag = true;
            println!("{}", x);
            continue 'out;
        }
    }
}
