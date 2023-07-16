rust
#![feature(nll)]

struct Parent;
struct Child<'a>(Option<&'a Parent>);

// the signature implies that the child borrows the parent
fn adopt<'a>(p: &'a Parent, c: &mut Child<'a>) {
    c.0 = Some(p);
}

fn main() {
    let mut parents: Vec<Parent> = vec![];
    let mut children: Vec<Child<'_>> = vec![];

    for _ in 0..2 {
        parents.push(Parent);
        let p = parents.last().unwrap();
        
        children.push(Child(None));
        let c = children.last_mut().unwrap();
        
        adopt(p, c); // comment this line to make the program compile
    }
}
