rust
let mut cmd = Command::new("...");
fill_in_the_environment(&mut cmd);

let child_env = cmd.get_the_environment();
validate_the_environment(child_env)?;

/* Environment is global state, suppose it changes in between. Now the environment
   we validated is no longer the one that the process will run with. */

cmd.spawn()?;
