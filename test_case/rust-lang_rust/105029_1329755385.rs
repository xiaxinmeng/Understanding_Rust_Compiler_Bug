rust
#![feature(type_name_of_val)] // not actually required to reproduce

struct Event {
    field: (),
}

trait Handler {
    fn call(self, arg: Event);
}

impl<F: FnOnce(Event)> Handler for F {
    fn call(self, arg: Event) {
        self(arg);
    }
}

fn call_handler(handler: impl Handler) {
    handler.call(Event { field: () });
}

fn main() {
    call_handler(|e| println!("type = {}", core::any::type_name_of_val(&e))); // OK: type = playground::Event
    call_handler(|e| e.field); // ERROR: type annotations needed
}
