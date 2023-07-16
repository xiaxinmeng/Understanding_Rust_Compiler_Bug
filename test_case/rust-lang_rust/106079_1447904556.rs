rust
trait X {
    type Y<'a>;
}
fn f2<'a>(arg : Box<dyn X<Y<1> = &'a ()>>) {}
