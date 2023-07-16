rust
struct Attr {
    name: String,
    value: String,
}

struct Element {
    attrs: Vec<Box<Attr>>,
}

impl Element {
    pub unsafe fn get_attr<'a>(&'a self, name: &str) {
        self.attrs
            .iter()
            .find(|attr| {
                      // To remove error: |attr: & & 'a Box<Attr>| {
                      let attr: &&Box<Attr> = std::mem::transmute(attr);
                      true
                  });
    }
}

pub fn main() {
    let element = Element { attrs: Vec::new() };
    let _ = unsafe { element.get_attr("foo") };
}
