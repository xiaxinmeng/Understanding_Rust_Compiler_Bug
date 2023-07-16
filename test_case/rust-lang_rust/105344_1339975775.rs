rust
fn main() {
    TranslatorI.visit_pre();
}

impl TranslatorI {
    fn visit_pre(self) {
        Some(())
            .map(|_| self.flags())
            .unwrap_or_else(|| self.flags());
    }
}

struct TranslatorI;

impl TranslatorI {
    fn flags(&self) {}
}
