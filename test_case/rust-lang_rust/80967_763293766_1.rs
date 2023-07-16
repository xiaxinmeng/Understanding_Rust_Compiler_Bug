
    fn f() -> Option<usize> {
        let x: Option<Vec<u32>> = Some(vec![]);
        true.then(|| x?.len())
    }
