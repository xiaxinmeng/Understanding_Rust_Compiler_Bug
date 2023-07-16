rust
// One edition
macro pass(item:$item) { $item }

// Another edition
pass! {
    macro matches(... $pat: pat | ...) { ... }
}
