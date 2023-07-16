rust
use core::panic::Location;

#[track_caller]
fn demo(r: Result<u8, String>) -> Result<u8, String> {
    let location = Location::caller();
    r.map_err(|e| {
        Location::pretend_we_are_at(location);
        another_function();
        e
    })
}

#[track_caller]
fn another_location() {}
