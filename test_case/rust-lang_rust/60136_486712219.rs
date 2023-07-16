rust
struct CustomElements {
    processor: Box<fn(&mut CustomElements)>,
}

fn main() {
    move |controller: &mut CustomElements| {
        (controller.processor)(controller);
    };
}
