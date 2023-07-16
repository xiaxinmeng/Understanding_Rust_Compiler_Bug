 rust
        let text = re.replace_all(text, |&mut: refs: &Captures| -> String {
            let pre = refs.at(1).unwrap_or("");
            let name = refs.at(2).unwrap_or("");
