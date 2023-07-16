
use std::num::{pow, log10};

fn main() {
    let mut out = ~"";
    let mut n: f32 = -0.1231;
    if (n < 0.0) { n *= -1.0; out.push_char('-');}
    let mut m: f32 = log10(n).floor();
    if (m < 1.0) { m = 0.0 }
    loop {
        if (n > 0.000001f32) || (m >= 0.0) {
            let w: f32 = pow(10.0, m);
            let d = (n / w).floor();
            n -= d*w;
            out.push_str((d as uint).to_str_radix(10u));
            if (m == 0.0) { out.push_char('.') }
            m -= 1.0;
        }
        else {
            break
        }
    }
    println!("{}", out);
}
