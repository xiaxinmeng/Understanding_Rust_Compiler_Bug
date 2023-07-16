rust
use {a, {self::b}, {c, {super::d}}};
=>
use ::{a, {self::b}, {c, {super::d}}}; // `::self::b` and `::super::d` are errors
