
        for i in v[0].drain(..) {}
        for i in std::mem::replace(&mut m[0], BTreeSet::new()).into_iter() {}
