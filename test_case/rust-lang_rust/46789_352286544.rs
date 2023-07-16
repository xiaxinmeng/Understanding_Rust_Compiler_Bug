rust
// Is this visible to child process? I would hope so.
os::set_var("A", "A");

let mut command = Command::new("...");

// Is this visible? Basically arbitrary, but currently yes.
os::set_var("B", "B");

if cond() {
    // Set in child env but not parent process.
    command.env("C", "C");
}

// Is this visible? It seems obvious to me that the answer should be the same
// as for B. Currently the answer is maybe! Depends on cond(). Child process
// sees either C or D but not both.
os::set_var("D", "D");

command.spawn()
