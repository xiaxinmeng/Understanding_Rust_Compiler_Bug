 rust
fn main() {
// This is ok
    match &[(~5,~7)] {
        ps => {
           let (ref y, _) = ps[0];
           println!("1. y = {}", **y);
           assert!(**y == 5);
        }
    }

// This is not entirely ok
    match Some(&[(~5,)]) {
        Some(ps) => {
           let (ref y,) = ps[0];
           println!("2. y = {}", **y);
           if **y != 5 { println("sadness"); }
        }
        None => ()
    }

// This is not ok
    match Some(&[(~5,~7)]) {
        Some(ps) => {
           let (ref y, ref z) = ps[0];
           println!("3. y = {} z = {}", **y, **z);
           assert!(**y == 5);
        }
        None => ()
    }
}
