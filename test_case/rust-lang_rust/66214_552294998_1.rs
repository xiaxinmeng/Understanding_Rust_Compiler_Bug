rust
if empty_exponent {
    let mut err = self.struct_span_fatal(
        start, self.pos,
        "expected at least one digit in exponent"
    );
    err.emit();
}

match base {
    Base::Hexadecimal => {
        self.err_span_(start, suffix_start,
                       "hexadecimal float literal is not supported")
    }
    Base::Octal => {
        self.err_span_(start, suffix_start,
                       "octal float literal is not supported")
    }
    Base::Binary => {
        self.err_span_(start, suffix_start,
                       "binary float literal is not supported")
    }
    _ => ()
}
