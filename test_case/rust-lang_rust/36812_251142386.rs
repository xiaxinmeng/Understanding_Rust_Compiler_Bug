 rust
fn visit_name(&mut self, span: Span, name: Name) {
        debug!("visit_name: st={:?}", self.st);
        SawIdent(name.as_str()).hash(self.st);
        hash_span!(self, span);
}
