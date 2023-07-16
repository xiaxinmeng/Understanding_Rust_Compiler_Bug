 rust
$ ag unhygienize
clippy_lints/src/shadow.rs
69:            bindings.push((ident.node.unhygienize(), ident.span))
123:            let name = ident.node.unhygienize();
330:    !path.global && path.segments.len() == 1 && path.segments[0].name.unhygienize() == name
340:        if self.name == name.unhygienize() {

clippy_lints/src/misc.rs
414:                   segment != segment.unhygienize() && // not in bang macro
