
#![feature(unsized_fn_params)]

pub enum Tree {
    Node(Box<Tree>, Box<Tree>),
    Leaf,
}

pub type Cont<'a> = dyn FnOnce(u64) -> u64 + 'a;

pub fn height_cps<'a>(x : &'a Tree, f : Cont<'a>) -> u64 {
    match x {
        Tree::Leaf => f(0),
        Tree::Node(l, r) => {
            height_cps(l, {let k : Box<Cont> = Box::new(move |lh| {
                height_cps(r, {let k : Box<Cont> = Box::new(move |lr| { let lh = lh; f(1 + lh.max(lr)) }); *k})
            }); *k})
        }
    }
}
