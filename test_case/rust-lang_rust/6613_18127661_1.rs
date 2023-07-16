
fn insert_and_get_borrow<'r>(arr: &'r mut ~[int], i: int) -> &'r int {
    let mut j = 0, n = arr.len();
    while j < n {
        let p = &arr[j]; // (*)
        if *p == i { return p; }
    }
    // If not included yet, add it
    arr.push(i);
    &arr[arr.len() - 1]
}

fn main() { }
