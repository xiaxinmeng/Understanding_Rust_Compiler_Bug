 rust
use std::any::Any;

fn log<T>(value: &T) {
    let value_any = value as &Any;

    // try to convert our value to a String.  If successful, we want to
    // output the String's length as well as its value.  If not, it's a
    // different type: just print it out unadorned.
    match value_any.downcast_ref::<String>() {
        Some(as_string) => {
        }
        None => {
        }
    }
}

fn main() {}
