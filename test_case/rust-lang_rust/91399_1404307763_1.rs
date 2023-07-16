
let x = 1f64;
match x.next_up() {
    y if y.is_nan() => panic!("not a number"),
    y => {
        // do something with y
    }
}