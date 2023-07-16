rust
fn main() {
    let wrapped_vec = WrappedVec {
        vec: vec!["1"],
    };
    
    let first = first_str_len(wrapped_vec);
    println!("first: {}", first);
}

struct WrappedVec<'a> {
    vec: Vec<&'a str>,
}

impl<'a> WrappedVec<'a> {
    fn iter(&'a self) -> impl Iterator<Item=&'a &'a str> {
        self.vec.iter()
    }
}

fn first_str_len<'a: 'a>(arg: WrappedVec<'a>) -> usize {
    let first = arg.iter().next().unwrap();

    let return_arg = |str: &'a str| -> &'a str {
        str
    };

    return_arg(first).len()
}
