rust
fn is_dupes_while(idxs: [usize; N]) -> bool {
    let mut i = 0;
    loop {
        match {
            let _t = i < N;
            _t
        } {
            true => {
                let mut j = i + 1;
                loop {
                    match {
                        let _t = j < N;
                        _t
                    } {
                        true => {
                            if idxs[i] == idxs[j] {
                                return true;
                            }
                            j += 1;
                        }
                        _ => break,
                    }
                }
                i += 1;
            }
            _ => break,
        }
    }
    false
}
