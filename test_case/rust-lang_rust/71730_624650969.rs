rust
fn main() {
    let c: (u8, u8) = (0, 1);
    //
    let (x, y): &(u8, u8) = &c;
    let _: (&u8, &u8) = (x, y);
    //
    let (ref x, ref y): &(u8, u8) = &c;
    let _: (&u8, &u8) = (x, y);
    //
    let &(x, y): &(u8, u8) = &c;
    let _: (u8, u8) = (x, y);
    //
    let (x, y): &(u8, u8) = &c;
    let (&x, &y): (&u8, &u8) = (x, y);
    let _: (u8, u8) = (x, y);
    //
    let &(x, ref y): &(u8, u8) = &c;
    let _: (u8, &u8) = (x, y);
}

fn foo3((a, b): &(i32, i32)) {
    let _: &i32 = a;
    let _: &i32 = b;
}

fn foo3_2((ref a, ref b): &(i32, i32)) {
    let _: &i32 = a;
    let _: &i32 = b;
}

// fn foo4((&a, &b): &(i32, i32)) {}

fn foo4_2(&(a, b): &(i32, i32)) {
    let _: i32 = a;
    let _: i32 = b;
}
