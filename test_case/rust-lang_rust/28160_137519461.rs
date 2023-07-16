 rust
fn report(a: &Vec<i32>) {
    println!("{:?} {:?} {:?} {:?}", a.len(), a.capacity(), a[0], &a[0] as *const _);
}

fn main() -> () {
    let mut a = vec![0];
    report(&a);
    let xxx = &a[0] as *const _;
    println!("{:?} {:?}", xxx, unsafe { *xxx });
    a[0] += {
        report(&a);
        a.push(1);
        report(&a);
        a.push(2);
        report(&a);
        3
    };
    report(&a);
    println!("{:?} {:?}", xxx, unsafe { *xxx });
}

1 1 0 0x7f4386024008
0x7f4386024008 0
1 1 0 0x7f4386024008
2 2 0 0x7f4386024008
3 4 0 0x7f4386023000
3 4 0 0x7f4386023000
0x7f4386024008 3
