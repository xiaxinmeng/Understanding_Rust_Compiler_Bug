
enum Path { Left, Right };

fn find_or_insert_with(key: K, value: V, ...) -> &'mut V {
    let mut path = ~[];
    self.insert_helper(key, value, &mut path);
    self.find_from_path(path)
}

fn insert_helper(key: K, value: V, ..., path: &mut ~[Path]) {
    ...
    if should insert on left left {
        path.push(Left);
        self.left.insert_helper(key, value, ...);
    } else { ... }
}

fn find_from_path(&mut self, path: &[Path]) -> &'a mut V {
    if path.is_empty() { return &mut self.value; }
    match path[0] {
        Left => self.left.find_from_path(path.slice_from(1)),
        Right => ...    
    }
}
