
    #[test]
    #[cfg(not(windows))]
    fn test_non_utf8_glob() {
        let dir = tempfile::TempDir::new("").unwrap();
        let p = dir.path().join(&[0xFFu8]);
        fs::mkdir(&p, S_IRWXU as u32);

        let pat = p.with_filename("*");
        assert_eq!(glob(pat.as_str().expect("tmpdir is not utf-8")).collect::<~[Path]>(), ~[p])
    }
