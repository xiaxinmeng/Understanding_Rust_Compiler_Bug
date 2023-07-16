
    let fam = if let Some(ref fam) = sess.target.target.options.target_family {
        Symbol::intern(fam)
    } else if sess.target.target.options.is_like_windows {
        Symbol::intern("windows")
    } else {
        Symbol::intern("unix")
    };

    let mut ret = HashSet::new();
    ret.insert((Symbol::intern("target_family"), Some(fam)));
    if fam == "windows" || fam == "unix" {
        ret.insert((fam, None));
    }
