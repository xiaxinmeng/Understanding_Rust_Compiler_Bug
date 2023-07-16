 rust
trait face { fn x() -> int; }
impl <T> of face for @T { fn x() -> int { 5 } }

fn foo<T>(t: @T) -> int {
    (t as face).x()
}
