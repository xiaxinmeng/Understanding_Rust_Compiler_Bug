
process1: is_dir(path) # false
process2: is_dir(path) # false
process1: mkdir(path) # Ok(())
process2: mkdir(path) # Err(ErrorKind::AlreadyExists)
process1: rmdir # Ok(())
process2: is_dir(path) # false
process2 returns Err(ErrorKind::AlreadyExists)
