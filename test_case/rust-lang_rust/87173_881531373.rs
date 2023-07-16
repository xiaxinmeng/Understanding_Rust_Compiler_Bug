rust
use tracked::fs::path as track_path;
use tracked::env::var as tracked_env_var;

fn my_proc_macro() {
    track_path("my_file.txt"); // returns nothing
    let var = tracked_env_var("MY_VAR"); // returns the variable's value
}
