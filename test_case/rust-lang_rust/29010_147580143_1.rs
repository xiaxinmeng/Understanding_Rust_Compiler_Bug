 rust
#![feature(catch_panic)]
fn test( i: usize ) -> i32 {
    let data = [0i32;10];
    data[i]
}

fn main() {
    match std::thread::catch_panic( || test( 50 ) ) {
        Ok(n) => println!( "{:?}", n ),
        Err(e) => println!(  "{:?}", e )
    }
}
