 rust
for method in methods.iter() {
    match method.vis {
        ast::public => {
            self.worklist.push(method.id);
        }
        _ => (),
    }
}
