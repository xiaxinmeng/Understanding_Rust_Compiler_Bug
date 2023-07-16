
iface to_str { fn to_str() -> str; }
impl of to_str for int { fn to_str() -> str { int::str(self) } }
impl of to_str for str { fn to_str() -> str { self } }
impl <A: to_str> of to_str for [A] { fn to_str() -> str {
    "[" + str::connect(vec::map(self, {|e| e.to_str()}), ", ") + "]"
} }

fn main() { [1, 2, 3].to_str(); }
