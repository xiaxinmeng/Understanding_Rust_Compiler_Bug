
let x = 1f64;
let y = x.next_up();
if y.is_nan() {
     panic!("not a number");
}
