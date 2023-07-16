rust
fn binary_search(x: i32, a: &Vec<i32>) -> usize {
    let mut i = 0;
    let mut j = a.len() - 1;

//    let mut m = 0;

    while i < j {
        let mut m = 0;
        m = i + (j - i) / 2;
        if x > a[m] {
            i = m + 1;
        } else {
            j = m;
        }
    }
    if x == a[i] {
        return i;
    } else {
        return a.len();
    }
}

fn main() {
    let mut a = vec![2, 1, 4, 3, 6, 5];
    a.sort();
    println!("5 at {}", binary_search(5, &a));
}
