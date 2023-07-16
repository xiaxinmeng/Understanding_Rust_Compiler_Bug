
    let vc = vec![
        ("a", "foo"),
        ("b", "bar"),
        ("c", "baz")
    ];
-   test(&vc);
+   test(vc.iter().map(|&(ref k, ref v)| (k, v)));
