 C
// Define message type
enum Message { Add (str), Quit(task::pid), Show (task::pid) } 
fn main () {
  // spawn the new task with a "base string" value
  pid = task::spawn<Message> ({ || new ("base string"); });
  comm::send (pid, Add ("Hello"));
  comm::send (pid, Show (self)); // self = current pid
  let s : str = comm:recv (); // Main receive strings ? Maybe spawn an other task would be better
  std::io::println (#fmt ("%s", s));
  comm::send (pid, Quit(self));
}

fn new (string : str) {
  let msg : Message;
  msg = comm::recv ();
  alt msg {
    Add (x) { new (string + x); }
    Show (pid) { comm::send (pid, string); new (string); }
    Quit (pid) { comm::send (pid, "Quit"); }
  }
}
