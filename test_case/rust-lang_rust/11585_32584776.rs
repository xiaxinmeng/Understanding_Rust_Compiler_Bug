 rust
// this compiles and works
fn xs_1() {
    let xs = ['1','2','3','4','5','6','7','8','9','0'];
    for i in xs.iter() { println!("{:c}", *i); }
}

// this works, too
fn xs_2() {
    for i in "1234567890".chars() { println!("{:c}", i); }
}

// but this doesn't with compiler error
//    error: borrowed value does not live long enough
fn xs_3() {
    for i in ['1','2','3','4','5','6','7','8','9','0'].iter() {
        println!("{:c}", *i);
    }
}
