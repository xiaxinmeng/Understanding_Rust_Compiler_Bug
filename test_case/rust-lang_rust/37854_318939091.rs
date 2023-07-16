rust
#![feature(exclusive_range_pattern)]
fn main() {
    let i = 9; //this hits whichever branch is first in the match without warning
    match i {
        1...10 => println!("hit inclusive"),
        1..10 => println!("hit exclusive"),//FIXME: there's no warning here
        // ^ can shuffle the above two, still no warnings.
        1...10 => println!("hit inclusive2"),//this does get warning: unreachable pattern
        1..10 => println!("hit exclusive2"),//this also gets warning: unreachable pattern

        _ => (),
    };

    //expecting a warning like in this:
    match i {
        1..12 => println!("hit 12"),
        1..11 => println!("hit 11"),//warning: unreachable pattern
        _ => (),
    }
}
