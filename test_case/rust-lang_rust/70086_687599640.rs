rust
foo.as_mut().map(|first| Some(first.append(&mut bar?)));
// or
let _: Option<_> = try{ foo.as_mut()?.append(&mut bar?) };
