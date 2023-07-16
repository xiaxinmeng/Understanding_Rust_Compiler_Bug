
fn main() {

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Annotation {
    pub start_col: u32,
    pub end_col: u32,
    pub is_primary: bool,
    pub label: Option<String>,
}

let mut a : Vec<Annotation> = Vec::new();
a.push(Annotation {start_col: 12, end_col: 16, is_primary: true, 
                   label: Some(String::from("help: consider using a reference instead `&v[0]`"))});
a.push(Annotation {start_col: 12, end_col: 16, is_primary: true, 
                   label: Some(String::from("cannot move out of indexed content"))});

println!("{:?}", a);
a.sort();
println!("{:?}", a);
}
