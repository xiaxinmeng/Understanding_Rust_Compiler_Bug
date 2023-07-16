
struct Parent;
struct Child<'a>(Option<&'a Parent>);

fn main() {
    let mut parents: Vec<Parent> = vec![];
    let mut children: Vec<Child<'_>> = vec![];

    for _ in 0..2 {
        parents.push(Parent);
        let p = parents.last().unwrap();
        
        children.push(Child(None));
        children.last_mut().unwrap().0 = Some(p);
    }
}
