
breakpoint set --file 'borrowed-basic.rs' --line 158
DEBUG: breakpoint added, id = 1
Breakpoint 1: where = a`borrowed_basic::main::hb88626abe4f7b94a + 298 at borrowed-basic.rs:158:5, address = 0x0000000100003cba 
DEBUG: registering breakpoint callback, id = 1
Error while trying to register breakpoint callback, id = 1, message = error: could not get num args: can't find callable: breakpoint_callback

run
error: failed to get reply to handshake packet within timeout of 6.0 seconds

print *bool_ref
TIMEOUT: lldb_batchmode.py has been running for too long. Aborting!
