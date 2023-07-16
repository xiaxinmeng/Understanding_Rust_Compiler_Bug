rust
use {a, {b}, {c, {d}}};
=>
use {::a, {self::b}, {::c, {super::d}}}; // OK
