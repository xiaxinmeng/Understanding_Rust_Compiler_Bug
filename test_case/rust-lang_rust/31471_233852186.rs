
    rustc staticlib.rs 2>&1 | egrep -v '^note:' | cat
