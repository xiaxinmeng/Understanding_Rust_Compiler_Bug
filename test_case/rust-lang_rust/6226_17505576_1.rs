
priv mod a { pub fn b() }
priv mod c {
    use super::a::b;
}
