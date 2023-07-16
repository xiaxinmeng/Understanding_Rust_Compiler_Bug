
// impl_span: `impl<T> Iterator for A<T> { ... }`
// impl_span: `impl Iterator for A { ... }`
// impl_span: `impl      Iterator for A { ... }`
let cm = self.session.codemap();
let sp = cm.span_until_char(impl_span, '<'); // `impl`, `impl Iterator for A { ... }`, `impl    Iterator for A { ... }`
let sp = cm.span_until_char(sp, ' '); // `impl`, `impl`, `impl    Iterator for A { ... }`
let sp = cm.span_until_char(sp, '\t'); // `impl`, `impl`,`impl`
