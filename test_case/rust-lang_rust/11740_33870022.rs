 rust
struct Element {
    attrs: ~[()],
}

impl Element {
    pub unsafe fn get_attr<'a>(&'a self, name: &str) {
        self.attrs.iter().find(|attr| {
                let attr: () = std::cast::transmute(attr);
                true
        });
    }
}

pub fn main() {}
