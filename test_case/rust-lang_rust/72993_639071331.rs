rust
        // We used to run `git ls-remote origin master`, but you still need that commit locally to
        // do things like `merge-base` or `rev-list`, so we might as well fetch it.
        run(Command::new("git").arg("fetch").arg("origin").arg("master").current_dir(&self.src));
