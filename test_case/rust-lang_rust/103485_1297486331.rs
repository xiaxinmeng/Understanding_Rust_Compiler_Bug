rust
        self.run(Command::new("git").args(&["stash", "save"]).current_dir(&absolute_path));
        self.run(Command::new("git").args(&["stash", "apply"]).current_dir(&absolute_path));
        self.run(Command::new("git").args(&["reset", "-q", "--hard"]).current_dir(&absolute_path));
        self.run(Command::new("git").args(&["clean", "-qdfx"]).current_dir(absolute_path));
