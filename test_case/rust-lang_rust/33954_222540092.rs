 rust
fn die() {
    power_off_the_machine() // does what it says
}

#[test]
#[should_panic]
fn it_dies() {
    die(); // after this test runs no other tests get run nor summary is printed 
           // and even the machine gets shut down!
}
