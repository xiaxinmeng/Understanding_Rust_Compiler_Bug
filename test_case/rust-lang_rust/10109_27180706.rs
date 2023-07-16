 rust
enum ProcessExit {
   ExitOk, // status == 0, could be merged into ExitStatus.
   ExitStatus(int),
   ExitSignal(int)
}
