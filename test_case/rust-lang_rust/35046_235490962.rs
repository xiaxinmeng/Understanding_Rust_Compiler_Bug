
       WEXITSTATUS(wstatus)
              returns the exit status of the child.  This consists of the
              least significant 8 bits of the wstatus argument that the
              child specified in a call to exit(3) or _exit(2) or as the
              argument for a return statement in main().  This macro should
              be employed only if WIFEXITED returned true.
