rust
use num_integer::Roots;

fn main() {
    let v = 984067;

    for i in 1..=v {
        ramanujan(i)
    }
}

fn ramanujan(m: i32) {
    let maxcube = m.cbrt();
    let mut res1 = 0;
    let mut res2 = 0;
    let mut _res3 = 0;
    let mut _res4 = 0;
    
    for i in 1..=maxcube {
        for j in 1..=maxcube {
            if i * i * i + j * j * j == m {
                res1 = i;
                res2 = j;
                break;
            }
        }
    }
    
    for k in 1..=maxcube {
        for l in 1..=maxcube {
            if k == res1 || k == res2 || l == res1 || l == res2 {
                continue;
            }
            if k * k * k + l * l * l == m {
                _res3 = k;
                _res4 = l;
                break;
            }
        }
    }
}
