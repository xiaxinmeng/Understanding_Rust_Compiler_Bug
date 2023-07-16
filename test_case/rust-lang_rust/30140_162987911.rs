 rust
// serialize:
serialize((&self.foo, &self.bar)); // serialize first 2 fields, but not self.span

// deserialize:
let (foo, bar) = deserialize();
MyType { foo: foo, bar: bar, span: DUMMY_SP }
