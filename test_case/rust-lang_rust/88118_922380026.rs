rust
#![feature(if_let_guard)]

fn print_error_count(registry: &Registry) {
    |x: &Registry| {
        match &x {
            Registry if let _ = registry.try_find_description() => { }
            _ => {}
        }
    };
}

struct Registry;
impl Registry {
    pub fn try_find_description(self) {
        unimplemented!()
    }
}

fn main() {}
