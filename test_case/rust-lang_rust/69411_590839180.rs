rust
    let outdir = if let Some(mut path) = options.persist_doctests {
        path.push(format!(
            "{}_{}",
            filename.to_string().rsplit('/').next().unwrap().replace(".", "_"),
            line
        ));
        std::fs::create_dir_all(&path).expect("Couldn't create directory for doctest executables");

        DirState::Perm(path)
    }
