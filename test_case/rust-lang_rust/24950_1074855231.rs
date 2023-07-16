rust
pub trait Flow {
    fn foo(&self);
}

pub trait LayoutDamageComputation {
    fn compute_layout_damage(self);
    fn children(&self) -> Vec<Self> where Self: Sized;
}

impl<'a> LayoutDamageComputation for &'a mut (dyn Flow + 'a) {
    fn compute_layout_damage(self) {
        for k in self.children() {
            k.compute_layout_damage();
            k.compute_layout_damage();
        }
    }
    fn children(&self) -> Vec<&'a mut (dyn Flow + 'a)> { vec![] }
}

fn main() { }
