rust
fn err1() {
    let x;
    
    if true {
        x = Default::default();
    } else {
        x = return;
    }
}

fn err2() {
    let x = match true {
        true => Default::default(),
        false => return,
    };
}

fn err3() {
    let x;
    
    if true {
        x = Some(Default::default());
    } else {
        x = Some(return);
    }
}
