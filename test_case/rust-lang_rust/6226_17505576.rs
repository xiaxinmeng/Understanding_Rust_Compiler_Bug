
priv mod a { pub fn b() }
priv mod c {
    use a::b; // a is private so resolve thinks we can't access it, even though `c` is a sibling of `a`
}
