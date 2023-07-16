
#[cfg(test)]
#[macro_escape]
mod anonymous20130731 {
    macro_rules! if_test(($t:expr) => ($t))
}

#[cfg(not(test))]
#[macro_escape]
mod anonymous20130731 {
    macro_rules! if_test(($t:expr) => (()))
}


fn f() {
    if_test!(std::io::println("I am in testing mode."));
    std::io::println("The function f is all done.");
}

fn main () {
    f();
}

#[test] fn t1 () {
    f();
}
