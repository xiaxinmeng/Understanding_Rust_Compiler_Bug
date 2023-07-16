rust
let x = [-100, -50, 50, 100];

assert_eq!(&x[..], &[-100, -50, 50, 100]);
assert_eq!(&x[..2], &[-100, -50]);
assert_eq!(&x[2..], &[50, 100]);
assert_eq!(&x[1..3], &[-50, 50]);

let [y @ ..] = x;
assert_eq!(x, y);
