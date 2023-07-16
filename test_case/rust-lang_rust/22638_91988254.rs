
struct Button<T: FnMut()> {
    f: Option<T>,
    title: &'static str,
}

impl<T: FnMut()> Button<T> {
    fn new() -> Button<T>
    { Button { f: None, title: "ClickMe" } }

    fn set(&mut self, f: T)
    { self.f = Some(f); }

    fn title(&mut self, title: &'static str)
    { self.title = title; }

    fn click(&mut self) {
        match self.f {
            Some(ref mut f) => f(),
            None => ()
        }
    }
}

fn main() {
    let mut btn = Button::new();
    btn.set(|| btn.title("Done"));
    btn.click();
    btn.click();
}
