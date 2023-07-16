
let x = 5i;
let p = &x;
assert_eq!(format!("{}", &p), format!("{:p}", &p));
