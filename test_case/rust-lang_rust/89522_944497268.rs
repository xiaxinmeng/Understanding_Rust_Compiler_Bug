c
/* Call the clone syscall with fork semantic.  The CTID address is used
   to store the child thread ID at its locationm, to erase it in child memory
   when the child exits, and do a wakeup on the futex at that address.

   The architecture with non-default kernel abi semantic should correctlly
   override it with one of the supported calling convention (check generic
   kernel-features.h for the clone abi variants).  */
static inline pid_t
arch_fork (void *ctid)
{
  const int flags = CLONE_CHILD_SETTID | CLONE_CHILD_CLEARTID | SIGCHLD;
