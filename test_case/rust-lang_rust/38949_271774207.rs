
// validate pathes by joining them one by one:
env::join_paths(new_paths.filter(|p| env::join_paths(iter::once(path)).is_ok()));
