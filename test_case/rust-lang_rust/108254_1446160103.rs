rust
trait Fancy {}
trait Good {}
impl <'a, T> Fancy for &'a T where T: Good {}
impl <S> Good for Option<S> where S: Iterator {}

fn want_fancy<F>(f: F) where F: Fancy {}

fn example() {
    want_fancy(&Some(5));
//  (BEFORE)   ^^^^^^^^ `{integer}` is not an iterator 
//  (AFTER)          ^  `{integer}` is not an iterator
}
