rust
use std::borrow::Cow;

fn one() {
    let v = vec![1, 2, 3];
    let c: Cow<'static, [u8]> = Cow::Owned(v);
    for elem in &c {
        println!("elem: {}", elem);
    }
}

fn two() {
    let v = vec![1, 2, 3];
    let c: Cow<'_, [u8]> = Cow::Borrowed(v.as_slice());
    for elem in &c {
        println!("elem: {}", elem);
    }
}

fn three() {
    let v = vec![1, 2, 3];
    let c: Cow<'static, [u8]> = Cow::Owned(v);
    for elem in &*c {
        println!("elem: {}", elem);
    }
}

fn four() {
    let v = vec![1, 2, 3];
    let c: Cow<'_, [u8]> = Cow::Borrowed(v.as_slice());
    for elem in &*c {
        println!("elem: {}", elem);
    }
}
