rust
static DIR: Dir = include_dir!("stuff");

// expands to

static DIR: Dir = Dir {
    path: "stuff",
    files: &[
        File {
            path: r"stuff/jstree.min.js",
            contents: include_bytes!(r"stuff/jstree.min.js"),
        },
        File {
            path: r"stuff/test.txt",
            contents: include_bytes!(r"stuff/test.txt"),
        },
    ],
    dirs: &[],
};
