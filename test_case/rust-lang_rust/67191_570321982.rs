rust
if false { const FOO: ! = panic!(); }

const CONST_FALSE: bool = false;
if CONST_FALSE { const FOO: ! = panic!(); }

const fn const_false_fn() -> bool { false }
if const_false_fn() { const FOO: ! = panic!(); }

const fn const_false_fn() -> bool { false }
if const_false_fn() { return AnError; }
const FOO: ! = panic!();
