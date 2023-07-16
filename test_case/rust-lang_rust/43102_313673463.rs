rust
extern crate x11_dl;

pub use x11_dl::xlib::Xlib;
pub use x11_dl::xcursor::Xcursor;
use x11_dl::error::OpenError;
use std::thread;
use std::time::Duration;

fn foo() -> Result<Xlib, OpenError> {
    // opening the libraries
    println!("new 1");
    let xlib = try!(Xlib::open());
    println!("new 2");
    let _xcursor = try!(Xcursor::open());

    Ok(xlib)
}

fn main() {
    thread::spawn(|| {
        foo().unwrap();
    });
    thread::sleep(Duration::new(999,0));
}
