rust
fn is_dupes_for(idxs: [usize; N]) -> bool {
    {
        let _t = match (0..N).into_iter() {
            mut iter => loop {
                let mut __next;
                match (&mut iter).next() {
                    Some(0) => __next = val,
                    None => break,
                }
                let i = __next;
                {
                    {
                        let _t = match (i + 1..N).into_iter() {
                            mut iter => loop {
                                let mut __next;
                                match #[lang = "next"]
                                (&mut iter)
                                {
                                    Some(0) => __next = val,
                                    None => break,
                                }
                                let j = __next;
                                {
                                    if idxs[i] == idxs[j] {
                                        return true;
                                    }
                                }
                            },
                        };
                        _t
                    }
                }
            },
        };
        _t
    };
    false
}
