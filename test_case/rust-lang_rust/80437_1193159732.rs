rust
#![feature(box_into_inner)]
#[allow(unused_mut)]

fn main() {
    let vec = vec![Box::new(1.0), Box::new(2.0), Box::new(3.0)];
    //   `i` will be moved
    let v : Vec<_> = vec.into_iter().map(|i| {
        let v = Box::into_inner(i) + 1.0; 
        // i;  // Error :use of moved value: `i`
        v
    }).collect();
    assert_eq!([2.0, 3.0, 4.0], &v[..]);

    let vec = vec![Box::new(1.0), Box::new(2.0), Box::new(3.0)];
    //  `i` will be copied
    let v : Vec<_> = vec.into_iter().map(|i| *i + 1.0).collect();
    assert_eq!([2.0, 3.0, 4.0], &v[..]);

    let vec = vec![Box::new(1.0), Box::new(2.0), Box::new(3.0)];
    // Error: cannot move out of `*i` which is behind a shared reference
    // let v : Vec<_> = vec.iter().map(|i| Box::into_inner(*i) + 1.0).collect();

    let vec = vec![Box::new(1.0), Box::new(2.0), Box::new(3.0)];
    //  `i` will be copied
    let v : Vec<_> = vec.iter().map(|i| **i + 1.0 ).collect();
    assert_eq!([2.0, 3.0, 4.0], &v[..]);
}

