
    let fam = sess.target.target.options.target_family;
    if let Some(fam) {
        ret.insert((Symbol::intern("target_family"), Some(fam)));
        ret.insert((fam, None));
    }
