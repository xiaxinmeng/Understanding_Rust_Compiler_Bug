rust
#[derive(Clone)]
pub struct Node {
    pub children: Vec<Node>,
}

pub fn works(mut root: Vec<Node>) {
    let mut null = Vec::new();
    let mut elements = &mut root;

    {
        let mut tmp = &mut null;
        if let Some(el) = elements.get_mut(0) {
            tmp = &mut el.children;
        }
        elements = tmp;
    }

    elements.clear();
}

pub fn breaks(mut root: Vec<Node>) {
    let mut elements = &mut root;

    if let Some(el) = elements.get_mut(0) {
        elements = &mut el.children;
    }

    elements.clear();
}
