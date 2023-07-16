rust
match node {
    Node::Block { name, body } => {
        if blocks.contains_key(&name) {
            bail!("Block `{}` is duplicated", name);
        }
        let string = name.to_string();
        let children = body.get_children();

        blocks.insert(string, Node::Block { name, body });
        find_blocks(children, blocks)?;
    },
    // ...
}
