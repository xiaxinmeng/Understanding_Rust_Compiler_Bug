c
/* Unconditionally read all potential arguments.  This may pass
   garbage values to the kernel, but avoids the need for teaching
   glibc the argument counts of individual options (including ones
   that are added to the kernel in the future).  */
