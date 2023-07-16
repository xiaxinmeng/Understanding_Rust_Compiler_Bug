
   Signal dispositions
       Each signal has a current disposition, which determines how the process behaves when it is delivered the signal.

       The entries in the "Action" column of the tables below specify the default disposition for each signal, as follows:

       Core   Default action is to terminate the process and dump core (see core(5)).


   Standard signals
       Linux supports the standard signals listed below.  Several signal numbers are architecture-dependent, as indicated in the "Value" column.  (Where three values are
       given,  the  first  one  is  usually valid for alpha and sparc, the middle one for x86, arm, and most other architectures, and the last one for mips.  (Values for
       parisc are not shown; see the Linux kernel source for signal numbering on that architecture.)  A dash (-) denotes that a signal is absent on the corresponding ar‐
       chitecture.

       First the signals described in the original POSIX.1-1990 standard.

       Signal     Value     Action   Comment
       ──────────────────────────────────────────────────────────────────────
SIGILL        4       Core    Illegal Instruction
