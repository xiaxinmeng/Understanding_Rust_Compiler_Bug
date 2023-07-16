diff
#![feature(euclidean_division)]

fn main() {
    let m = 3.0f64;
    let f = m - std::f64::EPSILON;

    let q = f.div_euc(m);
    let r = f.mod_euc(m);

-   assert_eq!(f, m * r + q);
+   assert_eq!(f, m * q + r);
}
