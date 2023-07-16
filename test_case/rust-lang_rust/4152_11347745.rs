 rust
trait Element {
    fn draw( &self );
}
struct Frame {
    element : @Element,
}
impl Frame {
    fn draw_all() {
        self.element.draw();
    }
}
fn main() {}
