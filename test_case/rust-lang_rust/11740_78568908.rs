 rust
struct Element {
    attrs: Box<[()]>,
}

impl Element {
    pub unsafe fn get_attr<'a>(&'a self, name: &str) {
        self.attrs.iter().find(|attr| { // or |attr: &&()|
                let attr: Box<()> = std::mem::transmute(attr);
                true
        });
    }
}

pub fn main() {}
