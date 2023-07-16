
    let f = File::create("foo.bar")?;
    let metadata = f.metadata()?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o755);
    assert_eq!(permissions.mode(), 0o755);
    fs::set_permissions("foo.bar", permissions)?;

    let f = File::open("foo.bar")?;
    let metadata = f.metadata()?;
    let permissions = metadata.permissions();
    assert_eq!(permissions.mode(), 0o755);   // we are actually getting 0o100755 from the OS here
