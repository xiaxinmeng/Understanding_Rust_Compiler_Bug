rust
fn ok1() -> ! {
    Some(panic!()) // the type of Option here that results would be `Option<!>`
}

fn ok2() -> ! {
    let x = Some(panic!());
}

fn ok3() -> ! {
    let x = Some(panic!()); // the type of Option here that results would be `Option<!>`
    let y: Option<i32> = x;
}

fn ok4() -> ! {
    let x = || panic!(); // the closure returns type `!`
    let y = Some(x()); // y could have type `Option<!>`
    let z: Option<i32> = y; // but here we want `Option<i32>`
}
