rs
#![feature(box_patterns)]
struct Type {
    pub name: Box<str>,
    pub generics: Option<Vec<Type>>
}

fn main(){
    let kind = Type {
        name: "User".into(),
        generics: Some(Vec::new())
    };

    match kind {
        Type {
            name: box "User",
            generics
        } => {}
        _ => unimplemented!()
    }
}
