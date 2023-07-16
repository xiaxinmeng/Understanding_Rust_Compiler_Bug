
    fn f() -> Option<usize> {
        let x: Option<Vec<u32>> = Some(vec![]);
        let len = x?.len();
        true.then(|| len)
    }
