 rust
 #![feature(macro_rules)]

macro_rules! dispute(
    ($cond:expr) => (
        if $cond {
            panic!(concat!("disputation refuted: ", stringify!($cond)))
        }
    );
    ($cond:expr, $($arg:expr),+) => (
        if $cond {
            panic!($($arg),+)
        }
    );
)

macro_rules! dispute_eq(
    ($given:expr , $expected:expr) => ({
        match (&($given), &($expected)) {
            (given_val, expected_val) => {
                // check both directions of equality....
                if ((*given_val == *expected_val) ||
                     (*expected_val == *given_val)) {
                    panic!("disputation refuted: `(left != right) || (right != left)` \
                           (left: `{}`, right: `{}`)", *given_val, *expected_val)
                }
            }
        }
    })
)

fn five_back() -> int { 5i }


 fn main () {
     println!("yo");
 }

#[test]
fn five_back_failure() {
    dispute!( 6i == five_back())
    dispute!( true == true );
}

#[test]
fn five_back_test() {
    assert!( 5i == five_back());
    assert!( true == false );
}
