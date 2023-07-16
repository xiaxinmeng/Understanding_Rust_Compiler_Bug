 rust
pub trait Flow {
    fn foo();
}

pub trait LayoutDamageComputation {
    fn compute_layout_damage(self);
    fn children(&self) -> Vec<Self>;
}

impl<'a> LayoutDamageComputation for &'a mut (Flow + 'a) {
    fn compute_layout_damage(self) {
        for k in self.children() {
            k.compute_layout_damage();
            k.compute_layout_damage();
        }
    }
    fn children(&self) -> Vec<&'a mut (Flow + 'a)> { vec![] }
}

fn main() { }
