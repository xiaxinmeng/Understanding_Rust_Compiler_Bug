rust
struct Parent
{
    children: Option<Vec<String>>
}

impl Parent {
    pub fn children() -> impl Iterator<Item=&str> {
        self.children.unwrap_or(Vec::new()).iter()
    }
}
