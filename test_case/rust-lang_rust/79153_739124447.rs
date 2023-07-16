
// The main() functions in typical LLVM tools start with InitLLVM which does
// the following one-time initializations:
//
//  1. Setting up a signal handler so that pretty stack trace is printed out
//     if a process crashes. A signal handler that exits when a failed write to
//     a pipe occurs may optionally be installed: this is on-by-default.
//
//  2. Set up the global new-handler which is called when a memory allocation
//     attempt fails.
//
//  3. If running on Windows, obtain command line arguments using a
//     multibyte character-aware API and convert arguments into UTF-8
//     encoding, so that you can assume that command line arguments are
//     always encoded in UTF-8 on any platform.
//
// InitLLVM calls llvm_shutdown() on destruction, which cleans up
// ManagedStatic objects.
