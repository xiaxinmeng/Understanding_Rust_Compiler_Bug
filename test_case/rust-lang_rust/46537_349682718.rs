
#![allow(dead_code)]

fn read(_: &i32) { } 

fn ok() {
    let mut x = 3;
    let y = &mut x;
    { let z = &x; read(z); }
    *y += 1;
}

fn not_ok() {
    let mut x = 3;
    let y = &mut x;
    let z = &x;
    *y += 1;
    read(z); //~ ERROR somewhere in here
}
