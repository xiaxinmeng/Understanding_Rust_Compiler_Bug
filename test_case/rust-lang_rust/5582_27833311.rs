
mod foo {
    use vectors = std::vec;
    use vectors::raw;

    pub fn whatever() {
        use vectors::from_elem;
        let _ = from_elem(1, 5);
    }
}

fn main() {
    foo::whatever();
}
