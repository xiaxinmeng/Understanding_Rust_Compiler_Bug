rust
macro m() {
    use a::b::c; // "def-site" as `a` appears in `def-site`
    use a::$b; // def-site
    use ::$a; // XXX interesting example, see below
    use $a; // global path, but call-site determines relative to what
    $use_passed_from_the_outside a::b::c; // def-site
    $use_passed_from_the_outside $a; // same as `use $a`
}
