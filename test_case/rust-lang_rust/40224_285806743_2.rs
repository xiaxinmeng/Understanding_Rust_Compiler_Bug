rust
fn err3() {
    let x: ?T;
    
    if true {
        x = Some::<?U>(<?V as Default>::default());
    } else {
        x = Some::<?W>(return);
    }
}
