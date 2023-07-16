
<<<<<<< HEAD
        let text = re.replace_all(text, |&mut: refs: &Captures| -> String {
            let (pre, name) = (refs.at(1), refs.at(2));
=======
        let text = re.replace_all(text, |refs: &Captures| -> String {
            let pre = refs.at(1).unwrap_or("");
            let name = refs.at(2).unwrap_or("");
>>>>>>> regex_at_name_opt
